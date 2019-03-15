#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDOR {
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
#[doc = "Possible values of the field `PDO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO0R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO0R {
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
            PDO0R::_0 => false,
            PDO0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO0R {
        match value {
            false => PDO0R::_0,
            true => PDO0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO0R::_1
    }
}
#[doc = "Possible values of the field `PDO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO1R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO1R {
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
            PDO1R::_0 => false,
            PDO1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO1R {
        match value {
            false => PDO1R::_0,
            true => PDO1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO1R::_1
    }
}
#[doc = "Possible values of the field `PDO2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO2R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO2R {
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
            PDO2R::_0 => false,
            PDO2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO2R {
        match value {
            false => PDO2R::_0,
            true => PDO2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO2R::_1
    }
}
#[doc = "Possible values of the field `PDO3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO3R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO3R {
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
            PDO3R::_0 => false,
            PDO3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO3R {
        match value {
            false => PDO3R::_0,
            true => PDO3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO3R::_1
    }
}
#[doc = "Possible values of the field `PDO4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO4R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO4R {
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
            PDO4R::_0 => false,
            PDO4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO4R {
        match value {
            false => PDO4R::_0,
            true => PDO4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO4R::_1
    }
}
#[doc = "Possible values of the field `PDO5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO5R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO5R {
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
            PDO5R::_0 => false,
            PDO5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO5R {
        match value {
            false => PDO5R::_0,
            true => PDO5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO5R::_1
    }
}
#[doc = "Possible values of the field `PDO6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO6R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO6R {
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
            PDO6R::_0 => false,
            PDO6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO6R {
        match value {
            false => PDO6R::_0,
            true => PDO6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO6R::_1
    }
}
#[doc = "Possible values of the field `PDO7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO7R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO7R {
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
            PDO7R::_0 => false,
            PDO7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO7R {
        match value {
            false => PDO7R::_0,
            true => PDO7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO7R::_1
    }
}
#[doc = "Possible values of the field `PDO8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO8R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO8R {
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
            PDO8R::_0 => false,
            PDO8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO8R {
        match value {
            false => PDO8R::_0,
            true => PDO8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO8R::_1
    }
}
#[doc = "Possible values of the field `PDO9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO9R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO9R {
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
            PDO9R::_0 => false,
            PDO9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO9R {
        match value {
            false => PDO9R::_0,
            true => PDO9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO9R::_1
    }
}
#[doc = "Possible values of the field `PDO10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO10R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO10R {
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
            PDO10R::_0 => false,
            PDO10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO10R {
        match value {
            false => PDO10R::_0,
            true => PDO10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO10R::_1
    }
}
#[doc = "Possible values of the field `PDO11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO11R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO11R {
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
            PDO11R::_0 => false,
            PDO11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO11R {
        match value {
            false => PDO11R::_0,
            true => PDO11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO11R::_1
    }
}
#[doc = "Possible values of the field `PDO12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO12R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO12R {
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
            PDO12R::_0 => false,
            PDO12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO12R {
        match value {
            false => PDO12R::_0,
            true => PDO12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO12R::_1
    }
}
#[doc = "Possible values of the field `PDO13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO13R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO13R {
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
            PDO13R::_0 => false,
            PDO13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO13R {
        match value {
            false => PDO13R::_0,
            true => PDO13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO13R::_1
    }
}
#[doc = "Possible values of the field `PDO14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO14R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO14R {
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
            PDO14R::_0 => false,
            PDO14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO14R {
        match value {
            false => PDO14R::_0,
            true => PDO14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO14R::_1
    }
}
#[doc = "Possible values of the field `PDO15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO15R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO15R {
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
            PDO15R::_0 => false,
            PDO15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO15R {
        match value {
            false => PDO15R::_0,
            true => PDO15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO15R::_1
    }
}
#[doc = "Possible values of the field `PDO16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO16R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO16R {
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
            PDO16R::_0 => false,
            PDO16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO16R {
        match value {
            false => PDO16R::_0,
            true => PDO16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO16R::_1
    }
}
#[doc = "Possible values of the field `PDO17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO17R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO17R {
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
            PDO17R::_0 => false,
            PDO17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO17R {
        match value {
            false => PDO17R::_0,
            true => PDO17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO17R::_1
    }
}
#[doc = "Possible values of the field `PDO18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO18R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO18R {
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
            PDO18R::_0 => false,
            PDO18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO18R {
        match value {
            false => PDO18R::_0,
            true => PDO18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO18R::_1
    }
}
#[doc = "Possible values of the field `PDO19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO19R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO19R {
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
            PDO19R::_0 => false,
            PDO19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO19R {
        match value {
            false => PDO19R::_0,
            true => PDO19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO19R::_1
    }
}
#[doc = "Possible values of the field `PDO20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO20R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO20R {
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
            PDO20R::_0 => false,
            PDO20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO20R {
        match value {
            false => PDO20R::_0,
            true => PDO20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO20R::_1
    }
}
#[doc = "Possible values of the field `PDO21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO21R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO21R {
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
            PDO21R::_0 => false,
            PDO21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO21R {
        match value {
            false => PDO21R::_0,
            true => PDO21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO21R::_1
    }
}
#[doc = "Possible values of the field `PDO22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO22R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO22R {
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
            PDO22R::_0 => false,
            PDO22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO22R {
        match value {
            false => PDO22R::_0,
            true => PDO22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO22R::_1
    }
}
#[doc = "Possible values of the field `PDO23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO23R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO23R {
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
            PDO23R::_0 => false,
            PDO23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO23R {
        match value {
            false => PDO23R::_0,
            true => PDO23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO23R::_1
    }
}
#[doc = "Possible values of the field `PDO24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO24R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO24R {
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
            PDO24R::_0 => false,
            PDO24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO24R {
        match value {
            false => PDO24R::_0,
            true => PDO24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO24R::_1
    }
}
#[doc = "Possible values of the field `PDO25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO25R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO25R {
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
            PDO25R::_0 => false,
            PDO25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO25R {
        match value {
            false => PDO25R::_0,
            true => PDO25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO25R::_1
    }
}
#[doc = "Possible values of the field `PDO26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO26R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO26R {
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
            PDO26R::_0 => false,
            PDO26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO26R {
        match value {
            false => PDO26R::_0,
            true => PDO26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO26R::_1
    }
}
#[doc = "Possible values of the field `PDO27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO27R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO27R {
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
            PDO27R::_0 => false,
            PDO27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO27R {
        match value {
            false => PDO27R::_0,
            true => PDO27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO27R::_1
    }
}
#[doc = "Possible values of the field `PDO28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO28R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO28R {
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
            PDO28R::_0 => false,
            PDO28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO28R {
        match value {
            false => PDO28R::_0,
            true => PDO28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO28R::_1
    }
}
#[doc = "Possible values of the field `PDO29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO29R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO29R {
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
            PDO29R::_0 => false,
            PDO29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO29R {
        match value {
            false => PDO29R::_0,
            true => PDO29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO29R::_1
    }
}
#[doc = "Possible values of the field `PDO30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO30R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO30R {
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
            PDO30R::_0 => false,
            PDO30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO30R {
        match value {
            false => PDO30R::_0,
            true => PDO30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO30R::_1
    }
}
#[doc = "Possible values of the field `PDO31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDO31R {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO31R {
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
            PDO31R::_0 => false,
            PDO31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDO31R {
        match value {
            false => PDO31R::_0,
            true => PDO31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDO31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDO31R::_1
    }
}
#[doc = "Values that can be written to the field `PDO0`"]
pub enum PDO0W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO0W::_0 => false,
            PDO0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO0W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO0W::_1)
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
#[doc = "Values that can be written to the field `PDO1`"]
pub enum PDO1W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO1W::_0 => false,
            PDO1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO1W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO1W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO1W::_1)
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
#[doc = "Values that can be written to the field `PDO2`"]
pub enum PDO2W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO2W::_0 => false,
            PDO2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO2W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO2W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO2W::_1)
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
#[doc = "Values that can be written to the field `PDO3`"]
pub enum PDO3W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO3W::_0 => false,
            PDO3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO3W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO3W::_1)
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
#[doc = "Values that can be written to the field `PDO4`"]
pub enum PDO4W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO4W::_0 => false,
            PDO4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO4W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO4W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO4W::_1)
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
#[doc = "Values that can be written to the field `PDO5`"]
pub enum PDO5W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO5W::_0 => false,
            PDO5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO5W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO5W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO5W::_1)
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
#[doc = "Values that can be written to the field `PDO6`"]
pub enum PDO6W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO6W::_0 => false,
            PDO6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO6W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO6W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO6W::_1)
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
#[doc = "Values that can be written to the field `PDO7`"]
pub enum PDO7W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO7W::_0 => false,
            PDO7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO7W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO7W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO7W::_1)
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
#[doc = "Values that can be written to the field `PDO8`"]
pub enum PDO8W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO8W::_0 => false,
            PDO8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO8W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO8W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO8W::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO9`"]
pub enum PDO9W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO9W::_0 => false,
            PDO9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO9W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO9W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO9W::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO10`"]
pub enum PDO10W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO10W::_0 => false,
            PDO10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO10W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO10W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO10W::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO11`"]
pub enum PDO11W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO11W::_0 => false,
            PDO11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO11W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO11W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO11W::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO12`"]
pub enum PDO12W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO12W::_0 => false,
            PDO12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO12W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO12W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO12W::_1)
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
#[doc = "Values that can be written to the field `PDO13`"]
pub enum PDO13W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO13W::_0 => false,
            PDO13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO13W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO13W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO13W::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO14`"]
pub enum PDO14W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO14W::_0 => false,
            PDO14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO14W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO14W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO14W::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO15`"]
pub enum PDO15W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO15W::_0 => false,
            PDO15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO15W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO15W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO15W::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO16`"]
pub enum PDO16W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO16W::_0 => false,
            PDO16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO16W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO16W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO16W::_1)
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
#[doc = "Values that can be written to the field `PDO17`"]
pub enum PDO17W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO17W::_0 => false,
            PDO17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO17W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO17W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO17W::_1)
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
#[doc = "Values that can be written to the field `PDO18`"]
pub enum PDO18W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO18W::_0 => false,
            PDO18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO18W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO18W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO18W::_1)
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
#[doc = "Values that can be written to the field `PDO19`"]
pub enum PDO19W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO19W::_0 => false,
            PDO19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO19W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO19W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO19W::_1)
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
#[doc = "Values that can be written to the field `PDO20`"]
pub enum PDO20W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO20W::_0 => false,
            PDO20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO20W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO20W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO20W::_1)
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
#[doc = "Values that can be written to the field `PDO21`"]
pub enum PDO21W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO21W::_0 => false,
            PDO21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO21W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO21W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO21W::_1)
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
#[doc = "Values that can be written to the field `PDO22`"]
pub enum PDO22W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO22W::_0 => false,
            PDO22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO22W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO22W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO22W::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO23`"]
pub enum PDO23W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO23W::_0 => false,
            PDO23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO23W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO23W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO23W::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO24`"]
pub enum PDO24W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO24W::_0 => false,
            PDO24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO24W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO24W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO24W::_1)
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
#[doc = "Values that can be written to the field `PDO25`"]
pub enum PDO25W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO25W::_0 => false,
            PDO25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO25W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO25W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO25W::_1)
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
#[doc = "Values that can be written to the field `PDO26`"]
pub enum PDO26W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO26W::_0 => false,
            PDO26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO26W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO26W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO26W::_1)
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
#[doc = "Values that can be written to the field `PDO27`"]
pub enum PDO27W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO27W::_0 => false,
            PDO27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO27W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO27W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO27W::_1)
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
#[doc = "Values that can be written to the field `PDO28`"]
pub enum PDO28W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO28W::_0 => false,
            PDO28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO28W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO28W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO28W::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO29`"]
pub enum PDO29W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO29W::_0 => false,
            PDO29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO29W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO29W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO29W::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO30`"]
pub enum PDO30W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO30W::_0 => false,
            PDO30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO30W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO30W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO30W::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDO31`"]
pub enum PDO31W {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0,
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1,
}
impl PDO31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDO31W::_0 => false,
            PDO31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDO31W<'a> {
    w: &'a mut W,
}
impl<'a> _PDO31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDO31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO31W::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO31W::_1)
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
    #[doc = "Bit 0 - Port Data Output"]
    #[inline]
    pub fn pdo0(&self) -> PDO0R {
        PDO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline]
    pub fn pdo1(&self) -> PDO1R {
        PDO1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline]
    pub fn pdo2(&self) -> PDO2R {
        PDO2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline]
    pub fn pdo3(&self) -> PDO3R {
        PDO3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline]
    pub fn pdo4(&self) -> PDO4R {
        PDO4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline]
    pub fn pdo5(&self) -> PDO5R {
        PDO5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline]
    pub fn pdo6(&self) -> PDO6R {
        PDO6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline]
    pub fn pdo7(&self) -> PDO7R {
        PDO7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline]
    pub fn pdo8(&self) -> PDO8R {
        PDO8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline]
    pub fn pdo9(&self) -> PDO9R {
        PDO9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline]
    pub fn pdo10(&self) -> PDO10R {
        PDO10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline]
    pub fn pdo11(&self) -> PDO11R {
        PDO11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline]
    pub fn pdo12(&self) -> PDO12R {
        PDO12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline]
    pub fn pdo13(&self) -> PDO13R {
        PDO13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline]
    pub fn pdo14(&self) -> PDO14R {
        PDO14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline]
    pub fn pdo15(&self) -> PDO15R {
        PDO15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline]
    pub fn pdo16(&self) -> PDO16R {
        PDO16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline]
    pub fn pdo17(&self) -> PDO17R {
        PDO17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline]
    pub fn pdo18(&self) -> PDO18R {
        PDO18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline]
    pub fn pdo19(&self) -> PDO19R {
        PDO19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline]
    pub fn pdo20(&self) -> PDO20R {
        PDO20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline]
    pub fn pdo21(&self) -> PDO21R {
        PDO21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline]
    pub fn pdo22(&self) -> PDO22R {
        PDO22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline]
    pub fn pdo23(&self) -> PDO23R {
        PDO23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline]
    pub fn pdo24(&self) -> PDO24R {
        PDO24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline]
    pub fn pdo25(&self) -> PDO25R {
        PDO25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline]
    pub fn pdo26(&self) -> PDO26R {
        PDO26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline]
    pub fn pdo27(&self) -> PDO27R {
        PDO27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline]
    pub fn pdo28(&self) -> PDO28R {
        PDO28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline]
    pub fn pdo29(&self) -> PDO29R {
        PDO29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline]
    pub fn pdo30(&self) -> PDO30R {
        PDO30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline]
    pub fn pdo31(&self) -> PDO31R {
        PDO31R::_from({
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
    #[doc = "Bit 0 - Port Data Output"]
    #[inline]
    pub fn pdo0(&mut self) -> _PDO0W {
        _PDO0W { w: self }
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline]
    pub fn pdo1(&mut self) -> _PDO1W {
        _PDO1W { w: self }
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline]
    pub fn pdo2(&mut self) -> _PDO2W {
        _PDO2W { w: self }
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline]
    pub fn pdo3(&mut self) -> _PDO3W {
        _PDO3W { w: self }
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline]
    pub fn pdo4(&mut self) -> _PDO4W {
        _PDO4W { w: self }
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline]
    pub fn pdo5(&mut self) -> _PDO5W {
        _PDO5W { w: self }
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline]
    pub fn pdo6(&mut self) -> _PDO6W {
        _PDO6W { w: self }
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline]
    pub fn pdo7(&mut self) -> _PDO7W {
        _PDO7W { w: self }
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline]
    pub fn pdo8(&mut self) -> _PDO8W {
        _PDO8W { w: self }
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline]
    pub fn pdo9(&mut self) -> _PDO9W {
        _PDO9W { w: self }
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline]
    pub fn pdo10(&mut self) -> _PDO10W {
        _PDO10W { w: self }
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline]
    pub fn pdo11(&mut self) -> _PDO11W {
        _PDO11W { w: self }
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline]
    pub fn pdo12(&mut self) -> _PDO12W {
        _PDO12W { w: self }
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline]
    pub fn pdo13(&mut self) -> _PDO13W {
        _PDO13W { w: self }
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline]
    pub fn pdo14(&mut self) -> _PDO14W {
        _PDO14W { w: self }
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline]
    pub fn pdo15(&mut self) -> _PDO15W {
        _PDO15W { w: self }
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline]
    pub fn pdo16(&mut self) -> _PDO16W {
        _PDO16W { w: self }
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline]
    pub fn pdo17(&mut self) -> _PDO17W {
        _PDO17W { w: self }
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline]
    pub fn pdo18(&mut self) -> _PDO18W {
        _PDO18W { w: self }
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline]
    pub fn pdo19(&mut self) -> _PDO19W {
        _PDO19W { w: self }
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline]
    pub fn pdo20(&mut self) -> _PDO20W {
        _PDO20W { w: self }
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline]
    pub fn pdo21(&mut self) -> _PDO21W {
        _PDO21W { w: self }
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline]
    pub fn pdo22(&mut self) -> _PDO22W {
        _PDO22W { w: self }
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline]
    pub fn pdo23(&mut self) -> _PDO23W {
        _PDO23W { w: self }
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline]
    pub fn pdo24(&mut self) -> _PDO24W {
        _PDO24W { w: self }
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline]
    pub fn pdo25(&mut self) -> _PDO25W {
        _PDO25W { w: self }
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline]
    pub fn pdo26(&mut self) -> _PDO26W {
        _PDO26W { w: self }
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline]
    pub fn pdo27(&mut self) -> _PDO27W {
        _PDO27W { w: self }
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline]
    pub fn pdo28(&mut self) -> _PDO28W {
        _PDO28W { w: self }
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline]
    pub fn pdo29(&mut self) -> _PDO29W {
        _PDO29W { w: self }
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline]
    pub fn pdo30(&mut self) -> _PDO30W {
        _PDO30W { w: self }
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline]
    pub fn pdo31(&mut self) -> _PDO31W {
        _PDO31W { w: self }
    }
}
