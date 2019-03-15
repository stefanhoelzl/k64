#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PDIR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PDI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI0R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI0R {
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
            PDI0R::_0 => false,
            PDI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI0R {
        match value {
            false => PDI0R::_0,
            true => PDI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI0R::_1
    }
}
#[doc = "Possible values of the field `PDI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI1R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI1R {
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
            PDI1R::_0 => false,
            PDI1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI1R {
        match value {
            false => PDI1R::_0,
            true => PDI1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI1R::_1
    }
}
#[doc = "Possible values of the field `PDI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI2R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI2R {
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
            PDI2R::_0 => false,
            PDI2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI2R {
        match value {
            false => PDI2R::_0,
            true => PDI2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI2R::_1
    }
}
#[doc = "Possible values of the field `PDI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI3R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI3R {
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
            PDI3R::_0 => false,
            PDI3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI3R {
        match value {
            false => PDI3R::_0,
            true => PDI3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI3R::_1
    }
}
#[doc = "Possible values of the field `PDI4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI4R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI4R {
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
            PDI4R::_0 => false,
            PDI4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI4R {
        match value {
            false => PDI4R::_0,
            true => PDI4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI4R::_1
    }
}
#[doc = "Possible values of the field `PDI5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI5R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI5R {
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
            PDI5R::_0 => false,
            PDI5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI5R {
        match value {
            false => PDI5R::_0,
            true => PDI5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI5R::_1
    }
}
#[doc = "Possible values of the field `PDI6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI6R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI6R {
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
            PDI6R::_0 => false,
            PDI6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI6R {
        match value {
            false => PDI6R::_0,
            true => PDI6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI6R::_1
    }
}
#[doc = "Possible values of the field `PDI7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI7R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI7R {
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
            PDI7R::_0 => false,
            PDI7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI7R {
        match value {
            false => PDI7R::_0,
            true => PDI7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI7R::_1
    }
}
#[doc = "Possible values of the field `PDI8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI8R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI8R {
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
            PDI8R::_0 => false,
            PDI8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI8R {
        match value {
            false => PDI8R::_0,
            true => PDI8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI8R::_1
    }
}
#[doc = "Possible values of the field `PDI9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI9R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI9R {
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
            PDI9R::_0 => false,
            PDI9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI9R {
        match value {
            false => PDI9R::_0,
            true => PDI9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI9R::_1
    }
}
#[doc = "Possible values of the field `PDI10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI10R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI10R {
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
            PDI10R::_0 => false,
            PDI10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI10R {
        match value {
            false => PDI10R::_0,
            true => PDI10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI10R::_1
    }
}
#[doc = "Possible values of the field `PDI11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI11R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI11R {
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
            PDI11R::_0 => false,
            PDI11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI11R {
        match value {
            false => PDI11R::_0,
            true => PDI11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI11R::_1
    }
}
#[doc = "Possible values of the field `PDI12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI12R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI12R {
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
            PDI12R::_0 => false,
            PDI12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI12R {
        match value {
            false => PDI12R::_0,
            true => PDI12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI12R::_1
    }
}
#[doc = "Possible values of the field `PDI13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI13R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI13R {
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
            PDI13R::_0 => false,
            PDI13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI13R {
        match value {
            false => PDI13R::_0,
            true => PDI13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI13R::_1
    }
}
#[doc = "Possible values of the field `PDI14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI14R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI14R {
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
            PDI14R::_0 => false,
            PDI14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI14R {
        match value {
            false => PDI14R::_0,
            true => PDI14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI14R::_1
    }
}
#[doc = "Possible values of the field `PDI15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI15R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI15R {
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
            PDI15R::_0 => false,
            PDI15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI15R {
        match value {
            false => PDI15R::_0,
            true => PDI15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI15R::_1
    }
}
#[doc = "Possible values of the field `PDI16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI16R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI16R {
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
            PDI16R::_0 => false,
            PDI16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI16R {
        match value {
            false => PDI16R::_0,
            true => PDI16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI16R::_1
    }
}
#[doc = "Possible values of the field `PDI17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI17R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI17R {
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
            PDI17R::_0 => false,
            PDI17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI17R {
        match value {
            false => PDI17R::_0,
            true => PDI17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI17R::_1
    }
}
#[doc = "Possible values of the field `PDI18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI18R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI18R {
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
            PDI18R::_0 => false,
            PDI18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI18R {
        match value {
            false => PDI18R::_0,
            true => PDI18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI18R::_1
    }
}
#[doc = "Possible values of the field `PDI19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI19R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI19R {
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
            PDI19R::_0 => false,
            PDI19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI19R {
        match value {
            false => PDI19R::_0,
            true => PDI19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI19R::_1
    }
}
#[doc = "Possible values of the field `PDI20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI20R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI20R {
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
            PDI20R::_0 => false,
            PDI20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI20R {
        match value {
            false => PDI20R::_0,
            true => PDI20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI20R::_1
    }
}
#[doc = "Possible values of the field `PDI21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI21R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI21R {
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
            PDI21R::_0 => false,
            PDI21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI21R {
        match value {
            false => PDI21R::_0,
            true => PDI21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI21R::_1
    }
}
#[doc = "Possible values of the field `PDI22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI22R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI22R {
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
            PDI22R::_0 => false,
            PDI22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI22R {
        match value {
            false => PDI22R::_0,
            true => PDI22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI22R::_1
    }
}
#[doc = "Possible values of the field `PDI23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI23R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI23R {
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
            PDI23R::_0 => false,
            PDI23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI23R {
        match value {
            false => PDI23R::_0,
            true => PDI23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI23R::_1
    }
}
#[doc = "Possible values of the field `PDI24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI24R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI24R {
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
            PDI24R::_0 => false,
            PDI24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI24R {
        match value {
            false => PDI24R::_0,
            true => PDI24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI24R::_1
    }
}
#[doc = "Possible values of the field `PDI25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI25R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI25R {
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
            PDI25R::_0 => false,
            PDI25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI25R {
        match value {
            false => PDI25R::_0,
            true => PDI25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI25R::_1
    }
}
#[doc = "Possible values of the field `PDI26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI26R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI26R {
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
            PDI26R::_0 => false,
            PDI26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI26R {
        match value {
            false => PDI26R::_0,
            true => PDI26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI26R::_1
    }
}
#[doc = "Possible values of the field `PDI27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI27R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI27R {
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
            PDI27R::_0 => false,
            PDI27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI27R {
        match value {
            false => PDI27R::_0,
            true => PDI27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI27R::_1
    }
}
#[doc = "Possible values of the field `PDI28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI28R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI28R {
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
            PDI28R::_0 => false,
            PDI28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI28R {
        match value {
            false => PDI28R::_0,
            true => PDI28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI28R::_1
    }
}
#[doc = "Possible values of the field `PDI29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI29R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI29R {
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
            PDI29R::_0 => false,
            PDI29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI29R {
        match value {
            false => PDI29R::_0,
            true => PDI29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI29R::_1
    }
}
#[doc = "Possible values of the field `PDI30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI30R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI30R {
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
            PDI30R::_0 => false,
            PDI30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI30R {
        match value {
            false => PDI30R::_0,
            true => PDI30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI30R::_1
    }
}
#[doc = "Possible values of the field `PDI31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI31R {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
}
impl PDI31R {
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
            PDI31R::_0 => false,
            PDI31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDI31R {
        match value {
            false => PDI31R::_0,
            true => PDI31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDI31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDI31R::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port Data Input"]
    #[inline]
    pub fn pdi0(&self) -> PDI0R {
        PDI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port Data Input"]
    #[inline]
    pub fn pdi1(&self) -> PDI1R {
        PDI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port Data Input"]
    #[inline]
    pub fn pdi2(&self) -> PDI2R {
        PDI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port Data Input"]
    #[inline]
    pub fn pdi3(&self) -> PDI3R {
        PDI3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port Data Input"]
    #[inline]
    pub fn pdi4(&self) -> PDI4R {
        PDI4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port Data Input"]
    #[inline]
    pub fn pdi5(&self) -> PDI5R {
        PDI5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port Data Input"]
    #[inline]
    pub fn pdi6(&self) -> PDI6R {
        PDI6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port Data Input"]
    #[inline]
    pub fn pdi7(&self) -> PDI7R {
        PDI7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port Data Input"]
    #[inline]
    pub fn pdi8(&self) -> PDI8R {
        PDI8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port Data Input"]
    #[inline]
    pub fn pdi9(&self) -> PDI9R {
        PDI9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port Data Input"]
    #[inline]
    pub fn pdi10(&self) -> PDI10R {
        PDI10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port Data Input"]
    #[inline]
    pub fn pdi11(&self) -> PDI11R {
        PDI11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port Data Input"]
    #[inline]
    pub fn pdi12(&self) -> PDI12R {
        PDI12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port Data Input"]
    #[inline]
    pub fn pdi13(&self) -> PDI13R {
        PDI13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port Data Input"]
    #[inline]
    pub fn pdi14(&self) -> PDI14R {
        PDI14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port Data Input"]
    #[inline]
    pub fn pdi15(&self) -> PDI15R {
        PDI15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Port Data Input"]
    #[inline]
    pub fn pdi16(&self) -> PDI16R {
        PDI16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Port Data Input"]
    #[inline]
    pub fn pdi17(&self) -> PDI17R {
        PDI17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Port Data Input"]
    #[inline]
    pub fn pdi18(&self) -> PDI18R {
        PDI18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Port Data Input"]
    #[inline]
    pub fn pdi19(&self) -> PDI19R {
        PDI19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Port Data Input"]
    #[inline]
    pub fn pdi20(&self) -> PDI20R {
        PDI20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Port Data Input"]
    #[inline]
    pub fn pdi21(&self) -> PDI21R {
        PDI21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Port Data Input"]
    #[inline]
    pub fn pdi22(&self) -> PDI22R {
        PDI22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Port Data Input"]
    #[inline]
    pub fn pdi23(&self) -> PDI23R {
        PDI23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port Data Input"]
    #[inline]
    pub fn pdi24(&self) -> PDI24R {
        PDI24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Port Data Input"]
    #[inline]
    pub fn pdi25(&self) -> PDI25R {
        PDI25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Port Data Input"]
    #[inline]
    pub fn pdi26(&self) -> PDI26R {
        PDI26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Port Data Input"]
    #[inline]
    pub fn pdi27(&self) -> PDI27R {
        PDI27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Port Data Input"]
    #[inline]
    pub fn pdi28(&self) -> PDI28R {
        PDI28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Port Data Input"]
    #[inline]
    pub fn pdi29(&self) -> PDI29R {
        PDI29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Port Data Input"]
    #[inline]
    pub fn pdi30(&self) -> PDI30R {
        PDI30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Port Data Input"]
    #[inline]
    pub fn pdi31(&self) -> PDI31R {
        PDI31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
