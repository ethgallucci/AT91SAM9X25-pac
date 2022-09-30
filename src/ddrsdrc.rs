#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DDRSDRC Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - DDRSDRC Refresh Timer Register"]
    pub rtr: RTR,
    #[doc = "0x08 - DDRSDRC Configuration Register"]
    pub cr: CR,
    #[doc = "0x0c - DDRSDRC Timing Parameter 0 Register"]
    pub tpr0: TPR0,
    #[doc = "0x10 - DDRSDRC Timing Parameter 1 Register"]
    pub tpr1: TPR1,
    #[doc = "0x14 - DDRSDRC Timing Parameter 2 Register"]
    pub tpr2: TPR2,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DDRSDRC Low-power Register"]
    pub lpr: LPR,
    #[doc = "0x20 - DDRSDRC Memory Device Register"]
    pub md: MD,
    #[doc = "0x24 - DDRSDRC DLL Information Register"]
    pub dll: DLL,
    _reserved9: [u8; 0x04],
    #[doc = "0x2c - DDRSDRC High Speed Register"]
    pub hs: HS,
    _reserved10: [u8; 0xb4],
    #[doc = "0xe4 - DDRSDRC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - DDRSDRC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "DDRSDRC Mode Register"]
pub mod mr;
#[doc = "RTR (rw) register accessor: an alias for `Reg<RTR_SPEC>`"]
pub type RTR = crate::Reg<rtr::RTR_SPEC>;
#[doc = "DDRSDRC Refresh Timer Register"]
pub mod rtr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DDRSDRC Configuration Register"]
pub mod cr;
#[doc = "TPR0 (rw) register accessor: an alias for `Reg<TPR0_SPEC>`"]
pub type TPR0 = crate::Reg<tpr0::TPR0_SPEC>;
#[doc = "DDRSDRC Timing Parameter 0 Register"]
pub mod tpr0;
#[doc = "TPR1 (rw) register accessor: an alias for `Reg<TPR1_SPEC>`"]
pub type TPR1 = crate::Reg<tpr1::TPR1_SPEC>;
#[doc = "DDRSDRC Timing Parameter 1 Register"]
pub mod tpr1;
#[doc = "TPR2 (rw) register accessor: an alias for `Reg<TPR2_SPEC>`"]
pub type TPR2 = crate::Reg<tpr2::TPR2_SPEC>;
#[doc = "DDRSDRC Timing Parameter 2 Register"]
pub mod tpr2;
#[doc = "LPR (rw) register accessor: an alias for `Reg<LPR_SPEC>`"]
pub type LPR = crate::Reg<lpr::LPR_SPEC>;
#[doc = "DDRSDRC Low-power Register"]
pub mod lpr;
#[doc = "MD (rw) register accessor: an alias for `Reg<MD_SPEC>`"]
pub type MD = crate::Reg<md::MD_SPEC>;
#[doc = "DDRSDRC Memory Device Register"]
pub mod md;
#[doc = "DLL (r) register accessor: an alias for `Reg<DLL_SPEC>`"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "DDRSDRC DLL Information Register"]
pub mod dll;
#[doc = "HS (rw) register accessor: an alias for `Reg<HS_SPEC>`"]
pub type HS = crate::Reg<hs::HS_SPEC>;
#[doc = "DDRSDRC High Speed Register"]
pub mod hs;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "DDRSDRC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "DDRSDRC Write Protect Status Register"]
pub mod wpsr;
