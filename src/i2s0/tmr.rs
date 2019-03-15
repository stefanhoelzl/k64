#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TMR {
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
#[doc = "Possible values of the field `TWM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM0R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM0R {
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
            TWM0R::_0 => false,
            TWM0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM0R {
        match value {
            false => TWM0R::_0,
            true => TWM0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM0R::_1
    }
}
#[doc = "Possible values of the field `TWM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM1R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM1R {
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
            TWM1R::_0 => false,
            TWM1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM1R {
        match value {
            false => TWM1R::_0,
            true => TWM1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM1R::_1
    }
}
#[doc = "Possible values of the field `TWM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM2R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM2R {
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
            TWM2R::_0 => false,
            TWM2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM2R {
        match value {
            false => TWM2R::_0,
            true => TWM2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM2R::_1
    }
}
#[doc = "Possible values of the field `TWM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM3R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM3R {
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
            TWM3R::_0 => false,
            TWM3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM3R {
        match value {
            false => TWM3R::_0,
            true => TWM3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM3R::_1
    }
}
#[doc = "Possible values of the field `TWM4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM4R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM4R {
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
            TWM4R::_0 => false,
            TWM4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM4R {
        match value {
            false => TWM4R::_0,
            true => TWM4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM4R::_1
    }
}
#[doc = "Possible values of the field `TWM5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM5R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM5R {
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
            TWM5R::_0 => false,
            TWM5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM5R {
        match value {
            false => TWM5R::_0,
            true => TWM5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM5R::_1
    }
}
#[doc = "Possible values of the field `TWM6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM6R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM6R {
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
            TWM6R::_0 => false,
            TWM6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM6R {
        match value {
            false => TWM6R::_0,
            true => TWM6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM6R::_1
    }
}
#[doc = "Possible values of the field `TWM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM7R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM7R {
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
            TWM7R::_0 => false,
            TWM7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM7R {
        match value {
            false => TWM7R::_0,
            true => TWM7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM7R::_1
    }
}
#[doc = "Possible values of the field `TWM8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM8R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM8R {
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
            TWM8R::_0 => false,
            TWM8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM8R {
        match value {
            false => TWM8R::_0,
            true => TWM8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM8R::_1
    }
}
#[doc = "Possible values of the field `TWM9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM9R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM9R {
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
            TWM9R::_0 => false,
            TWM9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM9R {
        match value {
            false => TWM9R::_0,
            true => TWM9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM9R::_1
    }
}
#[doc = "Possible values of the field `TWM10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM10R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM10R {
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
            TWM10R::_0 => false,
            TWM10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM10R {
        match value {
            false => TWM10R::_0,
            true => TWM10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM10R::_1
    }
}
#[doc = "Possible values of the field `TWM11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM11R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM11R {
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
            TWM11R::_0 => false,
            TWM11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM11R {
        match value {
            false => TWM11R::_0,
            true => TWM11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM11R::_1
    }
}
#[doc = "Possible values of the field `TWM12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM12R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM12R {
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
            TWM12R::_0 => false,
            TWM12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM12R {
        match value {
            false => TWM12R::_0,
            true => TWM12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM12R::_1
    }
}
#[doc = "Possible values of the field `TWM13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM13R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM13R {
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
            TWM13R::_0 => false,
            TWM13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM13R {
        match value {
            false => TWM13R::_0,
            true => TWM13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM13R::_1
    }
}
#[doc = "Possible values of the field `TWM14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM14R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM14R {
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
            TWM14R::_0 => false,
            TWM14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM14R {
        match value {
            false => TWM14R::_0,
            true => TWM14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM14R::_1
    }
}
#[doc = "Possible values of the field `TWM15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM15R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM15R {
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
            TWM15R::_0 => false,
            TWM15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM15R {
        match value {
            false => TWM15R::_0,
            true => TWM15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM15R::_1
    }
}
#[doc = "Possible values of the field `TWM16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM16R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM16R {
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
            TWM16R::_0 => false,
            TWM16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM16R {
        match value {
            false => TWM16R::_0,
            true => TWM16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM16R::_1
    }
}
#[doc = "Possible values of the field `TWM17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM17R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM17R {
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
            TWM17R::_0 => false,
            TWM17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM17R {
        match value {
            false => TWM17R::_0,
            true => TWM17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM17R::_1
    }
}
#[doc = "Possible values of the field `TWM18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM18R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM18R {
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
            TWM18R::_0 => false,
            TWM18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM18R {
        match value {
            false => TWM18R::_0,
            true => TWM18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM18R::_1
    }
}
#[doc = "Possible values of the field `TWM19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM19R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM19R {
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
            TWM19R::_0 => false,
            TWM19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM19R {
        match value {
            false => TWM19R::_0,
            true => TWM19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM19R::_1
    }
}
#[doc = "Possible values of the field `TWM20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM20R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM20R {
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
            TWM20R::_0 => false,
            TWM20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM20R {
        match value {
            false => TWM20R::_0,
            true => TWM20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM20R::_1
    }
}
#[doc = "Possible values of the field `TWM21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM21R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM21R {
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
            TWM21R::_0 => false,
            TWM21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM21R {
        match value {
            false => TWM21R::_0,
            true => TWM21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM21R::_1
    }
}
#[doc = "Possible values of the field `TWM22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM22R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM22R {
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
            TWM22R::_0 => false,
            TWM22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM22R {
        match value {
            false => TWM22R::_0,
            true => TWM22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM22R::_1
    }
}
#[doc = "Possible values of the field `TWM23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM23R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM23R {
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
            TWM23R::_0 => false,
            TWM23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM23R {
        match value {
            false => TWM23R::_0,
            true => TWM23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM23R::_1
    }
}
#[doc = "Possible values of the field `TWM24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM24R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM24R {
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
            TWM24R::_0 => false,
            TWM24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM24R {
        match value {
            false => TWM24R::_0,
            true => TWM24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM24R::_1
    }
}
#[doc = "Possible values of the field `TWM25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM25R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM25R {
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
            TWM25R::_0 => false,
            TWM25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM25R {
        match value {
            false => TWM25R::_0,
            true => TWM25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM25R::_1
    }
}
#[doc = "Possible values of the field `TWM26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM26R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM26R {
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
            TWM26R::_0 => false,
            TWM26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM26R {
        match value {
            false => TWM26R::_0,
            true => TWM26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM26R::_1
    }
}
#[doc = "Possible values of the field `TWM27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM27R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM27R {
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
            TWM27R::_0 => false,
            TWM27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM27R {
        match value {
            false => TWM27R::_0,
            true => TWM27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM27R::_1
    }
}
#[doc = "Possible values of the field `TWM28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM28R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM28R {
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
            TWM28R::_0 => false,
            TWM28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM28R {
        match value {
            false => TWM28R::_0,
            true => TWM28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM28R::_1
    }
}
#[doc = "Possible values of the field `TWM29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM29R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM29R {
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
            TWM29R::_0 => false,
            TWM29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM29R {
        match value {
            false => TWM29R::_0,
            true => TWM29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM29R::_1
    }
}
#[doc = "Possible values of the field `TWM30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM30R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM30R {
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
            TWM30R::_0 => false,
            TWM30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM30R {
        match value {
            false => TWM30R::_0,
            true => TWM30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM30R::_1
    }
}
#[doc = "Possible values of the field `TWM31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWM31R {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM31R {
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
            TWM31R::_0 => false,
            TWM31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWM31R {
        match value {
            false => TWM31R::_0,
            true => TWM31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWM31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWM31R::_1
    }
}
#[doc = "Values that can be written to the field `TWM0`"]
pub enum TWM0W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM0W::_0 => false,
            TWM0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM0W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM0W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM0W::_1)
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
#[doc = "Values that can be written to the field `TWM1`"]
pub enum TWM1W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM1W::_0 => false,
            TWM1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM1W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM1W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM1W::_1)
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
#[doc = "Values that can be written to the field `TWM2`"]
pub enum TWM2W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM2W::_0 => false,
            TWM2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM2W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM2W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM2W::_1)
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
#[doc = "Values that can be written to the field `TWM3`"]
pub enum TWM3W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM3W::_0 => false,
            TWM3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM3W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM3W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM3W::_1)
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
#[doc = "Values that can be written to the field `TWM4`"]
pub enum TWM4W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM4W::_0 => false,
            TWM4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM4W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM4W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM4W::_1)
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
#[doc = "Values that can be written to the field `TWM5`"]
pub enum TWM5W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM5W::_0 => false,
            TWM5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM5W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM5W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM5W::_1)
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
#[doc = "Values that can be written to the field `TWM6`"]
pub enum TWM6W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM6W::_0 => false,
            TWM6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM6W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM6W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM6W::_1)
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
#[doc = "Values that can be written to the field `TWM7`"]
pub enum TWM7W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM7W::_0 => false,
            TWM7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM7W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM7W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM7W::_1)
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
#[doc = "Values that can be written to the field `TWM8`"]
pub enum TWM8W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM8W::_0 => false,
            TWM8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM8W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM8W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM8W::_1)
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
#[doc = "Values that can be written to the field `TWM9`"]
pub enum TWM9W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM9W::_0 => false,
            TWM9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM9W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM9W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM9W::_1)
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
#[doc = "Values that can be written to the field `TWM10`"]
pub enum TWM10W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM10W::_0 => false,
            TWM10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM10W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM10W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM10W::_1)
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
#[doc = "Values that can be written to the field `TWM11`"]
pub enum TWM11W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM11W::_0 => false,
            TWM11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM11W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM11W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM11W::_1)
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
#[doc = "Values that can be written to the field `TWM12`"]
pub enum TWM12W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM12W::_0 => false,
            TWM12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM12W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM12W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM12W::_1)
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
#[doc = "Values that can be written to the field `TWM13`"]
pub enum TWM13W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM13W::_0 => false,
            TWM13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM13W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM13W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM13W::_1)
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
#[doc = "Values that can be written to the field `TWM14`"]
pub enum TWM14W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM14W::_0 => false,
            TWM14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM14W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM14W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM14W::_1)
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
#[doc = "Values that can be written to the field `TWM15`"]
pub enum TWM15W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM15W::_0 => false,
            TWM15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM15W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM15W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM15W::_1)
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
#[doc = "Values that can be written to the field `TWM16`"]
pub enum TWM16W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM16W::_0 => false,
            TWM16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM16W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM16W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM16W::_1)
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
#[doc = "Values that can be written to the field `TWM17`"]
pub enum TWM17W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM17W::_0 => false,
            TWM17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM17W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM17W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM17W::_1)
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
#[doc = "Values that can be written to the field `TWM18`"]
pub enum TWM18W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM18W::_0 => false,
            TWM18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM18W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM18W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM18W::_1)
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
#[doc = "Values that can be written to the field `TWM19`"]
pub enum TWM19W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM19W::_0 => false,
            TWM19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM19W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM19W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM19W::_1)
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
#[doc = "Values that can be written to the field `TWM20`"]
pub enum TWM20W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM20W::_0 => false,
            TWM20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM20W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM20W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM20W::_1)
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
#[doc = "Values that can be written to the field `TWM21`"]
pub enum TWM21W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM21W::_0 => false,
            TWM21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM21W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM21W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM21W::_1)
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
#[doc = "Values that can be written to the field `TWM22`"]
pub enum TWM22W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM22W::_0 => false,
            TWM22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM22W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM22W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM22W::_1)
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
#[doc = "Values that can be written to the field `TWM23`"]
pub enum TWM23W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM23W::_0 => false,
            TWM23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM23W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM23W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM23W::_1)
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
#[doc = "Values that can be written to the field `TWM24`"]
pub enum TWM24W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM24W::_0 => false,
            TWM24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM24W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM24W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM24W::_1)
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
#[doc = "Values that can be written to the field `TWM25`"]
pub enum TWM25W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM25W::_0 => false,
            TWM25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM25W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM25W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM25W::_1)
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
#[doc = "Values that can be written to the field `TWM26`"]
pub enum TWM26W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM26W::_0 => false,
            TWM26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM26W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM26W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM26W::_1)
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
#[doc = "Values that can be written to the field `TWM27`"]
pub enum TWM27W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM27W::_0 => false,
            TWM27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM27W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM27W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM27W::_1)
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
#[doc = "Values that can be written to the field `TWM28`"]
pub enum TWM28W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM28W::_0 => false,
            TWM28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM28W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM28W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM28W::_1)
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
#[doc = "Values that can be written to the field `TWM29`"]
pub enum TWM29W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM29W::_0 => false,
            TWM29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM29W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM29W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM29W::_1)
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
#[doc = "Values that can be written to the field `TWM30`"]
pub enum TWM30W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM30W::_0 => false,
            TWM30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM30W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM30W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM30W::_1)
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
#[doc = "Values that can be written to the field `TWM31`"]
pub enum TWM31W {
    #[doc = "Word N is enabled."]
    _0,
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    _1,
}
impl TWM31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWM31W::_0 => false,
            TWM31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWM31W<'a> {
    w: &'a mut W,
}
impl<'a> _TWM31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWM31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Word N is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWM31W::_0)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWM31W::_1)
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
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline]
    pub fn twm0(&self) -> TWM0R {
        TWM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline]
    pub fn twm1(&self) -> TWM1R {
        TWM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmit Word Mask"]
    #[inline]
    pub fn twm2(&self) -> TWM2R {
        TWM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit Word Mask"]
    #[inline]
    pub fn twm3(&self) -> TWM3R {
        TWM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transmit Word Mask"]
    #[inline]
    pub fn twm4(&self) -> TWM4R {
        TWM4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmit Word Mask"]
    #[inline]
    pub fn twm5(&self) -> TWM5R {
        TWM5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmit Word Mask"]
    #[inline]
    pub fn twm6(&self) -> TWM6R {
        TWM6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit Word Mask"]
    #[inline]
    pub fn twm7(&self) -> TWM7R {
        TWM7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Transmit Word Mask"]
    #[inline]
    pub fn twm8(&self) -> TWM8R {
        TWM8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit Word Mask"]
    #[inline]
    pub fn twm9(&self) -> TWM9R {
        TWM9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit Word Mask"]
    #[inline]
    pub fn twm10(&self) -> TWM10R {
        TWM10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit Word Mask"]
    #[inline]
    pub fn twm11(&self) -> TWM11R {
        TWM11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Transmit Word Mask"]
    #[inline]
    pub fn twm12(&self) -> TWM12R {
        TWM12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmit Word Mask"]
    #[inline]
    pub fn twm13(&self) -> TWM13R {
        TWM13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Transmit Word Mask"]
    #[inline]
    pub fn twm14(&self) -> TWM14R {
        TWM14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Transmit Word Mask"]
    #[inline]
    pub fn twm15(&self) -> TWM15R {
        TWM15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Transmit Word Mask"]
    #[inline]
    pub fn twm16(&self) -> TWM16R {
        TWM16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmit Word Mask"]
    #[inline]
    pub fn twm17(&self) -> TWM17R {
        TWM17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Transmit Word Mask"]
    #[inline]
    pub fn twm18(&self) -> TWM18R {
        TWM18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Transmit Word Mask"]
    #[inline]
    pub fn twm19(&self) -> TWM19R {
        TWM19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Transmit Word Mask"]
    #[inline]
    pub fn twm20(&self) -> TWM20R {
        TWM20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Transmit Word Mask"]
    #[inline]
    pub fn twm21(&self) -> TWM21R {
        TWM21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Transmit Word Mask"]
    #[inline]
    pub fn twm22(&self) -> TWM22R {
        TWM22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Word Mask"]
    #[inline]
    pub fn twm23(&self) -> TWM23R {
        TWM23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Transmit Word Mask"]
    #[inline]
    pub fn twm24(&self) -> TWM24R {
        TWM24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Transmit Word Mask"]
    #[inline]
    pub fn twm25(&self) -> TWM25R {
        TWM25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Transmit Word Mask"]
    #[inline]
    pub fn twm26(&self) -> TWM26R {
        TWM26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Transmit Word Mask"]
    #[inline]
    pub fn twm27(&self) -> TWM27R {
        TWM27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Transmit Word Mask"]
    #[inline]
    pub fn twm28(&self) -> TWM28R {
        TWM28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Transmit Word Mask"]
    #[inline]
    pub fn twm29(&self) -> TWM29R {
        TWM29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Transmit Word Mask"]
    #[inline]
    pub fn twm30(&self) -> TWM30R {
        TWM30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Transmit Word Mask"]
    #[inline]
    pub fn twm31(&self) -> TWM31R {
        TWM31R::_from({
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
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline]
    pub fn twm0(&mut self) -> _TWM0W {
        _TWM0W { w: self }
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline]
    pub fn twm1(&mut self) -> _TWM1W {
        _TWM1W { w: self }
    }
    #[doc = "Bit 2 - Transmit Word Mask"]
    #[inline]
    pub fn twm2(&mut self) -> _TWM2W {
        _TWM2W { w: self }
    }
    #[doc = "Bit 3 - Transmit Word Mask"]
    #[inline]
    pub fn twm3(&mut self) -> _TWM3W {
        _TWM3W { w: self }
    }
    #[doc = "Bit 4 - Transmit Word Mask"]
    #[inline]
    pub fn twm4(&mut self) -> _TWM4W {
        _TWM4W { w: self }
    }
    #[doc = "Bit 5 - Transmit Word Mask"]
    #[inline]
    pub fn twm5(&mut self) -> _TWM5W {
        _TWM5W { w: self }
    }
    #[doc = "Bit 6 - Transmit Word Mask"]
    #[inline]
    pub fn twm6(&mut self) -> _TWM6W {
        _TWM6W { w: self }
    }
    #[doc = "Bit 7 - Transmit Word Mask"]
    #[inline]
    pub fn twm7(&mut self) -> _TWM7W {
        _TWM7W { w: self }
    }
    #[doc = "Bit 8 - Transmit Word Mask"]
    #[inline]
    pub fn twm8(&mut self) -> _TWM8W {
        _TWM8W { w: self }
    }
    #[doc = "Bit 9 - Transmit Word Mask"]
    #[inline]
    pub fn twm9(&mut self) -> _TWM9W {
        _TWM9W { w: self }
    }
    #[doc = "Bit 10 - Transmit Word Mask"]
    #[inline]
    pub fn twm10(&mut self) -> _TWM10W {
        _TWM10W { w: self }
    }
    #[doc = "Bit 11 - Transmit Word Mask"]
    #[inline]
    pub fn twm11(&mut self) -> _TWM11W {
        _TWM11W { w: self }
    }
    #[doc = "Bit 12 - Transmit Word Mask"]
    #[inline]
    pub fn twm12(&mut self) -> _TWM12W {
        _TWM12W { w: self }
    }
    #[doc = "Bit 13 - Transmit Word Mask"]
    #[inline]
    pub fn twm13(&mut self) -> _TWM13W {
        _TWM13W { w: self }
    }
    #[doc = "Bit 14 - Transmit Word Mask"]
    #[inline]
    pub fn twm14(&mut self) -> _TWM14W {
        _TWM14W { w: self }
    }
    #[doc = "Bit 15 - Transmit Word Mask"]
    #[inline]
    pub fn twm15(&mut self) -> _TWM15W {
        _TWM15W { w: self }
    }
    #[doc = "Bit 16 - Transmit Word Mask"]
    #[inline]
    pub fn twm16(&mut self) -> _TWM16W {
        _TWM16W { w: self }
    }
    #[doc = "Bit 17 - Transmit Word Mask"]
    #[inline]
    pub fn twm17(&mut self) -> _TWM17W {
        _TWM17W { w: self }
    }
    #[doc = "Bit 18 - Transmit Word Mask"]
    #[inline]
    pub fn twm18(&mut self) -> _TWM18W {
        _TWM18W { w: self }
    }
    #[doc = "Bit 19 - Transmit Word Mask"]
    #[inline]
    pub fn twm19(&mut self) -> _TWM19W {
        _TWM19W { w: self }
    }
    #[doc = "Bit 20 - Transmit Word Mask"]
    #[inline]
    pub fn twm20(&mut self) -> _TWM20W {
        _TWM20W { w: self }
    }
    #[doc = "Bit 21 - Transmit Word Mask"]
    #[inline]
    pub fn twm21(&mut self) -> _TWM21W {
        _TWM21W { w: self }
    }
    #[doc = "Bit 22 - Transmit Word Mask"]
    #[inline]
    pub fn twm22(&mut self) -> _TWM22W {
        _TWM22W { w: self }
    }
    #[doc = "Bit 23 - Transmit Word Mask"]
    #[inline]
    pub fn twm23(&mut self) -> _TWM23W {
        _TWM23W { w: self }
    }
    #[doc = "Bit 24 - Transmit Word Mask"]
    #[inline]
    pub fn twm24(&mut self) -> _TWM24W {
        _TWM24W { w: self }
    }
    #[doc = "Bit 25 - Transmit Word Mask"]
    #[inline]
    pub fn twm25(&mut self) -> _TWM25W {
        _TWM25W { w: self }
    }
    #[doc = "Bit 26 - Transmit Word Mask"]
    #[inline]
    pub fn twm26(&mut self) -> _TWM26W {
        _TWM26W { w: self }
    }
    #[doc = "Bit 27 - Transmit Word Mask"]
    #[inline]
    pub fn twm27(&mut self) -> _TWM27W {
        _TWM27W { w: self }
    }
    #[doc = "Bit 28 - Transmit Word Mask"]
    #[inline]
    pub fn twm28(&mut self) -> _TWM28W {
        _TWM28W { w: self }
    }
    #[doc = "Bit 29 - Transmit Word Mask"]
    #[inline]
    pub fn twm29(&mut self) -> _TWM29W {
        _TWM29W { w: self }
    }
    #[doc = "Bit 30 - Transmit Word Mask"]
    #[inline]
    pub fn twm30(&mut self) -> _TWM30W {
        _TWM30W { w: self }
    }
    #[doc = "Bit 31 - Transmit Word Mask"]
    #[inline]
    pub fn twm31(&mut self) -> _TWM31W {
        _TWM31W { w: self }
    }
}
