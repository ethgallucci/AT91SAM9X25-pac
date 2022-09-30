#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC Global Configuration Register"]
    pub gcfg: GCFG,
    #[doc = "0x04 - DMAC Enable Register"]
    pub en: EN,
    #[doc = "0x08 - DMAC Software Single Request Register"]
    pub sreq: SREQ,
    #[doc = "0x0c - DMAC Software Chunk Transfer Request Register"]
    pub creq: CREQ,
    #[doc = "0x10 - DMAC Software Last Transfer Flag Register"]
    pub last: LAST,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
    pub ebcier: EBCIER,
    #[doc = "0x1c - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
    pub ebcidr: EBCIDR,
    #[doc = "0x20 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
    pub ebcimr: EBCIMR,
    #[doc = "0x24 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
    pub ebcisr: EBCISR,
    #[doc = "0x28 - DMAC Channel Handler Enable Register"]
    pub cher: CHER,
    #[doc = "0x2c - DMAC Channel Handler Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x30 - DMAC Channel Handler Status Register"]
    pub chsr: CHSR,
    _reserved12: [u8; 0x08],
    #[doc = "0x3c - DMAC Channel Source Address Register (ch_num = 0)"]
    pub saddr0: SADDR0,
    #[doc = "0x40 - DMAC Channel Destination Address Register (ch_num = 0)"]
    pub daddr0: DADDR0,
    #[doc = "0x44 - DMAC Channel Descriptor Address Register (ch_num = 0)"]
    pub dscr0: DSCR0,
    #[doc = "0x48 - DMAC Channel Control A Register (ch_num = 0)"]
    pub ctrla0: CTRLA0,
    #[doc = "0x4c - DMAC Channel Control B Register (ch_num = 0)"]
    pub ctrlb0: CTRLB0,
    #[doc = "0x50 - DMAC Channel Configuration Register (ch_num = 0)"]
    pub cfg0: CFG0,
    #[doc = "0x54 - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 0)"]
    pub spip0: SPIP0,
    #[doc = "0x58 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 0)"]
    pub dpip0: DPIP0,
    _reserved20: [u8; 0x08],
    #[doc = "0x64 - DMAC Channel Source Address Register (ch_num = 1)"]
    pub saddr1: SADDR1,
    #[doc = "0x68 - DMAC Channel Destination Address Register (ch_num = 1)"]
    pub daddr1: DADDR1,
    #[doc = "0x6c - DMAC Channel Descriptor Address Register (ch_num = 1)"]
    pub dscr1: DSCR1,
    #[doc = "0x70 - DMAC Channel Control A Register (ch_num = 1)"]
    pub ctrla1: CTRLA1,
    #[doc = "0x74 - DMAC Channel Control B Register (ch_num = 1)"]
    pub ctrlb1: CTRLB1,
    #[doc = "0x78 - DMAC Channel Configuration Register (ch_num = 1)"]
    pub cfg1: CFG1,
    #[doc = "0x7c - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 1)"]
    pub spip1: SPIP1,
    #[doc = "0x80 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 1)"]
    pub dpip1: DPIP1,
    _reserved28: [u8; 0x08],
    #[doc = "0x8c - DMAC Channel Source Address Register (ch_num = 2)"]
    pub saddr2: SADDR2,
    #[doc = "0x90 - DMAC Channel Destination Address Register (ch_num = 2)"]
    pub daddr2: DADDR2,
    #[doc = "0x94 - DMAC Channel Descriptor Address Register (ch_num = 2)"]
    pub dscr2: DSCR2,
    #[doc = "0x98 - DMAC Channel Control A Register (ch_num = 2)"]
    pub ctrla2: CTRLA2,
    #[doc = "0x9c - DMAC Channel Control B Register (ch_num = 2)"]
    pub ctrlb2: CTRLB2,
    #[doc = "0xa0 - DMAC Channel Configuration Register (ch_num = 2)"]
    pub cfg2: CFG2,
    #[doc = "0xa4 - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 2)"]
    pub spip2: SPIP2,
    #[doc = "0xa8 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 2)"]
    pub dpip2: DPIP2,
    _reserved36: [u8; 0x08],
    #[doc = "0xb4 - DMAC Channel Source Address Register (ch_num = 3)"]
    pub saddr3: SADDR3,
    #[doc = "0xb8 - DMAC Channel Destination Address Register (ch_num = 3)"]
    pub daddr3: DADDR3,
    #[doc = "0xbc - DMAC Channel Descriptor Address Register (ch_num = 3)"]
    pub dscr3: DSCR3,
    #[doc = "0xc0 - DMAC Channel Control A Register (ch_num = 3)"]
    pub ctrla3: CTRLA3,
    #[doc = "0xc4 - DMAC Channel Control B Register (ch_num = 3)"]
    pub ctrlb3: CTRLB3,
    #[doc = "0xc8 - DMAC Channel Configuration Register (ch_num = 3)"]
    pub cfg3: CFG3,
    #[doc = "0xcc - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 3)"]
    pub spip3: SPIP3,
    #[doc = "0xd0 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 3)"]
    pub dpip3: DPIP3,
    _reserved44: [u8; 0x08],
    #[doc = "0xdc - DMAC Channel Source Address Register (ch_num = 4)"]
    pub saddr4: SADDR4,
    #[doc = "0xe0 - DMAC Channel Destination Address Register (ch_num = 4)"]
    pub daddr4: DADDR4,
    #[doc = "0xe4 - DMAC Channel Descriptor Address Register (ch_num = 4)"]
    pub dscr4: DSCR4,
    #[doc = "0xe8 - DMAC Channel Control A Register (ch_num = 4)"]
    pub ctrla4: CTRLA4,
    #[doc = "0xec - DMAC Channel Control B Register (ch_num = 4)"]
    pub ctrlb4: CTRLB4,
    #[doc = "0xf0 - DMAC Channel Configuration Register (ch_num = 4)"]
    pub cfg4: CFG4,
    #[doc = "0xf4 - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 4)"]
    pub spip4: SPIP4,
    #[doc = "0xf8 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 4)"]
    pub dpip4: DPIP4,
    _reserved52: [u8; 0x08],
    #[doc = "0x104 - DMAC Channel Source Address Register (ch_num = 5)"]
    pub saddr5: SADDR5,
    #[doc = "0x108 - DMAC Channel Destination Address Register (ch_num = 5)"]
    pub daddr5: DADDR5,
    #[doc = "0x10c - DMAC Channel Descriptor Address Register (ch_num = 5)"]
    pub dscr5: DSCR5,
    #[doc = "0x110 - DMAC Channel Control A Register (ch_num = 5)"]
    pub ctrla5: CTRLA5,
    #[doc = "0x114 - DMAC Channel Control B Register (ch_num = 5)"]
    pub ctrlb5: CTRLB5,
    #[doc = "0x118 - DMAC Channel Configuration Register (ch_num = 5)"]
    pub cfg5: CFG5,
    #[doc = "0x11c - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 5)"]
    pub spip5: SPIP5,
    #[doc = "0x120 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 5)"]
    pub dpip5: DPIP5,
    _reserved60: [u8; 0x08],
    #[doc = "0x12c - DMAC Channel Source Address Register (ch_num = 6)"]
    pub saddr6: SADDR6,
    #[doc = "0x130 - DMAC Channel Destination Address Register (ch_num = 6)"]
    pub daddr6: DADDR6,
    #[doc = "0x134 - DMAC Channel Descriptor Address Register (ch_num = 6)"]
    pub dscr6: DSCR6,
    #[doc = "0x138 - DMAC Channel Control A Register (ch_num = 6)"]
    pub ctrla6: CTRLA6,
    #[doc = "0x13c - DMAC Channel Control B Register (ch_num = 6)"]
    pub ctrlb6: CTRLB6,
    #[doc = "0x140 - DMAC Channel Configuration Register (ch_num = 6)"]
    pub cfg6: CFG6,
    #[doc = "0x144 - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 6)"]
    pub spip6: SPIP6,
    #[doc = "0x148 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 6)"]
    pub dpip6: DPIP6,
    _reserved68: [u8; 0x08],
    #[doc = "0x154 - DMAC Channel Source Address Register (ch_num = 7)"]
    pub saddr7: SADDR7,
    #[doc = "0x158 - DMAC Channel Destination Address Register (ch_num = 7)"]
    pub daddr7: DADDR7,
    #[doc = "0x15c - DMAC Channel Descriptor Address Register (ch_num = 7)"]
    pub dscr7: DSCR7,
    #[doc = "0x160 - DMAC Channel Control A Register (ch_num = 7)"]
    pub ctrla7: CTRLA7,
    #[doc = "0x164 - DMAC Channel Control B Register (ch_num = 7)"]
    pub ctrlb7: CTRLB7,
    #[doc = "0x168 - DMAC Channel Configuration Register (ch_num = 7)"]
    pub cfg7: CFG7,
    #[doc = "0x16c - DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 7)"]
    pub spip7: SPIP7,
    #[doc = "0x170 - DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 7)"]
    pub dpip7: DPIP7,
    _reserved76: [u8; 0x70],
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "GCFG (rw) register accessor: an alias for `Reg<GCFG_SPEC>`"]
pub type GCFG = crate::Reg<gcfg::GCFG_SPEC>;
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "SREQ (rw) register accessor: an alias for `Reg<SREQ_SPEC>`"]
pub type SREQ = crate::Reg<sreq::SREQ_SPEC>;
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "CREQ (rw) register accessor: an alias for `Reg<CREQ_SPEC>`"]
pub type CREQ = crate::Reg<creq::CREQ_SPEC>;
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "LAST (rw) register accessor: an alias for `Reg<LAST_SPEC>`"]
pub type LAST = crate::Reg<last::LAST_SPEC>;
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "EBCIER (w) register accessor: an alias for `Reg<EBCIER_SPEC>`"]
pub type EBCIER = crate::Reg<ebcier::EBCIER_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "EBCIDR (w) register accessor: an alias for `Reg<EBCIDR_SPEC>`"]
pub type EBCIDR = crate::Reg<ebcidr::EBCIDR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "EBCIMR (r) register accessor: an alias for `Reg<EBCIMR_SPEC>`"]
pub type EBCIMR = crate::Reg<ebcimr::EBCIMR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "EBCISR (r) register accessor: an alias for `Reg<EBCISR_SPEC>`"]
pub type EBCISR = crate::Reg<ebcisr::EBCISR_SPEC>;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "CHER (w) register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "SADDR0 (rw) register accessor: an alias for `Reg<SADDR0_SPEC>`"]
pub type SADDR0 = crate::Reg<saddr0::SADDR0_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr0;
#[doc = "DADDR0 (rw) register accessor: an alias for `Reg<DADDR0_SPEC>`"]
pub type DADDR0 = crate::Reg<daddr0::DADDR0_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr0;
#[doc = "DSCR0 (rw) register accessor: an alias for `Reg<DSCR0_SPEC>`"]
pub type DSCR0 = crate::Reg<dscr0::DSCR0_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr0;
#[doc = "CTRLA0 (rw) register accessor: an alias for `Reg<CTRLA0_SPEC>`"]
pub type CTRLA0 = crate::Reg<ctrla0::CTRLA0_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla0;
#[doc = "CTRLB0 (rw) register accessor: an alias for `Reg<CTRLB0_SPEC>`"]
pub type CTRLB0 = crate::Reg<ctrlb0::CTRLB0_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb0;
#[doc = "CFG0 (rw) register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg0;
#[doc = "SPIP0 (rw) register accessor: an alias for `Reg<SPIP0_SPEC>`"]
pub type SPIP0 = crate::Reg<spip0::SPIP0_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 0)"]
pub mod spip0;
#[doc = "DPIP0 (rw) register accessor: an alias for `Reg<DPIP0_SPEC>`"]
pub type DPIP0 = crate::Reg<dpip0::DPIP0_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 0)"]
pub mod dpip0;
#[doc = "SADDR1 (rw) register accessor: an alias for `Reg<SADDR1_SPEC>`"]
pub type SADDR1 = crate::Reg<saddr1::SADDR1_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub mod saddr1;
#[doc = "DADDR1 (rw) register accessor: an alias for `Reg<DADDR1_SPEC>`"]
pub type DADDR1 = crate::Reg<daddr1::DADDR1_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub mod daddr1;
#[doc = "DSCR1 (rw) register accessor: an alias for `Reg<DSCR1_SPEC>`"]
pub type DSCR1 = crate::Reg<dscr1::DSCR1_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub mod dscr1;
#[doc = "CTRLA1 (rw) register accessor: an alias for `Reg<CTRLA1_SPEC>`"]
pub type CTRLA1 = crate::Reg<ctrla1::CTRLA1_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub mod ctrla1;
#[doc = "CTRLB1 (rw) register accessor: an alias for `Reg<CTRLB1_SPEC>`"]
pub type CTRLB1 = crate::Reg<ctrlb1::CTRLB1_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub mod ctrlb1;
#[doc = "CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub mod cfg1;
#[doc = "SPIP1 (rw) register accessor: an alias for `Reg<SPIP1_SPEC>`"]
pub type SPIP1 = crate::Reg<spip1::SPIP1_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 1)"]
pub mod spip1;
#[doc = "DPIP1 (rw) register accessor: an alias for `Reg<DPIP1_SPEC>`"]
pub type DPIP1 = crate::Reg<dpip1::DPIP1_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 1)"]
pub mod dpip1;
#[doc = "SADDR2 (rw) register accessor: an alias for `Reg<SADDR2_SPEC>`"]
pub type SADDR2 = crate::Reg<saddr2::SADDR2_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub mod saddr2;
#[doc = "DADDR2 (rw) register accessor: an alias for `Reg<DADDR2_SPEC>`"]
pub type DADDR2 = crate::Reg<daddr2::DADDR2_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub mod daddr2;
#[doc = "DSCR2 (rw) register accessor: an alias for `Reg<DSCR2_SPEC>`"]
pub type DSCR2 = crate::Reg<dscr2::DSCR2_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub mod dscr2;
#[doc = "CTRLA2 (rw) register accessor: an alias for `Reg<CTRLA2_SPEC>`"]
pub type CTRLA2 = crate::Reg<ctrla2::CTRLA2_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub mod ctrla2;
#[doc = "CTRLB2 (rw) register accessor: an alias for `Reg<CTRLB2_SPEC>`"]
pub type CTRLB2 = crate::Reg<ctrlb2::CTRLB2_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub mod ctrlb2;
#[doc = "CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub mod cfg2;
#[doc = "SPIP2 (rw) register accessor: an alias for `Reg<SPIP2_SPEC>`"]
pub type SPIP2 = crate::Reg<spip2::SPIP2_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 2)"]
pub mod spip2;
#[doc = "DPIP2 (rw) register accessor: an alias for `Reg<DPIP2_SPEC>`"]
pub type DPIP2 = crate::Reg<dpip2::DPIP2_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 2)"]
pub mod dpip2;
#[doc = "SADDR3 (rw) register accessor: an alias for `Reg<SADDR3_SPEC>`"]
pub type SADDR3 = crate::Reg<saddr3::SADDR3_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub mod saddr3;
#[doc = "DADDR3 (rw) register accessor: an alias for `Reg<DADDR3_SPEC>`"]
pub type DADDR3 = crate::Reg<daddr3::DADDR3_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub mod daddr3;
#[doc = "DSCR3 (rw) register accessor: an alias for `Reg<DSCR3_SPEC>`"]
pub type DSCR3 = crate::Reg<dscr3::DSCR3_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub mod dscr3;
#[doc = "CTRLA3 (rw) register accessor: an alias for `Reg<CTRLA3_SPEC>`"]
pub type CTRLA3 = crate::Reg<ctrla3::CTRLA3_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub mod ctrla3;
#[doc = "CTRLB3 (rw) register accessor: an alias for `Reg<CTRLB3_SPEC>`"]
pub type CTRLB3 = crate::Reg<ctrlb3::CTRLB3_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub mod ctrlb3;
#[doc = "CFG3 (rw) register accessor: an alias for `Reg<CFG3_SPEC>`"]
pub type CFG3 = crate::Reg<cfg3::CFG3_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub mod cfg3;
#[doc = "SPIP3 (rw) register accessor: an alias for `Reg<SPIP3_SPEC>`"]
pub type SPIP3 = crate::Reg<spip3::SPIP3_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 3)"]
pub mod spip3;
#[doc = "DPIP3 (rw) register accessor: an alias for `Reg<DPIP3_SPEC>`"]
pub type DPIP3 = crate::Reg<dpip3::DPIP3_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 3)"]
pub mod dpip3;
#[doc = "SADDR4 (rw) register accessor: an alias for `Reg<SADDR4_SPEC>`"]
pub type SADDR4 = crate::Reg<saddr4::SADDR4_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 4)"]
pub mod saddr4;
#[doc = "DADDR4 (rw) register accessor: an alias for `Reg<DADDR4_SPEC>`"]
pub type DADDR4 = crate::Reg<daddr4::DADDR4_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)"]
pub mod daddr4;
#[doc = "DSCR4 (rw) register accessor: an alias for `Reg<DSCR4_SPEC>`"]
pub type DSCR4 = crate::Reg<dscr4::DSCR4_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)"]
pub mod dscr4;
#[doc = "CTRLA4 (rw) register accessor: an alias for `Reg<CTRLA4_SPEC>`"]
pub type CTRLA4 = crate::Reg<ctrla4::CTRLA4_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 4)"]
pub mod ctrla4;
#[doc = "CTRLB4 (rw) register accessor: an alias for `Reg<CTRLB4_SPEC>`"]
pub type CTRLB4 = crate::Reg<ctrlb4::CTRLB4_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 4)"]
pub mod ctrlb4;
#[doc = "CFG4 (rw) register accessor: an alias for `Reg<CFG4_SPEC>`"]
pub type CFG4 = crate::Reg<cfg4::CFG4_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 4)"]
pub mod cfg4;
#[doc = "SPIP4 (rw) register accessor: an alias for `Reg<SPIP4_SPEC>`"]
pub type SPIP4 = crate::Reg<spip4::SPIP4_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 4)"]
pub mod spip4;
#[doc = "DPIP4 (rw) register accessor: an alias for `Reg<DPIP4_SPEC>`"]
pub type DPIP4 = crate::Reg<dpip4::DPIP4_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 4)"]
pub mod dpip4;
#[doc = "SADDR5 (rw) register accessor: an alias for `Reg<SADDR5_SPEC>`"]
pub type SADDR5 = crate::Reg<saddr5::SADDR5_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 5)"]
pub mod saddr5;
#[doc = "DADDR5 (rw) register accessor: an alias for `Reg<DADDR5_SPEC>`"]
pub type DADDR5 = crate::Reg<daddr5::DADDR5_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 5)"]
pub mod daddr5;
#[doc = "DSCR5 (rw) register accessor: an alias for `Reg<DSCR5_SPEC>`"]
pub type DSCR5 = crate::Reg<dscr5::DSCR5_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)"]
pub mod dscr5;
#[doc = "CTRLA5 (rw) register accessor: an alias for `Reg<CTRLA5_SPEC>`"]
pub type CTRLA5 = crate::Reg<ctrla5::CTRLA5_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 5)"]
pub mod ctrla5;
#[doc = "CTRLB5 (rw) register accessor: an alias for `Reg<CTRLB5_SPEC>`"]
pub type CTRLB5 = crate::Reg<ctrlb5::CTRLB5_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 5)"]
pub mod ctrlb5;
#[doc = "CFG5 (rw) register accessor: an alias for `Reg<CFG5_SPEC>`"]
pub type CFG5 = crate::Reg<cfg5::CFG5_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 5)"]
pub mod cfg5;
#[doc = "SPIP5 (rw) register accessor: an alias for `Reg<SPIP5_SPEC>`"]
pub type SPIP5 = crate::Reg<spip5::SPIP5_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 5)"]
pub mod spip5;
#[doc = "DPIP5 (rw) register accessor: an alias for `Reg<DPIP5_SPEC>`"]
pub type DPIP5 = crate::Reg<dpip5::DPIP5_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 5)"]
pub mod dpip5;
#[doc = "SADDR6 (rw) register accessor: an alias for `Reg<SADDR6_SPEC>`"]
pub type SADDR6 = crate::Reg<saddr6::SADDR6_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 6)"]
pub mod saddr6;
#[doc = "DADDR6 (rw) register accessor: an alias for `Reg<DADDR6_SPEC>`"]
pub type DADDR6 = crate::Reg<daddr6::DADDR6_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 6)"]
pub mod daddr6;
#[doc = "DSCR6 (rw) register accessor: an alias for `Reg<DSCR6_SPEC>`"]
pub type DSCR6 = crate::Reg<dscr6::DSCR6_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 6)"]
pub mod dscr6;
#[doc = "CTRLA6 (rw) register accessor: an alias for `Reg<CTRLA6_SPEC>`"]
pub type CTRLA6 = crate::Reg<ctrla6::CTRLA6_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 6)"]
pub mod ctrla6;
#[doc = "CTRLB6 (rw) register accessor: an alias for `Reg<CTRLB6_SPEC>`"]
pub type CTRLB6 = crate::Reg<ctrlb6::CTRLB6_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 6)"]
pub mod ctrlb6;
#[doc = "CFG6 (rw) register accessor: an alias for `Reg<CFG6_SPEC>`"]
pub type CFG6 = crate::Reg<cfg6::CFG6_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 6)"]
pub mod cfg6;
#[doc = "SPIP6 (rw) register accessor: an alias for `Reg<SPIP6_SPEC>`"]
pub type SPIP6 = crate::Reg<spip6::SPIP6_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 6)"]
pub mod spip6;
#[doc = "DPIP6 (rw) register accessor: an alias for `Reg<DPIP6_SPEC>`"]
pub type DPIP6 = crate::Reg<dpip6::DPIP6_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 6)"]
pub mod dpip6;
#[doc = "SADDR7 (rw) register accessor: an alias for `Reg<SADDR7_SPEC>`"]
pub type SADDR7 = crate::Reg<saddr7::SADDR7_SPEC>;
#[doc = "DMAC Channel Source Address Register (ch_num = 7)"]
pub mod saddr7;
#[doc = "DADDR7 (rw) register accessor: an alias for `Reg<DADDR7_SPEC>`"]
pub type DADDR7 = crate::Reg<daddr7::DADDR7_SPEC>;
#[doc = "DMAC Channel Destination Address Register (ch_num = 7)"]
pub mod daddr7;
#[doc = "DSCR7 (rw) register accessor: an alias for `Reg<DSCR7_SPEC>`"]
pub type DSCR7 = crate::Reg<dscr7::DSCR7_SPEC>;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 7)"]
pub mod dscr7;
#[doc = "CTRLA7 (rw) register accessor: an alias for `Reg<CTRLA7_SPEC>`"]
pub type CTRLA7 = crate::Reg<ctrla7::CTRLA7_SPEC>;
#[doc = "DMAC Channel Control A Register (ch_num = 7)"]
pub mod ctrla7;
#[doc = "CTRLB7 (rw) register accessor: an alias for `Reg<CTRLB7_SPEC>`"]
pub type CTRLB7 = crate::Reg<ctrlb7::CTRLB7_SPEC>;
#[doc = "DMAC Channel Control B Register (ch_num = 7)"]
pub mod ctrlb7;
#[doc = "CFG7 (rw) register accessor: an alias for `Reg<CFG7_SPEC>`"]
pub type CFG7 = crate::Reg<cfg7::CFG7_SPEC>;
#[doc = "DMAC Channel Configuration Register (ch_num = 7)"]
pub mod cfg7;
#[doc = "SPIP7 (rw) register accessor: an alias for `Reg<SPIP7_SPEC>`"]
pub type SPIP7 = crate::Reg<spip7::SPIP7_SPEC>;
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 7)"]
pub mod spip7;
#[doc = "DPIP7 (rw) register accessor: an alias for `Reg<DPIP7_SPEC>`"]
pub type DPIP7 = crate::Reg<dpip7::DPIP7_SPEC>;
#[doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 7)"]
pub mod dpip7;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
