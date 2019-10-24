#[doc = "Reader of register MMFR"]
pub type R = crate::R<u32, super::MMFR>;
#[doc = "Writer for register MMFR"]
pub type W = crate::W<u32, super::MMFR>;
#[doc = "Register MMFR `reset()`'s with value 0"]
impl crate::ResetValue for super::MMFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TA`"]
pub type TA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TA`"]
pub struct TA_W<'a> {
    w: &'a mut W,
}
impl<'a> TA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RA`"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `PA`"]
pub type PA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA`"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | (((value as u32) & 0x1f) << 23);
        self.w
    }
}
#[doc = "Operation Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP_A {
    #[doc = "0: Write frame operation, but not MII compliant."]
    _00,
    #[doc = "1: Write frame operation for a valid MII management frame."]
    _01,
    #[doc = "2: Read frame operation for a valid MII management frame."]
    _10,
    #[doc = "3: Read frame operation, but not MII compliant."]
    _11,
}
impl From<OP_A> for u8 {
    #[inline(always)]
    fn from(variant: OP_A) -> Self {
        match variant {
            OP_A::_00 => 0,
            OP_A::_01 => 1,
            OP_A::_10 => 2,
            OP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `OP`"]
pub type OP_R = crate::R<u8, OP_A>;
impl OP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OP_A {
        match self.bits {
            0 => OP_A::_00,
            1 => OP_A::_01,
            2 => OP_A::_10,
            3 => OP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OP_A::_11
    }
}
#[doc = "Write proxy for field `OP`"]
pub struct OP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Write frame operation, but not MII compliant."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OP_A::_00)
    }
    #[doc = "Write frame operation for a valid MII management frame."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OP_A::_01)
    }
    #[doc = "Read frame operation for a valid MII management frame."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OP_A::_10)
    }
    #[doc = "Read frame operation, but not MII compliant."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline(always)]
    pub fn ta(&mut self) -> TA_W {
        TA_W { w: self }
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W {
        OP_W { w: self }
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
}
