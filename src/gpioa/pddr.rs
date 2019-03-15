#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDDR {
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
#[doc = "Possible values of the field `PDD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD0R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD0R {
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
            PDD0R::_0 => false,
            PDD0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD0R {
        match value {
            false => PDD0R::_0,
            true => PDD0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD0R::_1
    }
}
#[doc = "Possible values of the field `PDD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD1R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD1R {
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
            PDD1R::_0 => false,
            PDD1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD1R {
        match value {
            false => PDD1R::_0,
            true => PDD1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD1R::_1
    }
}
#[doc = "Possible values of the field `PDD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD2R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD2R {
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
            PDD2R::_0 => false,
            PDD2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD2R {
        match value {
            false => PDD2R::_0,
            true => PDD2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD2R::_1
    }
}
#[doc = "Possible values of the field `PDD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD3R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD3R {
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
            PDD3R::_0 => false,
            PDD3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD3R {
        match value {
            false => PDD3R::_0,
            true => PDD3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD3R::_1
    }
}
#[doc = "Possible values of the field `PDD4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD4R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD4R {
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
            PDD4R::_0 => false,
            PDD4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD4R {
        match value {
            false => PDD4R::_0,
            true => PDD4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD4R::_1
    }
}
#[doc = "Possible values of the field `PDD5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD5R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD5R {
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
            PDD5R::_0 => false,
            PDD5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD5R {
        match value {
            false => PDD5R::_0,
            true => PDD5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD5R::_1
    }
}
#[doc = "Possible values of the field `PDD6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD6R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD6R {
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
            PDD6R::_0 => false,
            PDD6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD6R {
        match value {
            false => PDD6R::_0,
            true => PDD6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD6R::_1
    }
}
#[doc = "Possible values of the field `PDD7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD7R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD7R {
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
            PDD7R::_0 => false,
            PDD7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD7R {
        match value {
            false => PDD7R::_0,
            true => PDD7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD7R::_1
    }
}
#[doc = "Possible values of the field `PDD8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD8R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD8R {
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
            PDD8R::_0 => false,
            PDD8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD8R {
        match value {
            false => PDD8R::_0,
            true => PDD8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD8R::_1
    }
}
#[doc = "Possible values of the field `PDD9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD9R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD9R {
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
            PDD9R::_0 => false,
            PDD9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD9R {
        match value {
            false => PDD9R::_0,
            true => PDD9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD9R::_1
    }
}
#[doc = "Possible values of the field `PDD10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD10R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD10R {
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
            PDD10R::_0 => false,
            PDD10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD10R {
        match value {
            false => PDD10R::_0,
            true => PDD10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD10R::_1
    }
}
#[doc = "Possible values of the field `PDD11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD11R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD11R {
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
            PDD11R::_0 => false,
            PDD11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD11R {
        match value {
            false => PDD11R::_0,
            true => PDD11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD11R::_1
    }
}
#[doc = "Possible values of the field `PDD12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD12R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD12R {
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
            PDD12R::_0 => false,
            PDD12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD12R {
        match value {
            false => PDD12R::_0,
            true => PDD12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD12R::_1
    }
}
#[doc = "Possible values of the field `PDD13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD13R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD13R {
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
            PDD13R::_0 => false,
            PDD13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD13R {
        match value {
            false => PDD13R::_0,
            true => PDD13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD13R::_1
    }
}
#[doc = "Possible values of the field `PDD14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD14R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD14R {
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
            PDD14R::_0 => false,
            PDD14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD14R {
        match value {
            false => PDD14R::_0,
            true => PDD14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD14R::_1
    }
}
#[doc = "Possible values of the field `PDD15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD15R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD15R {
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
            PDD15R::_0 => false,
            PDD15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD15R {
        match value {
            false => PDD15R::_0,
            true => PDD15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD15R::_1
    }
}
#[doc = "Possible values of the field `PDD16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD16R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD16R {
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
            PDD16R::_0 => false,
            PDD16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD16R {
        match value {
            false => PDD16R::_0,
            true => PDD16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD16R::_1
    }
}
#[doc = "Possible values of the field `PDD17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD17R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD17R {
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
            PDD17R::_0 => false,
            PDD17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD17R {
        match value {
            false => PDD17R::_0,
            true => PDD17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD17R::_1
    }
}
#[doc = "Possible values of the field `PDD18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD18R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD18R {
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
            PDD18R::_0 => false,
            PDD18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD18R {
        match value {
            false => PDD18R::_0,
            true => PDD18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD18R::_1
    }
}
#[doc = "Possible values of the field `PDD19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD19R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD19R {
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
            PDD19R::_0 => false,
            PDD19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD19R {
        match value {
            false => PDD19R::_0,
            true => PDD19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD19R::_1
    }
}
#[doc = "Possible values of the field `PDD20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD20R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD20R {
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
            PDD20R::_0 => false,
            PDD20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD20R {
        match value {
            false => PDD20R::_0,
            true => PDD20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD20R::_1
    }
}
#[doc = "Possible values of the field `PDD21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD21R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD21R {
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
            PDD21R::_0 => false,
            PDD21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD21R {
        match value {
            false => PDD21R::_0,
            true => PDD21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD21R::_1
    }
}
#[doc = "Possible values of the field `PDD22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD22R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD22R {
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
            PDD22R::_0 => false,
            PDD22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD22R {
        match value {
            false => PDD22R::_0,
            true => PDD22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD22R::_1
    }
}
#[doc = "Possible values of the field `PDD23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD23R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD23R {
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
            PDD23R::_0 => false,
            PDD23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD23R {
        match value {
            false => PDD23R::_0,
            true => PDD23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD23R::_1
    }
}
#[doc = "Possible values of the field `PDD24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD24R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD24R {
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
            PDD24R::_0 => false,
            PDD24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD24R {
        match value {
            false => PDD24R::_0,
            true => PDD24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD24R::_1
    }
}
#[doc = "Possible values of the field `PDD25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD25R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD25R {
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
            PDD25R::_0 => false,
            PDD25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD25R {
        match value {
            false => PDD25R::_0,
            true => PDD25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD25R::_1
    }
}
#[doc = "Possible values of the field `PDD26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD26R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD26R {
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
            PDD26R::_0 => false,
            PDD26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD26R {
        match value {
            false => PDD26R::_0,
            true => PDD26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD26R::_1
    }
}
#[doc = "Possible values of the field `PDD27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD27R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD27R {
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
            PDD27R::_0 => false,
            PDD27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD27R {
        match value {
            false => PDD27R::_0,
            true => PDD27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD27R::_1
    }
}
#[doc = "Possible values of the field `PDD28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD28R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD28R {
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
            PDD28R::_0 => false,
            PDD28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD28R {
        match value {
            false => PDD28R::_0,
            true => PDD28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD28R::_1
    }
}
#[doc = "Possible values of the field `PDD29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD29R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD29R {
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
            PDD29R::_0 => false,
            PDD29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD29R {
        match value {
            false => PDD29R::_0,
            true => PDD29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD29R::_1
    }
}
#[doc = "Possible values of the field `PDD30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD30R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD30R {
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
            PDD30R::_0 => false,
            PDD30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD30R {
        match value {
            false => PDD30R::_0,
            true => PDD30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD30R::_1
    }
}
#[doc = "Possible values of the field `PDD31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD31R {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD31R {
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
            PDD31R::_0 => false,
            PDD31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDD31R {
        match value {
            false => PDD31R::_0,
            true => PDD31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDD31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDD31R::_1
    }
}
#[doc = "Values that can be written to the field `PDD0`"]
pub enum PDD0W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD0W::_0 => false,
            PDD0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD0W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD0W::_1)
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
#[doc = "Values that can be written to the field `PDD1`"]
pub enum PDD1W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD1W::_0 => false,
            PDD1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD1W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD1W::_1)
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
#[doc = "Values that can be written to the field `PDD2`"]
pub enum PDD2W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD2W::_0 => false,
            PDD2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD2W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD2W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD2W::_1)
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
#[doc = "Values that can be written to the field `PDD3`"]
pub enum PDD3W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD3W::_0 => false,
            PDD3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD3W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD3W::_1)
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
#[doc = "Values that can be written to the field `PDD4`"]
pub enum PDD4W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD4W::_0 => false,
            PDD4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD4W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD4W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD4W::_1)
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
#[doc = "Values that can be written to the field `PDD5`"]
pub enum PDD5W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD5W::_0 => false,
            PDD5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD5W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD5W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD5W::_1)
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
#[doc = "Values that can be written to the field `PDD6`"]
pub enum PDD6W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD6W::_0 => false,
            PDD6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD6W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD6W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD6W::_1)
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
#[doc = "Values that can be written to the field `PDD7`"]
pub enum PDD7W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD7W::_0 => false,
            PDD7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD7W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD7W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD7W::_1)
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
#[doc = "Values that can be written to the field `PDD8`"]
pub enum PDD8W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD8W::_0 => false,
            PDD8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD8W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD8W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD8W::_1)
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
#[doc = "Values that can be written to the field `PDD9`"]
pub enum PDD9W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD9W::_0 => false,
            PDD9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD9W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD9W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD9W::_1)
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
#[doc = "Values that can be written to the field `PDD10`"]
pub enum PDD10W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD10W::_0 => false,
            PDD10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD10W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD10W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD10W::_1)
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
#[doc = "Values that can be written to the field `PDD11`"]
pub enum PDD11W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD11W::_0 => false,
            PDD11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD11W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD11W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD11W::_1)
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
#[doc = "Values that can be written to the field `PDD12`"]
pub enum PDD12W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD12W::_0 => false,
            PDD12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD12W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD12W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD12W::_1)
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
#[doc = "Values that can be written to the field `PDD13`"]
pub enum PDD13W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD13W::_0 => false,
            PDD13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD13W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD13W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD13W::_1)
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
#[doc = "Values that can be written to the field `PDD14`"]
pub enum PDD14W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD14W::_0 => false,
            PDD14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD14W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD14W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD14W::_1)
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
#[doc = "Values that can be written to the field `PDD15`"]
pub enum PDD15W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD15W::_0 => false,
            PDD15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD15W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD15W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD15W::_1)
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
#[doc = "Values that can be written to the field `PDD16`"]
pub enum PDD16W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD16W::_0 => false,
            PDD16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD16W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD16W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD16W::_1)
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
#[doc = "Values that can be written to the field `PDD17`"]
pub enum PDD17W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD17W::_0 => false,
            PDD17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD17W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD17W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD17W::_1)
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
#[doc = "Values that can be written to the field `PDD18`"]
pub enum PDD18W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD18W::_0 => false,
            PDD18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD18W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD18W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD18W::_1)
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
#[doc = "Values that can be written to the field `PDD19`"]
pub enum PDD19W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD19W::_0 => false,
            PDD19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD19W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD19W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD19W::_1)
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
#[doc = "Values that can be written to the field `PDD20`"]
pub enum PDD20W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD20W::_0 => false,
            PDD20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD20W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD20W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD20W::_1)
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
#[doc = "Values that can be written to the field `PDD21`"]
pub enum PDD21W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD21W::_0 => false,
            PDD21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD21W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD21W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD21W::_1)
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
#[doc = "Values that can be written to the field `PDD22`"]
pub enum PDD22W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD22W::_0 => false,
            PDD22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD22W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD22W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD22W::_1)
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
#[doc = "Values that can be written to the field `PDD23`"]
pub enum PDD23W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD23W::_0 => false,
            PDD23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD23W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD23W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD23W::_1)
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
#[doc = "Values that can be written to the field `PDD24`"]
pub enum PDD24W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD24W::_0 => false,
            PDD24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD24W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD24W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD24W::_1)
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
#[doc = "Values that can be written to the field `PDD25`"]
pub enum PDD25W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD25W::_0 => false,
            PDD25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD25W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD25W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD25W::_1)
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
#[doc = "Values that can be written to the field `PDD26`"]
pub enum PDD26W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD26W::_0 => false,
            PDD26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD26W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD26W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD26W::_1)
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
#[doc = "Values that can be written to the field `PDD27`"]
pub enum PDD27W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD27W::_0 => false,
            PDD27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD27W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD27W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD27W::_1)
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
#[doc = "Values that can be written to the field `PDD28`"]
pub enum PDD28W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD28W::_0 => false,
            PDD28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD28W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD28W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD28W::_1)
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
#[doc = "Values that can be written to the field `PDD29`"]
pub enum PDD29W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD29W::_0 => false,
            PDD29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD29W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD29W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD29W::_1)
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
#[doc = "Values that can be written to the field `PDD30`"]
pub enum PDD30W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD30W::_0 => false,
            PDD30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD30W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD30W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD30W::_1)
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
#[doc = "Values that can be written to the field `PDD31`"]
pub enum PDD31W {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl PDD31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDD31W::_0 => false,
            PDD31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDD31W<'a> {
    w: &'a mut W,
}
impl<'a> _PDD31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDD31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD31W::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD31W::_1)
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
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline]
    pub fn pdd0(&self) -> PDD0R {
        PDD0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline]
    pub fn pdd1(&self) -> PDD1R {
        PDD1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline]
    pub fn pdd2(&self) -> PDD2R {
        PDD2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline]
    pub fn pdd3(&self) -> PDD3R {
        PDD3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline]
    pub fn pdd4(&self) -> PDD4R {
        PDD4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline]
    pub fn pdd5(&self) -> PDD5R {
        PDD5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline]
    pub fn pdd6(&self) -> PDD6R {
        PDD6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline]
    pub fn pdd7(&self) -> PDD7R {
        PDD7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline]
    pub fn pdd8(&self) -> PDD8R {
        PDD8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline]
    pub fn pdd9(&self) -> PDD9R {
        PDD9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline]
    pub fn pdd10(&self) -> PDD10R {
        PDD10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline]
    pub fn pdd11(&self) -> PDD11R {
        PDD11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline]
    pub fn pdd12(&self) -> PDD12R {
        PDD12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline]
    pub fn pdd13(&self) -> PDD13R {
        PDD13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline]
    pub fn pdd14(&self) -> PDD14R {
        PDD14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline]
    pub fn pdd15(&self) -> PDD15R {
        PDD15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline]
    pub fn pdd16(&self) -> PDD16R {
        PDD16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline]
    pub fn pdd17(&self) -> PDD17R {
        PDD17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline]
    pub fn pdd18(&self) -> PDD18R {
        PDD18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline]
    pub fn pdd19(&self) -> PDD19R {
        PDD19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline]
    pub fn pdd20(&self) -> PDD20R {
        PDD20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline]
    pub fn pdd21(&self) -> PDD21R {
        PDD21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline]
    pub fn pdd22(&self) -> PDD22R {
        PDD22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline]
    pub fn pdd23(&self) -> PDD23R {
        PDD23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline]
    pub fn pdd24(&self) -> PDD24R {
        PDD24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline]
    pub fn pdd25(&self) -> PDD25R {
        PDD25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline]
    pub fn pdd26(&self) -> PDD26R {
        PDD26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline]
    pub fn pdd27(&self) -> PDD27R {
        PDD27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline]
    pub fn pdd28(&self) -> PDD28R {
        PDD28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline]
    pub fn pdd29(&self) -> PDD29R {
        PDD29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline]
    pub fn pdd30(&self) -> PDD30R {
        PDD30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline]
    pub fn pdd31(&self) -> PDD31R {
        PDD31R::_from({
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
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline]
    pub fn pdd0(&mut self) -> _PDD0W {
        _PDD0W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline]
    pub fn pdd1(&mut self) -> _PDD1W {
        _PDD1W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline]
    pub fn pdd2(&mut self) -> _PDD2W {
        _PDD2W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline]
    pub fn pdd3(&mut self) -> _PDD3W {
        _PDD3W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline]
    pub fn pdd4(&mut self) -> _PDD4W {
        _PDD4W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline]
    pub fn pdd5(&mut self) -> _PDD5W {
        _PDD5W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline]
    pub fn pdd6(&mut self) -> _PDD6W {
        _PDD6W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline]
    pub fn pdd7(&mut self) -> _PDD7W {
        _PDD7W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline]
    pub fn pdd8(&mut self) -> _PDD8W {
        _PDD8W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline]
    pub fn pdd9(&mut self) -> _PDD9W {
        _PDD9W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline]
    pub fn pdd10(&mut self) -> _PDD10W {
        _PDD10W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline]
    pub fn pdd11(&mut self) -> _PDD11W {
        _PDD11W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline]
    pub fn pdd12(&mut self) -> _PDD12W {
        _PDD12W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline]
    pub fn pdd13(&mut self) -> _PDD13W {
        _PDD13W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline]
    pub fn pdd14(&mut self) -> _PDD14W {
        _PDD14W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline]
    pub fn pdd15(&mut self) -> _PDD15W {
        _PDD15W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline]
    pub fn pdd16(&mut self) -> _PDD16W {
        _PDD16W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline]
    pub fn pdd17(&mut self) -> _PDD17W {
        _PDD17W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline]
    pub fn pdd18(&mut self) -> _PDD18W {
        _PDD18W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline]
    pub fn pdd19(&mut self) -> _PDD19W {
        _PDD19W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline]
    pub fn pdd20(&mut self) -> _PDD20W {
        _PDD20W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline]
    pub fn pdd21(&mut self) -> _PDD21W {
        _PDD21W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline]
    pub fn pdd22(&mut self) -> _PDD22W {
        _PDD22W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline]
    pub fn pdd23(&mut self) -> _PDD23W {
        _PDD23W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline]
    pub fn pdd24(&mut self) -> _PDD24W {
        _PDD24W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline]
    pub fn pdd25(&mut self) -> _PDD25W {
        _PDD25W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline]
    pub fn pdd26(&mut self) -> _PDD26W {
        _PDD26W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline]
    pub fn pdd27(&mut self) -> _PDD27W {
        _PDD27W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline]
    pub fn pdd28(&mut self) -> _PDD28W {
        _PDD28W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline]
    pub fn pdd29(&mut self) -> _PDD29W {
        _PDD29W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline]
    pub fn pdd30(&mut self) -> _PDD30W {
        _PDD30W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline]
    pub fn pdd31(&mut self) -> _PDD31W {
        _PDD31W { w: self }
    }
}
