#[doc = "Reader of register C5"]
pub type R = crate::R<u8, super::C5>;
#[doc = "Writer for register C5"]
pub type W = crate::W<u8, super::C5>;
#[doc = "Register C5 `reset()`'s with value 0"]
impl crate::ResetValue for super::C5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LIN Break Detect DMA Select Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDDMAS_A {
    #[doc = "0: If BDH\\[LBKDIE\\] and S2\\[LBKDIF\\] are set, the LBKDIF interrupt signal is asserted to request an interrupt service."]
    _0,
    #[doc = "1: If BDH\\[LBKDIE\\] and S2\\[LBKDIF\\] are set, the LBKDIF DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<LBKDDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDDMAS_A) -> Self {
        match variant {
            LBKDDMAS_A::_0 => false,
            LBKDDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LBKDDMAS`"]
pub type LBKDDMAS_R = crate::R<bool, LBKDDMAS_A>;
impl LBKDDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDDMAS_A {
        match self.bits {
            false => LBKDDMAS_A::_0,
            true => LBKDDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDDMAS_A::_1
    }
}
#[doc = "Write proxy for field `LBKDDMAS`"]
pub struct LBKDDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If BDH\\[LBKDIE\\] and S2\\[LBKDIF\\] are set, the LBKDIF interrupt signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDDMAS_A::_0)
    }
    #[doc = "If BDH\\[LBKDIE\\] and S2\\[LBKDIF\\] are set, the LBKDIF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDDMAS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Idle Line DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILDMAS_A {
    #[doc = "0: If C2\\[ILIE\\] and S1\\[IDLE\\] are set, the IDLE interrupt request signal is asserted to request an interrupt service."]
    _0,
    #[doc = "1: If C2\\[ILIE\\] and S1\\[IDLE\\] are set, the IDLE DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<ILDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: ILDMAS_A) -> Self {
        match variant {
            ILDMAS_A::_0 => false,
            ILDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ILDMAS`"]
pub type ILDMAS_R = crate::R<bool, ILDMAS_A>;
impl ILDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILDMAS_A {
        match self.bits {
            false => ILDMAS_A::_0,
            true => ILDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILDMAS_A::_1
    }
}
#[doc = "Write proxy for field `ILDMAS`"]
pub struct ILDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ILDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If C2\\[ILIE\\] and S1\\[IDLE\\] are set, the IDLE interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILDMAS_A::_0)
    }
    #[doc = "If C2\\[ILIE\\] and S1\\[IDLE\\] are set, the IDLE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILDMAS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Receiver Full DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAS_A {
    #[doc = "0: If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    _0,
    #[doc = "1: If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<RDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAS_A) -> Self {
        match variant {
            RDMAS_A::_0 => false,
            RDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RDMAS`"]
pub type RDMAS_R = crate::R<bool, RDMAS_A>;
impl RDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAS_A {
        match self.bits {
            false => RDMAS_A::_0,
            true => RDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDMAS_A::_1
    }
}
#[doc = "Write proxy for field `RDMAS`"]
pub struct RDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAS_A::_0)
    }
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAS_A::_1)
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
#[doc = "Transmission Complete DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCDMAS_A {
    #[doc = "0: If C2\\[TCIE\\] is set and the S1\\[TC\\] flag is set, the TC interrupt request signal is asserted to request an interrupt service."]
    _0,
    #[doc = "1: If C2\\[TCIE\\] is set and the S1\\[TC\\] flag is set, the TC DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<TCDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: TCDMAS_A) -> Self {
        match variant {
            TCDMAS_A::_0 => false,
            TCDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCDMAS`"]
pub type TCDMAS_R = crate::R<bool, TCDMAS_A>;
impl TCDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCDMAS_A {
        match self.bits {
            false => TCDMAS_A::_0,
            true => TCDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCDMAS_A::_1
    }
}
#[doc = "Write proxy for field `TCDMAS`"]
pub struct TCDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If C2\\[TCIE\\] is set and the S1\\[TC\\] flag is set, the TC interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCDMAS_A::_0)
    }
    #[doc = "If C2\\[TCIE\\] is set and the S1\\[TC\\] flag is set, the TC DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCDMAS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmitter DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAS_A {
    #[doc = "0: If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    _0,
    #[doc = "1: If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<TDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAS_A) -> Self {
        match variant {
            TDMAS_A::_0 => false,
            TDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TDMAS`"]
pub type TDMAS_R = crate::R<bool, TDMAS_A>;
impl TDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAS_A {
        match self.bits {
            false => TDMAS_A::_0,
            true => TDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDMAS_A::_1
    }
}
#[doc = "Write proxy for field `TDMAS`"]
pub struct TDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAS_A::_0)
    }
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - LIN Break Detect DMA Select Bit"]
    #[inline(always)]
    pub fn lbkddmas(&self) -> LBKDDMAS_R {
        LBKDDMAS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle Line DMA Select"]
    #[inline(always)]
    pub fn ildmas(&self) -> ILDMAS_R {
        ILDMAS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&self) -> RDMAS_R {
        RDMAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete DMA Select"]
    #[inline(always)]
    pub fn tcdmas(&self) -> TCDMAS_R {
        TCDMAS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&self) -> TDMAS_R {
        TDMAS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LIN Break Detect DMA Select Bit"]
    #[inline(always)]
    pub fn lbkddmas(&mut self) -> LBKDDMAS_W {
        LBKDDMAS_W { w: self }
    }
    #[doc = "Bit 4 - Idle Line DMA Select"]
    #[inline(always)]
    pub fn ildmas(&mut self) -> ILDMAS_W {
        ILDMAS_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&mut self) -> RDMAS_W {
        RDMAS_W { w: self }
    }
    #[doc = "Bit 6 - Transmission Complete DMA Select"]
    #[inline(always)]
    pub fn tcdmas(&mut self) -> TCDMAS_W {
        TCDMAS_W { w: self }
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&mut self) -> TDMAS_W {
        TDMAS_W { w: self }
    }
}
