#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Channel Sequence Register 1"]
    pub seqr1: SEQR1,
    #[doc = "0x0c - Channel Sequence Register 2"]
    pub seqr2: SEQR2,
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: CHSR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Last Converted Data Register"]
    pub lcdr: LCDR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    _reserved12: [u8; 0x08],
    #[doc = "0x3c - Overrun Status Register"]
    pub over: OVER,
    #[doc = "0x40 - Extended Mode Register"]
    pub emr: EMR,
    #[doc = "0x44 - Compare Window Register"]
    pub cwr: CWR,
    _reserved15: [u8; 0x08],
    #[doc = "0x50..0x80 - Channel Data Register"]
    pub cdr: [CDR; 12],
    _reserved16: [u8; 0x14],
    #[doc = "0x94 - Analog Control Register"]
    pub acr: ACR,
    _reserved17: [u8; 0x28],
    #[doc = "0xc0 - Trigger Register"]
    pub trgr: TRGR,
    _reserved18: [u8; 0x20],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SEQR1 (rw) register accessor: an alias for `Reg<SEQR1_SPEC>`"]
pub type SEQR1 = crate::Reg<seqr1::SEQR1_SPEC>;
#[doc = "Channel Sequence Register 1"]
pub mod seqr1;
#[doc = "SEQR2 (rw) register accessor: an alias for `Reg<SEQR2_SPEC>`"]
pub type SEQR2 = crate::Reg<seqr2::SEQR2_SPEC>;
#[doc = "Channel Sequence Register 2"]
pub mod seqr2;
#[doc = "CHER (w) register accessor: an alias for `Reg<CHER_SPEC>`"]
pub type CHER = crate::Reg<cher::CHER_SPEC>;
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "CHDR (w) register accessor: an alias for `Reg<CHDR_SPEC>`"]
pub type CHDR = crate::Reg<chdr::CHDR_SPEC>;
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "CHSR (r) register accessor: an alias for `Reg<CHSR_SPEC>`"]
pub type CHSR = crate::Reg<chsr::CHSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "LCDR (r) register accessor: an alias for `Reg<LCDR_SPEC>`"]
pub type LCDR = crate::Reg<lcdr::LCDR_SPEC>;
#[doc = "Last Converted Data Register"]
pub mod lcdr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "OVER (r) register accessor: an alias for `Reg<OVER_SPEC>`"]
pub type OVER = crate::Reg<over::OVER_SPEC>;
#[doc = "Overrun Status Register"]
pub mod over;
#[doc = "EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`"]
pub type EMR = crate::Reg<emr::EMR_SPEC>;
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "CWR (rw) register accessor: an alias for `Reg<CWR_SPEC>`"]
pub type CWR = crate::Reg<cwr::CWR_SPEC>;
#[doc = "Compare Window Register"]
pub mod cwr;
#[doc = "CDR (r) register accessor: an alias for `Reg<CDR_SPEC>`"]
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
#[doc = "Channel Data Register"]
pub mod cdr;
#[doc = "ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Analog Control Register"]
pub mod acr;
#[doc = "TRGR (rw) register accessor: an alias for `Reg<TRGR_SPEC>`"]
pub type TRGR = crate::Reg<trgr::TRGR_SPEC>;
#[doc = "Trigger Register"]
pub mod trgr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
