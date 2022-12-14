#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Error Location Configuration Register"]
    pub elcfg: ELCFG,
    #[doc = "0x04 - Error Location Primitive Register"]
    pub elprim: ELPRIM,
    #[doc = "0x08 - Error Location Enable Register"]
    pub elen: ELEN,
    #[doc = "0x0c - Error Location Disable Register"]
    pub eldis: ELDIS,
    #[doc = "0x10 - Error Location Status Register"]
    pub elsr: ELSR,
    #[doc = "0x14 - Error Location Interrupt Enable register"]
    pub elier: ELIER,
    #[doc = "0x18 - Error Location Interrupt Disable Register"]
    pub elidr: ELIDR,
    #[doc = "0x1c - Error Location Interrupt Mask Register"]
    pub elimr: ELIMR,
    #[doc = "0x20 - Error Location Interrupt Status Register"]
    pub elisr: ELISR,
    _reserved9: [u8; 0x04],
    #[doc = "0x28..0x8c - PMECC SIGMA 0 Register"]
    pub sigma: [SIGMA; 25],
    #[doc = "0x8c..0xec - PMECC Error Location 0 Register"]
    pub el: [EL; 24],
}
#[doc = "ELCFG (rw) register accessor: an alias for `Reg<ELCFG_SPEC>`"]
pub type ELCFG = crate::Reg<elcfg::ELCFG_SPEC>;
#[doc = "Error Location Configuration Register"]
pub mod elcfg;
#[doc = "ELPRIM (r) register accessor: an alias for `Reg<ELPRIM_SPEC>`"]
pub type ELPRIM = crate::Reg<elprim::ELPRIM_SPEC>;
#[doc = "Error Location Primitive Register"]
pub mod elprim;
#[doc = "ELEN (rw) register accessor: an alias for `Reg<ELEN_SPEC>`"]
pub type ELEN = crate::Reg<elen::ELEN_SPEC>;
#[doc = "Error Location Enable Register"]
pub mod elen;
#[doc = "ELDIS (rw) register accessor: an alias for `Reg<ELDIS_SPEC>`"]
pub type ELDIS = crate::Reg<eldis::ELDIS_SPEC>;
#[doc = "Error Location Disable Register"]
pub mod eldis;
#[doc = "ELSR (rw) register accessor: an alias for `Reg<ELSR_SPEC>`"]
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
#[doc = "Error Location Status Register"]
pub mod elsr;
#[doc = "ELIER (r) register accessor: an alias for `Reg<ELIER_SPEC>`"]
pub type ELIER = crate::Reg<elier::ELIER_SPEC>;
#[doc = "Error Location Interrupt Enable register"]
pub mod elier;
#[doc = "ELIDR (r) register accessor: an alias for `Reg<ELIDR_SPEC>`"]
pub type ELIDR = crate::Reg<elidr::ELIDR_SPEC>;
#[doc = "Error Location Interrupt Disable Register"]
pub mod elidr;
#[doc = "ELIMR (r) register accessor: an alias for `Reg<ELIMR_SPEC>`"]
pub type ELIMR = crate::Reg<elimr::ELIMR_SPEC>;
#[doc = "Error Location Interrupt Mask Register"]
pub mod elimr;
#[doc = "ELISR (r) register accessor: an alias for `Reg<ELISR_SPEC>`"]
pub type ELISR = crate::Reg<elisr::ELISR_SPEC>;
#[doc = "Error Location Interrupt Status Register"]
pub mod elisr;
#[doc = "SIGMA (rw) register accessor: an alias for `Reg<SIGMA_SPEC>`"]
pub type SIGMA = crate::Reg<sigma::SIGMA_SPEC>;
#[doc = "PMECC SIGMA 0 Register"]
pub mod sigma;
#[doc = "EL (r) register accessor: an alias for `Reg<EL_SPEC>`"]
pub type EL = crate::Reg<el::EL_SPEC>;
#[doc = "PMECC Error Location 0 Register"]
pub mod el;
