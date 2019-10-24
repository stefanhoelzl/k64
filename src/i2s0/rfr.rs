#[doc = "Reader of register RFR%s"]
pub type R = crate::R<u32, super::RFR>;
#[doc = "Reader of field `RFP`"]
pub type RFP_R = crate::R<u8, u8>;
#[doc = "Reader of field `WFP`"]
pub type WFP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
