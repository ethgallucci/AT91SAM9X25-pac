#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Status Register"]
    pub sr: SR,
    #[doc = "0x08 - Periodic Interval Value Register"]
    pub pivr: PIVR,
    #[doc = "0x0c - Periodic Interval Image Register"]
    pub piir: PIIR,
}
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "PIVR (r) register accessor: an alias for `Reg<PIVR_SPEC>`"]
pub type PIVR = crate::Reg<pivr::PIVR_SPEC>;
#[doc = "Periodic Interval Value Register"]
pub mod pivr;
#[doc = "PIIR (r) register accessor: an alias for `Reg<PIIR_SPEC>`"]
pub type PIIR = crate::Reg<piir::PIIR_SPEC>;
#[doc = "Periodic Interval Image Register"]
pub mod piir;
