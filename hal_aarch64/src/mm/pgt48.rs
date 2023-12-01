use hal_core::{
    mm::{self, PageAlloc, PageEntry, PageMap, Permissions},
    Error,
};

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::registers::{ReadOnly, ReadWrite};

register_bitfields! [u64,
    pub VAddrInner [
        BLOCK_OFFSET OFFSET(0) NUMBITS(12) [],
        LEVEL3_TABLE_IDX OFFSET(12) NUMBITS(9) [],
        LEVEL2_TABLE_IDX OFFSET(21) NUMBITS(9) [],
        LEVEL1_TABLE_IDX OFFSET(30) NUMBITS(9) [],
        LEVEL0_TABLE_IDX OFFSET(39) NUMBITS(9) [],
    ],

    pub TableEntryInner [
        TYPE OFFSET(0) NUMBITS(2) [
            TABLE_ENTRY = 0b11,
            INVALID_ENTRY = 0b00,
        ],

        INDX OFFSET(2) NUMBITS(2) [],

        AP OFFSET(6) NUMBITS(2) [
            U_NONE_K_RW = 0b00,
            U_RW_K_RW = 0b01,
            U_NONE_K_R = 0b10,
            U_R_K_R = 0b11,
        ],

        SH OFFSET(8) NUMBITS(2) [
            NON_SHAREABLE = 0b00,
            RESERVED = 0b01,
            OUTER_SHAREABLE = 0b10,
            INNER_SHAREABLE = 0b11,
        ],

        AF OFFSET(10) NUMBITS(1) [
            FALSE = 0b0,
            TRUE = 0b1,
        ],

        DEST OFFSET(12) NUMBITS(36) [],

        PXN OFFSET(53) NUMBITS(1) [],
        UXN OFFSET(54) NUMBITS(1) [],
    ],

    pub TableDescriptorInner [
        TYPE OFFSET(0) NUMBITS(2) [
            TABLE_DESCRIPTOR = 0b11,
            INVALID_ENTRY = 0b00,
        ],

        DEST OFFSET(12) NUMBITS(36) [],
    ],
];

struct VAddr(ReadOnly<u64, VAddrInner::Register>);

impl VAddr {
    fn get_level_offset(&self, level: u8) -> usize {
        let offset = match level {
            0 => self.0.read(VAddrInner::LEVEL0_TABLE_IDX),
            1 => self.0.read(VAddrInner::LEVEL1_TABLE_IDX),
            2 => self.0.read(VAddrInner::LEVEL2_TABLE_IDX),
            3 => self.0.read(VAddrInner::LEVEL3_TABLE_IDX),
            _ => panic!("There are only 4 levels"),
        };

        offset as usize
    }
}

impl From<mm::VAddr> for VAddr {
    fn from(paddr: mm::VAddr) -> Self {
        assert_eq!(usize::BITS, u64::BITS);
        let val = paddr.val as u64;
        Self(unsafe { core::mem::transmute::<u64, ReadOnly<u64, VAddrInner::Register>>(val) })
    }
}

pub struct PAddr(u64);

impl From<mm::PAddr> for PAddr {
    fn from(paddr: mm::PAddr) -> Self {
        assert_eq!(usize::BITS, u64::BITS);
        Self(paddr.val as u64)
    }
}

impl From<&PAddr> for u64 {
    fn from(paddr: &PAddr) -> Self {
        paddr.0
    }
}

struct TableDescriptor(ReadWrite<u64, TableDescriptorInner::Register>);

impl TableDescriptor {
    fn get_next_level(&mut self) -> &mut PageTable {
        let raw_pgt = (self.0.read(TableDescriptorInner::DEST) << 12) as *mut PageTable;

        // Safety: there is no conceivable way for us to know if this pointer is valid.
        // If the pointer in our pagetable are invalid, then we're lost...
        unsafe { raw_pgt.as_mut().unwrap() }
    }

    fn set_next_level(&mut self, next_level: &mut PageTable) {
        let next_level_addr = (next_level as *const PageTable) as u64;
        self.0
            .modify(TableDescriptorInner::DEST.val(next_level_addr >> 12));
        self.0.modify(TableDescriptorInner::TYPE::TABLE_DESCRIPTOR);
    }

    fn is_invalid(&self) -> bool {
        self.0.read(TableDescriptorInner::TYPE) == TableDescriptorInner::TYPE::INVALID_ENTRY.into()
    }

    fn set_invalid(&mut self) {
        self.0.write(TableDescriptorInner::TYPE::INVALID_ENTRY);
    }
}

pub struct TableEntry(ReadWrite<u64, TableEntryInner::Register>);

impl TableEntry {
    fn set_target(&mut self, addr: u64) {
        let field = TableEntryInner::DEST.val(addr >> 12);
        self.0.modify(TableEntryInner::TYPE::TABLE_ENTRY);
        self.0.modify(field);
    }

    fn set_permissions(&mut self, perms: mm::Permissions) {
        // TODO: Can we improve this?
        if perms.contains(mm::Permissions::USER) {
            if perms.contains(mm::Permissions::WRITE) {
                self.0.modify(TableEntryInner::AP::U_RW_K_RW);
            } else {
                self.0.modify(TableEntryInner::AP::U_R_K_R);
            }

            self.0.modify(TableEntryInner::PXN.val(1));
            if perms.contains(mm::Permissions::EXECUTE) {
                self.0.modify(TableEntryInner::UXN.val(0));
            } else {
                self.0.modify(TableEntryInner::UXN.val(1));
            }
        } else {
            if perms.contains(mm::Permissions::WRITE) {
                self.0.modify(TableEntryInner::AP::U_NONE_K_RW);
            } else {
                self.0.modify(TableEntryInner::AP::U_NONE_K_R);
            }

            self.0.modify(TableEntryInner::UXN.val(1));
            if perms.contains(mm::Permissions::EXECUTE) {
                self.0.modify(TableEntryInner::PXN.val(0));
            } else {
                self.0.modify(TableEntryInner::PXN.val(1));
            }
        }
    }

    fn set_mair_index(&mut self, index: usize) {
        // MAIR can store only 8 attributes
        assert!(index < 8);

        self.0.modify(TableEntryInner::INDX.val(index as u64));
    }

    fn set_shareable(&mut self) {
        self.0.modify(TableEntryInner::SH::INNER_SHAREABLE);
    }

    fn set_access_flag(&mut self) {
        self.0.modify(TableEntryInner::AF::TRUE);
    }
}

impl PageEntry for TableEntry {
    fn set_invalid(&mut self) {
        self.0.write(TableEntryInner::TYPE::INVALID_ENTRY);
    }
}

/// Depending on the level of the pagetable walk, the actual data (u64) needs to be interpreted
/// differently: Descriptor for levels 0 to 2 and Entry for level 3
union PageTableContent {
    descriptor: core::mem::ManuallyDrop<TableDescriptor>,
    entry: core::mem::ManuallyDrop<TableEntry>,
}

impl PageTableContent {
    /// With Aarch64 Pgt48OA, if the first two bits are set to 0b00, entry/descriptor is invalid.
    const fn new_invalid() -> Self {
        unsafe { core::mem::transmute::<u64, Self>(0b00u64) }
    }
}

#[repr(align(0x1000))]
pub struct PageTable {
    entries: [PageTableContent; 512],
}

impl PageTable {
    pub const fn zeroed() -> Self {
        #[allow(clippy::uninit_assumed_init)]
        let mut entries: [PageTableContent; 512] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        while i < 512 {
            entries[i] = PageTableContent::new_invalid();
            i += 1;
        }
        Self { entries }
    }
}

impl PageMap for PageTable {
    const PAGE_SIZE: usize = 4096;
    type Entry = TableEntry;

    fn new(allocator: &impl PageAlloc) -> Result<&'static mut Self, Error> {
        let page = allocator.alloc(1)?;
        let page_table = page as *mut PageTable;
        // Safety: the PMM gave us the memory, it should be a valid pointer.
        let page_table: &mut PageTable = unsafe { page_table.as_mut().unwrap() };

        page_table
            .entries
            .iter_mut()
            .for_each(|content| unsafe { &mut content.descriptor }.set_invalid());

        Ok(page_table)
    }

    fn map(
        &mut self,
        va: mm::VAddr,
        pa: mm::PAddr,
        perms: Permissions,
        allocator: &impl PageAlloc,
    ) -> Result<&mut TableEntry, Error> {
        let va = VAddr::from(va);
        let pa = PAddr::from(pa);
        let mut pagetable = self;

        for lvl in 0..=3 {
            let offset = va.get_level_offset(lvl);
            let content = &mut pagetable.entries[offset];

            if lvl == 3 {
                let entry = unsafe { &mut content.entry };
                entry.set_target(u64::from(&pa));
                entry.set_permissions(perms);
                entry.set_mair_index(0);
                entry.set_shareable();
                entry.set_access_flag();

                return Ok(entry);
            }

            let descriptor = unsafe { &mut content.descriptor };
            if descriptor.is_invalid() {
                let new_page_table = PageTable::new(allocator)?;
                descriptor.set_next_level(new_page_table);
            }

            pagetable = descriptor.get_next_level();
        }

        unreachable!("We should have reached lvl 3 and returned by now...");
    }

    fn add_invalid_entry(
        &mut self,
        va: mm::VAddr,
        allocator: &impl PageAlloc,
    ) -> Result<(), Error> {
        let entry = self.map(
            va,
            mm::PAddr {
                val: 0x0A0A_0A0A_0A0A_0A0A,
            },
            mm::Permissions::READ,
            allocator,
        )?;

        entry.set_invalid();

        Ok(())
    }
}
