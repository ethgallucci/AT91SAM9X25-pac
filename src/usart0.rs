#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 0x04],
    _reserved_1_mr: [u8; 0x04],
    _reserved_2_ier: [u8; 0x04],
    _reserved_3_idr: [u8; 0x04],
    _reserved_4_imr: [u8; 0x04],
    _reserved_5_csr: [u8; 0x04],
    #[doc = "0x18 - Receiver Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmitter Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: TTGR,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: NER,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - IrDA Filter Register"]
    pub if_: IF,
    #[doc = "0x50 - Manchester Encoder Decoder Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    _reserved17: [u8; 0x88],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn spi_mode_cr_spi_mode(&self) -> &SPI_MODE_CR_SPI_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const SPI_MODE_CR_SPI_MODE)
        }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &CR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn spi_mode_mr_spi_mode(&self) -> &SPI_MODE_MR_SPI_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const SPI_MODE_MR_SPI_MODE)
        }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &MR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn lin_mode_ier_lin_mode(&self) -> &LIN_MODE_IER_LIN_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const LIN_MODE_IER_LIN_MODE)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn spi_mode_ier_spi_mode(&self) -> &SPI_MODE_IER_SPI_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const SPI_MODE_IER_SPI_MODE)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn lin_mode_idr_lin_mode(&self) -> &LIN_MODE_IDR_LIN_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize) as *const LIN_MODE_IDR_LIN_MODE)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn spi_mode_idr_spi_mode(&self) -> &SPI_MODE_IDR_SPI_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize) as *const SPI_MODE_IDR_SPI_MODE)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr(&self) -> &IDR {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn lin_mode_imr_lin_mode(&self) -> &LIN_MODE_IMR_LIN_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize) as *const LIN_MODE_IMR_LIN_MODE)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn spi_mode_imr_spi_mode(&self) -> &SPI_MODE_IMR_SPI_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize) as *const SPI_MODE_IMR_SPI_MODE)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr(&self) -> &IMR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn lin_mode_csr_lin_mode(&self) -> &LIN_MODE_CSR_LIN_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize) as *const LIN_MODE_CSR_LIN_MODE)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn spi_mode_csr_spi_mode(&self) -> &SPI_MODE_CSR_SPI_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize) as *const SPI_MODE_CSR_SPI_MODE)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &CSR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR) }
    }
}
#[doc = "CR (w) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SPI_MODE_CR_SPI_MODE (w) register accessor: an alias for `Reg<SPI_MODE_CR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_CR_SPI_MODE = crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC>;
#[doc = "Control Register"]
pub mod spi_mode_cr_spi_mode;
#[doc = "MR (rw) register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SPI_MODE_MR_SPI_MODE (rw) register accessor: an alias for `Reg<SPI_MODE_MR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_MR_SPI_MODE = crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC>;
#[doc = "Mode Register"]
pub mod spi_mode_mr_spi_mode;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "SPI_MODE_IER_SPI_MODE (w) register accessor: an alias for `Reg<SPI_MODE_IER_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IER_SPI_MODE = crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod spi_mode_ier_spi_mode;
#[doc = "LIN_MODE_IER_LIN_MODE (w) register accessor: an alias for `Reg<LIN_MODE_IER_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IER_LIN_MODE = crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod lin_mode_ier_lin_mode;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "SPI_MODE_IDR_SPI_MODE (w) register accessor: an alias for `Reg<SPI_MODE_IDR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IDR_SPI_MODE = crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod spi_mode_idr_spi_mode;
#[doc = "LIN_MODE_IDR_LIN_MODE (w) register accessor: an alias for `Reg<LIN_MODE_IDR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IDR_LIN_MODE = crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod lin_mode_idr_lin_mode;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SPI_MODE_IMR_SPI_MODE (r) register accessor: an alias for `Reg<SPI_MODE_IMR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IMR_SPI_MODE = crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod spi_mode_imr_spi_mode;
#[doc = "LIN_MODE_IMR_LIN_MODE (r) register accessor: an alias for `Reg<LIN_MODE_IMR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IMR_LIN_MODE = crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod lin_mode_imr_lin_mode;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "SPI_MODE_CSR_SPI_MODE (r) register accessor: an alias for `Reg<SPI_MODE_CSR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_CSR_SPI_MODE = crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod spi_mode_csr_spi_mode;
#[doc = "LIN_MODE_CSR_LIN_MODE (r) register accessor: an alias for `Reg<LIN_MODE_CSR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_CSR_LIN_MODE = crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod lin_mode_csr_lin_mode;
#[doc = "RHR (r) register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "THR (w) register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmitter Holding Register"]
pub mod thr;
#[doc = "BRGR (rw) register accessor: an alias for `Reg<BRGR_SPEC>`"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "RTOR (rw) register accessor: an alias for `Reg<RTOR_SPEC>`"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "TTGR (rw) register accessor: an alias for `Reg<TTGR_SPEC>`"]
pub type TTGR = crate::Reg<ttgr::TTGR_SPEC>;
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FIDI (rw) register accessor: an alias for `Reg<FIDI_SPEC>`"]
pub type FIDI = crate::Reg<fidi::FIDI_SPEC>;
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "NER (r) register accessor: an alias for `Reg<NER_SPEC>`"]
pub type NER = crate::Reg<ner::NER_SPEC>;
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "IF (rw) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "IrDA Filter Register"]
pub mod if_;
#[doc = "MAN (rw) register accessor: an alias for `Reg<MAN_SPEC>`"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "Manchester Encoder Decoder Register"]
pub mod man;
#[doc = "LINMR (rw) register accessor: an alias for `Reg<LINMR_SPEC>`"]
pub type LINMR = crate::Reg<linmr::LINMR_SPEC>;
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LINIR (rw) register accessor: an alias for `Reg<LINIR_SPEC>`"]
pub type LINIR = crate::Reg<linir::LINIR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
