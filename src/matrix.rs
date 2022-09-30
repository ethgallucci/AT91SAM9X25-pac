#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x2c - Master Configuration Register"]
    pub mcfg: [MCFG; 11],
    _reserved1: [u8; 0x14],
    #[doc = "0x40..0x68 - Slave Configuration Register"]
    pub scfg: [SCFG; 10],
    _reserved2: [u8; 0x18],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub pras0: PRAS0,
    #[doc = "0x84 - Priority Register B for Slave 0"]
    pub prbs0: PRBS0,
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub pras1: PRAS1,
    #[doc = "0x8c - Priority Register B for Slave 1"]
    pub prbs1: PRBS1,
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub pras2: PRAS2,
    #[doc = "0x94 - Priority Register B for Slave 2"]
    pub prbs2: PRBS2,
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub pras3: PRAS3,
    #[doc = "0x9c - Priority Register B for Slave 3"]
    pub prbs3: PRBS3,
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub pras4: PRAS4,
    #[doc = "0xa4 - Priority Register B for Slave 4"]
    pub prbs4: PRBS4,
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub pras5: PRAS5,
    #[doc = "0xac - Priority Register B for Slave 5"]
    pub prbs5: PRBS5,
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub pras6: PRAS6,
    #[doc = "0xb4 - Priority Register B for Slave 6"]
    pub prbs6: PRBS6,
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub pras7: PRAS7,
    #[doc = "0xbc - Priority Register B for Slave 7"]
    pub prbs7: PRBS7,
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub pras8: PRAS8,
    #[doc = "0xc4 - Priority Register B for Slave 8"]
    pub prbs8: PRBS8,
    #[doc = "0xc8 - Priority Register A for Slave 9"]
    pub pras9: PRAS9,
    #[doc = "0xcc - Priority Register B for Slave 9"]
    pub prbs9: PRBS9,
    _reserved22: [u8; 0x30],
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: MRCR,
    _reserved23: [u8; 0xe0],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "MCFG (rw) register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "SCFG (rw) register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "PRAS0 (rw) register accessor: an alias for `Reg<PRAS0_SPEC>`"]
pub type PRAS0 = crate::Reg<pras0::PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod pras0;
#[doc = "PRBS0 (rw) register accessor: an alias for `Reg<PRBS0_SPEC>`"]
pub type PRBS0 = crate::Reg<prbs0::PRBS0_SPEC>;
#[doc = "Priority Register B for Slave 0"]
pub mod prbs0;
#[doc = "PRAS1 (rw) register accessor: an alias for `Reg<PRAS1_SPEC>`"]
pub type PRAS1 = crate::Reg<pras1::PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod pras1;
#[doc = "PRBS1 (rw) register accessor: an alias for `Reg<PRBS1_SPEC>`"]
pub type PRBS1 = crate::Reg<prbs1::PRBS1_SPEC>;
#[doc = "Priority Register B for Slave 1"]
pub mod prbs1;
#[doc = "PRAS2 (rw) register accessor: an alias for `Reg<PRAS2_SPEC>`"]
pub type PRAS2 = crate::Reg<pras2::PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod pras2;
#[doc = "PRBS2 (rw) register accessor: an alias for `Reg<PRBS2_SPEC>`"]
pub type PRBS2 = crate::Reg<prbs2::PRBS2_SPEC>;
#[doc = "Priority Register B for Slave 2"]
pub mod prbs2;
#[doc = "PRAS3 (rw) register accessor: an alias for `Reg<PRAS3_SPEC>`"]
pub type PRAS3 = crate::Reg<pras3::PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod pras3;
#[doc = "PRBS3 (rw) register accessor: an alias for `Reg<PRBS3_SPEC>`"]
pub type PRBS3 = crate::Reg<prbs3::PRBS3_SPEC>;
#[doc = "Priority Register B for Slave 3"]
pub mod prbs3;
#[doc = "PRAS4 (rw) register accessor: an alias for `Reg<PRAS4_SPEC>`"]
pub type PRAS4 = crate::Reg<pras4::PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod pras4;
#[doc = "PRBS4 (rw) register accessor: an alias for `Reg<PRBS4_SPEC>`"]
pub type PRBS4 = crate::Reg<prbs4::PRBS4_SPEC>;
#[doc = "Priority Register B for Slave 4"]
pub mod prbs4;
#[doc = "PRAS5 (rw) register accessor: an alias for `Reg<PRAS5_SPEC>`"]
pub type PRAS5 = crate::Reg<pras5::PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod pras5;
#[doc = "PRBS5 (rw) register accessor: an alias for `Reg<PRBS5_SPEC>`"]
pub type PRBS5 = crate::Reg<prbs5::PRBS5_SPEC>;
#[doc = "Priority Register B for Slave 5"]
pub mod prbs5;
#[doc = "PRAS6 (rw) register accessor: an alias for `Reg<PRAS6_SPEC>`"]
pub type PRAS6 = crate::Reg<pras6::PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod pras6;
#[doc = "PRBS6 (rw) register accessor: an alias for `Reg<PRBS6_SPEC>`"]
pub type PRBS6 = crate::Reg<prbs6::PRBS6_SPEC>;
#[doc = "Priority Register B for Slave 6"]
pub mod prbs6;
#[doc = "PRAS7 (rw) register accessor: an alias for `Reg<PRAS7_SPEC>`"]
pub type PRAS7 = crate::Reg<pras7::PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod pras7;
#[doc = "PRBS7 (rw) register accessor: an alias for `Reg<PRBS7_SPEC>`"]
pub type PRBS7 = crate::Reg<prbs7::PRBS7_SPEC>;
#[doc = "Priority Register B for Slave 7"]
pub mod prbs7;
#[doc = "PRAS8 (rw) register accessor: an alias for `Reg<PRAS8_SPEC>`"]
pub type PRAS8 = crate::Reg<pras8::PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod pras8;
#[doc = "PRBS8 (rw) register accessor: an alias for `Reg<PRBS8_SPEC>`"]
pub type PRBS8 = crate::Reg<prbs8::PRBS8_SPEC>;
#[doc = "Priority Register B for Slave 8"]
pub mod prbs8;
#[doc = "PRAS9 (rw) register accessor: an alias for `Reg<PRAS9_SPEC>`"]
pub type PRAS9 = crate::Reg<pras9::PRAS9_SPEC>;
#[doc = "Priority Register A for Slave 9"]
pub mod pras9;
#[doc = "PRBS9 (rw) register accessor: an alias for `Reg<PRBS9_SPEC>`"]
pub type PRBS9 = crate::Reg<prbs9::PRBS9_SPEC>;
#[doc = "Priority Register B for Slave 9"]
pub mod prbs9;
#[doc = "MRCR (rw) register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
