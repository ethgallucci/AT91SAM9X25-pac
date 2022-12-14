#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMECC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x04 - PMECC Spare Area Size Register"]
    pub sarea: SAREA,
    #[doc = "0x08 - PMECC Start Address Register"]
    pub saddr: SADDR,
    #[doc = "0x0c - PMECC End Address Register"]
    pub eaddr: EADDR,
    #[doc = "0x10 - PMECC Clock Control Register"]
    pub clk: CLK,
    #[doc = "0x14 - PMECC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x18 - PMECC Status Register"]
    pub sr: SR,
    #[doc = "0x1c - PMECC Interrupt Enable register"]
    pub ier: IER,
    #[doc = "0x20 - PMECC Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x24 - PMECC Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x28 - PMECC Interrupt Status Register"]
    pub isr: ISR,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - PMECC ECC 0 Register (sec_num = 0)"]
    pub ecc0_0: ECC0_0,
    #[doc = "0x44 - PMECC ECC 1 Register (sec_num = 0)"]
    pub ecc1_0: ECC1_0,
    #[doc = "0x48 - PMECC ECC 2 Register (sec_num = 0)"]
    pub ecc2_0: ECC2_0,
    #[doc = "0x4c - PMECC ECC 3 Register (sec_num = 0)"]
    pub ecc3_0: ECC3_0,
    #[doc = "0x50 - PMECC ECC 4 Register (sec_num = 0)"]
    pub ecc4_0: ECC4_0,
    #[doc = "0x54 - PMECC ECC 5 Register (sec_num = 0)"]
    pub ecc5_0: ECC5_0,
    #[doc = "0x58 - PMECC ECC 6 Register (sec_num = 0)"]
    pub ecc6_0: ECC6_0,
    #[doc = "0x5c - PMECC ECC 7 Register (sec_num = 0)"]
    pub ecc7_0: ECC7_0,
    #[doc = "0x60 - PMECC ECC 8 Register (sec_num = 0)"]
    pub ecc8_0: ECC8_0,
    #[doc = "0x64 - PMECC ECC 9 Register (sec_num = 0)"]
    pub ecc9_0: ECC9_0,
    #[doc = "0x68 - PMECC ECC 10 Register (sec_num = 0)"]
    pub ecc10_0: ECC10_0,
    _reserved22: [u8; 0x14],
    #[doc = "0x80 - PMECC ECC 0 Register (sec_num = 1)"]
    pub ecc0_1: ECC0_1,
    #[doc = "0x84 - PMECC ECC 1 Register (sec_num = 1)"]
    pub ecc1_1: ECC1_1,
    #[doc = "0x88 - PMECC ECC 2 Register (sec_num = 1)"]
    pub ecc2_1: ECC2_1,
    #[doc = "0x8c - PMECC ECC 3 Register (sec_num = 1)"]
    pub ecc3_1: ECC3_1,
    #[doc = "0x90 - PMECC ECC 4 Register (sec_num = 1)"]
    pub ecc4_1: ECC4_1,
    #[doc = "0x94 - PMECC ECC 5 Register (sec_num = 1)"]
    pub ecc5_1: ECC5_1,
    #[doc = "0x98 - PMECC ECC 6 Register (sec_num = 1)"]
    pub ecc6_1: ECC6_1,
    #[doc = "0x9c - PMECC ECC 7 Register (sec_num = 1)"]
    pub ecc7_1: ECC7_1,
    #[doc = "0xa0 - PMECC ECC 8 Register (sec_num = 1)"]
    pub ecc8_1: ECC8_1,
    #[doc = "0xa4 - PMECC ECC 9 Register (sec_num = 1)"]
    pub ecc9_1: ECC9_1,
    #[doc = "0xa8 - PMECC ECC 10 Register (sec_num = 1)"]
    pub ecc10_1: ECC10_1,
    _reserved33: [u8; 0x14],
    #[doc = "0xc0 - PMECC ECC 0 Register (sec_num = 2)"]
    pub ecc0_2: ECC0_2,
    #[doc = "0xc4 - PMECC ECC 1 Register (sec_num = 2)"]
    pub ecc1_2: ECC1_2,
    #[doc = "0xc8 - PMECC ECC 2 Register (sec_num = 2)"]
    pub ecc2_2: ECC2_2,
    #[doc = "0xcc - PMECC ECC 3 Register (sec_num = 2)"]
    pub ecc3_2: ECC3_2,
    #[doc = "0xd0 - PMECC ECC 4 Register (sec_num = 2)"]
    pub ecc4_2: ECC4_2,
    #[doc = "0xd4 - PMECC ECC 5 Register (sec_num = 2)"]
    pub ecc5_2: ECC5_2,
    #[doc = "0xd8 - PMECC ECC 6 Register (sec_num = 2)"]
    pub ecc6_2: ECC6_2,
    #[doc = "0xdc - PMECC ECC 7 Register (sec_num = 2)"]
    pub ecc7_2: ECC7_2,
    #[doc = "0xe0 - PMECC ECC 8 Register (sec_num = 2)"]
    pub ecc8_2: ECC8_2,
    #[doc = "0xe4 - PMECC ECC 9 Register (sec_num = 2)"]
    pub ecc9_2: ECC9_2,
    #[doc = "0xe8 - PMECC ECC 10 Register (sec_num = 2)"]
    pub ecc10_2: ECC10_2,
    _reserved44: [u8; 0x14],
    #[doc = "0x100 - PMECC ECC 0 Register (sec_num = 3)"]
    pub ecc0_3: ECC0_3,
    #[doc = "0x104 - PMECC ECC 1 Register (sec_num = 3)"]
    pub ecc1_3: ECC1_3,
    #[doc = "0x108 - PMECC ECC 2 Register (sec_num = 3)"]
    pub ecc2_3: ECC2_3,
    #[doc = "0x10c - PMECC ECC 3 Register (sec_num = 3)"]
    pub ecc3_3: ECC3_3,
    #[doc = "0x110 - PMECC ECC 4 Register (sec_num = 3)"]
    pub ecc4_3: ECC4_3,
    #[doc = "0x114 - PMECC ECC 5 Register (sec_num = 3)"]
    pub ecc5_3: ECC5_3,
    #[doc = "0x118 - PMECC ECC 6 Register (sec_num = 3)"]
    pub ecc6_3: ECC6_3,
    #[doc = "0x11c - PMECC ECC 7 Register (sec_num = 3)"]
    pub ecc7_3: ECC7_3,
    #[doc = "0x120 - PMECC ECC 8 Register (sec_num = 3)"]
    pub ecc8_3: ECC8_3,
    #[doc = "0x124 - PMECC ECC 9 Register (sec_num = 3)"]
    pub ecc9_3: ECC9_3,
    #[doc = "0x128 - PMECC ECC 10 Register (sec_num = 3)"]
    pub ecc10_3: ECC10_3,
    _reserved55: [u8; 0x14],
    #[doc = "0x140 - PMECC ECC 0 Register (sec_num = 4)"]
    pub ecc0_4: ECC0_4,
    #[doc = "0x144 - PMECC ECC 1 Register (sec_num = 4)"]
    pub ecc1_4: ECC1_4,
    #[doc = "0x148 - PMECC ECC 2 Register (sec_num = 4)"]
    pub ecc2_4: ECC2_4,
    #[doc = "0x14c - PMECC ECC 3 Register (sec_num = 4)"]
    pub ecc3_4: ECC3_4,
    #[doc = "0x150 - PMECC ECC 4 Register (sec_num = 4)"]
    pub ecc4_4: ECC4_4,
    #[doc = "0x154 - PMECC ECC 5 Register (sec_num = 4)"]
    pub ecc5_4: ECC5_4,
    #[doc = "0x158 - PMECC ECC 6 Register (sec_num = 4)"]
    pub ecc6_4: ECC6_4,
    #[doc = "0x15c - PMECC ECC 7 Register (sec_num = 4)"]
    pub ecc7_4: ECC7_4,
    #[doc = "0x160 - PMECC ECC 8 Register (sec_num = 4)"]
    pub ecc8_4: ECC8_4,
    #[doc = "0x164 - PMECC ECC 9 Register (sec_num = 4)"]
    pub ecc9_4: ECC9_4,
    #[doc = "0x168 - PMECC ECC 10 Register (sec_num = 4)"]
    pub ecc10_4: ECC10_4,
    _reserved66: [u8; 0x14],
    #[doc = "0x180 - PMECC ECC 0 Register (sec_num = 5)"]
    pub ecc0_5: ECC0_5,
    #[doc = "0x184 - PMECC ECC 1 Register (sec_num = 5)"]
    pub ecc1_5: ECC1_5,
    #[doc = "0x188 - PMECC ECC 2 Register (sec_num = 5)"]
    pub ecc2_5: ECC2_5,
    #[doc = "0x18c - PMECC ECC 3 Register (sec_num = 5)"]
    pub ecc3_5: ECC3_5,
    #[doc = "0x190 - PMECC ECC 4 Register (sec_num = 5)"]
    pub ecc4_5: ECC4_5,
    #[doc = "0x194 - PMECC ECC 5 Register (sec_num = 5)"]
    pub ecc5_5: ECC5_5,
    #[doc = "0x198 - PMECC ECC 6 Register (sec_num = 5)"]
    pub ecc6_5: ECC6_5,
    #[doc = "0x19c - PMECC ECC 7 Register (sec_num = 5)"]
    pub ecc7_5: ECC7_5,
    #[doc = "0x1a0 - PMECC ECC 8 Register (sec_num = 5)"]
    pub ecc8_5: ECC8_5,
    #[doc = "0x1a4 - PMECC ECC 9 Register (sec_num = 5)"]
    pub ecc9_5: ECC9_5,
    #[doc = "0x1a8 - PMECC ECC 10 Register (sec_num = 5)"]
    pub ecc10_5: ECC10_5,
    _reserved77: [u8; 0x14],
    #[doc = "0x1c0 - PMECC ECC 0 Register (sec_num = 6)"]
    pub ecc0_6: ECC0_6,
    #[doc = "0x1c4 - PMECC ECC 1 Register (sec_num = 6)"]
    pub ecc1_6: ECC1_6,
    #[doc = "0x1c8 - PMECC ECC 2 Register (sec_num = 6)"]
    pub ecc2_6: ECC2_6,
    #[doc = "0x1cc - PMECC ECC 3 Register (sec_num = 6)"]
    pub ecc3_6: ECC3_6,
    #[doc = "0x1d0 - PMECC ECC 4 Register (sec_num = 6)"]
    pub ecc4_6: ECC4_6,
    #[doc = "0x1d4 - PMECC ECC 5 Register (sec_num = 6)"]
    pub ecc5_6: ECC5_6,
    #[doc = "0x1d8 - PMECC ECC 6 Register (sec_num = 6)"]
    pub ecc6_6: ECC6_6,
    #[doc = "0x1dc - PMECC ECC 7 Register (sec_num = 6)"]
    pub ecc7_6: ECC7_6,
    #[doc = "0x1e0 - PMECC ECC 8 Register (sec_num = 6)"]
    pub ecc8_6: ECC8_6,
    #[doc = "0x1e4 - PMECC ECC 9 Register (sec_num = 6)"]
    pub ecc9_6: ECC9_6,
    #[doc = "0x1e8 - PMECC ECC 10 Register (sec_num = 6)"]
    pub ecc10_6: ECC10_6,
    _reserved88: [u8; 0x14],
    #[doc = "0x200 - PMECC ECC 0 Register (sec_num = 7)"]
    pub ecc0_7: ECC0_7,
    #[doc = "0x204 - PMECC ECC 1 Register (sec_num = 7)"]
    pub ecc1_7: ECC1_7,
    #[doc = "0x208 - PMECC ECC 2 Register (sec_num = 7)"]
    pub ecc2_7: ECC2_7,
    #[doc = "0x20c - PMECC ECC 3 Register (sec_num = 7)"]
    pub ecc3_7: ECC3_7,
    #[doc = "0x210 - PMECC ECC 4 Register (sec_num = 7)"]
    pub ecc4_7: ECC4_7,
    #[doc = "0x214 - PMECC ECC 5 Register (sec_num = 7)"]
    pub ecc5_7: ECC5_7,
    #[doc = "0x218 - PMECC ECC 6 Register (sec_num = 7)"]
    pub ecc6_7: ECC6_7,
    #[doc = "0x21c - PMECC ECC 7 Register (sec_num = 7)"]
    pub ecc7_7: ECC7_7,
    #[doc = "0x220 - PMECC ECC 8 Register (sec_num = 7)"]
    pub ecc8_7: ECC8_7,
    #[doc = "0x224 - PMECC ECC 9 Register (sec_num = 7)"]
    pub ecc9_7: ECC9_7,
    #[doc = "0x228 - PMECC ECC 10 Register (sec_num = 7)"]
    pub ecc10_7: ECC10_7,
    _reserved99: [u8; 0x14],
    #[doc = "0x240 - PMECC REM 0 Register (sec_num = 0)"]
    pub rem0_0: REM0_0,
    #[doc = "0x244 - PMECC REM 1 Register (sec_num = 0)"]
    pub rem1_0: REM1_0,
    #[doc = "0x248 - PMECC REM 2 Register (sec_num = 0)"]
    pub rem2_0: REM2_0,
    #[doc = "0x24c - PMECC REM 3 Register (sec_num = 0)"]
    pub rem3_0: REM3_0,
    #[doc = "0x250 - PMECC REM 4 Register (sec_num = 0)"]
    pub rem4_0: REM4_0,
    #[doc = "0x254 - PMECC REM 5 Register (sec_num = 0)"]
    pub rem5_0: REM5_0,
    #[doc = "0x258 - PMECC REM 6 Register (sec_num = 0)"]
    pub rem6_0: REM6_0,
    #[doc = "0x25c - PMECC REM 7 Register (sec_num = 0)"]
    pub rem7_0: REM7_0,
    #[doc = "0x260 - PMECC REM 8 Register (sec_num = 0)"]
    pub rem8_0: REM8_0,
    #[doc = "0x264 - PMECC REM 9 Register (sec_num = 0)"]
    pub rem9_0: REM9_0,
    #[doc = "0x268 - PMECC REM 10 Register (sec_num = 0)"]
    pub rem10_0: REM10_0,
    #[doc = "0x26c - PMECC REM 11 Register (sec_num = 0)"]
    pub rem11_0: REM11_0,
    _reserved111: [u8; 0x10],
    #[doc = "0x280 - PMECC REM 0 Register (sec_num = 1)"]
    pub rem0_1: REM0_1,
    #[doc = "0x284 - PMECC REM 1 Register (sec_num = 1)"]
    pub rem1_1: REM1_1,
    #[doc = "0x288 - PMECC REM 2 Register (sec_num = 1)"]
    pub rem2_1: REM2_1,
    #[doc = "0x28c - PMECC REM 3 Register (sec_num = 1)"]
    pub rem3_1: REM3_1,
    #[doc = "0x290 - PMECC REM 4 Register (sec_num = 1)"]
    pub rem4_1: REM4_1,
    #[doc = "0x294 - PMECC REM 5 Register (sec_num = 1)"]
    pub rem5_1: REM5_1,
    #[doc = "0x298 - PMECC REM 6 Register (sec_num = 1)"]
    pub rem6_1: REM6_1,
    #[doc = "0x29c - PMECC REM 7 Register (sec_num = 1)"]
    pub rem7_1: REM7_1,
    #[doc = "0x2a0 - PMECC REM 8 Register (sec_num = 1)"]
    pub rem8_1: REM8_1,
    #[doc = "0x2a4 - PMECC REM 9 Register (sec_num = 1)"]
    pub rem9_1: REM9_1,
    #[doc = "0x2a8 - PMECC REM 10 Register (sec_num = 1)"]
    pub rem10_1: REM10_1,
    #[doc = "0x2ac - PMECC REM 11 Register (sec_num = 1)"]
    pub rem11_1: REM11_1,
    _reserved123: [u8; 0x10],
    #[doc = "0x2c0 - PMECC REM 0 Register (sec_num = 2)"]
    pub rem0_2: REM0_2,
    #[doc = "0x2c4 - PMECC REM 1 Register (sec_num = 2)"]
    pub rem1_2: REM1_2,
    #[doc = "0x2c8 - PMECC REM 2 Register (sec_num = 2)"]
    pub rem2_2: REM2_2,
    #[doc = "0x2cc - PMECC REM 3 Register (sec_num = 2)"]
    pub rem3_2: REM3_2,
    #[doc = "0x2d0 - PMECC REM 4 Register (sec_num = 2)"]
    pub rem4_2: REM4_2,
    #[doc = "0x2d4 - PMECC REM 5 Register (sec_num = 2)"]
    pub rem5_2: REM5_2,
    #[doc = "0x2d8 - PMECC REM 6 Register (sec_num = 2)"]
    pub rem6_2: REM6_2,
    #[doc = "0x2dc - PMECC REM 7 Register (sec_num = 2)"]
    pub rem7_2: REM7_2,
    #[doc = "0x2e0 - PMECC REM 8 Register (sec_num = 2)"]
    pub rem8_2: REM8_2,
    #[doc = "0x2e4 - PMECC REM 9 Register (sec_num = 2)"]
    pub rem9_2: REM9_2,
    #[doc = "0x2e8 - PMECC REM 10 Register (sec_num = 2)"]
    pub rem10_2: REM10_2,
    #[doc = "0x2ec - PMECC REM 11 Register (sec_num = 2)"]
    pub rem11_2: REM11_2,
    _reserved135: [u8; 0x10],
    #[doc = "0x300 - PMECC REM 0 Register (sec_num = 3)"]
    pub rem0_3: REM0_3,
    #[doc = "0x304 - PMECC REM 1 Register (sec_num = 3)"]
    pub rem1_3: REM1_3,
    #[doc = "0x308 - PMECC REM 2 Register (sec_num = 3)"]
    pub rem2_3: REM2_3,
    #[doc = "0x30c - PMECC REM 3 Register (sec_num = 3)"]
    pub rem3_3: REM3_3,
    #[doc = "0x310 - PMECC REM 4 Register (sec_num = 3)"]
    pub rem4_3: REM4_3,
    #[doc = "0x314 - PMECC REM 5 Register (sec_num = 3)"]
    pub rem5_3: REM5_3,
    #[doc = "0x318 - PMECC REM 6 Register (sec_num = 3)"]
    pub rem6_3: REM6_3,
    #[doc = "0x31c - PMECC REM 7 Register (sec_num = 3)"]
    pub rem7_3: REM7_3,
    #[doc = "0x320 - PMECC REM 8 Register (sec_num = 3)"]
    pub rem8_3: REM8_3,
    #[doc = "0x324 - PMECC REM 9 Register (sec_num = 3)"]
    pub rem9_3: REM9_3,
    #[doc = "0x328 - PMECC REM 10 Register (sec_num = 3)"]
    pub rem10_3: REM10_3,
    #[doc = "0x32c - PMECC REM 11 Register (sec_num = 3)"]
    pub rem11_3: REM11_3,
    _reserved147: [u8; 0x10],
    #[doc = "0x340 - PMECC REM 0 Register (sec_num = 4)"]
    pub rem0_4: REM0_4,
    #[doc = "0x344 - PMECC REM 1 Register (sec_num = 4)"]
    pub rem1_4: REM1_4,
    #[doc = "0x348 - PMECC REM 2 Register (sec_num = 4)"]
    pub rem2_4: REM2_4,
    #[doc = "0x34c - PMECC REM 3 Register (sec_num = 4)"]
    pub rem3_4: REM3_4,
    #[doc = "0x350 - PMECC REM 4 Register (sec_num = 4)"]
    pub rem4_4: REM4_4,
    #[doc = "0x354 - PMECC REM 5 Register (sec_num = 4)"]
    pub rem5_4: REM5_4,
    #[doc = "0x358 - PMECC REM 6 Register (sec_num = 4)"]
    pub rem6_4: REM6_4,
    #[doc = "0x35c - PMECC REM 7 Register (sec_num = 4)"]
    pub rem7_4: REM7_4,
    #[doc = "0x360 - PMECC REM 8 Register (sec_num = 4)"]
    pub rem8_4: REM8_4,
    #[doc = "0x364 - PMECC REM 9 Register (sec_num = 4)"]
    pub rem9_4: REM9_4,
    #[doc = "0x368 - PMECC REM 10 Register (sec_num = 4)"]
    pub rem10_4: REM10_4,
    #[doc = "0x36c - PMECC REM 11 Register (sec_num = 4)"]
    pub rem11_4: REM11_4,
    _reserved159: [u8; 0x10],
    #[doc = "0x380 - PMECC REM 0 Register (sec_num = 5)"]
    pub rem0_5: REM0_5,
    #[doc = "0x384 - PMECC REM 1 Register (sec_num = 5)"]
    pub rem1_5: REM1_5,
    #[doc = "0x388 - PMECC REM 2 Register (sec_num = 5)"]
    pub rem2_5: REM2_5,
    #[doc = "0x38c - PMECC REM 3 Register (sec_num = 5)"]
    pub rem3_5: REM3_5,
    #[doc = "0x390 - PMECC REM 4 Register (sec_num = 5)"]
    pub rem4_5: REM4_5,
    #[doc = "0x394 - PMECC REM 5 Register (sec_num = 5)"]
    pub rem5_5: REM5_5,
    #[doc = "0x398 - PMECC REM 6 Register (sec_num = 5)"]
    pub rem6_5: REM6_5,
    #[doc = "0x39c - PMECC REM 7 Register (sec_num = 5)"]
    pub rem7_5: REM7_5,
    #[doc = "0x3a0 - PMECC REM 8 Register (sec_num = 5)"]
    pub rem8_5: REM8_5,
    #[doc = "0x3a4 - PMECC REM 9 Register (sec_num = 5)"]
    pub rem9_5: REM9_5,
    #[doc = "0x3a8 - PMECC REM 10 Register (sec_num = 5)"]
    pub rem10_5: REM10_5,
    #[doc = "0x3ac - PMECC REM 11 Register (sec_num = 5)"]
    pub rem11_5: REM11_5,
    _reserved171: [u8; 0x10],
    #[doc = "0x3c0 - PMECC REM 0 Register (sec_num = 6)"]
    pub rem0_6: REM0_6,
    #[doc = "0x3c4 - PMECC REM 1 Register (sec_num = 6)"]
    pub rem1_6: REM1_6,
    #[doc = "0x3c8 - PMECC REM 2 Register (sec_num = 6)"]
    pub rem2_6: REM2_6,
    #[doc = "0x3cc - PMECC REM 3 Register (sec_num = 6)"]
    pub rem3_6: REM3_6,
    #[doc = "0x3d0 - PMECC REM 4 Register (sec_num = 6)"]
    pub rem4_6: REM4_6,
    #[doc = "0x3d4 - PMECC REM 5 Register (sec_num = 6)"]
    pub rem5_6: REM5_6,
    #[doc = "0x3d8 - PMECC REM 6 Register (sec_num = 6)"]
    pub rem6_6: REM6_6,
    #[doc = "0x3dc - PMECC REM 7 Register (sec_num = 6)"]
    pub rem7_6: REM7_6,
    #[doc = "0x3e0 - PMECC REM 8 Register (sec_num = 6)"]
    pub rem8_6: REM8_6,
    #[doc = "0x3e4 - PMECC REM 9 Register (sec_num = 6)"]
    pub rem9_6: REM9_6,
    #[doc = "0x3e8 - PMECC REM 10 Register (sec_num = 6)"]
    pub rem10_6: REM10_6,
    #[doc = "0x3ec - PMECC REM 11 Register (sec_num = 6)"]
    pub rem11_6: REM11_6,
    _reserved183: [u8; 0x10],
    #[doc = "0x400 - PMECC REM 0 Register (sec_num = 7)"]
    pub rem0_7: REM0_7,
    #[doc = "0x404 - PMECC REM 1 Register (sec_num = 7)"]
    pub rem1_7: REM1_7,
    #[doc = "0x408 - PMECC REM 2 Register (sec_num = 7)"]
    pub rem2_7: REM2_7,
    #[doc = "0x40c - PMECC REM 3 Register (sec_num = 7)"]
    pub rem3_7: REM3_7,
    #[doc = "0x410 - PMECC REM 4 Register (sec_num = 7)"]
    pub rem4_7: REM4_7,
    #[doc = "0x414 - PMECC REM 5 Register (sec_num = 7)"]
    pub rem5_7: REM5_7,
    #[doc = "0x418 - PMECC REM 6 Register (sec_num = 7)"]
    pub rem6_7: REM6_7,
    #[doc = "0x41c - PMECC REM 7 Register (sec_num = 7)"]
    pub rem7_7: REM7_7,
    #[doc = "0x420 - PMECC REM 8 Register (sec_num = 7)"]
    pub rem8_7: REM8_7,
    #[doc = "0x424 - PMECC REM 9 Register (sec_num = 7)"]
    pub rem9_7: REM9_7,
    #[doc = "0x428 - PMECC REM 10 Register (sec_num = 7)"]
    pub rem10_7: REM10_7,
    #[doc = "0x42c - PMECC REM 11 Register (sec_num = 7)"]
    pub rem11_7: REM11_7,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "PMECC Configuration Register"]
pub mod cfg;
#[doc = "SAREA (rw) register accessor: an alias for `Reg<SAREA_SPEC>`"]
pub type SAREA = crate::Reg<sarea::SAREA_SPEC>;
#[doc = "PMECC Spare Area Size Register"]
pub mod sarea;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "PMECC Start Address Register"]
pub mod saddr;
#[doc = "EADDR (rw) register accessor: an alias for `Reg<EADDR_SPEC>`"]
pub type EADDR = crate::Reg<eaddr::EADDR_SPEC>;
#[doc = "PMECC End Address Register"]
pub mod eaddr;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "PMECC Clock Control Register"]
pub mod clk;
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PMECC Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "PMECC Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "PMECC Interrupt Enable register"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "PMECC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "PMECC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "PMECC Interrupt Status Register"]
pub mod isr;
#[doc = "ECC0_0 (r) register accessor: an alias for `Reg<ECC0_0_SPEC>`"]
pub type ECC0_0 = crate::Reg<ecc0_0::ECC0_0_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 0)"]
pub mod ecc0_0;
#[doc = "ECC1_0 (r) register accessor: an alias for `Reg<ECC1_0_SPEC>`"]
pub type ECC1_0 = crate::Reg<ecc1_0::ECC1_0_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 0)"]
pub mod ecc1_0;
#[doc = "ECC2_0 (r) register accessor: an alias for `Reg<ECC2_0_SPEC>`"]
pub type ECC2_0 = crate::Reg<ecc2_0::ECC2_0_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 0)"]
pub mod ecc2_0;
#[doc = "ECC3_0 (r) register accessor: an alias for `Reg<ECC3_0_SPEC>`"]
pub type ECC3_0 = crate::Reg<ecc3_0::ECC3_0_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 0)"]
pub mod ecc3_0;
#[doc = "ECC4_0 (r) register accessor: an alias for `Reg<ECC4_0_SPEC>`"]
pub type ECC4_0 = crate::Reg<ecc4_0::ECC4_0_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 0)"]
pub mod ecc4_0;
#[doc = "ECC5_0 (r) register accessor: an alias for `Reg<ECC5_0_SPEC>`"]
pub type ECC5_0 = crate::Reg<ecc5_0::ECC5_0_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 0)"]
pub mod ecc5_0;
#[doc = "ECC6_0 (r) register accessor: an alias for `Reg<ECC6_0_SPEC>`"]
pub type ECC6_0 = crate::Reg<ecc6_0::ECC6_0_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 0)"]
pub mod ecc6_0;
#[doc = "ECC7_0 (r) register accessor: an alias for `Reg<ECC7_0_SPEC>`"]
pub type ECC7_0 = crate::Reg<ecc7_0::ECC7_0_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 0)"]
pub mod ecc7_0;
#[doc = "ECC8_0 (r) register accessor: an alias for `Reg<ECC8_0_SPEC>`"]
pub type ECC8_0 = crate::Reg<ecc8_0::ECC8_0_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 0)"]
pub mod ecc8_0;
#[doc = "ECC9_0 (r) register accessor: an alias for `Reg<ECC9_0_SPEC>`"]
pub type ECC9_0 = crate::Reg<ecc9_0::ECC9_0_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 0)"]
pub mod ecc9_0;
#[doc = "ECC10_0 (r) register accessor: an alias for `Reg<ECC10_0_SPEC>`"]
pub type ECC10_0 = crate::Reg<ecc10_0::ECC10_0_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 0)"]
pub mod ecc10_0;
#[doc = "ECC0_1 (r) register accessor: an alias for `Reg<ECC0_1_SPEC>`"]
pub type ECC0_1 = crate::Reg<ecc0_1::ECC0_1_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 1)"]
pub mod ecc0_1;
#[doc = "ECC1_1 (r) register accessor: an alias for `Reg<ECC1_1_SPEC>`"]
pub type ECC1_1 = crate::Reg<ecc1_1::ECC1_1_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 1)"]
pub mod ecc1_1;
#[doc = "ECC2_1 (r) register accessor: an alias for `Reg<ECC2_1_SPEC>`"]
pub type ECC2_1 = crate::Reg<ecc2_1::ECC2_1_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 1)"]
pub mod ecc2_1;
#[doc = "ECC3_1 (r) register accessor: an alias for `Reg<ECC3_1_SPEC>`"]
pub type ECC3_1 = crate::Reg<ecc3_1::ECC3_1_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 1)"]
pub mod ecc3_1;
#[doc = "ECC4_1 (r) register accessor: an alias for `Reg<ECC4_1_SPEC>`"]
pub type ECC4_1 = crate::Reg<ecc4_1::ECC4_1_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 1)"]
pub mod ecc4_1;
#[doc = "ECC5_1 (r) register accessor: an alias for `Reg<ECC5_1_SPEC>`"]
pub type ECC5_1 = crate::Reg<ecc5_1::ECC5_1_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 1)"]
pub mod ecc5_1;
#[doc = "ECC6_1 (r) register accessor: an alias for `Reg<ECC6_1_SPEC>`"]
pub type ECC6_1 = crate::Reg<ecc6_1::ECC6_1_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 1)"]
pub mod ecc6_1;
#[doc = "ECC7_1 (r) register accessor: an alias for `Reg<ECC7_1_SPEC>`"]
pub type ECC7_1 = crate::Reg<ecc7_1::ECC7_1_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 1)"]
pub mod ecc7_1;
#[doc = "ECC8_1 (r) register accessor: an alias for `Reg<ECC8_1_SPEC>`"]
pub type ECC8_1 = crate::Reg<ecc8_1::ECC8_1_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 1)"]
pub mod ecc8_1;
#[doc = "ECC9_1 (r) register accessor: an alias for `Reg<ECC9_1_SPEC>`"]
pub type ECC9_1 = crate::Reg<ecc9_1::ECC9_1_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 1)"]
pub mod ecc9_1;
#[doc = "ECC10_1 (r) register accessor: an alias for `Reg<ECC10_1_SPEC>`"]
pub type ECC10_1 = crate::Reg<ecc10_1::ECC10_1_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 1)"]
pub mod ecc10_1;
#[doc = "ECC0_2 (r) register accessor: an alias for `Reg<ECC0_2_SPEC>`"]
pub type ECC0_2 = crate::Reg<ecc0_2::ECC0_2_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 2)"]
pub mod ecc0_2;
#[doc = "ECC1_2 (r) register accessor: an alias for `Reg<ECC1_2_SPEC>`"]
pub type ECC1_2 = crate::Reg<ecc1_2::ECC1_2_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 2)"]
pub mod ecc1_2;
#[doc = "ECC2_2 (r) register accessor: an alias for `Reg<ECC2_2_SPEC>`"]
pub type ECC2_2 = crate::Reg<ecc2_2::ECC2_2_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 2)"]
pub mod ecc2_2;
#[doc = "ECC3_2 (r) register accessor: an alias for `Reg<ECC3_2_SPEC>`"]
pub type ECC3_2 = crate::Reg<ecc3_2::ECC3_2_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 2)"]
pub mod ecc3_2;
#[doc = "ECC4_2 (r) register accessor: an alias for `Reg<ECC4_2_SPEC>`"]
pub type ECC4_2 = crate::Reg<ecc4_2::ECC4_2_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 2)"]
pub mod ecc4_2;
#[doc = "ECC5_2 (r) register accessor: an alias for `Reg<ECC5_2_SPEC>`"]
pub type ECC5_2 = crate::Reg<ecc5_2::ECC5_2_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 2)"]
pub mod ecc5_2;
#[doc = "ECC6_2 (r) register accessor: an alias for `Reg<ECC6_2_SPEC>`"]
pub type ECC6_2 = crate::Reg<ecc6_2::ECC6_2_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 2)"]
pub mod ecc6_2;
#[doc = "ECC7_2 (r) register accessor: an alias for `Reg<ECC7_2_SPEC>`"]
pub type ECC7_2 = crate::Reg<ecc7_2::ECC7_2_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 2)"]
pub mod ecc7_2;
#[doc = "ECC8_2 (r) register accessor: an alias for `Reg<ECC8_2_SPEC>`"]
pub type ECC8_2 = crate::Reg<ecc8_2::ECC8_2_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 2)"]
pub mod ecc8_2;
#[doc = "ECC9_2 (r) register accessor: an alias for `Reg<ECC9_2_SPEC>`"]
pub type ECC9_2 = crate::Reg<ecc9_2::ECC9_2_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 2)"]
pub mod ecc9_2;
#[doc = "ECC10_2 (r) register accessor: an alias for `Reg<ECC10_2_SPEC>`"]
pub type ECC10_2 = crate::Reg<ecc10_2::ECC10_2_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 2)"]
pub mod ecc10_2;
#[doc = "ECC0_3 (r) register accessor: an alias for `Reg<ECC0_3_SPEC>`"]
pub type ECC0_3 = crate::Reg<ecc0_3::ECC0_3_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 3)"]
pub mod ecc0_3;
#[doc = "ECC1_3 (r) register accessor: an alias for `Reg<ECC1_3_SPEC>`"]
pub type ECC1_3 = crate::Reg<ecc1_3::ECC1_3_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 3)"]
pub mod ecc1_3;
#[doc = "ECC2_3 (r) register accessor: an alias for `Reg<ECC2_3_SPEC>`"]
pub type ECC2_3 = crate::Reg<ecc2_3::ECC2_3_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 3)"]
pub mod ecc2_3;
#[doc = "ECC3_3 (r) register accessor: an alias for `Reg<ECC3_3_SPEC>`"]
pub type ECC3_3 = crate::Reg<ecc3_3::ECC3_3_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 3)"]
pub mod ecc3_3;
#[doc = "ECC4_3 (r) register accessor: an alias for `Reg<ECC4_3_SPEC>`"]
pub type ECC4_3 = crate::Reg<ecc4_3::ECC4_3_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 3)"]
pub mod ecc4_3;
#[doc = "ECC5_3 (r) register accessor: an alias for `Reg<ECC5_3_SPEC>`"]
pub type ECC5_3 = crate::Reg<ecc5_3::ECC5_3_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 3)"]
pub mod ecc5_3;
#[doc = "ECC6_3 (r) register accessor: an alias for `Reg<ECC6_3_SPEC>`"]
pub type ECC6_3 = crate::Reg<ecc6_3::ECC6_3_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 3)"]
pub mod ecc6_3;
#[doc = "ECC7_3 (r) register accessor: an alias for `Reg<ECC7_3_SPEC>`"]
pub type ECC7_3 = crate::Reg<ecc7_3::ECC7_3_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 3)"]
pub mod ecc7_3;
#[doc = "ECC8_3 (r) register accessor: an alias for `Reg<ECC8_3_SPEC>`"]
pub type ECC8_3 = crate::Reg<ecc8_3::ECC8_3_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 3)"]
pub mod ecc8_3;
#[doc = "ECC9_3 (r) register accessor: an alias for `Reg<ECC9_3_SPEC>`"]
pub type ECC9_3 = crate::Reg<ecc9_3::ECC9_3_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 3)"]
pub mod ecc9_3;
#[doc = "ECC10_3 (r) register accessor: an alias for `Reg<ECC10_3_SPEC>`"]
pub type ECC10_3 = crate::Reg<ecc10_3::ECC10_3_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 3)"]
pub mod ecc10_3;
#[doc = "ECC0_4 (r) register accessor: an alias for `Reg<ECC0_4_SPEC>`"]
pub type ECC0_4 = crate::Reg<ecc0_4::ECC0_4_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 4)"]
pub mod ecc0_4;
#[doc = "ECC1_4 (r) register accessor: an alias for `Reg<ECC1_4_SPEC>`"]
pub type ECC1_4 = crate::Reg<ecc1_4::ECC1_4_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 4)"]
pub mod ecc1_4;
#[doc = "ECC2_4 (r) register accessor: an alias for `Reg<ECC2_4_SPEC>`"]
pub type ECC2_4 = crate::Reg<ecc2_4::ECC2_4_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 4)"]
pub mod ecc2_4;
#[doc = "ECC3_4 (r) register accessor: an alias for `Reg<ECC3_4_SPEC>`"]
pub type ECC3_4 = crate::Reg<ecc3_4::ECC3_4_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 4)"]
pub mod ecc3_4;
#[doc = "ECC4_4 (r) register accessor: an alias for `Reg<ECC4_4_SPEC>`"]
pub type ECC4_4 = crate::Reg<ecc4_4::ECC4_4_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 4)"]
pub mod ecc4_4;
#[doc = "ECC5_4 (r) register accessor: an alias for `Reg<ECC5_4_SPEC>`"]
pub type ECC5_4 = crate::Reg<ecc5_4::ECC5_4_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 4)"]
pub mod ecc5_4;
#[doc = "ECC6_4 (r) register accessor: an alias for `Reg<ECC6_4_SPEC>`"]
pub type ECC6_4 = crate::Reg<ecc6_4::ECC6_4_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 4)"]
pub mod ecc6_4;
#[doc = "ECC7_4 (r) register accessor: an alias for `Reg<ECC7_4_SPEC>`"]
pub type ECC7_4 = crate::Reg<ecc7_4::ECC7_4_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 4)"]
pub mod ecc7_4;
#[doc = "ECC8_4 (r) register accessor: an alias for `Reg<ECC8_4_SPEC>`"]
pub type ECC8_4 = crate::Reg<ecc8_4::ECC8_4_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 4)"]
pub mod ecc8_4;
#[doc = "ECC9_4 (r) register accessor: an alias for `Reg<ECC9_4_SPEC>`"]
pub type ECC9_4 = crate::Reg<ecc9_4::ECC9_4_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 4)"]
pub mod ecc9_4;
#[doc = "ECC10_4 (r) register accessor: an alias for `Reg<ECC10_4_SPEC>`"]
pub type ECC10_4 = crate::Reg<ecc10_4::ECC10_4_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 4)"]
pub mod ecc10_4;
#[doc = "ECC0_5 (r) register accessor: an alias for `Reg<ECC0_5_SPEC>`"]
pub type ECC0_5 = crate::Reg<ecc0_5::ECC0_5_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 5)"]
pub mod ecc0_5;
#[doc = "ECC1_5 (r) register accessor: an alias for `Reg<ECC1_5_SPEC>`"]
pub type ECC1_5 = crate::Reg<ecc1_5::ECC1_5_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 5)"]
pub mod ecc1_5;
#[doc = "ECC2_5 (r) register accessor: an alias for `Reg<ECC2_5_SPEC>`"]
pub type ECC2_5 = crate::Reg<ecc2_5::ECC2_5_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 5)"]
pub mod ecc2_5;
#[doc = "ECC3_5 (r) register accessor: an alias for `Reg<ECC3_5_SPEC>`"]
pub type ECC3_5 = crate::Reg<ecc3_5::ECC3_5_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 5)"]
pub mod ecc3_5;
#[doc = "ECC4_5 (r) register accessor: an alias for `Reg<ECC4_5_SPEC>`"]
pub type ECC4_5 = crate::Reg<ecc4_5::ECC4_5_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 5)"]
pub mod ecc4_5;
#[doc = "ECC5_5 (r) register accessor: an alias for `Reg<ECC5_5_SPEC>`"]
pub type ECC5_5 = crate::Reg<ecc5_5::ECC5_5_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 5)"]
pub mod ecc5_5;
#[doc = "ECC6_5 (r) register accessor: an alias for `Reg<ECC6_5_SPEC>`"]
pub type ECC6_5 = crate::Reg<ecc6_5::ECC6_5_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 5)"]
pub mod ecc6_5;
#[doc = "ECC7_5 (r) register accessor: an alias for `Reg<ECC7_5_SPEC>`"]
pub type ECC7_5 = crate::Reg<ecc7_5::ECC7_5_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 5)"]
pub mod ecc7_5;
#[doc = "ECC8_5 (r) register accessor: an alias for `Reg<ECC8_5_SPEC>`"]
pub type ECC8_5 = crate::Reg<ecc8_5::ECC8_5_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 5)"]
pub mod ecc8_5;
#[doc = "ECC9_5 (r) register accessor: an alias for `Reg<ECC9_5_SPEC>`"]
pub type ECC9_5 = crate::Reg<ecc9_5::ECC9_5_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 5)"]
pub mod ecc9_5;
#[doc = "ECC10_5 (r) register accessor: an alias for `Reg<ECC10_5_SPEC>`"]
pub type ECC10_5 = crate::Reg<ecc10_5::ECC10_5_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 5)"]
pub mod ecc10_5;
#[doc = "ECC0_6 (r) register accessor: an alias for `Reg<ECC0_6_SPEC>`"]
pub type ECC0_6 = crate::Reg<ecc0_6::ECC0_6_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 6)"]
pub mod ecc0_6;
#[doc = "ECC1_6 (r) register accessor: an alias for `Reg<ECC1_6_SPEC>`"]
pub type ECC1_6 = crate::Reg<ecc1_6::ECC1_6_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 6)"]
pub mod ecc1_6;
#[doc = "ECC2_6 (r) register accessor: an alias for `Reg<ECC2_6_SPEC>`"]
pub type ECC2_6 = crate::Reg<ecc2_6::ECC2_6_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 6)"]
pub mod ecc2_6;
#[doc = "ECC3_6 (r) register accessor: an alias for `Reg<ECC3_6_SPEC>`"]
pub type ECC3_6 = crate::Reg<ecc3_6::ECC3_6_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 6)"]
pub mod ecc3_6;
#[doc = "ECC4_6 (r) register accessor: an alias for `Reg<ECC4_6_SPEC>`"]
pub type ECC4_6 = crate::Reg<ecc4_6::ECC4_6_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 6)"]
pub mod ecc4_6;
#[doc = "ECC5_6 (r) register accessor: an alias for `Reg<ECC5_6_SPEC>`"]
pub type ECC5_6 = crate::Reg<ecc5_6::ECC5_6_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 6)"]
pub mod ecc5_6;
#[doc = "ECC6_6 (r) register accessor: an alias for `Reg<ECC6_6_SPEC>`"]
pub type ECC6_6 = crate::Reg<ecc6_6::ECC6_6_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 6)"]
pub mod ecc6_6;
#[doc = "ECC7_6 (r) register accessor: an alias for `Reg<ECC7_6_SPEC>`"]
pub type ECC7_6 = crate::Reg<ecc7_6::ECC7_6_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 6)"]
pub mod ecc7_6;
#[doc = "ECC8_6 (r) register accessor: an alias for `Reg<ECC8_6_SPEC>`"]
pub type ECC8_6 = crate::Reg<ecc8_6::ECC8_6_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 6)"]
pub mod ecc8_6;
#[doc = "ECC9_6 (r) register accessor: an alias for `Reg<ECC9_6_SPEC>`"]
pub type ECC9_6 = crate::Reg<ecc9_6::ECC9_6_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 6)"]
pub mod ecc9_6;
#[doc = "ECC10_6 (r) register accessor: an alias for `Reg<ECC10_6_SPEC>`"]
pub type ECC10_6 = crate::Reg<ecc10_6::ECC10_6_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 6)"]
pub mod ecc10_6;
#[doc = "ECC0_7 (r) register accessor: an alias for `Reg<ECC0_7_SPEC>`"]
pub type ECC0_7 = crate::Reg<ecc0_7::ECC0_7_SPEC>;
#[doc = "PMECC ECC 0 Register (sec_num = 7)"]
pub mod ecc0_7;
#[doc = "ECC1_7 (r) register accessor: an alias for `Reg<ECC1_7_SPEC>`"]
pub type ECC1_7 = crate::Reg<ecc1_7::ECC1_7_SPEC>;
#[doc = "PMECC ECC 1 Register (sec_num = 7)"]
pub mod ecc1_7;
#[doc = "ECC2_7 (r) register accessor: an alias for `Reg<ECC2_7_SPEC>`"]
pub type ECC2_7 = crate::Reg<ecc2_7::ECC2_7_SPEC>;
#[doc = "PMECC ECC 2 Register (sec_num = 7)"]
pub mod ecc2_7;
#[doc = "ECC3_7 (r) register accessor: an alias for `Reg<ECC3_7_SPEC>`"]
pub type ECC3_7 = crate::Reg<ecc3_7::ECC3_7_SPEC>;
#[doc = "PMECC ECC 3 Register (sec_num = 7)"]
pub mod ecc3_7;
#[doc = "ECC4_7 (r) register accessor: an alias for `Reg<ECC4_7_SPEC>`"]
pub type ECC4_7 = crate::Reg<ecc4_7::ECC4_7_SPEC>;
#[doc = "PMECC ECC 4 Register (sec_num = 7)"]
pub mod ecc4_7;
#[doc = "ECC5_7 (r) register accessor: an alias for `Reg<ECC5_7_SPEC>`"]
pub type ECC5_7 = crate::Reg<ecc5_7::ECC5_7_SPEC>;
#[doc = "PMECC ECC 5 Register (sec_num = 7)"]
pub mod ecc5_7;
#[doc = "ECC6_7 (r) register accessor: an alias for `Reg<ECC6_7_SPEC>`"]
pub type ECC6_7 = crate::Reg<ecc6_7::ECC6_7_SPEC>;
#[doc = "PMECC ECC 6 Register (sec_num = 7)"]
pub mod ecc6_7;
#[doc = "ECC7_7 (r) register accessor: an alias for `Reg<ECC7_7_SPEC>`"]
pub type ECC7_7 = crate::Reg<ecc7_7::ECC7_7_SPEC>;
#[doc = "PMECC ECC 7 Register (sec_num = 7)"]
pub mod ecc7_7;
#[doc = "ECC8_7 (r) register accessor: an alias for `Reg<ECC8_7_SPEC>`"]
pub type ECC8_7 = crate::Reg<ecc8_7::ECC8_7_SPEC>;
#[doc = "PMECC ECC 8 Register (sec_num = 7)"]
pub mod ecc8_7;
#[doc = "ECC9_7 (r) register accessor: an alias for `Reg<ECC9_7_SPEC>`"]
pub type ECC9_7 = crate::Reg<ecc9_7::ECC9_7_SPEC>;
#[doc = "PMECC ECC 9 Register (sec_num = 7)"]
pub mod ecc9_7;
#[doc = "ECC10_7 (r) register accessor: an alias for `Reg<ECC10_7_SPEC>`"]
pub type ECC10_7 = crate::Reg<ecc10_7::ECC10_7_SPEC>;
#[doc = "PMECC ECC 10 Register (sec_num = 7)"]
pub mod ecc10_7;
#[doc = "REM0_0 (r) register accessor: an alias for `Reg<REM0_0_SPEC>`"]
pub type REM0_0 = crate::Reg<rem0_0::REM0_0_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 0)"]
pub mod rem0_0;
#[doc = "REM1_0 (r) register accessor: an alias for `Reg<REM1_0_SPEC>`"]
pub type REM1_0 = crate::Reg<rem1_0::REM1_0_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 0)"]
pub mod rem1_0;
#[doc = "REM2_0 (r) register accessor: an alias for `Reg<REM2_0_SPEC>`"]
pub type REM2_0 = crate::Reg<rem2_0::REM2_0_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 0)"]
pub mod rem2_0;
#[doc = "REM3_0 (r) register accessor: an alias for `Reg<REM3_0_SPEC>`"]
pub type REM3_0 = crate::Reg<rem3_0::REM3_0_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 0)"]
pub mod rem3_0;
#[doc = "REM4_0 (r) register accessor: an alias for `Reg<REM4_0_SPEC>`"]
pub type REM4_0 = crate::Reg<rem4_0::REM4_0_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 0)"]
pub mod rem4_0;
#[doc = "REM5_0 (r) register accessor: an alias for `Reg<REM5_0_SPEC>`"]
pub type REM5_0 = crate::Reg<rem5_0::REM5_0_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 0)"]
pub mod rem5_0;
#[doc = "REM6_0 (r) register accessor: an alias for `Reg<REM6_0_SPEC>`"]
pub type REM6_0 = crate::Reg<rem6_0::REM6_0_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 0)"]
pub mod rem6_0;
#[doc = "REM7_0 (r) register accessor: an alias for `Reg<REM7_0_SPEC>`"]
pub type REM7_0 = crate::Reg<rem7_0::REM7_0_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 0)"]
pub mod rem7_0;
#[doc = "REM8_0 (r) register accessor: an alias for `Reg<REM8_0_SPEC>`"]
pub type REM8_0 = crate::Reg<rem8_0::REM8_0_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 0)"]
pub mod rem8_0;
#[doc = "REM9_0 (r) register accessor: an alias for `Reg<REM9_0_SPEC>`"]
pub type REM9_0 = crate::Reg<rem9_0::REM9_0_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 0)"]
pub mod rem9_0;
#[doc = "REM10_0 (r) register accessor: an alias for `Reg<REM10_0_SPEC>`"]
pub type REM10_0 = crate::Reg<rem10_0::REM10_0_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 0)"]
pub mod rem10_0;
#[doc = "REM11_0 (r) register accessor: an alias for `Reg<REM11_0_SPEC>`"]
pub type REM11_0 = crate::Reg<rem11_0::REM11_0_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 0)"]
pub mod rem11_0;
#[doc = "REM0_1 (r) register accessor: an alias for `Reg<REM0_1_SPEC>`"]
pub type REM0_1 = crate::Reg<rem0_1::REM0_1_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 1)"]
pub mod rem0_1;
#[doc = "REM1_1 (r) register accessor: an alias for `Reg<REM1_1_SPEC>`"]
pub type REM1_1 = crate::Reg<rem1_1::REM1_1_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 1)"]
pub mod rem1_1;
#[doc = "REM2_1 (r) register accessor: an alias for `Reg<REM2_1_SPEC>`"]
pub type REM2_1 = crate::Reg<rem2_1::REM2_1_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 1)"]
pub mod rem2_1;
#[doc = "REM3_1 (r) register accessor: an alias for `Reg<REM3_1_SPEC>`"]
pub type REM3_1 = crate::Reg<rem3_1::REM3_1_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 1)"]
pub mod rem3_1;
#[doc = "REM4_1 (r) register accessor: an alias for `Reg<REM4_1_SPEC>`"]
pub type REM4_1 = crate::Reg<rem4_1::REM4_1_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 1)"]
pub mod rem4_1;
#[doc = "REM5_1 (r) register accessor: an alias for `Reg<REM5_1_SPEC>`"]
pub type REM5_1 = crate::Reg<rem5_1::REM5_1_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 1)"]
pub mod rem5_1;
#[doc = "REM6_1 (r) register accessor: an alias for `Reg<REM6_1_SPEC>`"]
pub type REM6_1 = crate::Reg<rem6_1::REM6_1_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 1)"]
pub mod rem6_1;
#[doc = "REM7_1 (r) register accessor: an alias for `Reg<REM7_1_SPEC>`"]
pub type REM7_1 = crate::Reg<rem7_1::REM7_1_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 1)"]
pub mod rem7_1;
#[doc = "REM8_1 (r) register accessor: an alias for `Reg<REM8_1_SPEC>`"]
pub type REM8_1 = crate::Reg<rem8_1::REM8_1_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 1)"]
pub mod rem8_1;
#[doc = "REM9_1 (r) register accessor: an alias for `Reg<REM9_1_SPEC>`"]
pub type REM9_1 = crate::Reg<rem9_1::REM9_1_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 1)"]
pub mod rem9_1;
#[doc = "REM10_1 (r) register accessor: an alias for `Reg<REM10_1_SPEC>`"]
pub type REM10_1 = crate::Reg<rem10_1::REM10_1_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 1)"]
pub mod rem10_1;
#[doc = "REM11_1 (r) register accessor: an alias for `Reg<REM11_1_SPEC>`"]
pub type REM11_1 = crate::Reg<rem11_1::REM11_1_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 1)"]
pub mod rem11_1;
#[doc = "REM0_2 (r) register accessor: an alias for `Reg<REM0_2_SPEC>`"]
pub type REM0_2 = crate::Reg<rem0_2::REM0_2_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 2)"]
pub mod rem0_2;
#[doc = "REM1_2 (r) register accessor: an alias for `Reg<REM1_2_SPEC>`"]
pub type REM1_2 = crate::Reg<rem1_2::REM1_2_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 2)"]
pub mod rem1_2;
#[doc = "REM2_2 (r) register accessor: an alias for `Reg<REM2_2_SPEC>`"]
pub type REM2_2 = crate::Reg<rem2_2::REM2_2_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 2)"]
pub mod rem2_2;
#[doc = "REM3_2 (r) register accessor: an alias for `Reg<REM3_2_SPEC>`"]
pub type REM3_2 = crate::Reg<rem3_2::REM3_2_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 2)"]
pub mod rem3_2;
#[doc = "REM4_2 (r) register accessor: an alias for `Reg<REM4_2_SPEC>`"]
pub type REM4_2 = crate::Reg<rem4_2::REM4_2_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 2)"]
pub mod rem4_2;
#[doc = "REM5_2 (r) register accessor: an alias for `Reg<REM5_2_SPEC>`"]
pub type REM5_2 = crate::Reg<rem5_2::REM5_2_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 2)"]
pub mod rem5_2;
#[doc = "REM6_2 (r) register accessor: an alias for `Reg<REM6_2_SPEC>`"]
pub type REM6_2 = crate::Reg<rem6_2::REM6_2_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 2)"]
pub mod rem6_2;
#[doc = "REM7_2 (r) register accessor: an alias for `Reg<REM7_2_SPEC>`"]
pub type REM7_2 = crate::Reg<rem7_2::REM7_2_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 2)"]
pub mod rem7_2;
#[doc = "REM8_2 (r) register accessor: an alias for `Reg<REM8_2_SPEC>`"]
pub type REM8_2 = crate::Reg<rem8_2::REM8_2_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 2)"]
pub mod rem8_2;
#[doc = "REM9_2 (r) register accessor: an alias for `Reg<REM9_2_SPEC>`"]
pub type REM9_2 = crate::Reg<rem9_2::REM9_2_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 2)"]
pub mod rem9_2;
#[doc = "REM10_2 (r) register accessor: an alias for `Reg<REM10_2_SPEC>`"]
pub type REM10_2 = crate::Reg<rem10_2::REM10_2_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 2)"]
pub mod rem10_2;
#[doc = "REM11_2 (r) register accessor: an alias for `Reg<REM11_2_SPEC>`"]
pub type REM11_2 = crate::Reg<rem11_2::REM11_2_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 2)"]
pub mod rem11_2;
#[doc = "REM0_3 (r) register accessor: an alias for `Reg<REM0_3_SPEC>`"]
pub type REM0_3 = crate::Reg<rem0_3::REM0_3_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 3)"]
pub mod rem0_3;
#[doc = "REM1_3 (r) register accessor: an alias for `Reg<REM1_3_SPEC>`"]
pub type REM1_3 = crate::Reg<rem1_3::REM1_3_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 3)"]
pub mod rem1_3;
#[doc = "REM2_3 (r) register accessor: an alias for `Reg<REM2_3_SPEC>`"]
pub type REM2_3 = crate::Reg<rem2_3::REM2_3_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 3)"]
pub mod rem2_3;
#[doc = "REM3_3 (r) register accessor: an alias for `Reg<REM3_3_SPEC>`"]
pub type REM3_3 = crate::Reg<rem3_3::REM3_3_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 3)"]
pub mod rem3_3;
#[doc = "REM4_3 (r) register accessor: an alias for `Reg<REM4_3_SPEC>`"]
pub type REM4_3 = crate::Reg<rem4_3::REM4_3_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 3)"]
pub mod rem4_3;
#[doc = "REM5_3 (r) register accessor: an alias for `Reg<REM5_3_SPEC>`"]
pub type REM5_3 = crate::Reg<rem5_3::REM5_3_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 3)"]
pub mod rem5_3;
#[doc = "REM6_3 (r) register accessor: an alias for `Reg<REM6_3_SPEC>`"]
pub type REM6_3 = crate::Reg<rem6_3::REM6_3_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 3)"]
pub mod rem6_3;
#[doc = "REM7_3 (r) register accessor: an alias for `Reg<REM7_3_SPEC>`"]
pub type REM7_3 = crate::Reg<rem7_3::REM7_3_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 3)"]
pub mod rem7_3;
#[doc = "REM8_3 (r) register accessor: an alias for `Reg<REM8_3_SPEC>`"]
pub type REM8_3 = crate::Reg<rem8_3::REM8_3_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 3)"]
pub mod rem8_3;
#[doc = "REM9_3 (r) register accessor: an alias for `Reg<REM9_3_SPEC>`"]
pub type REM9_3 = crate::Reg<rem9_3::REM9_3_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 3)"]
pub mod rem9_3;
#[doc = "REM10_3 (r) register accessor: an alias for `Reg<REM10_3_SPEC>`"]
pub type REM10_3 = crate::Reg<rem10_3::REM10_3_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 3)"]
pub mod rem10_3;
#[doc = "REM11_3 (r) register accessor: an alias for `Reg<REM11_3_SPEC>`"]
pub type REM11_3 = crate::Reg<rem11_3::REM11_3_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 3)"]
pub mod rem11_3;
#[doc = "REM0_4 (r) register accessor: an alias for `Reg<REM0_4_SPEC>`"]
pub type REM0_4 = crate::Reg<rem0_4::REM0_4_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 4)"]
pub mod rem0_4;
#[doc = "REM1_4 (r) register accessor: an alias for `Reg<REM1_4_SPEC>`"]
pub type REM1_4 = crate::Reg<rem1_4::REM1_4_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 4)"]
pub mod rem1_4;
#[doc = "REM2_4 (r) register accessor: an alias for `Reg<REM2_4_SPEC>`"]
pub type REM2_4 = crate::Reg<rem2_4::REM2_4_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 4)"]
pub mod rem2_4;
#[doc = "REM3_4 (r) register accessor: an alias for `Reg<REM3_4_SPEC>`"]
pub type REM3_4 = crate::Reg<rem3_4::REM3_4_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 4)"]
pub mod rem3_4;
#[doc = "REM4_4 (r) register accessor: an alias for `Reg<REM4_4_SPEC>`"]
pub type REM4_4 = crate::Reg<rem4_4::REM4_4_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 4)"]
pub mod rem4_4;
#[doc = "REM5_4 (r) register accessor: an alias for `Reg<REM5_4_SPEC>`"]
pub type REM5_4 = crate::Reg<rem5_4::REM5_4_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 4)"]
pub mod rem5_4;
#[doc = "REM6_4 (r) register accessor: an alias for `Reg<REM6_4_SPEC>`"]
pub type REM6_4 = crate::Reg<rem6_4::REM6_4_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 4)"]
pub mod rem6_4;
#[doc = "REM7_4 (r) register accessor: an alias for `Reg<REM7_4_SPEC>`"]
pub type REM7_4 = crate::Reg<rem7_4::REM7_4_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 4)"]
pub mod rem7_4;
#[doc = "REM8_4 (r) register accessor: an alias for `Reg<REM8_4_SPEC>`"]
pub type REM8_4 = crate::Reg<rem8_4::REM8_4_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 4)"]
pub mod rem8_4;
#[doc = "REM9_4 (r) register accessor: an alias for `Reg<REM9_4_SPEC>`"]
pub type REM9_4 = crate::Reg<rem9_4::REM9_4_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 4)"]
pub mod rem9_4;
#[doc = "REM10_4 (r) register accessor: an alias for `Reg<REM10_4_SPEC>`"]
pub type REM10_4 = crate::Reg<rem10_4::REM10_4_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 4)"]
pub mod rem10_4;
#[doc = "REM11_4 (r) register accessor: an alias for `Reg<REM11_4_SPEC>`"]
pub type REM11_4 = crate::Reg<rem11_4::REM11_4_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 4)"]
pub mod rem11_4;
#[doc = "REM0_5 (r) register accessor: an alias for `Reg<REM0_5_SPEC>`"]
pub type REM0_5 = crate::Reg<rem0_5::REM0_5_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 5)"]
pub mod rem0_5;
#[doc = "REM1_5 (r) register accessor: an alias for `Reg<REM1_5_SPEC>`"]
pub type REM1_5 = crate::Reg<rem1_5::REM1_5_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 5)"]
pub mod rem1_5;
#[doc = "REM2_5 (r) register accessor: an alias for `Reg<REM2_5_SPEC>`"]
pub type REM2_5 = crate::Reg<rem2_5::REM2_5_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 5)"]
pub mod rem2_5;
#[doc = "REM3_5 (r) register accessor: an alias for `Reg<REM3_5_SPEC>`"]
pub type REM3_5 = crate::Reg<rem3_5::REM3_5_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 5)"]
pub mod rem3_5;
#[doc = "REM4_5 (r) register accessor: an alias for `Reg<REM4_5_SPEC>`"]
pub type REM4_5 = crate::Reg<rem4_5::REM4_5_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 5)"]
pub mod rem4_5;
#[doc = "REM5_5 (r) register accessor: an alias for `Reg<REM5_5_SPEC>`"]
pub type REM5_5 = crate::Reg<rem5_5::REM5_5_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 5)"]
pub mod rem5_5;
#[doc = "REM6_5 (r) register accessor: an alias for `Reg<REM6_5_SPEC>`"]
pub type REM6_5 = crate::Reg<rem6_5::REM6_5_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 5)"]
pub mod rem6_5;
#[doc = "REM7_5 (r) register accessor: an alias for `Reg<REM7_5_SPEC>`"]
pub type REM7_5 = crate::Reg<rem7_5::REM7_5_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 5)"]
pub mod rem7_5;
#[doc = "REM8_5 (r) register accessor: an alias for `Reg<REM8_5_SPEC>`"]
pub type REM8_5 = crate::Reg<rem8_5::REM8_5_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 5)"]
pub mod rem8_5;
#[doc = "REM9_5 (r) register accessor: an alias for `Reg<REM9_5_SPEC>`"]
pub type REM9_5 = crate::Reg<rem9_5::REM9_5_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 5)"]
pub mod rem9_5;
#[doc = "REM10_5 (r) register accessor: an alias for `Reg<REM10_5_SPEC>`"]
pub type REM10_5 = crate::Reg<rem10_5::REM10_5_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 5)"]
pub mod rem10_5;
#[doc = "REM11_5 (r) register accessor: an alias for `Reg<REM11_5_SPEC>`"]
pub type REM11_5 = crate::Reg<rem11_5::REM11_5_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 5)"]
pub mod rem11_5;
#[doc = "REM0_6 (r) register accessor: an alias for `Reg<REM0_6_SPEC>`"]
pub type REM0_6 = crate::Reg<rem0_6::REM0_6_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 6)"]
pub mod rem0_6;
#[doc = "REM1_6 (r) register accessor: an alias for `Reg<REM1_6_SPEC>`"]
pub type REM1_6 = crate::Reg<rem1_6::REM1_6_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 6)"]
pub mod rem1_6;
#[doc = "REM2_6 (r) register accessor: an alias for `Reg<REM2_6_SPEC>`"]
pub type REM2_6 = crate::Reg<rem2_6::REM2_6_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 6)"]
pub mod rem2_6;
#[doc = "REM3_6 (r) register accessor: an alias for `Reg<REM3_6_SPEC>`"]
pub type REM3_6 = crate::Reg<rem3_6::REM3_6_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 6)"]
pub mod rem3_6;
#[doc = "REM4_6 (r) register accessor: an alias for `Reg<REM4_6_SPEC>`"]
pub type REM4_6 = crate::Reg<rem4_6::REM4_6_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 6)"]
pub mod rem4_6;
#[doc = "REM5_6 (r) register accessor: an alias for `Reg<REM5_6_SPEC>`"]
pub type REM5_6 = crate::Reg<rem5_6::REM5_6_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 6)"]
pub mod rem5_6;
#[doc = "REM6_6 (r) register accessor: an alias for `Reg<REM6_6_SPEC>`"]
pub type REM6_6 = crate::Reg<rem6_6::REM6_6_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 6)"]
pub mod rem6_6;
#[doc = "REM7_6 (r) register accessor: an alias for `Reg<REM7_6_SPEC>`"]
pub type REM7_6 = crate::Reg<rem7_6::REM7_6_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 6)"]
pub mod rem7_6;
#[doc = "REM8_6 (r) register accessor: an alias for `Reg<REM8_6_SPEC>`"]
pub type REM8_6 = crate::Reg<rem8_6::REM8_6_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 6)"]
pub mod rem8_6;
#[doc = "REM9_6 (r) register accessor: an alias for `Reg<REM9_6_SPEC>`"]
pub type REM9_6 = crate::Reg<rem9_6::REM9_6_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 6)"]
pub mod rem9_6;
#[doc = "REM10_6 (r) register accessor: an alias for `Reg<REM10_6_SPEC>`"]
pub type REM10_6 = crate::Reg<rem10_6::REM10_6_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 6)"]
pub mod rem10_6;
#[doc = "REM11_6 (r) register accessor: an alias for `Reg<REM11_6_SPEC>`"]
pub type REM11_6 = crate::Reg<rem11_6::REM11_6_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 6)"]
pub mod rem11_6;
#[doc = "REM0_7 (r) register accessor: an alias for `Reg<REM0_7_SPEC>`"]
pub type REM0_7 = crate::Reg<rem0_7::REM0_7_SPEC>;
#[doc = "PMECC REM 0 Register (sec_num = 7)"]
pub mod rem0_7;
#[doc = "REM1_7 (r) register accessor: an alias for `Reg<REM1_7_SPEC>`"]
pub type REM1_7 = crate::Reg<rem1_7::REM1_7_SPEC>;
#[doc = "PMECC REM 1 Register (sec_num = 7)"]
pub mod rem1_7;
#[doc = "REM2_7 (r) register accessor: an alias for `Reg<REM2_7_SPEC>`"]
pub type REM2_7 = crate::Reg<rem2_7::REM2_7_SPEC>;
#[doc = "PMECC REM 2 Register (sec_num = 7)"]
pub mod rem2_7;
#[doc = "REM3_7 (r) register accessor: an alias for `Reg<REM3_7_SPEC>`"]
pub type REM3_7 = crate::Reg<rem3_7::REM3_7_SPEC>;
#[doc = "PMECC REM 3 Register (sec_num = 7)"]
pub mod rem3_7;
#[doc = "REM4_7 (r) register accessor: an alias for `Reg<REM4_7_SPEC>`"]
pub type REM4_7 = crate::Reg<rem4_7::REM4_7_SPEC>;
#[doc = "PMECC REM 4 Register (sec_num = 7)"]
pub mod rem4_7;
#[doc = "REM5_7 (r) register accessor: an alias for `Reg<REM5_7_SPEC>`"]
pub type REM5_7 = crate::Reg<rem5_7::REM5_7_SPEC>;
#[doc = "PMECC REM 5 Register (sec_num = 7)"]
pub mod rem5_7;
#[doc = "REM6_7 (r) register accessor: an alias for `Reg<REM6_7_SPEC>`"]
pub type REM6_7 = crate::Reg<rem6_7::REM6_7_SPEC>;
#[doc = "PMECC REM 6 Register (sec_num = 7)"]
pub mod rem6_7;
#[doc = "REM7_7 (r) register accessor: an alias for `Reg<REM7_7_SPEC>`"]
pub type REM7_7 = crate::Reg<rem7_7::REM7_7_SPEC>;
#[doc = "PMECC REM 7 Register (sec_num = 7)"]
pub mod rem7_7;
#[doc = "REM8_7 (r) register accessor: an alias for `Reg<REM8_7_SPEC>`"]
pub type REM8_7 = crate::Reg<rem8_7::REM8_7_SPEC>;
#[doc = "PMECC REM 8 Register (sec_num = 7)"]
pub mod rem8_7;
#[doc = "REM9_7 (r) register accessor: an alias for `Reg<REM9_7_SPEC>`"]
pub type REM9_7 = crate::Reg<rem9_7::REM9_7_SPEC>;
#[doc = "PMECC REM 9 Register (sec_num = 7)"]
pub mod rem9_7;
#[doc = "REM10_7 (r) register accessor: an alias for `Reg<REM10_7_SPEC>`"]
pub type REM10_7 = crate::Reg<rem10_7::REM10_7_SPEC>;
#[doc = "PMECC REM 10 Register (sec_num = 7)"]
pub mod rem10_7;
#[doc = "REM11_7 (r) register accessor: an alias for `Reg<REM11_7_SPEC>`"]
pub type REM11_7 = crate::Reg<rem11_7::REM11_7_SPEC>;
#[doc = "PMECC REM 11 Register (sec_num = 7)"]
pub mod rem11_7;
