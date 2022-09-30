#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup0: SETUP0,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse0: PULSE0,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle0: CYCLE0,
    #[doc = "0x0c - SMC Mode Register (CS_number = 0)"]
    pub mode0: MODE0,
    #[doc = "0x10 - SMC Setup Register (CS_number = 1)"]
    pub setup1: SETUP1,
    #[doc = "0x14 - SMC Pulse Register (CS_number = 1)"]
    pub pulse1: PULSE1,
    #[doc = "0x18 - SMC Cycle Register (CS_number = 1)"]
    pub cycle1: CYCLE1,
    #[doc = "0x1c - SMC Mode Register (CS_number = 1)"]
    pub mode1: MODE1,
    #[doc = "0x20 - SMC Setup Register (CS_number = 2)"]
    pub setup2: SETUP2,
    #[doc = "0x24 - SMC Pulse Register (CS_number = 2)"]
    pub pulse2: PULSE2,
    #[doc = "0x28 - SMC Cycle Register (CS_number = 2)"]
    pub cycle2: CYCLE2,
    #[doc = "0x2c - SMC Mode Register (CS_number = 2)"]
    pub mode2: MODE2,
    #[doc = "0x30 - SMC Setup Register (CS_number = 3)"]
    pub setup3: SETUP3,
    #[doc = "0x34 - SMC Pulse Register (CS_number = 3)"]
    pub pulse3: PULSE3,
    #[doc = "0x38 - SMC Cycle Register (CS_number = 3)"]
    pub cycle3: CYCLE3,
    #[doc = "0x3c - SMC Mode Register (CS_number = 3)"]
    pub mode3: MODE3,
    #[doc = "0x40 - SMC Setup Register (CS_number = 4)"]
    pub setup4: SETUP4,
    #[doc = "0x44 - SMC Pulse Register (CS_number = 4)"]
    pub pulse4: PULSE4,
    #[doc = "0x48 - SMC Cycle Register (CS_number = 4)"]
    pub cycle4: CYCLE4,
    #[doc = "0x4c - SMC Mode Register (CS_number = 4)"]
    pub mode4: MODE4,
    #[doc = "0x50 - SMC Setup Register (CS_number = 5)"]
    pub setup5: SETUP5,
    #[doc = "0x54 - SMC Pulse Register (CS_number = 5)"]
    pub pulse5: PULSE5,
    #[doc = "0x58 - SMC Cycle Register (CS_number = 5)"]
    pub cycle5: CYCLE5,
    #[doc = "0x5c - SMC Mode Register (CS_number = 5)"]
    pub mode5: MODE5,
    _reserved24: [u8; 0x60],
    #[doc = "0xc0 - SMC Delay on I/O"]
    pub delay1: DELAY1,
    #[doc = "0xc4 - SMC Delay on I/O"]
    pub delay2: DELAY2,
    #[doc = "0xc8 - SMC Delay on I/O"]
    pub delay3: DELAY3,
    #[doc = "0xcc - SMC Delay on I/O"]
    pub delay4: DELAY4,
    #[doc = "0xd0 - SMC Delay on I/O"]
    pub delay5: DELAY5,
    #[doc = "0xd4 - SMC Delay on I/O"]
    pub delay6: DELAY6,
    #[doc = "0xd8 - SMC Delay on I/O"]
    pub delay7: DELAY7,
    #[doc = "0xdc - SMC Delay on I/O"]
    pub delay8: DELAY8,
    _reserved32: [u8; 0x04],
    #[doc = "0xe4 - SMC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - SMC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "SETUP0 (rw) register accessor: an alias for `Reg<SETUP0_SPEC>`"]
pub type SETUP0 = crate::Reg<setup0::SETUP0_SPEC>;
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod setup0;
#[doc = "PULSE0 (rw) register accessor: an alias for `Reg<PULSE0_SPEC>`"]
pub type PULSE0 = crate::Reg<pulse0::PULSE0_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 0)"]
pub mod pulse0;
#[doc = "CYCLE0 (rw) register accessor: an alias for `Reg<CYCLE0_SPEC>`"]
pub type CYCLE0 = crate::Reg<cycle0::CYCLE0_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 0)"]
pub mod cycle0;
#[doc = "MODE0 (rw) register accessor: an alias for `Reg<MODE0_SPEC>`"]
pub type MODE0 = crate::Reg<mode0::MODE0_SPEC>;
#[doc = "SMC Mode Register (CS_number = 0)"]
pub mod mode0;
#[doc = "SETUP1 (rw) register accessor: an alias for `Reg<SETUP1_SPEC>`"]
pub type SETUP1 = crate::Reg<setup1::SETUP1_SPEC>;
#[doc = "SMC Setup Register (CS_number = 1)"]
pub mod setup1;
#[doc = "PULSE1 (rw) register accessor: an alias for `Reg<PULSE1_SPEC>`"]
pub type PULSE1 = crate::Reg<pulse1::PULSE1_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 1)"]
pub mod pulse1;
#[doc = "CYCLE1 (rw) register accessor: an alias for `Reg<CYCLE1_SPEC>`"]
pub type CYCLE1 = crate::Reg<cycle1::CYCLE1_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 1)"]
pub mod cycle1;
#[doc = "MODE1 (rw) register accessor: an alias for `Reg<MODE1_SPEC>`"]
pub type MODE1 = crate::Reg<mode1::MODE1_SPEC>;
#[doc = "SMC Mode Register (CS_number = 1)"]
pub mod mode1;
#[doc = "SETUP2 (rw) register accessor: an alias for `Reg<SETUP2_SPEC>`"]
pub type SETUP2 = crate::Reg<setup2::SETUP2_SPEC>;
#[doc = "SMC Setup Register (CS_number = 2)"]
pub mod setup2;
#[doc = "PULSE2 (rw) register accessor: an alias for `Reg<PULSE2_SPEC>`"]
pub type PULSE2 = crate::Reg<pulse2::PULSE2_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 2)"]
pub mod pulse2;
#[doc = "CYCLE2 (rw) register accessor: an alias for `Reg<CYCLE2_SPEC>`"]
pub type CYCLE2 = crate::Reg<cycle2::CYCLE2_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 2)"]
pub mod cycle2;
#[doc = "MODE2 (rw) register accessor: an alias for `Reg<MODE2_SPEC>`"]
pub type MODE2 = crate::Reg<mode2::MODE2_SPEC>;
#[doc = "SMC Mode Register (CS_number = 2)"]
pub mod mode2;
#[doc = "SETUP3 (rw) register accessor: an alias for `Reg<SETUP3_SPEC>`"]
pub type SETUP3 = crate::Reg<setup3::SETUP3_SPEC>;
#[doc = "SMC Setup Register (CS_number = 3)"]
pub mod setup3;
#[doc = "PULSE3 (rw) register accessor: an alias for `Reg<PULSE3_SPEC>`"]
pub type PULSE3 = crate::Reg<pulse3::PULSE3_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 3)"]
pub mod pulse3;
#[doc = "CYCLE3 (rw) register accessor: an alias for `Reg<CYCLE3_SPEC>`"]
pub type CYCLE3 = crate::Reg<cycle3::CYCLE3_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 3)"]
pub mod cycle3;
#[doc = "MODE3 (rw) register accessor: an alias for `Reg<MODE3_SPEC>`"]
pub type MODE3 = crate::Reg<mode3::MODE3_SPEC>;
#[doc = "SMC Mode Register (CS_number = 3)"]
pub mod mode3;
#[doc = "SETUP4 (rw) register accessor: an alias for `Reg<SETUP4_SPEC>`"]
pub type SETUP4 = crate::Reg<setup4::SETUP4_SPEC>;
#[doc = "SMC Setup Register (CS_number = 4)"]
pub mod setup4;
#[doc = "PULSE4 (rw) register accessor: an alias for `Reg<PULSE4_SPEC>`"]
pub type PULSE4 = crate::Reg<pulse4::PULSE4_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 4)"]
pub mod pulse4;
#[doc = "CYCLE4 (rw) register accessor: an alias for `Reg<CYCLE4_SPEC>`"]
pub type CYCLE4 = crate::Reg<cycle4::CYCLE4_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 4)"]
pub mod cycle4;
#[doc = "MODE4 (rw) register accessor: an alias for `Reg<MODE4_SPEC>`"]
pub type MODE4 = crate::Reg<mode4::MODE4_SPEC>;
#[doc = "SMC Mode Register (CS_number = 4)"]
pub mod mode4;
#[doc = "SETUP5 (rw) register accessor: an alias for `Reg<SETUP5_SPEC>`"]
pub type SETUP5 = crate::Reg<setup5::SETUP5_SPEC>;
#[doc = "SMC Setup Register (CS_number = 5)"]
pub mod setup5;
#[doc = "PULSE5 (rw) register accessor: an alias for `Reg<PULSE5_SPEC>`"]
pub type PULSE5 = crate::Reg<pulse5::PULSE5_SPEC>;
#[doc = "SMC Pulse Register (CS_number = 5)"]
pub mod pulse5;
#[doc = "CYCLE5 (rw) register accessor: an alias for `Reg<CYCLE5_SPEC>`"]
pub type CYCLE5 = crate::Reg<cycle5::CYCLE5_SPEC>;
#[doc = "SMC Cycle Register (CS_number = 5)"]
pub mod cycle5;
#[doc = "MODE5 (rw) register accessor: an alias for `Reg<MODE5_SPEC>`"]
pub type MODE5 = crate::Reg<mode5::MODE5_SPEC>;
#[doc = "SMC Mode Register (CS_number = 5)"]
pub mod mode5;
#[doc = "DELAY1 (rw) register accessor: an alias for `Reg<DELAY1_SPEC>`"]
pub type DELAY1 = crate::Reg<delay1::DELAY1_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay1;
#[doc = "DELAY2 (rw) register accessor: an alias for `Reg<DELAY2_SPEC>`"]
pub type DELAY2 = crate::Reg<delay2::DELAY2_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay2;
#[doc = "DELAY3 (rw) register accessor: an alias for `Reg<DELAY3_SPEC>`"]
pub type DELAY3 = crate::Reg<delay3::DELAY3_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay3;
#[doc = "DELAY4 (rw) register accessor: an alias for `Reg<DELAY4_SPEC>`"]
pub type DELAY4 = crate::Reg<delay4::DELAY4_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay4;
#[doc = "DELAY5 (rw) register accessor: an alias for `Reg<DELAY5_SPEC>`"]
pub type DELAY5 = crate::Reg<delay5::DELAY5_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay5;
#[doc = "DELAY6 (rw) register accessor: an alias for `Reg<DELAY6_SPEC>`"]
pub type DELAY6 = crate::Reg<delay6::DELAY6_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay6;
#[doc = "DELAY7 (rw) register accessor: an alias for `Reg<DELAY7_SPEC>`"]
pub type DELAY7 = crate::Reg<delay7::DELAY7_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay7;
#[doc = "DELAY8 (rw) register accessor: an alias for `Reg<DELAY8_SPEC>`"]
pub type DELAY8 = crate::Reg<delay8::DELAY8_SPEC>;
#[doc = "SMC Delay on I/O"]
pub mod delay8;
#[doc = "WPMR (rw) register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "SMC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "SMC Write Protect Status Register"]
pub mod wpsr;
