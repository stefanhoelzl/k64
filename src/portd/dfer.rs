#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DFER {
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
#[doc = "Possible values of the field `DFE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE0R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE0R {
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
            DFE0R::_0 => false,
            DFE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE0R {
        match value {
            false => DFE0R::_0,
            true => DFE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE0R::_1
    }
}
#[doc = "Possible values of the field `DFE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE1R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE1R {
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
            DFE1R::_0 => false,
            DFE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE1R {
        match value {
            false => DFE1R::_0,
            true => DFE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE1R::_1
    }
}
#[doc = "Possible values of the field `DFE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE2R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE2R {
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
            DFE2R::_0 => false,
            DFE2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE2R {
        match value {
            false => DFE2R::_0,
            true => DFE2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE2R::_1
    }
}
#[doc = "Possible values of the field `DFE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE3R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE3R {
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
            DFE3R::_0 => false,
            DFE3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE3R {
        match value {
            false => DFE3R::_0,
            true => DFE3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE3R::_1
    }
}
#[doc = "Possible values of the field `DFE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE4R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE4R {
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
            DFE4R::_0 => false,
            DFE4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE4R {
        match value {
            false => DFE4R::_0,
            true => DFE4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE4R::_1
    }
}
#[doc = "Possible values of the field `DFE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE5R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE5R {
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
            DFE5R::_0 => false,
            DFE5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE5R {
        match value {
            false => DFE5R::_0,
            true => DFE5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE5R::_1
    }
}
#[doc = "Possible values of the field `DFE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE6R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE6R {
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
            DFE6R::_0 => false,
            DFE6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE6R {
        match value {
            false => DFE6R::_0,
            true => DFE6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE6R::_1
    }
}
#[doc = "Possible values of the field `DFE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE7R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE7R {
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
            DFE7R::_0 => false,
            DFE7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE7R {
        match value {
            false => DFE7R::_0,
            true => DFE7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE7R::_1
    }
}
#[doc = "Possible values of the field `DFE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE8R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE8R {
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
            DFE8R::_0 => false,
            DFE8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE8R {
        match value {
            false => DFE8R::_0,
            true => DFE8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE8R::_1
    }
}
#[doc = "Possible values of the field `DFE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE9R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE9R {
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
            DFE9R::_0 => false,
            DFE9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE9R {
        match value {
            false => DFE9R::_0,
            true => DFE9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE9R::_1
    }
}
#[doc = "Possible values of the field `DFE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE10R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE10R {
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
            DFE10R::_0 => false,
            DFE10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE10R {
        match value {
            false => DFE10R::_0,
            true => DFE10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE10R::_1
    }
}
#[doc = "Possible values of the field `DFE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE11R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE11R {
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
            DFE11R::_0 => false,
            DFE11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE11R {
        match value {
            false => DFE11R::_0,
            true => DFE11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE11R::_1
    }
}
#[doc = "Possible values of the field `DFE12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE12R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE12R {
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
            DFE12R::_0 => false,
            DFE12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE12R {
        match value {
            false => DFE12R::_0,
            true => DFE12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE12R::_1
    }
}
#[doc = "Possible values of the field `DFE13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE13R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE13R {
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
            DFE13R::_0 => false,
            DFE13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE13R {
        match value {
            false => DFE13R::_0,
            true => DFE13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE13R::_1
    }
}
#[doc = "Possible values of the field `DFE14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE14R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE14R {
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
            DFE14R::_0 => false,
            DFE14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE14R {
        match value {
            false => DFE14R::_0,
            true => DFE14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE14R::_1
    }
}
#[doc = "Possible values of the field `DFE15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE15R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE15R {
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
            DFE15R::_0 => false,
            DFE15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE15R {
        match value {
            false => DFE15R::_0,
            true => DFE15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE15R::_1
    }
}
#[doc = "Possible values of the field `DFE16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE16R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE16R {
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
            DFE16R::_0 => false,
            DFE16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE16R {
        match value {
            false => DFE16R::_0,
            true => DFE16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE16R::_1
    }
}
#[doc = "Possible values of the field `DFE17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE17R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE17R {
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
            DFE17R::_0 => false,
            DFE17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE17R {
        match value {
            false => DFE17R::_0,
            true => DFE17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE17R::_1
    }
}
#[doc = "Possible values of the field `DFE18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE18R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE18R {
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
            DFE18R::_0 => false,
            DFE18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE18R {
        match value {
            false => DFE18R::_0,
            true => DFE18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE18R::_1
    }
}
#[doc = "Possible values of the field `DFE19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE19R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE19R {
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
            DFE19R::_0 => false,
            DFE19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE19R {
        match value {
            false => DFE19R::_0,
            true => DFE19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE19R::_1
    }
}
#[doc = "Possible values of the field `DFE20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE20R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE20R {
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
            DFE20R::_0 => false,
            DFE20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE20R {
        match value {
            false => DFE20R::_0,
            true => DFE20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE20R::_1
    }
}
#[doc = "Possible values of the field `DFE21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE21R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE21R {
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
            DFE21R::_0 => false,
            DFE21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE21R {
        match value {
            false => DFE21R::_0,
            true => DFE21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE21R::_1
    }
}
#[doc = "Possible values of the field `DFE22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE22R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE22R {
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
            DFE22R::_0 => false,
            DFE22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE22R {
        match value {
            false => DFE22R::_0,
            true => DFE22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE22R::_1
    }
}
#[doc = "Possible values of the field `DFE23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE23R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE23R {
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
            DFE23R::_0 => false,
            DFE23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE23R {
        match value {
            false => DFE23R::_0,
            true => DFE23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE23R::_1
    }
}
#[doc = "Possible values of the field `DFE24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE24R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE24R {
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
            DFE24R::_0 => false,
            DFE24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE24R {
        match value {
            false => DFE24R::_0,
            true => DFE24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE24R::_1
    }
}
#[doc = "Possible values of the field `DFE25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE25R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE25R {
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
            DFE25R::_0 => false,
            DFE25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE25R {
        match value {
            false => DFE25R::_0,
            true => DFE25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE25R::_1
    }
}
#[doc = "Possible values of the field `DFE26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE26R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE26R {
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
            DFE26R::_0 => false,
            DFE26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE26R {
        match value {
            false => DFE26R::_0,
            true => DFE26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE26R::_1
    }
}
#[doc = "Possible values of the field `DFE27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE27R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE27R {
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
            DFE27R::_0 => false,
            DFE27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE27R {
        match value {
            false => DFE27R::_0,
            true => DFE27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE27R::_1
    }
}
#[doc = "Possible values of the field `DFE28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE28R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE28R {
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
            DFE28R::_0 => false,
            DFE28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE28R {
        match value {
            false => DFE28R::_0,
            true => DFE28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE28R::_1
    }
}
#[doc = "Possible values of the field `DFE29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE29R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE29R {
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
            DFE29R::_0 => false,
            DFE29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE29R {
        match value {
            false => DFE29R::_0,
            true => DFE29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE29R::_1
    }
}
#[doc = "Possible values of the field `DFE30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE30R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE30R {
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
            DFE30R::_0 => false,
            DFE30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE30R {
        match value {
            false => DFE30R::_0,
            true => DFE30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE30R::_1
    }
}
#[doc = "Possible values of the field `DFE31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE31R {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE31R {
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
            DFE31R::_0 => false,
            DFE31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFE31R {
        match value {
            false => DFE31R::_0,
            true => DFE31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFE31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFE31R::_1
    }
}
#[doc = "Values that can be written to the field `DFE0`"]
pub enum DFE0W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE0W::_0 => false,
            DFE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE0W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE0W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE0W::_1)
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
#[doc = "Values that can be written to the field `DFE1`"]
pub enum DFE1W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE1W::_0 => false,
            DFE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE1W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE1W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE1W::_1)
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
#[doc = "Values that can be written to the field `DFE2`"]
pub enum DFE2W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE2W::_0 => false,
            DFE2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE2W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE2W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE2W::_1)
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
#[doc = "Values that can be written to the field `DFE3`"]
pub enum DFE3W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE3W::_0 => false,
            DFE3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE3W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE3W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE3W::_1)
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
#[doc = "Values that can be written to the field `DFE4`"]
pub enum DFE4W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE4W::_0 => false,
            DFE4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE4W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE4W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE4W::_1)
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
#[doc = "Values that can be written to the field `DFE5`"]
pub enum DFE5W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE5W::_0 => false,
            DFE5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE5W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE5W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE5W::_1)
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
#[doc = "Values that can be written to the field `DFE6`"]
pub enum DFE6W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE6W::_0 => false,
            DFE6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE6W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE6W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE6W::_1)
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
#[doc = "Values that can be written to the field `DFE7`"]
pub enum DFE7W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE7W::_0 => false,
            DFE7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE7W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE7W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE7W::_1)
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
#[doc = "Values that can be written to the field `DFE8`"]
pub enum DFE8W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE8W::_0 => false,
            DFE8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE8W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE8W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE8W::_1)
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
#[doc = "Values that can be written to the field `DFE9`"]
pub enum DFE9W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE9W::_0 => false,
            DFE9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE9W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE9W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE9W::_1)
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
#[doc = "Values that can be written to the field `DFE10`"]
pub enum DFE10W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE10W::_0 => false,
            DFE10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE10W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE10W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE10W::_1)
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
#[doc = "Values that can be written to the field `DFE11`"]
pub enum DFE11W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE11W::_0 => false,
            DFE11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE11W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE11W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE11W::_1)
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
#[doc = "Values that can be written to the field `DFE12`"]
pub enum DFE12W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE12W::_0 => false,
            DFE12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE12W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE12W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE12W::_1)
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
#[doc = "Values that can be written to the field `DFE13`"]
pub enum DFE13W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE13W::_0 => false,
            DFE13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE13W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE13W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE13W::_1)
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
#[doc = "Values that can be written to the field `DFE14`"]
pub enum DFE14W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE14W::_0 => false,
            DFE14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE14W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE14W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE14W::_1)
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
#[doc = "Values that can be written to the field `DFE15`"]
pub enum DFE15W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE15W::_0 => false,
            DFE15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE15W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE15W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE15W::_1)
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
#[doc = "Values that can be written to the field `DFE16`"]
pub enum DFE16W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE16W::_0 => false,
            DFE16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE16W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE16W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE16W::_1)
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
#[doc = "Values that can be written to the field `DFE17`"]
pub enum DFE17W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE17W::_0 => false,
            DFE17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE17W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE17W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE17W::_1)
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
#[doc = "Values that can be written to the field `DFE18`"]
pub enum DFE18W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE18W::_0 => false,
            DFE18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE18W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE18W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE18W::_1)
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
#[doc = "Values that can be written to the field `DFE19`"]
pub enum DFE19W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE19W::_0 => false,
            DFE19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE19W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE19W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE19W::_1)
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
#[doc = "Values that can be written to the field `DFE20`"]
pub enum DFE20W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE20W::_0 => false,
            DFE20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE20W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE20W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE20W::_1)
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
#[doc = "Values that can be written to the field `DFE21`"]
pub enum DFE21W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE21W::_0 => false,
            DFE21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE21W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE21W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE21W::_1)
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
#[doc = "Values that can be written to the field `DFE22`"]
pub enum DFE22W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE22W::_0 => false,
            DFE22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE22W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE22W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE22W::_1)
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
#[doc = "Values that can be written to the field `DFE23`"]
pub enum DFE23W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE23W::_0 => false,
            DFE23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE23W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE23W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE23W::_1)
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
#[doc = "Values that can be written to the field `DFE24`"]
pub enum DFE24W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE24W::_0 => false,
            DFE24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE24W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE24W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE24W::_1)
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
#[doc = "Values that can be written to the field `DFE25`"]
pub enum DFE25W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE25W::_0 => false,
            DFE25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE25W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE25W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE25W::_1)
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
#[doc = "Values that can be written to the field `DFE26`"]
pub enum DFE26W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE26W::_0 => false,
            DFE26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE26W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE26W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE26W::_1)
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
#[doc = "Values that can be written to the field `DFE27`"]
pub enum DFE27W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE27W::_0 => false,
            DFE27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE27W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE27W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE27W::_1)
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
#[doc = "Values that can be written to the field `DFE28`"]
pub enum DFE28W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE28W::_0 => false,
            DFE28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE28W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE28W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE28W::_1)
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
#[doc = "Values that can be written to the field `DFE29`"]
pub enum DFE29W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE29W::_0 => false,
            DFE29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE29W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE29W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE29W::_1)
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
#[doc = "Values that can be written to the field `DFE30`"]
pub enum DFE30W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE30W::_0 => false,
            DFE30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE30W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE30W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE30W::_1)
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
#[doc = "Values that can be written to the field `DFE31`"]
pub enum DFE31W {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFE31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFE31W::_0 => false,
            DFE31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFE31W<'a> {
    w: &'a mut W,
}
impl<'a> _DFE31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFE31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE31W::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE31W::_1)
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
    #[doc = "Bit 0 - Digital Filter Enable"]
    #[inline]
    pub fn dfe0(&self) -> DFE0R {
        DFE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Digital Filter Enable"]
    #[inline]
    pub fn dfe1(&self) -> DFE1R {
        DFE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Digital Filter Enable"]
    #[inline]
    pub fn dfe2(&self) -> DFE2R {
        DFE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Digital Filter Enable"]
    #[inline]
    pub fn dfe3(&self) -> DFE3R {
        DFE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Digital Filter Enable"]
    #[inline]
    pub fn dfe4(&self) -> DFE4R {
        DFE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline]
    pub fn dfe5(&self) -> DFE5R {
        DFE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Digital Filter Enable"]
    #[inline]
    pub fn dfe6(&self) -> DFE6R {
        DFE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Digital Filter Enable"]
    #[inline]
    pub fn dfe7(&self) -> DFE7R {
        DFE7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Digital Filter Enable"]
    #[inline]
    pub fn dfe8(&self) -> DFE8R {
        DFE8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Digital Filter Enable"]
    #[inline]
    pub fn dfe9(&self) -> DFE9R {
        DFE9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Digital Filter Enable"]
    #[inline]
    pub fn dfe10(&self) -> DFE10R {
        DFE10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Digital Filter Enable"]
    #[inline]
    pub fn dfe11(&self) -> DFE11R {
        DFE11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Digital Filter Enable"]
    #[inline]
    pub fn dfe12(&self) -> DFE12R {
        DFE12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Digital Filter Enable"]
    #[inline]
    pub fn dfe13(&self) -> DFE13R {
        DFE13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Digital Filter Enable"]
    #[inline]
    pub fn dfe14(&self) -> DFE14R {
        DFE14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Digital Filter Enable"]
    #[inline]
    pub fn dfe15(&self) -> DFE15R {
        DFE15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Digital Filter Enable"]
    #[inline]
    pub fn dfe16(&self) -> DFE16R {
        DFE16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Digital Filter Enable"]
    #[inline]
    pub fn dfe17(&self) -> DFE17R {
        DFE17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Digital Filter Enable"]
    #[inline]
    pub fn dfe18(&self) -> DFE18R {
        DFE18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Digital Filter Enable"]
    #[inline]
    pub fn dfe19(&self) -> DFE19R {
        DFE19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Digital Filter Enable"]
    #[inline]
    pub fn dfe20(&self) -> DFE20R {
        DFE20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Digital Filter Enable"]
    #[inline]
    pub fn dfe21(&self) -> DFE21R {
        DFE21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Digital Filter Enable"]
    #[inline]
    pub fn dfe22(&self) -> DFE22R {
        DFE22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Digital Filter Enable"]
    #[inline]
    pub fn dfe23(&self) -> DFE23R {
        DFE23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Digital Filter Enable"]
    #[inline]
    pub fn dfe24(&self) -> DFE24R {
        DFE24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Digital Filter Enable"]
    #[inline]
    pub fn dfe25(&self) -> DFE25R {
        DFE25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Digital Filter Enable"]
    #[inline]
    pub fn dfe26(&self) -> DFE26R {
        DFE26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Digital Filter Enable"]
    #[inline]
    pub fn dfe27(&self) -> DFE27R {
        DFE27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Digital Filter Enable"]
    #[inline]
    pub fn dfe28(&self) -> DFE28R {
        DFE28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Digital Filter Enable"]
    #[inline]
    pub fn dfe29(&self) -> DFE29R {
        DFE29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Digital Filter Enable"]
    #[inline]
    pub fn dfe30(&self) -> DFE30R {
        DFE30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Digital Filter Enable"]
    #[inline]
    pub fn dfe31(&self) -> DFE31R {
        DFE31R::_from({
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
    #[doc = "Bit 0 - Digital Filter Enable"]
    #[inline]
    pub fn dfe0(&mut self) -> _DFE0W {
        _DFE0W { w: self }
    }
    #[doc = "Bit 1 - Digital Filter Enable"]
    #[inline]
    pub fn dfe1(&mut self) -> _DFE1W {
        _DFE1W { w: self }
    }
    #[doc = "Bit 2 - Digital Filter Enable"]
    #[inline]
    pub fn dfe2(&mut self) -> _DFE2W {
        _DFE2W { w: self }
    }
    #[doc = "Bit 3 - Digital Filter Enable"]
    #[inline]
    pub fn dfe3(&mut self) -> _DFE3W {
        _DFE3W { w: self }
    }
    #[doc = "Bit 4 - Digital Filter Enable"]
    #[inline]
    pub fn dfe4(&mut self) -> _DFE4W {
        _DFE4W { w: self }
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline]
    pub fn dfe5(&mut self) -> _DFE5W {
        _DFE5W { w: self }
    }
    #[doc = "Bit 6 - Digital Filter Enable"]
    #[inline]
    pub fn dfe6(&mut self) -> _DFE6W {
        _DFE6W { w: self }
    }
    #[doc = "Bit 7 - Digital Filter Enable"]
    #[inline]
    pub fn dfe7(&mut self) -> _DFE7W {
        _DFE7W { w: self }
    }
    #[doc = "Bit 8 - Digital Filter Enable"]
    #[inline]
    pub fn dfe8(&mut self) -> _DFE8W {
        _DFE8W { w: self }
    }
    #[doc = "Bit 9 - Digital Filter Enable"]
    #[inline]
    pub fn dfe9(&mut self) -> _DFE9W {
        _DFE9W { w: self }
    }
    #[doc = "Bit 10 - Digital Filter Enable"]
    #[inline]
    pub fn dfe10(&mut self) -> _DFE10W {
        _DFE10W { w: self }
    }
    #[doc = "Bit 11 - Digital Filter Enable"]
    #[inline]
    pub fn dfe11(&mut self) -> _DFE11W {
        _DFE11W { w: self }
    }
    #[doc = "Bit 12 - Digital Filter Enable"]
    #[inline]
    pub fn dfe12(&mut self) -> _DFE12W {
        _DFE12W { w: self }
    }
    #[doc = "Bit 13 - Digital Filter Enable"]
    #[inline]
    pub fn dfe13(&mut self) -> _DFE13W {
        _DFE13W { w: self }
    }
    #[doc = "Bit 14 - Digital Filter Enable"]
    #[inline]
    pub fn dfe14(&mut self) -> _DFE14W {
        _DFE14W { w: self }
    }
    #[doc = "Bit 15 - Digital Filter Enable"]
    #[inline]
    pub fn dfe15(&mut self) -> _DFE15W {
        _DFE15W { w: self }
    }
    #[doc = "Bit 16 - Digital Filter Enable"]
    #[inline]
    pub fn dfe16(&mut self) -> _DFE16W {
        _DFE16W { w: self }
    }
    #[doc = "Bit 17 - Digital Filter Enable"]
    #[inline]
    pub fn dfe17(&mut self) -> _DFE17W {
        _DFE17W { w: self }
    }
    #[doc = "Bit 18 - Digital Filter Enable"]
    #[inline]
    pub fn dfe18(&mut self) -> _DFE18W {
        _DFE18W { w: self }
    }
    #[doc = "Bit 19 - Digital Filter Enable"]
    #[inline]
    pub fn dfe19(&mut self) -> _DFE19W {
        _DFE19W { w: self }
    }
    #[doc = "Bit 20 - Digital Filter Enable"]
    #[inline]
    pub fn dfe20(&mut self) -> _DFE20W {
        _DFE20W { w: self }
    }
    #[doc = "Bit 21 - Digital Filter Enable"]
    #[inline]
    pub fn dfe21(&mut self) -> _DFE21W {
        _DFE21W { w: self }
    }
    #[doc = "Bit 22 - Digital Filter Enable"]
    #[inline]
    pub fn dfe22(&mut self) -> _DFE22W {
        _DFE22W { w: self }
    }
    #[doc = "Bit 23 - Digital Filter Enable"]
    #[inline]
    pub fn dfe23(&mut self) -> _DFE23W {
        _DFE23W { w: self }
    }
    #[doc = "Bit 24 - Digital Filter Enable"]
    #[inline]
    pub fn dfe24(&mut self) -> _DFE24W {
        _DFE24W { w: self }
    }
    #[doc = "Bit 25 - Digital Filter Enable"]
    #[inline]
    pub fn dfe25(&mut self) -> _DFE25W {
        _DFE25W { w: self }
    }
    #[doc = "Bit 26 - Digital Filter Enable"]
    #[inline]
    pub fn dfe26(&mut self) -> _DFE26W {
        _DFE26W { w: self }
    }
    #[doc = "Bit 27 - Digital Filter Enable"]
    #[inline]
    pub fn dfe27(&mut self) -> _DFE27W {
        _DFE27W { w: self }
    }
    #[doc = "Bit 28 - Digital Filter Enable"]
    #[inline]
    pub fn dfe28(&mut self) -> _DFE28W {
        _DFE28W { w: self }
    }
    #[doc = "Bit 29 - Digital Filter Enable"]
    #[inline]
    pub fn dfe29(&mut self) -> _DFE29W {
        _DFE29W { w: self }
    }
    #[doc = "Bit 30 - Digital Filter Enable"]
    #[inline]
    pub fn dfe30(&mut self) -> _DFE30W {
        _DFE30W { w: self }
    }
    #[doc = "Bit 31 - Digital Filter Enable"]
    #[inline]
    pub fn dfe31(&mut self) -> _DFE31W {
        _DFE31W { w: self }
    }
}
