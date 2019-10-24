#[doc = "Reader of register MIBC"]
pub type R = crate::R<u32, super::MIBC>;
#[doc = "Writer for register MIBC"]
pub type W = crate::W<u32, super::MIBC>;
#[doc = "Register MIBC `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::MIBC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `MIB_CLEAR`"]
pub type MIB_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIB_CLEAR`"]
pub struct MIB_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MIB_CLEAR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `MIB_IDLE`"]
pub type MIB_IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MIB_DIS`"]
pub type MIB_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIB_DIS`"]
pub struct MIB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MIB_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&self) -> MIB_CLEAR_R {
        MIB_CLEAR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - MIB Idle"]
    #[inline(always)]
    pub fn mib_idle(&self) -> MIB_IDLE_R {
        MIB_IDLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&self) -> MIB_DIS_R {
        MIB_DIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MIB Clear"]
    #[inline(always)]
    pub fn mib_clear(&mut self) -> MIB_CLEAR_W {
        MIB_CLEAR_W { w: self }
    }
    #[doc = "Bit 31 - Disable MIB Logic"]
    #[inline(always)]
    pub fn mib_dis(&mut self) -> MIB_DIS_W {
        MIB_DIS_W { w: self }
    }
}
