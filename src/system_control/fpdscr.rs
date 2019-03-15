#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FPDSCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RMode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMODER {
    #[doc = "Round to Nearest (RN) mode"]
    _00,
    #[doc = "Round towards Plus Infinity (RP) mode."]
    _01,
    #[doc = "Round towards Minus Infinity (RM) mode."]
    _10,
    #[doc = "Round towards Zero (RZ) mode."]
    _11,
}
impl RMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RMODER::_00 => 0,
            RMODER::_01 => 1,
            RMODER::_10 => 2,
            RMODER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RMODER {
        match value {
            0 => RMODER::_00,
            1 => RMODER::_01,
            2 => RMODER::_10,
            3 => RMODER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RMODER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RMODER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RMODER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RMODER::_11
    }
}
#[doc = "Possible values of the field `FZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FZR {
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    _0,
    #[doc = "Flush-to-zero mode enabled."]
    _1,
}
impl FZR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FZR::_0 => false,
            FZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FZR {
        match value {
            false => FZR::_0,
            true => FZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FZR::_1
    }
}
#[doc = "Possible values of the field `DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNR {
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    _0,
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    _1,
}
impl DNR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DNR::_0 => false,
            DNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DNR {
        match value {
            false => DNR::_0,
            true => DNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DNR::_1
    }
}
#[doc = "Possible values of the field `AHP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHPR {
    #[doc = "IEEE half-precision format selected."]
    _0,
    #[doc = "Alternative half-precision format selected."]
    _1,
}
impl AHPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AHPR::_0 => false,
            AHPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHPR {
        match value {
            false => AHPR::_0,
            true => AHPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AHPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AHPR::_1
    }
}
#[doc = "Values that can be written to the field `RMode`"]
pub enum RMODEW {
    #[doc = "Round to Nearest (RN) mode"]
    _00,
    #[doc = "Round towards Plus Infinity (RP) mode."]
    _01,
    #[doc = "Round towards Minus Infinity (RM) mode."]
    _10,
    #[doc = "Round towards Zero (RZ) mode."]
    _11,
}
impl RMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RMODEW::_00 => 0,
            RMODEW::_01 => 1,
            RMODEW::_10 => 2,
            RMODEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Round to Nearest (RN) mode"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RMODEW::_00)
    }
    #[doc = "Round towards Plus Infinity (RP) mode."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RMODEW::_01)
    }
    #[doc = "Round towards Minus Infinity (RM) mode."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RMODEW::_10)
    }
    #[doc = "Round towards Zero (RZ) mode."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RMODEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FZ`"]
pub enum FZW {
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    _0,
    #[doc = "Flush-to-zero mode enabled."]
    _1,
}
impl FZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FZW::_0 => false,
            FZW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FZW<'a> {
    w: &'a mut W,
}
impl<'a> _FZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FZW::_0)
    }
    #[doc = "Flush-to-zero mode enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FZW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DN`"]
pub enum DNW {
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    _0,
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    _1,
}
impl DNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DNW::_0 => false,
            DNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DNW<'a> {
    w: &'a mut W,
}
impl<'a> _DNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DNW::_0)
    }
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DNW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHP`"]
pub enum AHPW {
    #[doc = "IEEE half-precision format selected."]
    _0,
    #[doc = "Alternative half-precision format selected."]
    _1,
}
impl AHPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHPW::_0 => false,
            AHPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHPW<'a> {
    w: &'a mut W,
}
impl<'a> _AHPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IEEE half-precision format selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AHPW::_0)
    }
    #[doc = "Alternative half-precision format selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AHPW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline]
    pub fn rmode(&self) -> RMODER {
        RMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline]
    pub fn fz(&self) -> FZR {
        FZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline]
    pub fn dn(&self) -> DNR {
        DNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline]
    pub fn ahp(&self) -> AHPR {
        AHPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline]
    pub fn rmode(&mut self) -> _RMODEW {
        _RMODEW { w: self }
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline]
    pub fn fz(&mut self) -> _FZW {
        _FZW { w: self }
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline]
    pub fn dn(&mut self) -> _DNW {
        _DNW { w: self }
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline]
    pub fn ahp(&mut self) -> _AHPW {
        _AHPW { w: self }
    }
}
