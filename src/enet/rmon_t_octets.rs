#[doc = "Reader of register RMON_T_OCTETS"]
pub type R = crate::R<u32, super::RMON_T_OCTETS>;
#[doc = "Reader of field `TXOCTS`"]
pub type TXOCTS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Octet count"]
    #[inline(always)]
    pub fn txocts(&self) -> TXOCTS_R {
        TXOCTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
