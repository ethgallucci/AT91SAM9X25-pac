#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    pub pmc_scer: PMC_SCER,
    #[doc = "0x04 - System Clock Disable Register"]
    pub pmc_scdr: PMC_SCDR,
    #[doc = "0x08 - System Clock Status Register"]
    pub pmc_scsr: PMC_SCSR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Peripheral Clock Enable Register"]
    pub pmc_pcer: PMC_PCER,
    #[doc = "0x14 - Peripheral Clock Disable Register"]
    pub pmc_pcdr: PMC_PCDR,
    #[doc = "0x18 - Peripheral Clock Status Register"]
    pub pmc_pcsr: PMC_PCSR,
    #[doc = "0x1c - UTMI Clock Register"]
    pub ckgr_uckr: CKGR_UCKR,
    #[doc = "0x20 - Main Oscillator Register"]
    pub ckgr_mor: CKGR_MOR,
    #[doc = "0x24 - Main Clock Frequency Register"]
    pub ckgr_mcfr: CKGR_MCFR,
    #[doc = "0x28 - PLLA Register"]
    pub ckgr_pllar: CKGR_PLLAR,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Master Clock Register"]
    pub pmc_mckr: PMC_MCKR,
    _reserved11: [u8; 0x04],
    #[doc = "0x38 - USB Clock Register"]
    pub pmc_usb: PMC_USB,
    #[doc = "0x3c - Soft Modem Clock Register"]
    pub pmc_smd: PMC_SMD,
    #[doc = "0x40..0x48 - Programmable Clock 0 Register"]
    pub pmc_pck: [PMC_PCK; 2],
    _reserved14: [u8; 0x18],
    #[doc = "0x60 - Interrupt Enable Register"]
    pub pmc_ier: PMC_IER,
    #[doc = "0x64 - Interrupt Disable Register"]
    pub pmc_idr: PMC_IDR,
    #[doc = "0x68 - Status Register"]
    pub pmc_sr: PMC_SR,
    #[doc = "0x6c - Interrupt Mask Register"]
    pub pmc_imr: PMC_IMR,
    _reserved18: [u8; 0x10],
    #[doc = "0x80 - PLL Charge Pump Current Register"]
    pub pmc_pllicpr: PMC_PLLICPR,
    _reserved19: [u8; 0x60],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub pmc_wpmr: PMC_WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub pmc_wpsr: PMC_WPSR,
    _reserved21: [u8; 0x20],
    #[doc = "0x10c - Peripheral Control Register"]
    pub pmc_pcr: PMC_PCR,
}
#[doc = "PMC_SCER (w) register accessor: an alias for `Reg<PMC_SCER_SPEC>`"]
pub type PMC_SCER = crate::Reg<pmc_scer::PMC_SCER_SPEC>;
#[doc = "System Clock Enable Register"]
pub mod pmc_scer;
#[doc = "PMC_SCDR (w) register accessor: an alias for `Reg<PMC_SCDR_SPEC>`"]
pub type PMC_SCDR = crate::Reg<pmc_scdr::PMC_SCDR_SPEC>;
#[doc = "System Clock Disable Register"]
pub mod pmc_scdr;
#[doc = "PMC_SCSR (r) register accessor: an alias for `Reg<PMC_SCSR_SPEC>`"]
pub type PMC_SCSR = crate::Reg<pmc_scsr::PMC_SCSR_SPEC>;
#[doc = "System Clock Status Register"]
pub mod pmc_scsr;
#[doc = "PMC_PCER (w) register accessor: an alias for `Reg<PMC_PCER_SPEC>`"]
pub type PMC_PCER = crate::Reg<pmc_pcer::PMC_PCER_SPEC>;
#[doc = "Peripheral Clock Enable Register"]
pub mod pmc_pcer;
#[doc = "PMC_PCDR (w) register accessor: an alias for `Reg<PMC_PCDR_SPEC>`"]
pub type PMC_PCDR = crate::Reg<pmc_pcdr::PMC_PCDR_SPEC>;
#[doc = "Peripheral Clock Disable Register"]
pub mod pmc_pcdr;
#[doc = "PMC_PCSR (r) register accessor: an alias for `Reg<PMC_PCSR_SPEC>`"]
pub type PMC_PCSR = crate::Reg<pmc_pcsr::PMC_PCSR_SPEC>;
#[doc = "Peripheral Clock Status Register"]
pub mod pmc_pcsr;
#[doc = "CKGR_UCKR (rw) register accessor: an alias for `Reg<CKGR_UCKR_SPEC>`"]
pub type CKGR_UCKR = crate::Reg<ckgr_uckr::CKGR_UCKR_SPEC>;
#[doc = "UTMI Clock Register"]
pub mod ckgr_uckr;
#[doc = "CKGR_MOR (rw) register accessor: an alias for `Reg<CKGR_MOR_SPEC>`"]
pub type CKGR_MOR = crate::Reg<ckgr_mor::CKGR_MOR_SPEC>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (r) register accessor: an alias for `Reg<CKGR_MCFR_SPEC>`"]
pub type CKGR_MCFR = crate::Reg<ckgr_mcfr::CKGR_MCFR_SPEC>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: an alias for `Reg<CKGR_PLLAR_SPEC>`"]
pub type CKGR_PLLAR = crate::Reg<ckgr_pllar::CKGR_PLLAR_SPEC>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "PMC_MCKR (rw) register accessor: an alias for `Reg<PMC_MCKR_SPEC>`"]
pub type PMC_MCKR = crate::Reg<pmc_mckr::PMC_MCKR_SPEC>;
#[doc = "Master Clock Register"]
pub mod pmc_mckr;
#[doc = "PMC_USB (rw) register accessor: an alias for `Reg<PMC_USB_SPEC>`"]
pub type PMC_USB = crate::Reg<pmc_usb::PMC_USB_SPEC>;
#[doc = "USB Clock Register"]
pub mod pmc_usb;
#[doc = "PMC_SMD (rw) register accessor: an alias for `Reg<PMC_SMD_SPEC>`"]
pub type PMC_SMD = crate::Reg<pmc_smd::PMC_SMD_SPEC>;
#[doc = "Soft Modem Clock Register"]
pub mod pmc_smd;
#[doc = "PMC_PCK (rw) register accessor: an alias for `Reg<PMC_PCK_SPEC>`"]
pub type PMC_PCK = crate::Reg<pmc_pck::PMC_PCK_SPEC>;
#[doc = "Programmable Clock 0 Register"]
pub mod pmc_pck;
#[doc = "PMC_IER (w) register accessor: an alias for `Reg<PMC_IER_SPEC>`"]
pub type PMC_IER = crate::Reg<pmc_ier::PMC_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pmc_ier;
#[doc = "PMC_IDR (w) register accessor: an alias for `Reg<PMC_IDR_SPEC>`"]
pub type PMC_IDR = crate::Reg<pmc_idr::PMC_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod pmc_idr;
#[doc = "PMC_SR (r) register accessor: an alias for `Reg<PMC_SR_SPEC>`"]
pub type PMC_SR = crate::Reg<pmc_sr::PMC_SR_SPEC>;
#[doc = "Status Register"]
pub mod pmc_sr;
#[doc = "PMC_IMR (r) register accessor: an alias for `Reg<PMC_IMR_SPEC>`"]
pub type PMC_IMR = crate::Reg<pmc_imr::PMC_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod pmc_imr;
#[doc = "PMC_PLLICPR (w) register accessor: an alias for `Reg<PMC_PLLICPR_SPEC>`"]
pub type PMC_PLLICPR = crate::Reg<pmc_pllicpr::PMC_PLLICPR_SPEC>;
#[doc = "PLL Charge Pump Current Register"]
pub mod pmc_pllicpr;
#[doc = "PMC_WPMR (rw) register accessor: an alias for `Reg<PMC_WPMR_SPEC>`"]
pub type PMC_WPMR = crate::Reg<pmc_wpmr::PMC_WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod pmc_wpmr;
#[doc = "PMC_WPSR (r) register accessor: an alias for `Reg<PMC_WPSR_SPEC>`"]
pub type PMC_WPSR = crate::Reg<pmc_wpsr::PMC_WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod pmc_wpsr;
#[doc = "PMC_PCR (rw) register accessor: an alias for `Reg<PMC_PCR_SPEC>`"]
pub type PMC_PCR = crate::Reg<pmc_pcr::PMC_PCR_SPEC>;
#[doc = "Peripheral Control Register"]
pub mod pmc_pcr;
