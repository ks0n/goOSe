use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite};
use tock_registers::register_bitfields;

pub struct GicV2 {
    pub distributor: &'static GicDistributor,
    pub cpu: &'static GicCpu,
}

impl GicV2 {
    pub fn new(distributor_base: usize, cpu_base: usize) -> Self {
        let distributor = unsafe { (distributor_base as *const GicDistributor).as_ref().unwrap() };
        let cpu = unsafe { (cpu_base as *const GicCpu).as_ref().unwrap() };
        let mut gic = Self { distributor, cpu };

        gic.init_distributor();

        gic
    }

    pub fn disable_interrupts(&mut self) {
        self.distributor
            .CTLR
            .modify(GICD_CTLR::EnableGrp0::Disable + GICD_CTLR::EnableGrp1::Disable);
    }

    pub fn enable_interrupts(&mut self) {
        self.distributor
            .CTLR
            .modify(GICD_CTLR::EnableGrp0::Enable + GICD_CTLR::EnableGrp1::Enable);
    }

    pub fn nlines(&self) -> usize {
        let n = self.distributor.TYPER.read(GICD_TYPER::ITLinesNumber) as usize;

        32 * (n + 1)
    }

    /// Put the Gic in a known state.
    fn init_distributor(&mut self) {
        self.disable_interrupts();

        for i in 0..(self.nlines() / 32) {
            // Disable all interrupts.
            // Each bit corresponds to a line, writing 1 disables forwarding of the corresponding
            // interrupt line.
            self.distributor.ICENABLER[i].set(0xffff_ffff);

            // Clear pending interrupts.
            self.distributor.ICPENDR[i].set(0xffff_ffff);

            // Clear active interrupts.
            self.distributor.ICACTIVER[i].set(0xffff_ffff);
        }

        for i in 0..(self.nlines() / 4) {
            // Targets all interrupts to core 0.
            self.distributor.ITARGETSR[i].set(0x0101_0101);
        }

        for i in 1..(self.nlines() / 16) {
            // Set all interrupts to level-triggered.
            self.distributor.ICFGR[i].set(0);
        }

        // TODO: this should be moved somewhere else so other cores can run it.
        self.init_cpu();
    }

    fn init_cpu(&self) {
        // Accept ALL interrupts.
        self.cpu.PMR.set(0xff);

        // Set maximum amount of bits to be used for Group priority field.
        self.cpu.BPR.set(0x0);

        self.cpu.CTLR.write(GICC_CTLR::EnableGrp0::Enable +
            GICC_CTLR::EnableGrp1::Enable +
            GICC_CTLR::FIQEn.val(0));

    }

    pub fn enable(&mut self, line: usize) {
        let enable_reg_index = line >> 5;
        let enable_bit: u32 = 1u32 << (line % 32);

        self.distributor.ISENABLER[enable_reg_index].set(
            self.distributor.ISENABLER[enable_reg_index].get() | enable_bit);
        self.distributor.IPRIORITYR[line].set(0x80);
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GicDistributor {
    /// Distributor Control Register
    pub CTLR: ReadWrite<u32, GICD_CTLR::Register>,
    /// Interrupt Controller Type Register
    pub TYPER: ReadOnly<u32, GICD_TYPER::Register>,
    /// Distributor Implementer Identification Register
    pub IIDR: ReadOnly<u32>,
    _reserved1: [u32; 5],
    _impdef1: [u32; 8],
    _reserved2: [u32; 16],
    /// Interrupt Group Registers
    pub IGROUPR: [ReadWrite<u32>; 32],
    // _reserved3: [u32; 31],
    /// Interrupt Set-Enable Registers
    pub ISENABLER: [ReadWrite<u32>; 32],
    /// Interrupt Clear-Enable Registers
    pub ICENABLER: [ReadWrite<u32>; 32],
    /// Interrupt Set-Pending Registers
    pub ISPENDR: [ReadWrite<u32>; 32],
    /// Interrupt Clear-Pending Registers
    pub ICPENDR: [ReadWrite<u32>; 32],
    /// Interrupt Set-Active Registers
    pub ISACTIVER: [ReadWrite<u32>; 32],
    /// Interrupt Clear-Active Registers
    pub ICACTIVER: [ReadWrite<u32>; 32],
    /// Interrupt Priority Registers
    pub IPRIORITYR: [ReadWrite<u8>; 1024],
    /// Interrupt Processor Targets Registers
    pub ITARGETSR: [ReadWrite<u32>; 255],
    _reserved5: u32,
    /// Interrupt Configuration Registers
    pub ICFGR: [ReadWrite<u32>; 64],
    _implementation_defined2: [u32; 64],
    /// Non-secure Access Control Registers, optional
    pub NSACR: [ReadWrite<u32>; 64],
    /// Software Generated Interrupt Register
    pub SGIR: ReadWrite<u32>,
    _reserved6: [u32; 3],
    /// SGI Clear-Pending Registers
    pub CPENDSGIR: [ReadWrite<u8>; 16],
    /// SGI Set-Pending Registers
    pub SPENDSGIR: [ReadWrite<u8>; 16],
    _reserved7: [u32; 40],
    _impdef3: [u32; 12],
}

register_bitfields! {u32,
    pub GICD_CTLR [
        EnableGrp0 OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        EnableGrp1 OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ]
    ],

    pub GICD_TYPER [
        ITLinesNumber OFFSET(0) NUMBITS(5) [],
        CPUNumber OFFSET(5) NUMBITS(3) [],
        SecurityExtn OFFSET(10) NUMBITS(1) [
            NotImplemented = 0,
            Implemented = 1,
        ],
        LSPI OFFSET(11) NUMBITS(5) [],
    ],

    pub GICD_IIDR [
        Implementer OFFSET(0) NUMBITS(12),
        Revision OFFSET(12) NUMBITS(4),
        Variant OFFSET(16) NUMBITS(4),
        ProductID OFFSET(24) NUMBITS(8),
    ],

    pub GICD_SGIR [
        SGIINTID OFFSET(0) NUMBITS(4) [],
        NSATT OFFSET(15) NUMBITS(1) [],
        CPUTargetList OFFSET(16) NUMBITS(8) [],
        TargetListFilter OFFSET(24) NUMBITS(2) [],
    ],

    pub GICD_ICPIDR2 [
        ArchRev OFFSET(4) NUMBITS(4),
    ],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GicCpu {
        /// CPU Interface Control Register
    pub CTLR: ReadWrite<u32, GICC_CTLR::Register>,
    /// Interrupt Priority Mask Register
    pub PMR: ReadWrite<u32>,
    /// Binary Point Register
    pub BPR: ReadWrite<u32>,
    /// Interrupt Acknowledge Register
    pub IAR: ReadWrite<u32>,
    /// End of Interrupt Register
    pub EOIR: ReadWrite<u32>,
    /// Running Priority Register
    pub RPR: ReadWrite<u32>,
    /// Highest Priority Pending Interrupt Register
    pub HPPIR: ReadWrite<u32>,
    /// Aliased Binary Point Register
    pub ABPR: ReadWrite<u32>,
    /// Aliased Interrupt Acknowledge Register
    pub AIAR: ReadWrite<u32>,
    /// Aliased End of Interrupt Register
    pub AEOIR: ReadWrite<u32>,
    /// Aliased Highest Priority Pending Interrupt Register
    pub AHPPIR: ReadWrite<u32>,
}

register_bitfields! {u32,
    pub GICC_CTLR [
        EnableGrp0 OFFSET(0) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        EnableGrp1 OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        FIQEn OFFSET(3) NUMBITS(1) [],
        FIQBypDisGrp1 OFFSET(5) NUMBITS(1) [],
        IRQBypDisGrp1 OFFSET(6) NUMBITS(1) [],
        EOImodeNS OFFSET(9) NUMBITS(1) [
            BothPriorityDropAndDeactiveInterrupt = 0,
            PriorityDropOnly = 1,
        ],
    ],

    pub GICC_PMR [
        Priority OFFSET(0) NUMBITS(8) [],
    ],

    pub GICC_IAR [
        InterruptID OFFSET(0) NUMBITS(10) [],
    ],

    pub GICC_EOIR [
        EOIINTID OFFSET(0) NUMBITS(10) []
    ],

    pub GICC_AHPPIR [
        PENDINTID OFFSET(0) NUMBITS(10) [],
        CPUID OFFSET(10) NUMBITS(3) [],
    ],
}
