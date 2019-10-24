#[doc = "Reader of register TCR4"]
pub type R = crate::R<u32, super::TCR4>;
#[doc = "Writer for register TCR4"]
pub type W = crate::W<u32, super::TCR4>;
#[doc = "Register TCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSD_A {
    #[doc = "0: Frame sync is generated externally in Slave mode."]
    _0,
    #[doc = "1: Frame sync is generated internally in Master mode."]
    _1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        match variant {
            FSD_A::_0 => false,
            FSD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSD`"]
pub type FSD_R = crate::R<bool, FSD_A>;
impl FSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::_0,
            true => FSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSD_A::_1
    }
}
#[doc = "Write proxy for field `FSD`"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSD_A::_0)
    }
    #[doc = "Frame sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSD_A::_1)
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
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSP_A {
    #[doc = "0: Frame sync is active high."]
    _0,
    #[doc = "1: Frame sync is active low."]
    _1,
}
impl From<FSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSP_A) -> Self {
        match variant {
            FSP_A::_0 => false,
            FSP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSP`"]
pub type FSP_R = crate::R<bool, FSP_A>;
impl FSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSP_A {
        match self.bits {
            false => FSP_A::_0,
            true => FSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSP_A::_1
    }
}
#[doc = "Write proxy for field `FSP`"]
pub struct FSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSP_A::_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSP_A::_1)
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
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSE_A {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    _0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    _1,
}
impl From<FSE_A> for bool {
    #[inline(always)]
    fn from(variant: FSE_A) -> Self {
        match variant {
            FSE_A::_0 => false,
            FSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FSE`"]
pub type FSE_R = crate::R<bool, FSE_A>;
impl FSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSE_A {
        match self.bits {
            false => FSE_A::_0,
            true => FSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSE_A::_1
    }
}
#[doc = "Write proxy for field `FSE`"]
pub struct FSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSE_A::_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MF_A {
    #[doc = "0: LSB is transmitted first."]
    _0,
    #[doc = "1: MSB is transmitted first."]
    _1,
}
impl From<MF_A> for bool {
    #[inline(always)]
    fn from(variant: MF_A) -> Self {
        match variant {
            MF_A::_0 => false,
            MF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MF`"]
pub type MF_R = crate::R<bool, MF_A>;
impl MF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MF_A {
        match self.bits {
            false => MF_A::_0,
            true => MF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MF_A::_1
    }
}
#[doc = "Write proxy for field `MF`"]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MF_A::_0)
    }
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SYWD`"]
pub type SYWD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYWD`"]
pub struct SYWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FRSZ`"]
pub type FRSZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRSZ`"]
pub struct FRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FSP_R {
        FSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SYWD_R {
        SYWD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&self) -> FRSZ_R {
        FRSZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&mut self) -> FSP_W {
        FSP_W { w: self }
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&mut self) -> FSE_W {
        FSE_W { w: self }
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&mut self) -> SYWD_W {
        SYWD_W { w: self }
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&mut self) -> FRSZ_W {
        FRSZ_W { w: self }
    }
}
