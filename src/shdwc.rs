#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Shutdown Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Shutdown Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Shutdown Status Register"]
    pub sr: SR,
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Shutdown Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Shutdown Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Shutdown Status Register"]
pub mod sr;
