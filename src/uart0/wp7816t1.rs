#[doc = "Reader of register WP7816T1"]
pub type R = crate::R<u8, super::WP7816T1>;
#[doc = "Writer for register WP7816T1"]
pub type W = crate::W<u8, super::WP7816T1>;
#[doc = "Register WP7816T1 `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::WP7816T1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `BWI`"]
pub type BWI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BWI`"]
pub struct BWI_W<'a> {
    w: &'a mut W,
}
impl<'a> BWI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CWI`"]
pub type CWI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CWI`"]
pub struct CWI_W<'a> {
    w: &'a mut W,
}
impl<'a> CWI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Block Wait Time Integer(C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bwi(&self) -> BWI_R {
        BWI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn cwi(&self) -> CWI_R {
        CWI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Block Wait Time Integer(C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bwi(&mut self) -> BWI_W {
        BWI_W { w: self }
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn cwi(&mut self) -> CWI_W {
        CWI_W { w: self }
    }
}
