#[doc = "Reader of register RST"]
pub type R = crate::R<u8, super::RST>;
#[doc = "Writer for register RST"]
pub type W = crate::W<u8, super::RST>;
#[doc = "Register RST `reset()`'s with value 0x02"]
impl crate::ResetValue for super::RST {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Digital Filter On RESET Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFILT_A {
    #[doc = "0: Filter not enabled"]
    _0,
    #[doc = "1: Filter enabled"]
    _1,
}
impl From<RSTFILT_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFILT_A) -> Self {
        match variant {
            RSTFILT_A::_0 => false,
            RSTFILT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RSTFILT`"]
pub type RSTFILT_R = crate::R<bool, RSTFILT_A>;
impl RSTFILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFILT_A {
        match self.bits {
            false => RSTFILT_A::_0,
            true => RSTFILT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTFILT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTFILT_A::_1
    }
}
#[doc = "Write proxy for field `RSTFILT`"]
pub struct RSTFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFILT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filter not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFILT_A::_0)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFILT_A::_1)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Low-Leakage Mode RESET Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLRSTE_A {
    #[doc = "0: RESET pin not enabled as a leakage mode exit source"]
    _0,
    #[doc = "1: RESET pin enabled as a low leakage mode exit source"]
    _1,
}
impl From<LLRSTE_A> for bool {
    #[inline(always)]
    fn from(variant: LLRSTE_A) -> Self {
        match variant {
            LLRSTE_A::_0 => false,
            LLRSTE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LLRSTE`"]
pub type LLRSTE_R = crate::R<bool, LLRSTE_A>;
impl LLRSTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLRSTE_A {
        match self.bits {
            false => LLRSTE_A::_0,
            true => LLRSTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LLRSTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LLRSTE_A::_1
    }
}
#[doc = "Write proxy for field `LLRSTE`"]
pub struct LLRSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> LLRSTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLRSTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RESET pin not enabled as a leakage mode exit source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLRSTE_A::_0)
    }
    #[doc = "RESET pin enabled as a low leakage mode exit source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLRSTE_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Digital Filter On RESET Pin"]
    #[inline(always)]
    pub fn rstfilt(&self) -> RSTFILT_R {
        RSTFILT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low-Leakage Mode RESET Enable"]
    #[inline(always)]
    pub fn llrste(&self) -> LLRSTE_R {
        LLRSTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Filter On RESET Pin"]
    #[inline(always)]
    pub fn rstfilt(&mut self) -> RSTFILT_W {
        RSTFILT_W { w: self }
    }
    #[doc = "Bit 1 - Low-Leakage Mode RESET Enable"]
    #[inline(always)]
    pub fn llrste(&mut self) -> LLRSTE_W {
        LLRSTE_W { w: self }
    }
}
