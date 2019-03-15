#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RMR {
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
#[doc = "Possible values of the field `RWM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM0R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM0R {
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
            RWM0R::_0 => false,
            RWM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM0R {
        match value {
            false => RWM0R::_0,
            true => RWM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM0R::_1
    }
}
#[doc = "Possible values of the field `RWM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM1R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM1R {
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
            RWM1R::_0 => false,
            RWM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM1R {
        match value {
            false => RWM1R::_0,
            true => RWM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM1R::_1
    }
}
#[doc = "Possible values of the field `RWM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM2R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM2R {
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
            RWM2R::_0 => false,
            RWM2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM2R {
        match value {
            false => RWM2R::_0,
            true => RWM2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM2R::_1
    }
}
#[doc = "Possible values of the field `RWM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM3R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM3R {
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
            RWM3R::_0 => false,
            RWM3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM3R {
        match value {
            false => RWM3R::_0,
            true => RWM3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM3R::_1
    }
}
#[doc = "Possible values of the field `RWM4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM4R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM4R {
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
            RWM4R::_0 => false,
            RWM4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM4R {
        match value {
            false => RWM4R::_0,
            true => RWM4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM4R::_1
    }
}
#[doc = "Possible values of the field `RWM5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM5R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM5R {
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
            RWM5R::_0 => false,
            RWM5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM5R {
        match value {
            false => RWM5R::_0,
            true => RWM5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM5R::_1
    }
}
#[doc = "Possible values of the field `RWM6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM6R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM6R {
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
            RWM6R::_0 => false,
            RWM6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM6R {
        match value {
            false => RWM6R::_0,
            true => RWM6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM6R::_1
    }
}
#[doc = "Possible values of the field `RWM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM7R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM7R {
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
            RWM7R::_0 => false,
            RWM7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM7R {
        match value {
            false => RWM7R::_0,
            true => RWM7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM7R::_1
    }
}
#[doc = "Possible values of the field `RWM8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM8R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM8R {
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
            RWM8R::_0 => false,
            RWM8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM8R {
        match value {
            false => RWM8R::_0,
            true => RWM8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM8R::_1
    }
}
#[doc = "Possible values of the field `RWM9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM9R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM9R {
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
            RWM9R::_0 => false,
            RWM9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM9R {
        match value {
            false => RWM9R::_0,
            true => RWM9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM9R::_1
    }
}
#[doc = "Possible values of the field `RWM10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM10R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM10R {
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
            RWM10R::_0 => false,
            RWM10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM10R {
        match value {
            false => RWM10R::_0,
            true => RWM10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM10R::_1
    }
}
#[doc = "Possible values of the field `RWM11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM11R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM11R {
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
            RWM11R::_0 => false,
            RWM11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM11R {
        match value {
            false => RWM11R::_0,
            true => RWM11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM11R::_1
    }
}
#[doc = "Possible values of the field `RWM12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM12R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM12R {
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
            RWM12R::_0 => false,
            RWM12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM12R {
        match value {
            false => RWM12R::_0,
            true => RWM12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM12R::_1
    }
}
#[doc = "Possible values of the field `RWM13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM13R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM13R {
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
            RWM13R::_0 => false,
            RWM13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM13R {
        match value {
            false => RWM13R::_0,
            true => RWM13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM13R::_1
    }
}
#[doc = "Possible values of the field `RWM14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM14R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM14R {
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
            RWM14R::_0 => false,
            RWM14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM14R {
        match value {
            false => RWM14R::_0,
            true => RWM14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM14R::_1
    }
}
#[doc = "Possible values of the field `RWM15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM15R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM15R {
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
            RWM15R::_0 => false,
            RWM15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM15R {
        match value {
            false => RWM15R::_0,
            true => RWM15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM15R::_1
    }
}
#[doc = "Possible values of the field `RWM16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM16R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM16R {
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
            RWM16R::_0 => false,
            RWM16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM16R {
        match value {
            false => RWM16R::_0,
            true => RWM16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM16R::_1
    }
}
#[doc = "Possible values of the field `RWM17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM17R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM17R {
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
            RWM17R::_0 => false,
            RWM17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM17R {
        match value {
            false => RWM17R::_0,
            true => RWM17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM17R::_1
    }
}
#[doc = "Possible values of the field `RWM18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM18R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM18R {
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
            RWM18R::_0 => false,
            RWM18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM18R {
        match value {
            false => RWM18R::_0,
            true => RWM18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM18R::_1
    }
}
#[doc = "Possible values of the field `RWM19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM19R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM19R {
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
            RWM19R::_0 => false,
            RWM19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM19R {
        match value {
            false => RWM19R::_0,
            true => RWM19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM19R::_1
    }
}
#[doc = "Possible values of the field `RWM20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM20R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM20R {
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
            RWM20R::_0 => false,
            RWM20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM20R {
        match value {
            false => RWM20R::_0,
            true => RWM20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM20R::_1
    }
}
#[doc = "Possible values of the field `RWM21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM21R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM21R {
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
            RWM21R::_0 => false,
            RWM21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM21R {
        match value {
            false => RWM21R::_0,
            true => RWM21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM21R::_1
    }
}
#[doc = "Possible values of the field `RWM22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM22R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM22R {
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
            RWM22R::_0 => false,
            RWM22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM22R {
        match value {
            false => RWM22R::_0,
            true => RWM22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM22R::_1
    }
}
#[doc = "Possible values of the field `RWM23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM23R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM23R {
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
            RWM23R::_0 => false,
            RWM23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM23R {
        match value {
            false => RWM23R::_0,
            true => RWM23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM23R::_1
    }
}
#[doc = "Possible values of the field `RWM24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM24R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM24R {
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
            RWM24R::_0 => false,
            RWM24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM24R {
        match value {
            false => RWM24R::_0,
            true => RWM24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM24R::_1
    }
}
#[doc = "Possible values of the field `RWM25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM25R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM25R {
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
            RWM25R::_0 => false,
            RWM25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM25R {
        match value {
            false => RWM25R::_0,
            true => RWM25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM25R::_1
    }
}
#[doc = "Possible values of the field `RWM26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM26R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM26R {
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
            RWM26R::_0 => false,
            RWM26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM26R {
        match value {
            false => RWM26R::_0,
            true => RWM26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM26R::_1
    }
}
#[doc = "Possible values of the field `RWM27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM27R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM27R {
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
            RWM27R::_0 => false,
            RWM27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM27R {
        match value {
            false => RWM27R::_0,
            true => RWM27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM27R::_1
    }
}
#[doc = "Possible values of the field `RWM28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM28R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM28R {
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
            RWM28R::_0 => false,
            RWM28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM28R {
        match value {
            false => RWM28R::_0,
            true => RWM28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM28R::_1
    }
}
#[doc = "Possible values of the field `RWM29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM29R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM29R {
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
            RWM29R::_0 => false,
            RWM29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM29R {
        match value {
            false => RWM29R::_0,
            true => RWM29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM29R::_1
    }
}
#[doc = "Possible values of the field `RWM30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM30R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM30R {
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
            RWM30R::_0 => false,
            RWM30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM30R {
        match value {
            false => RWM30R::_0,
            true => RWM30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM30R::_1
    }
}
#[doc = "Possible values of the field `RWM31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWM31R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM31R {
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
            RWM31R::_0 => false,
            RWM31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWM31R {
        match value {
            false => RWM31R::_0,
            true => RWM31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWM31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWM31R::_1
    }
}
#[doc = "Values that can be written to the field `RWM0`"]
pub enum RWM0W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM0W::_0 => false,
            RWM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM0W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM0W::_1)
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
#[doc = "Values that can be written to the field `RWM1`"]
pub enum RWM1W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM1W::_0 => false,
            RWM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM1W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM1W::_1)
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
#[doc = "Values that can be written to the field `RWM2`"]
pub enum RWM2W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM2W::_0 => false,
            RWM2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM2W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM2W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM2W::_1)
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
#[doc = "Values that can be written to the field `RWM3`"]
pub enum RWM3W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM3W::_0 => false,
            RWM3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM3W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM3W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM3W::_1)
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
#[doc = "Values that can be written to the field `RWM4`"]
pub enum RWM4W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM4W::_0 => false,
            RWM4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM4W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM4W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM4W::_1)
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
#[doc = "Values that can be written to the field `RWM5`"]
pub enum RWM5W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM5W::_0 => false,
            RWM5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM5W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM5W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM5W::_1)
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
#[doc = "Values that can be written to the field `RWM6`"]
pub enum RWM6W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM6W::_0 => false,
            RWM6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM6W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM6W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM6W::_1)
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
#[doc = "Values that can be written to the field `RWM7`"]
pub enum RWM7W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM7W::_0 => false,
            RWM7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM7W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM7W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM7W::_1)
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
#[doc = "Values that can be written to the field `RWM8`"]
pub enum RWM8W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM8W::_0 => false,
            RWM8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM8W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM8W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM8W::_1)
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
#[doc = "Values that can be written to the field `RWM9`"]
pub enum RWM9W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM9W::_0 => false,
            RWM9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM9W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM9W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM9W::_1)
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
#[doc = "Values that can be written to the field `RWM10`"]
pub enum RWM10W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM10W::_0 => false,
            RWM10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM10W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM10W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM10W::_1)
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
#[doc = "Values that can be written to the field `RWM11`"]
pub enum RWM11W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM11W::_0 => false,
            RWM11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM11W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM11W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM11W::_1)
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
#[doc = "Values that can be written to the field `RWM12`"]
pub enum RWM12W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM12W::_0 => false,
            RWM12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM12W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM12W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM12W::_1)
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
#[doc = "Values that can be written to the field `RWM13`"]
pub enum RWM13W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM13W::_0 => false,
            RWM13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM13W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM13W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM13W::_1)
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
#[doc = "Values that can be written to the field `RWM14`"]
pub enum RWM14W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM14W::_0 => false,
            RWM14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM14W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM14W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM14W::_1)
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
#[doc = "Values that can be written to the field `RWM15`"]
pub enum RWM15W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM15W::_0 => false,
            RWM15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM15W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM15W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM15W::_1)
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
#[doc = "Values that can be written to the field `RWM16`"]
pub enum RWM16W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM16W::_0 => false,
            RWM16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM16W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM16W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM16W::_1)
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
#[doc = "Values that can be written to the field `RWM17`"]
pub enum RWM17W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM17W::_0 => false,
            RWM17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM17W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM17W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM17W::_1)
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
#[doc = "Values that can be written to the field `RWM18`"]
pub enum RWM18W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM18W::_0 => false,
            RWM18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM18W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM18W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM18W::_1)
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
#[doc = "Values that can be written to the field `RWM19`"]
pub enum RWM19W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM19W::_0 => false,
            RWM19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM19W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM19W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM19W::_1)
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
#[doc = "Values that can be written to the field `RWM20`"]
pub enum RWM20W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM20W::_0 => false,
            RWM20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM20W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM20W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM20W::_1)
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
#[doc = "Values that can be written to the field `RWM21`"]
pub enum RWM21W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM21W::_0 => false,
            RWM21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM21W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM21W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM21W::_1)
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
#[doc = "Values that can be written to the field `RWM22`"]
pub enum RWM22W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM22W::_0 => false,
            RWM22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM22W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM22W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM22W::_1)
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
#[doc = "Values that can be written to the field `RWM23`"]
pub enum RWM23W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM23W::_0 => false,
            RWM23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM23W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM23W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM23W::_1)
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
#[doc = "Values that can be written to the field `RWM24`"]
pub enum RWM24W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM24W::_0 => false,
            RWM24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM24W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM24W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM24W::_1)
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
#[doc = "Values that can be written to the field `RWM25`"]
pub enum RWM25W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM25W::_0 => false,
            RWM25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM25W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM25W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM25W::_1)
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
#[doc = "Values that can be written to the field `RWM26`"]
pub enum RWM26W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM26W::_0 => false,
            RWM26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM26W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM26W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM26W::_1)
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
#[doc = "Values that can be written to the field `RWM27`"]
pub enum RWM27W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM27W::_0 => false,
            RWM27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM27W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM27W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM27W::_1)
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
#[doc = "Values that can be written to the field `RWM28`"]
pub enum RWM28W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM28W::_0 => false,
            RWM28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM28W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM28W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM28W::_1)
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
#[doc = "Values that can be written to the field `RWM29`"]
pub enum RWM29W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM29W::_0 => false,
            RWM29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM29W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM29W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM29W::_1)
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
#[doc = "Values that can be written to the field `RWM30`"]
pub enum RWM30W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM30W::_0 => false,
            RWM30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM30W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM30W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM30W::_1)
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
#[doc = "Values that can be written to the field `RWM31`"]
pub enum RWM31W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked."]
    _1,
}
impl RWM31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWM31W::_0 => false,
            RWM31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWM31W<'a> {
    w: &'a mut W,
}
impl<'a> _RWM31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWM31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM31W::_0)
    }
    #[doc = "Word N is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM31W::_1)
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
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline]
    pub fn rwm0(&self) -> RWM0R {
        RWM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline]
    pub fn rwm1(&self) -> RWM1R {
        RWM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receive Word Mask"]
    #[inline]
    pub fn rwm2(&self) -> RWM2R {
        RWM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receive Word Mask"]
    #[inline]
    pub fn rwm3(&self) -> RWM3R {
        RWM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Word Mask"]
    #[inline]
    pub fn rwm4(&self) -> RWM4R {
        RWM4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receive Word Mask"]
    #[inline]
    pub fn rwm5(&self) -> RWM5R {
        RWM5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receive Word Mask"]
    #[inline]
    pub fn rwm6(&self) -> RWM6R {
        RWM6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Receive Word Mask"]
    #[inline]
    pub fn rwm7(&self) -> RWM7R {
        RWM7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Receive Word Mask"]
    #[inline]
    pub fn rwm8(&self) -> RWM8R {
        RWM8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receive Word Mask"]
    #[inline]
    pub fn rwm9(&self) -> RWM9R {
        RWM9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Receive Word Mask"]
    #[inline]
    pub fn rwm10(&self) -> RWM10R {
        RWM10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Receive Word Mask"]
    #[inline]
    pub fn rwm11(&self) -> RWM11R {
        RWM11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Word Mask"]
    #[inline]
    pub fn rwm12(&self) -> RWM12R {
        RWM12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Receive Word Mask"]
    #[inline]
    pub fn rwm13(&self) -> RWM13R {
        RWM13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive Word Mask"]
    #[inline]
    pub fn rwm14(&self) -> RWM14R {
        RWM14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Receive Word Mask"]
    #[inline]
    pub fn rwm15(&self) -> RWM15R {
        RWM15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Receive Word Mask"]
    #[inline]
    pub fn rwm16(&self) -> RWM16R {
        RWM16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Receive Word Mask"]
    #[inline]
    pub fn rwm17(&self) -> RWM17R {
        RWM17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Receive Word Mask"]
    #[inline]
    pub fn rwm18(&self) -> RWM18R {
        RWM18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receive Word Mask"]
    #[inline]
    pub fn rwm19(&self) -> RWM19R {
        RWM19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Receive Word Mask"]
    #[inline]
    pub fn rwm20(&self) -> RWM20R {
        RWM20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receive Word Mask"]
    #[inline]
    pub fn rwm21(&self) -> RWM21R {
        RWM21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive Word Mask"]
    #[inline]
    pub fn rwm22(&self) -> RWM22R {
        RWM22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Receive Word Mask"]
    #[inline]
    pub fn rwm23(&self) -> RWM23R {
        RWM23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Receive Word Mask"]
    #[inline]
    pub fn rwm24(&self) -> RWM24R {
        RWM24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Receive Word Mask"]
    #[inline]
    pub fn rwm25(&self) -> RWM25R {
        RWM25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Receive Word Mask"]
    #[inline]
    pub fn rwm26(&self) -> RWM26R {
        RWM26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Receive Word Mask"]
    #[inline]
    pub fn rwm27(&self) -> RWM27R {
        RWM27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Receive Word Mask"]
    #[inline]
    pub fn rwm28(&self) -> RWM28R {
        RWM28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Receive Word Mask"]
    #[inline]
    pub fn rwm29(&self) -> RWM29R {
        RWM29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Receive Word Mask"]
    #[inline]
    pub fn rwm30(&self) -> RWM30R {
        RWM30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Receive Word Mask"]
    #[inline]
    pub fn rwm31(&self) -> RWM31R {
        RWM31R::_from({
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
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline]
    pub fn rwm0(&mut self) -> _RWM0W {
        _RWM0W { w: self }
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline]
    pub fn rwm1(&mut self) -> _RWM1W {
        _RWM1W { w: self }
    }
    #[doc = "Bit 2 - Receive Word Mask"]
    #[inline]
    pub fn rwm2(&mut self) -> _RWM2W {
        _RWM2W { w: self }
    }
    #[doc = "Bit 3 - Receive Word Mask"]
    #[inline]
    pub fn rwm3(&mut self) -> _RWM3W {
        _RWM3W { w: self }
    }
    #[doc = "Bit 4 - Receive Word Mask"]
    #[inline]
    pub fn rwm4(&mut self) -> _RWM4W {
        _RWM4W { w: self }
    }
    #[doc = "Bit 5 - Receive Word Mask"]
    #[inline]
    pub fn rwm5(&mut self) -> _RWM5W {
        _RWM5W { w: self }
    }
    #[doc = "Bit 6 - Receive Word Mask"]
    #[inline]
    pub fn rwm6(&mut self) -> _RWM6W {
        _RWM6W { w: self }
    }
    #[doc = "Bit 7 - Receive Word Mask"]
    #[inline]
    pub fn rwm7(&mut self) -> _RWM7W {
        _RWM7W { w: self }
    }
    #[doc = "Bit 8 - Receive Word Mask"]
    #[inline]
    pub fn rwm8(&mut self) -> _RWM8W {
        _RWM8W { w: self }
    }
    #[doc = "Bit 9 - Receive Word Mask"]
    #[inline]
    pub fn rwm9(&mut self) -> _RWM9W {
        _RWM9W { w: self }
    }
    #[doc = "Bit 10 - Receive Word Mask"]
    #[inline]
    pub fn rwm10(&mut self) -> _RWM10W {
        _RWM10W { w: self }
    }
    #[doc = "Bit 11 - Receive Word Mask"]
    #[inline]
    pub fn rwm11(&mut self) -> _RWM11W {
        _RWM11W { w: self }
    }
    #[doc = "Bit 12 - Receive Word Mask"]
    #[inline]
    pub fn rwm12(&mut self) -> _RWM12W {
        _RWM12W { w: self }
    }
    #[doc = "Bit 13 - Receive Word Mask"]
    #[inline]
    pub fn rwm13(&mut self) -> _RWM13W {
        _RWM13W { w: self }
    }
    #[doc = "Bit 14 - Receive Word Mask"]
    #[inline]
    pub fn rwm14(&mut self) -> _RWM14W {
        _RWM14W { w: self }
    }
    #[doc = "Bit 15 - Receive Word Mask"]
    #[inline]
    pub fn rwm15(&mut self) -> _RWM15W {
        _RWM15W { w: self }
    }
    #[doc = "Bit 16 - Receive Word Mask"]
    #[inline]
    pub fn rwm16(&mut self) -> _RWM16W {
        _RWM16W { w: self }
    }
    #[doc = "Bit 17 - Receive Word Mask"]
    #[inline]
    pub fn rwm17(&mut self) -> _RWM17W {
        _RWM17W { w: self }
    }
    #[doc = "Bit 18 - Receive Word Mask"]
    #[inline]
    pub fn rwm18(&mut self) -> _RWM18W {
        _RWM18W { w: self }
    }
    #[doc = "Bit 19 - Receive Word Mask"]
    #[inline]
    pub fn rwm19(&mut self) -> _RWM19W {
        _RWM19W { w: self }
    }
    #[doc = "Bit 20 - Receive Word Mask"]
    #[inline]
    pub fn rwm20(&mut self) -> _RWM20W {
        _RWM20W { w: self }
    }
    #[doc = "Bit 21 - Receive Word Mask"]
    #[inline]
    pub fn rwm21(&mut self) -> _RWM21W {
        _RWM21W { w: self }
    }
    #[doc = "Bit 22 - Receive Word Mask"]
    #[inline]
    pub fn rwm22(&mut self) -> _RWM22W {
        _RWM22W { w: self }
    }
    #[doc = "Bit 23 - Receive Word Mask"]
    #[inline]
    pub fn rwm23(&mut self) -> _RWM23W {
        _RWM23W { w: self }
    }
    #[doc = "Bit 24 - Receive Word Mask"]
    #[inline]
    pub fn rwm24(&mut self) -> _RWM24W {
        _RWM24W { w: self }
    }
    #[doc = "Bit 25 - Receive Word Mask"]
    #[inline]
    pub fn rwm25(&mut self) -> _RWM25W {
        _RWM25W { w: self }
    }
    #[doc = "Bit 26 - Receive Word Mask"]
    #[inline]
    pub fn rwm26(&mut self) -> _RWM26W {
        _RWM26W { w: self }
    }
    #[doc = "Bit 27 - Receive Word Mask"]
    #[inline]
    pub fn rwm27(&mut self) -> _RWM27W {
        _RWM27W { w: self }
    }
    #[doc = "Bit 28 - Receive Word Mask"]
    #[inline]
    pub fn rwm28(&mut self) -> _RWM28W {
        _RWM28W { w: self }
    }
    #[doc = "Bit 29 - Receive Word Mask"]
    #[inline]
    pub fn rwm29(&mut self) -> _RWM29W {
        _RWM29W { w: self }
    }
    #[doc = "Bit 30 - Receive Word Mask"]
    #[inline]
    pub fn rwm30(&mut self) -> _RWM30W {
        _RWM30W { w: self }
    }
    #[doc = "Bit 31 - Receive Word Mask"]
    #[inline]
    pub fn rwm31(&mut self) -> _RWM31W {
        _RWM31W { w: self }
    }
}
