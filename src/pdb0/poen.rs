#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POEN {
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
#[doc = "Possible values of the field `POEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN0R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN0R {
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
            POEN0R::_0 => false,
            POEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN0R {
        match value {
            false => POEN0R::_0,
            true => POEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN0R::_1
    }
}
#[doc = "Possible values of the field `POEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN1R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN1R {
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
            POEN1R::_0 => false,
            POEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN1R {
        match value {
            false => POEN1R::_0,
            true => POEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN1R::_1
    }
}
#[doc = "Possible values of the field `POEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN2R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN2R {
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
            POEN2R::_0 => false,
            POEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN2R {
        match value {
            false => POEN2R::_0,
            true => POEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN2R::_1
    }
}
#[doc = "Possible values of the field `POEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN3R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN3R {
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
            POEN3R::_0 => false,
            POEN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN3R {
        match value {
            false => POEN3R::_0,
            true => POEN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN3R::_1
    }
}
#[doc = "Possible values of the field `POEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN4R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN4R {
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
            POEN4R::_0 => false,
            POEN4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN4R {
        match value {
            false => POEN4R::_0,
            true => POEN4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN4R::_1
    }
}
#[doc = "Possible values of the field `POEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN5R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN5R {
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
            POEN5R::_0 => false,
            POEN5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN5R {
        match value {
            false => POEN5R::_0,
            true => POEN5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN5R::_1
    }
}
#[doc = "Possible values of the field `POEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN6R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN6R {
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
            POEN6R::_0 => false,
            POEN6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN6R {
        match value {
            false => POEN6R::_0,
            true => POEN6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN6R::_1
    }
}
#[doc = "Possible values of the field `POEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POEN7R {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN7R {
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
            POEN7R::_0 => false,
            POEN7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POEN7R {
        match value {
            false => POEN7R::_0,
            true => POEN7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POEN7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POEN7R::_1
    }
}
#[doc = "Values that can be written to the field `POEN0`"]
pub enum POEN0W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN0W::_0 => false,
            POEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN0W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN0W::_1)
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
#[doc = "Values that can be written to the field `POEN1`"]
pub enum POEN1W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN1W::_0 => false,
            POEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN1W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN1W::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POEN2`"]
pub enum POEN2W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN2W::_0 => false,
            POEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN2W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN2W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POEN3`"]
pub enum POEN3W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN3W::_0 => false,
            POEN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN3W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN3W::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POEN4`"]
pub enum POEN4W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN4W::_0 => false,
            POEN4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN4W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN4W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POEN5`"]
pub enum POEN5W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN5W::_0 => false,
            POEN5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN5W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN5W::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POEN6`"]
pub enum POEN6W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN6W::_0 => false,
            POEN6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN6W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN6W::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POEN7`"]
pub enum POEN7W {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POEN7W::_0 => false,
            POEN7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _POEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN7W::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN7W::_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen0(&self) -> POEN0R {
        POEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen1(&self) -> POEN1R {
        POEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen2(&self) -> POEN2R {
        POEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen3(&self) -> POEN3R {
        POEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen4(&self) -> POEN4R {
        POEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen5(&self) -> POEN5R {
        POEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen6(&self) -> POEN6R {
        POEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen7(&self) -> POEN7R {
        POEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen0(&mut self) -> _POEN0W {
        _POEN0W { w: self }
    }
    #[doc = "Bit 1 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen1(&mut self) -> _POEN1W {
        _POEN1W { w: self }
    }
    #[doc = "Bit 2 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen2(&mut self) -> _POEN2W {
        _POEN2W { w: self }
    }
    #[doc = "Bit 3 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen3(&mut self) -> _POEN3W {
        _POEN3W { w: self }
    }
    #[doc = "Bit 4 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen4(&mut self) -> _POEN4W {
        _POEN4W { w: self }
    }
    #[doc = "Bit 5 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen5(&mut self) -> _POEN5W {
        _POEN5W { w: self }
    }
    #[doc = "Bit 6 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen6(&mut self) -> _POEN6W {
        _POEN6W { w: self }
    }
    #[doc = "Bit 7 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen7(&mut self) -> _POEN7W {
        _POEN7W { w: self }
    }
}
