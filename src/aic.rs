#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Source Mode Register"]
    pub smr: [SMR; 32],
    #[doc = "0x80..0x100 - Source Vector Register"]
    pub svr: [SVR; 32],
    #[doc = "0x100 - Interrupt Vector Register"]
    pub ivr: IVR,
    #[doc = "0x104 - FIQ Interrupt Vector Register"]
    pub fvr: FVR,
    #[doc = "0x108 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x10c - Interrupt Pending Register"]
    pub ipr: IPR,
    #[doc = "0x110 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x114 - Core Interrupt Status Register"]
    pub cisr: CISR,
    _reserved8: [u8; 0x08],
    #[doc = "0x120 - Interrupt Enable Command Register"]
    pub iecr: IECR,
    #[doc = "0x124 - Interrupt Disable Command Register"]
    pub idcr: IDCR,
    #[doc = "0x128 - Interrupt Clear Command Register"]
    pub iccr: ICCR,
    #[doc = "0x12c - Interrupt Set Command Register"]
    pub iscr: ISCR,
    #[doc = "0x130 - End of Interrupt Command Register"]
    pub eoicr: EOICR,
    #[doc = "0x134 - Spurious Interrupt Vector Register"]
    pub spu: SPU,
    #[doc = "0x138 - Debug Control Register"]
    pub dcr: DCR,
    _reserved15: [u8; 0x04],
    #[doc = "0x140 - Fast Forcing Enable Register"]
    pub ffer: FFER,
    #[doc = "0x144 - Fast Forcing Disable Register"]
    pub ffdr: FFDR,
    #[doc = "0x148 - Fast Forcing Status Register"]
    pub ffsr: FFSR,
    _reserved18: [u8; 0x98],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "SMR (rw) register accessor: an alias for `Reg<SMR_SPEC>`"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "Source Mode Register"]
pub mod smr;
#[doc = "SVR (rw) register accessor: an alias for `Reg<SVR_SPEC>`"]
pub type SVR = crate::Reg<svr::SVR_SPEC>;
#[doc = "Source Vector Register"]
pub mod svr;
#[doc = "IVR (r) register accessor: an alias for `Reg<IVR_SPEC>`"]
pub type IVR = crate::Reg<ivr::IVR_SPEC>;
#[doc = "Interrupt Vector Register"]
pub mod ivr;
#[doc = "FVR (r) register accessor: an alias for `Reg<FVR_SPEC>`"]
pub type FVR = crate::Reg<fvr::FVR_SPEC>;
#[doc = "FIQ Interrupt Vector Register"]
pub mod fvr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IPR (r) register accessor: an alias for `Reg<IPR_SPEC>`"]
pub type IPR = crate::Reg<ipr::IPR_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "CISR (r) register accessor: an alias for `Reg<CISR_SPEC>`"]
pub type CISR = crate::Reg<cisr::CISR_SPEC>;
#[doc = "Core Interrupt Status Register"]
pub mod cisr;
#[doc = "IECR (w) register accessor: an alias for `Reg<IECR_SPEC>`"]
pub type IECR = crate::Reg<iecr::IECR_SPEC>;
#[doc = "Interrupt Enable Command Register"]
pub mod iecr;
#[doc = "IDCR (w) register accessor: an alias for `Reg<IDCR_SPEC>`"]
pub type IDCR = crate::Reg<idcr::IDCR_SPEC>;
#[doc = "Interrupt Disable Command Register"]
pub mod idcr;
#[doc = "ICCR (w) register accessor: an alias for `Reg<ICCR_SPEC>`"]
pub type ICCR = crate::Reg<iccr::ICCR_SPEC>;
#[doc = "Interrupt Clear Command Register"]
pub mod iccr;
#[doc = "ISCR (w) register accessor: an alias for `Reg<ISCR_SPEC>`"]
pub type ISCR = crate::Reg<iscr::ISCR_SPEC>;
#[doc = "Interrupt Set Command Register"]
pub mod iscr;
#[doc = "EOICR (w) register accessor: an alias for `Reg<EOICR_SPEC>`"]
pub type EOICR = crate::Reg<eoicr::EOICR_SPEC>;
#[doc = "End of Interrupt Command Register"]
pub mod eoicr;
#[doc = "SPU (rw) register accessor: an alias for `Reg<SPU_SPEC>`"]
pub type SPU = crate::Reg<spu::SPU_SPEC>;
#[doc = "Spurious Interrupt Vector Register"]
pub mod spu;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "Debug Control Register"]
pub mod dcr;
#[doc = "FFER (w) register accessor: an alias for `Reg<FFER_SPEC>`"]
pub type FFER = crate::Reg<ffer::FFER_SPEC>;
#[doc = "Fast Forcing Enable Register"]
pub mod ffer;
#[doc = "FFDR (w) register accessor: an alias for `Reg<FFDR_SPEC>`"]
pub type FFDR = crate::Reg<ffdr::FFDR_SPEC>;
#[doc = "Fast Forcing Disable Register"]
pub mod ffdr;
#[doc = "FFSR (r) register accessor: an alias for `Reg<FFSR_SPEC>`"]
pub type FFSR = crate::Reg<ffsr::FFSR_SPEC>;
#[doc = "Fast Forcing Status Register"]
pub mod ffsr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
