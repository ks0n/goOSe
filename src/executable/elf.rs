use core::arch::asm;
use core::iter::Iterator;

use crate::mm;
use crate::mm::PageAllocator;

use goblin;
use goblin::elf::header::header64::Header;
use goblin::elf::program_header::program_header64::ProgramHeader;
use goblin::elf::program_header::*;

pub struct Elf<'a> {
    load_addr: usize,
    data: &'a [u8],
}

impl<'a> Elf<'a> {
    /// Create a new Elf struct from a byte slice
    pub fn from_bytes(data: &'a [u8]) -> Self {
        Self {
            load_addr: data.as_ptr() as usize,
            data,
        }
    }

    /// Get the header struct of an ELF file
    fn header(&self) -> &Header {
        let header_slice = self.data[..64].try_into().unwrap();

        Header::from_bytes(header_slice)
    }

    /// Get an iterator over all the segment of an ELF file
    fn segments(&self) -> impl Iterator<Item = &ProgramHeader> + '_ {
        let header = self.header();

        (0..header.e_phnum)
            .map(|n| {
                self.load_addr
                    + header.e_phoff as usize
                    + (n as usize * header.e_phentsize as usize)
            })
            .map(|addr| unsafe { &(*(addr as *const ProgramHeader)) })
    }

    pub fn execute(&self) {
        let addr = self.header().e_entry;

        unsafe {
            asm!("jalr {}", in(reg) addr);
        }
    }

    fn pages_needed(
        segment: &goblin::elf64::program_header::ProgramHeader,
        mm: &mut dyn mm::MemoryManager,
    ) -> usize {
        let p_memsz = segment.p_memsz as usize;

        if p_memsz < mm.page_size() {
            1
        } else {
            p_memsz / mm.page_size()
        }
    }

    pub fn map(&self, mm: &mut dyn mm::MemoryManager) {
        let page_size = mm.page_size();

        for segment in self.segments() {
            if segment.p_type != PT_LOAD {
                continue;
            }
            let p_offset = segment.p_offset as usize;
            let p_filesz = segment.p_filesz as usize;
            let p_memsz = segment.p_memsz as usize;

            let pages_needed = Self::pages_needed(segment, mm);
            let physical_pages = mm::get_global_allocator()
                .lock()
                .alloc_pages(pages_needed)
                .unwrap();
            let virtual_pages = segment.p_paddr as *mut u8;

            let segment_data_src_addr = (self.load_addr + p_offset) as *const u8;
            let segment_data_dst_addr = (usize::from(physical_pages) + p_offset) as *mut u8;

            let segment_data_src: &[u8] =
                unsafe { core::slice::from_raw_parts(segment_data_src_addr, p_filesz) };
            let segment_data_dst: &mut [u8] = {
                let dst =
                    unsafe { core::slice::from_raw_parts_mut(segment_data_dst_addr, p_memsz) };

                // Zeroing uninitialized data
                for i in p_filesz..p_memsz {
                    dst[i as usize] = 0u8;
                }

                dst
            };

            segment_data_dst[0..p_filesz].clone_from_slice(segment_data_src);

            let perms = elf_to_mm_permissions(segment.p_flags);

            for i in 0..pages_needed {
                let page_offset = i * page_size;
                mm.map(
                    mm::PAddr::from(usize::from(physical_pages) + page_offset),
                    mm::VAddr::from(mm.align_down(virtual_pages as usize) + page_offset),
                    perms,
                );
            }
        }
    }

    fn entry_point(&self) -> usize {
        self.header().e_entry as usize
    }
}

/// Check if the bytes represent an ELF file
pub fn is_valid(data: &[u8]) -> bool {
    let elf_signature = [0x7f, b'E', b'L', b'F'];

    for i in 0..elf_signature.len() {
        if data[i] != elf_signature[i] {
            return false;
        }
    }

    true
}

/// Convert ELF p_flags permissions to mm::Permissions
fn elf_to_mm_permissions(elf_permsission: u32) -> mm::Permissions {
    let mut perms = mm::Permissions::empty();

    if elf_permsission & PF_R != 0 {
        perms |= mm::Permissions::READ;
    }

    if elf_permsission & PF_W != 0 {
        perms |= mm::Permissions::WRITE;
    }

    if elf_permsission & PF_X != 0 {
        perms |= mm::Permissions::EXECUTE;
    }

    perms
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_tests::*;
    use crate::mm::MemoryManager;

    static ELF_BYTES: &[u8] = core::include_bytes!("../../fixtures/small");

    #[test_case]
    fn elf_is_valid_fake(_ctx: &mut TestContext) {
        let fake_elf = [0, 1, 2, 3, 4, 5, 6, 7];
        assert!(!is_valid(&fake_elf));
    }

    #[test_case]
    fn elf_is_valid_real(_ctx: &mut TestContext) {
        assert!(is_valid(ELF_BYTES));
    }

    #[test_case]
    fn elf_load_and_execute_clean(ctx: &mut TestContext) {
        ctx.reset();

        let elf = Elf::from_bytes(ELF_BYTES);

        (&elf).map(&mut ctx.memory);
        ctx.memory.reload_page_table();
        elf.execute();

        let mut res: usize;
        unsafe { asm!("mv {}, a0", out(reg) res) };

        assert!(res == 42);
    }
}
