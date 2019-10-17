#[doc = "Reader of register VENDOR"]
pub type R = crate::R<u32, super::VENDOR>;
#[doc = "Writer for register VENDOR"]
pub type W = crate::W<u32, super::VENDOR>;
#[doc = "Register VENDOR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::VENDOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "External DMA Request Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTDMAEN_A {
    #[doc = "0: In any scenario, SDHC does not send out the external DMA request."]
    _0,
    #[doc = "1: When internal DMA is not active, the external DMA request will be sent out."]
    _1,
}
impl From<EXTDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDMAEN_A) -> Self {
        match variant {
            EXTDMAEN_A::_0 => false,
            EXTDMAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EXTDMAEN`"]
pub type EXTDMAEN_R = crate::R<bool, EXTDMAEN_A>;
impl EXTDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDMAEN_A {
        match self.bits {
            false => EXTDMAEN_A::_0,
            true => EXTDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTDMAEN_A::_1
    }
}
#[doc = "Write proxy for field `EXTDMAEN`"]
pub struct EXTDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In any scenario, SDHC does not send out the external DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTDMAEN_A::_0)
    }
    #[doc = "When internal DMA is not active, the external DMA request will be sent out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTDMAEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Exact Block Number Block Read Enable For SDIO CMD53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXBLKNU_A {
    #[doc = "0: None exact block read."]
    _0,
    #[doc = "1: Exact block read for SDIO CMD53."]
    _1,
}
impl From<EXBLKNU_A> for bool {
    #[inline(always)]
    fn from(variant: EXBLKNU_A) -> Self {
        match variant {
            EXBLKNU_A::_0 => false,
            EXBLKNU_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EXBLKNU`"]
pub type EXBLKNU_R = crate::R<bool, EXBLKNU_A>;
impl EXBLKNU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXBLKNU_A {
        match self.bits {
            false => EXBLKNU_A::_0,
            true => EXBLKNU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXBLKNU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXBLKNU_A::_1
    }
}
#[doc = "Write proxy for field `EXBLKNU`"]
pub struct EXBLKNU_W<'a> {
    w: &'a mut W,
}
impl<'a> EXBLKNU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXBLKNU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "None exact block read."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXBLKNU_A::_0)
    }
    #[doc = "Exact block read for SDIO CMD53."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXBLKNU_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INTSTVAL`"]
pub type INTSTVAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - External DMA Request Enable"]
    #[inline(always)]
    pub fn extdmaen(&self) -> EXTDMAEN_R {
        EXTDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Exact Block Number Block Read Enable For SDIO CMD53"]
    #[inline(always)]
    pub fn exblknu(&self) -> EXBLKNU_R {
        EXBLKNU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Internal State Value"]
    #[inline(always)]
    pub fn intstval(&self) -> INTSTVAL_R {
        INTSTVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External DMA Request Enable"]
    #[inline(always)]
    pub fn extdmaen(&mut self) -> EXTDMAEN_W {
        EXTDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - Exact Block Number Block Read Enable For SDIO CMD53"]
    #[inline(always)]
    pub fn exblknu(&mut self) -> EXBLKNU_W {
        EXBLKNU_W { w: self }
    }
}
