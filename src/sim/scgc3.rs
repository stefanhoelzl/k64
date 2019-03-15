#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCGC3 {
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
#[doc = "Possible values of the field `RNGA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGAR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl RNGAR {
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
            RNGAR::_0 => false,
            RNGAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RNGAR {
        match value {
            false => RNGAR::_0,
            true => RNGAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RNGAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RNGAR::_1
    }
}
#[doc = "Possible values of the field `SPI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SPI2R {
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
            SPI2R::_0 => false,
            SPI2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI2R {
        match value {
            false => SPI2R::_0,
            true => SPI2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPI2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPI2R::_1
    }
}
#[doc = "Possible values of the field `SDHC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDHCR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SDHCR {
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
            SDHCR::_0 => false,
            SDHCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDHCR {
        match value {
            false => SDHCR::_0,
            true => SDHCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SDHCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDHCR::_1
    }
}
#[doc = "Possible values of the field `FTM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl FTM2R {
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
            FTM2R::_0 => false,
            FTM2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2R {
        match value {
            false => FTM2R::_0,
            true => FTM2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2R::_1
    }
}
#[doc = "Possible values of the field `FTM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl FTM3R {
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
            FTM3R::_0 => false,
            FTM3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3R {
        match value {
            false => FTM3R::_0,
            true => FTM3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3R::_1
    }
}
#[doc = "Possible values of the field `ADC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1R {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl ADC1R {
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
            ADC1R::_0 => false,
            ADC1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1R {
        match value {
            false => ADC1R::_0,
            true => ADC1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC1R::_1
    }
}
#[doc = "Values that can be written to the field `RNGA`"]
pub enum RNGAW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl RNGAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RNGAW::_0 => false,
            RNGAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RNGAW<'a> {
    w: &'a mut W,
}
impl<'a> _RNGAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNGAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNGAW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNGAW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI2`"]
pub enum SPI2W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SPI2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI2W::_0 => false,
            SPI2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI2W<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI2W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI2W::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDHC`"]
pub enum SDHCW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl SDHCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDHCW::_0 => false,
            SDHCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDHCW<'a> {
    w: &'a mut W,
}
impl<'a> _SDHCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDHCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDHCW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDHCW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2`"]
pub enum FTM2W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl FTM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2W::_0 => false,
            FTM2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2W::_1)
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
#[doc = "Values that can be written to the field `FTM3`"]
pub enum FTM3W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl FTM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3W::_0 => false,
            FTM3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3W::_1)
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
#[doc = "Values that can be written to the field `ADC1`"]
pub enum ADC1W {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl ADC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1W::_0 => false,
            ADC1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1W::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1W::_1)
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - RNGA Clock Gate Control"]
    #[inline]
    pub fn rnga(&self) -> RNGAR {
        RNGAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI2 Clock Gate Control"]
    #[inline]
    pub fn spi2(&self) -> SPI2R {
        SPI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - SDHC Clock Gate Control"]
    #[inline]
    pub fn sdhc(&self) -> SDHCR {
        SDHCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline]
    pub fn ftm2(&self) -> FTM2R {
        FTM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - FTM3 Clock Gate Control"]
    #[inline]
    pub fn ftm3(&self) -> FTM3R {
        FTM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline]
    pub fn adc1(&self) -> ADC1R {
        ADC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - RNGA Clock Gate Control"]
    #[inline]
    pub fn rnga(&mut self) -> _RNGAW {
        _RNGAW { w: self }
    }
    #[doc = "Bit 12 - SPI2 Clock Gate Control"]
    #[inline]
    pub fn spi2(&mut self) -> _SPI2W {
        _SPI2W { w: self }
    }
    #[doc = "Bit 17 - SDHC Clock Gate Control"]
    #[inline]
    pub fn sdhc(&mut self) -> _SDHCW {
        _SDHCW { w: self }
    }
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline]
    pub fn ftm2(&mut self) -> _FTM2W {
        _FTM2W { w: self }
    }
    #[doc = "Bit 25 - FTM3 Clock Gate Control"]
    #[inline]
    pub fn ftm3(&mut self) -> _FTM3W {
        _FTM3W { w: self }
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline]
    pub fn adc1(&mut self) -> _ADC1W {
        _ADC1W { w: self }
    }
}
