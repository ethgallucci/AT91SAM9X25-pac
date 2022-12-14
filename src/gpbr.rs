#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - General Purpose Backup Register"]
    pub gpbr: [GPBR; 4],
}
#[doc = "GPBR (rw) register accessor: an alias for `Reg<GPBR_SPEC>`"]
pub type GPBR = crate::Reg<gpbr::GPBR_SPEC>;
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
