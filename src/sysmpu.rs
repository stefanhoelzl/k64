#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control/Error Status Register"]
    pub cesr: CESR,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Error Address Register, slave port n"]
    pub ear0: EAR,
    #[doc = "0x14 - Error Detail Register, slave port n"]
    pub edr0: EDR,
    #[doc = "0x18 - Error Address Register, slave port n"]
    pub ear1: EAR,
    #[doc = "0x1c - Error Detail Register, slave port n"]
    pub edr1: EDR,
    #[doc = "0x20 - Error Address Register, slave port n"]
    pub ear2: EAR,
    #[doc = "0x24 - Error Detail Register, slave port n"]
    pub edr2: EDR,
    #[doc = "0x28 - Error Address Register, slave port n"]
    pub ear3: EAR,
    #[doc = "0x2c - Error Detail Register, slave port n"]
    pub edr3: EDR,
    #[doc = "0x30 - Error Address Register, slave port n"]
    pub ear4: EAR,
    #[doc = "0x34 - Error Detail Register, slave port n"]
    pub edr4: EDR,
    _reserved1: [u8; 968usize],
    #[doc = "0x400 - Region Descriptor n, Word 0"]
    pub rgd0_word0: RGD_WORD0,
    #[doc = "0x404 - Region Descriptor n, Word 1"]
    pub rgd0_word1: RGD_WORD1,
    #[doc = "0x408 - Region Descriptor n, Word 2"]
    pub rgd0_word2: RGD_WORD2,
    #[doc = "0x40c - Region Descriptor n, Word 3"]
    pub rgd0_word3: RGD_WORD3,
    #[doc = "0x410 - Region Descriptor n, Word 0"]
    pub rgd1_word0: RGD_WORD0,
    #[doc = "0x414 - Region Descriptor n, Word 1"]
    pub rgd1_word1: RGD_WORD1,
    #[doc = "0x418 - Region Descriptor n, Word 2"]
    pub rgd1_word2: RGD_WORD2,
    #[doc = "0x41c - Region Descriptor n, Word 3"]
    pub rgd1_word3: RGD_WORD3,
    #[doc = "0x420 - Region Descriptor n, Word 0"]
    pub rgd2_word0: RGD_WORD0,
    #[doc = "0x424 - Region Descriptor n, Word 1"]
    pub rgd2_word1: RGD_WORD1,
    #[doc = "0x428 - Region Descriptor n, Word 2"]
    pub rgd2_word2: RGD_WORD2,
    #[doc = "0x42c - Region Descriptor n, Word 3"]
    pub rgd2_word3: RGD_WORD3,
    #[doc = "0x430 - Region Descriptor n, Word 0"]
    pub rgd3_word0: RGD_WORD0,
    #[doc = "0x434 - Region Descriptor n, Word 1"]
    pub rgd3_word1: RGD_WORD1,
    #[doc = "0x438 - Region Descriptor n, Word 2"]
    pub rgd3_word2: RGD_WORD2,
    #[doc = "0x43c - Region Descriptor n, Word 3"]
    pub rgd3_word3: RGD_WORD3,
    #[doc = "0x440 - Region Descriptor n, Word 0"]
    pub rgd4_word0: RGD_WORD0,
    #[doc = "0x444 - Region Descriptor n, Word 1"]
    pub rgd4_word1: RGD_WORD1,
    #[doc = "0x448 - Region Descriptor n, Word 2"]
    pub rgd4_word2: RGD_WORD2,
    #[doc = "0x44c - Region Descriptor n, Word 3"]
    pub rgd4_word3: RGD_WORD3,
    #[doc = "0x450 - Region Descriptor n, Word 0"]
    pub rgd5_word0: RGD_WORD0,
    #[doc = "0x454 - Region Descriptor n, Word 1"]
    pub rgd5_word1: RGD_WORD1,
    #[doc = "0x458 - Region Descriptor n, Word 2"]
    pub rgd5_word2: RGD_WORD2,
    #[doc = "0x45c - Region Descriptor n, Word 3"]
    pub rgd5_word3: RGD_WORD3,
    #[doc = "0x460 - Region Descriptor n, Word 0"]
    pub rgd6_word0: RGD_WORD0,
    #[doc = "0x464 - Region Descriptor n, Word 1"]
    pub rgd6_word1: RGD_WORD1,
    #[doc = "0x468 - Region Descriptor n, Word 2"]
    pub rgd6_word2: RGD_WORD2,
    #[doc = "0x46c - Region Descriptor n, Word 3"]
    pub rgd6_word3: RGD_WORD3,
    #[doc = "0x470 - Region Descriptor n, Word 0"]
    pub rgd7_word0: RGD_WORD0,
    #[doc = "0x474 - Region Descriptor n, Word 1"]
    pub rgd7_word1: RGD_WORD1,
    #[doc = "0x478 - Region Descriptor n, Word 2"]
    pub rgd7_word2: RGD_WORD2,
    #[doc = "0x47c - Region Descriptor n, Word 3"]
    pub rgd7_word3: RGD_WORD3,
    #[doc = "0x480 - Region Descriptor n, Word 0"]
    pub rgd8_word0: RGD_WORD0,
    #[doc = "0x484 - Region Descriptor n, Word 1"]
    pub rgd8_word1: RGD_WORD1,
    #[doc = "0x488 - Region Descriptor n, Word 2"]
    pub rgd8_word2: RGD_WORD2,
    #[doc = "0x48c - Region Descriptor n, Word 3"]
    pub rgd8_word3: RGD_WORD3,
    #[doc = "0x490 - Region Descriptor n, Word 0"]
    pub rgd9_word0: RGD_WORD0,
    #[doc = "0x494 - Region Descriptor n, Word 1"]
    pub rgd9_word1: RGD_WORD1,
    #[doc = "0x498 - Region Descriptor n, Word 2"]
    pub rgd9_word2: RGD_WORD2,
    #[doc = "0x49c - Region Descriptor n, Word 3"]
    pub rgd9_word3: RGD_WORD3,
    #[doc = "0x4a0 - Region Descriptor n, Word 0"]
    pub rgd10_word0: RGD_WORD0,
    #[doc = "0x4a4 - Region Descriptor n, Word 1"]
    pub rgd10_word1: RGD_WORD1,
    #[doc = "0x4a8 - Region Descriptor n, Word 2"]
    pub rgd10_word2: RGD_WORD2,
    #[doc = "0x4ac - Region Descriptor n, Word 3"]
    pub rgd10_word3: RGD_WORD3,
    #[doc = "0x4b0 - Region Descriptor n, Word 0"]
    pub rgd11_word0: RGD_WORD0,
    #[doc = "0x4b4 - Region Descriptor n, Word 1"]
    pub rgd11_word1: RGD_WORD1,
    #[doc = "0x4b8 - Region Descriptor n, Word 2"]
    pub rgd11_word2: RGD_WORD2,
    #[doc = "0x4bc - Region Descriptor n, Word 3"]
    pub rgd11_word3: RGD_WORD3,
    _reserved2: [u8; 832usize],
    #[doc = "0x800 - Region Descriptor Alternate Access Control n"]
    pub rgdaac: [RGDAAC; 12],
}
#[doc = "Control/Error Status Register"]
pub struct CESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control/Error Status Register"]
pub mod cesr;
#[doc = "Error Address Register, slave port n"]
pub struct EAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Address Register, slave port n"]
pub mod ear;
#[doc = "Error Detail Register, slave port n"]
pub struct EDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Detail Register, slave port n"]
pub mod edr;
#[doc = "Region Descriptor n, Word 0"]
pub struct RGD_WORD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor n, Word 0"]
pub mod rgd_word0;
#[doc = "Region Descriptor n, Word 1"]
pub struct RGD_WORD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor n, Word 1"]
pub mod rgd_word1;
#[doc = "Region Descriptor n, Word 2"]
pub struct RGD_WORD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor n, Word 2"]
pub mod rgd_word2;
#[doc = "Region Descriptor n, Word 3"]
pub struct RGD_WORD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor n, Word 3"]
pub mod rgd_word3;
#[doc = "Region Descriptor Alternate Access Control n"]
pub struct RGDAAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor Alternate Access Control n"]
pub mod rgdaac;
