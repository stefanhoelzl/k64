#[doc = r"Register block"]
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
#[doc = "Power Mode Protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmprot](pmprot) module"]
pub type PMPROT = crate::Reg<u8, _PMPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMPROT;
#[doc = "`read()` method returns [pmprot::R](pmprot::R) reader structure"]
impl crate::Readable for PMPROT {}
#[doc = "`write(|w| ..)` method takes [pmprot::W](pmprot::W) writer structure"]
impl crate::Writable for PMPROT {}
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "Power Mode Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmctrl](pmctrl) module"]
pub type PMCTRL = crate::Reg<u8, _PMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCTRL;
#[doc = "`read()` method returns [pmctrl::R](pmctrl::R) reader structure"]
impl crate::Readable for PMCTRL {}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](pmctrl::W) writer structure"]
impl crate::Writable for PMCTRL {}
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "VLLS Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vllsctrl](vllsctrl) module"]
pub type VLLSCTRL = crate::Reg<u8, _VLLSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLLSCTRL;
#[doc = "`read()` method returns [vllsctrl::R](vllsctrl::R) reader structure"]
impl crate::Readable for VLLSCTRL {}
#[doc = "`write(|w| ..)` method takes [vllsctrl::W](vllsctrl::W) writer structure"]
impl crate::Writable for VLLSCTRL {}
#[doc = "VLLS Control register"]
pub mod vllsctrl;
#[doc = "Power Mode Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmstat](pmstat) module"]
pub type PMSTAT = crate::Reg<u8, _PMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTAT;
#[doc = "`read()` method returns [pmstat::R](pmstat::R) reader structure"]
impl crate::Readable for PMSTAT {}
#[doc = "Power Mode Status register"]
pub mod pmstat;
