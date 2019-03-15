#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PUSHR {
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
#[doc = r" Value of the field"]
pub struct TXDATAR {
    bits: u16,
}
impl TXDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `PCS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS0R {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS0R {
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
            PCS0R::_0 => false,
            PCS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCS0R {
        match value {
            false => PCS0R::_0,
            true => PCS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCS0R::_1
    }
}
#[doc = "Possible values of the field `PCS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS1R {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS1R {
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
            PCS1R::_0 => false,
            PCS1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCS1R {
        match value {
            false => PCS1R::_0,
            true => PCS1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCS1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCS1R::_1
    }
}
#[doc = "Possible values of the field `PCS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS2R {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS2R {
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
            PCS2R::_0 => false,
            PCS2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCS2R {
        match value {
            false => PCS2R::_0,
            true => PCS2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCS2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCS2R::_1
    }
}
#[doc = "Possible values of the field `PCS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS3R {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS3R {
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
            PCS3R::_0 => false,
            PCS3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCS3R {
        match value {
            false => PCS3R::_0,
            true => PCS3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCS3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCS3R::_1
    }
}
#[doc = "Possible values of the field `PCS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS4R {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS4R {
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
            PCS4R::_0 => false,
            PCS4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCS4R {
        match value {
            false => PCS4R::_0,
            true => PCS4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCS4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCS4R::_1
    }
}
#[doc = "Possible values of the field `PCS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCS5R {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS5R {
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
            PCS5R::_0 => false,
            PCS5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCS5R {
        match value {
            false => PCS5R::_0,
            true => PCS5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCS5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCS5R::_1
    }
}
#[doc = "Possible values of the field `CTCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCNTR {
    #[doc = "Do not clear the TCR\\[TCNT\\] field."]
    _0,
    #[doc = "Clear the TCR\\[TCNT\\] field."]
    _1,
}
impl CTCNTR {
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
            CTCNTR::_0 => false,
            CTCNTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTCNTR {
        match value {
            false => CTCNTR::_0,
            true => CTCNTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CTCNTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CTCNTR::_1
    }
}
#[doc = "Possible values of the field `EOQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQR {
    #[doc = "The SPI data is not the last data to transfer."]
    _0,
    #[doc = "The SPI data is the last data to transfer."]
    _1,
}
impl EOQR {
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
            EOQR::_0 => false,
            EOQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOQR {
        match value {
            false => EOQR::_0,
            true => EOQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOQR::_1
    }
}
#[doc = "Possible values of the field `CTAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTASR {
    #[doc = "CTAR0"]
    _000,
    #[doc = "CTAR1"]
    _001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CTASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTASR::_000 => 0,
            CTASR::_001 => 1,
            CTASR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTASR {
        match value {
            0 => CTASR::_000,
            1 => CTASR::_001,
            i => CTASR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CTASR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == CTASR::_001
    }
}
#[doc = "Possible values of the field `CONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR {
    #[doc = "Return PCSn signals to their inactive state between transfers."]
    _0,
    #[doc = "Keep PCSn signals asserted between transfers."]
    _1,
}
impl CONTR {
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
            CONTR::_0 => false,
            CONTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTR {
        match value {
            false => CONTR::_0,
            true => CONTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CONTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CONTR::_1
    }
}
#[doc = r" Proxy"]
pub struct _TXDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS0`"]
pub enum PCS0W {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCS0W::_0 => false,
            PCS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PCS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS0W::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS0W::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS1`"]
pub enum PCS1W {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCS1W::_0 => false,
            PCS1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PCS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS1W::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS1W::_1)
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
#[doc = "Values that can be written to the field `PCS2`"]
pub enum PCS2W {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCS2W::_0 => false,
            PCS2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS2W::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS2W::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS3`"]
pub enum PCS3W {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCS3W::_0 => false,
            PCS3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS3W::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS3W::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS4`"]
pub enum PCS4W {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCS4W::_0 => false,
            PCS4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCS4W<'a> {
    w: &'a mut W,
}
impl<'a> _PCS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS4W::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS4W::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS5`"]
pub enum PCS5W {
    #[doc = "Negate the PCS\\[x\\] signal."]
    _0,
    #[doc = "Assert the PCS\\[x\\] signal."]
    _1,
}
impl PCS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCS5W::_0 => false,
            PCS5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PCS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS5W::_0)
    }
    #[doc = "Assert the PCS\\[x\\] signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS5W::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCNT`"]
pub enum CTCNTW {
    #[doc = "Do not clear the TCR\\[TCNT\\] field."]
    _0,
    #[doc = "Clear the TCR\\[TCNT\\] field."]
    _1,
}
impl CTCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCNTW::_0 => false,
            CTCNTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not clear the TCR\\[TCNT\\] field."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTCNTW::_0)
    }
    #[doc = "Clear the TCR\\[TCNT\\] field."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTCNTW::_1)
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
#[doc = "Values that can be written to the field `EOQ`"]
pub enum EOQW {
    #[doc = "The SPI data is not the last data to transfer."]
    _0,
    #[doc = "The SPI data is the last data to transfer."]
    _1,
}
impl EOQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOQW::_0 => false,
            EOQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOQW<'a> {
    w: &'a mut W,
}
impl<'a> _EOQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI data is not the last data to transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQW::_0)
    }
    #[doc = "The SPI data is the last data to transfer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQW::_1)
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
#[doc = "Values that can be written to the field `CTAS`"]
pub enum CTASW {
    #[doc = "CTAR0"]
    _000,
    #[doc = "CTAR1"]
    _001,
}
impl CTASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTASW::_000 => 0,
            CTASW::_001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTASW<'a> {
    w: &'a mut W,
}
impl<'a> _CTASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CTAR0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CTASW::_000)
    }
    #[doc = "CTAR1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(CTASW::_001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONT`"]
pub enum CONTW {
    #[doc = "Return PCSn signals to their inactive state between transfers."]
    _0,
    #[doc = "Keep PCSn signals asserted between transfers."]
    _1,
}
impl CONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTW::_0 => false,
            CONTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Return PCSn signals to their inactive state between transfers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTW::_0)
    }
    #[doc = "Keep PCSn signals asserted between transfers."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONTW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline]
    pub fn txdata(&self) -> TXDATAR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXDATAR { bits }
    }
    #[doc = "Bit 16 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs0(&self) -> PCS0R {
        PCS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs1(&self) -> PCS1R {
        PCS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs2(&self) -> PCS2R {
        PCS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs3(&self) -> PCS3R {
        PCS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs4(&self) -> PCS4R {
        PCS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs5(&self) -> PCS5R {
        PCS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline]
    pub fn ctcnt(&self) -> CTCNTR {
        CTCNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline]
    pub fn eoq(&self) -> EOQR {
        EOQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline]
    pub fn ctas(&self) -> CTASR {
        CTASR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        CONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline]
    pub fn txdata(&mut self) -> _TXDATAW {
        _TXDATAW { w: self }
    }
    #[doc = "Bit 16 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs0(&mut self) -> _PCS0W {
        _PCS0W { w: self }
    }
    #[doc = "Bit 17 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs1(&mut self) -> _PCS1W {
        _PCS1W { w: self }
    }
    #[doc = "Bit 18 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs2(&mut self) -> _PCS2W {
        _PCS2W { w: self }
    }
    #[doc = "Bit 19 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs3(&mut self) -> _PCS3W {
        _PCS3W { w: self }
    }
    #[doc = "Bit 20 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs4(&mut self) -> _PCS4W {
        _PCS4W { w: self }
    }
    #[doc = "Bit 21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline]
    pub fn pcs5(&mut self) -> _PCS5W {
        _PCS5W { w: self }
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline]
    pub fn ctcnt(&mut self) -> _CTCNTW {
        _CTCNTW { w: self }
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline]
    pub fn eoq(&mut self) -> _EOQW {
        _EOQW { w: self }
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline]
    pub fn ctas(&mut self) -> _CTASW {
        _CTASW { w: self }
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
}
