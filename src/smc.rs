#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Protection register"]
    pub pmprot: PMPROT,
    #[doc = "0x01 - Power Mode Control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x02 - VLLS Control register"]
    pub vllsctrl: VLLSCTRL,
    #[doc = "0x03 - Power Mode Status register"]
    pub pmstat: PMSTAT,
}
#[doc = "Power Mode Protection register"]
pub struct PMPROT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "Power Mode Control register"]
pub struct PMCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "VLLS Control register"]
pub struct VLLSCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "VLLS Control register"]
pub mod vllsctrl;
#[doc = "Power Mode Status register"]
pub struct PMSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Power Mode Status register"]
pub mod pmstat;
