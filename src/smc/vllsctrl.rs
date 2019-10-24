#[doc = "Reader of register VLLSCTRL"]
pub type R = crate::R<u8, super::VLLSCTRL>;
#[doc = "Writer for register VLLSCTRL"]
pub type W = crate::W<u8, super::VLLSCTRL>;
#[doc = "Register VLLSCTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::VLLSCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "VLLS Mode Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLLSM_A {
    #[doc = "0: VLLS0"]
    _000,
    #[doc = "1: VLLS1"]
    _001,
    #[doc = "2: VLLS2"]
    _010,
    #[doc = "3: VLLS3"]
    _011,
}
impl From<VLLSM_A> for u8 {
    #[inline(always)]
    fn from(variant: VLLSM_A) -> Self {
        match variant {
            VLLSM_A::_000 => 0,
            VLLSM_A::_001 => 1,
            VLLSM_A::_010 => 2,
            VLLSM_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `VLLSM`"]
pub type VLLSM_R = crate::R<u8, VLLSM_A>;
impl VLLSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VLLSM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VLLSM_A::_000),
            1 => Val(VLLSM_A::_001),
            2 => Val(VLLSM_A::_010),
            3 => Val(VLLSM_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == VLLSM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == VLLSM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == VLLSM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == VLLSM_A::_011
    }
}
#[doc = "Write proxy for field `VLLSM`"]
pub struct VLLSM_W<'a> {
    w: &'a mut W,
}
impl<'a> VLLSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLLSM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VLLS0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(VLLSM_A::_000)
    }
    #[doc = "VLLS1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSM_A::_001)
    }
    #[doc = "VLLS2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(VLLSM_A::_010)
    }
    #[doc = "VLLS3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSM_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "POR Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORPO_A {
    #[doc = "0: POR detect circuit is enabled in VLLS0."]
    _0,
    #[doc = "1: POR detect circuit is disabled in VLLS0."]
    _1,
}
impl From<PORPO_A> for bool {
    #[inline(always)]
    fn from(variant: PORPO_A) -> Self {
        match variant {
            PORPO_A::_0 => false,
            PORPO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORPO`"]
pub type PORPO_R = crate::R<bool, PORPO_A>;
impl PORPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORPO_A {
        match self.bits {
            false => PORPO_A::_0,
            true => PORPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORPO_A::_1
    }
}
#[doc = "Write proxy for field `PORPO`"]
pub struct PORPO_W<'a> {
    w: &'a mut W,
}
impl<'a> PORPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORPO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "POR detect circuit is enabled in VLLS0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORPO_A::_0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORPO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    pub fn vllsm(&self) -> VLLSM_R {
        VLLSM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&self) -> PORPO_R {
        PORPO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    pub fn vllsm(&mut self) -> VLLSM_W {
        VLLSM_W { w: self }
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&mut self) -> PORPO_W {
        PORPO_W { w: self }
    }
}
