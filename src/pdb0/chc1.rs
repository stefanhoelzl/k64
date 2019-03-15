#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHC1 {
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
#[doc = "Possible values of the field `EN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN0R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN0R {
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
            EN0R::_0 => false,
            EN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN0R {
        match value {
            false => EN0R::_0,
            true => EN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN0R::_1
    }
}
#[doc = "Possible values of the field `EN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN1R {
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
            EN1R::_0 => false,
            EN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN1R {
        match value {
            false => EN1R::_0,
            true => EN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN1R::_1
    }
}
#[doc = "Possible values of the field `EN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN2R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN2R {
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
            EN2R::_0 => false,
            EN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN2R {
        match value {
            false => EN2R::_0,
            true => EN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN2R::_1
    }
}
#[doc = "Possible values of the field `EN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN3R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN3R {
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
            EN3R::_0 => false,
            EN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN3R {
        match value {
            false => EN3R::_0,
            true => EN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN3R::_1
    }
}
#[doc = "Possible values of the field `EN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN4R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN4R {
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
            EN4R::_0 => false,
            EN4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN4R {
        match value {
            false => EN4R::_0,
            true => EN4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN4R::_1
    }
}
#[doc = "Possible values of the field `EN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN5R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN5R {
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
            EN5R::_0 => false,
            EN5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN5R {
        match value {
            false => EN5R::_0,
            true => EN5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN5R::_1
    }
}
#[doc = "Possible values of the field `EN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN6R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN6R {
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
            EN6R::_0 => false,
            EN6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN6R {
        match value {
            false => EN6R::_0,
            true => EN6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN6R::_1
    }
}
#[doc = "Possible values of the field `EN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN7R {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN7R {
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
            EN7R::_0 => false,
            EN7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN7R {
        match value {
            false => EN7R::_0,
            true => EN7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EN7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EN7R::_1
    }
}
#[doc = "Possible values of the field `TOS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS0R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS0R {
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
            TOS0R::_0 => false,
            TOS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS0R {
        match value {
            false => TOS0R::_0,
            true => TOS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS0R::_1
    }
}
#[doc = "Possible values of the field `TOS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS1R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS1R {
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
            TOS1R::_0 => false,
            TOS1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS1R {
        match value {
            false => TOS1R::_0,
            true => TOS1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS1R::_1
    }
}
#[doc = "Possible values of the field `TOS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS2R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS2R {
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
            TOS2R::_0 => false,
            TOS2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS2R {
        match value {
            false => TOS2R::_0,
            true => TOS2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS2R::_1
    }
}
#[doc = "Possible values of the field `TOS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS3R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS3R {
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
            TOS3R::_0 => false,
            TOS3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS3R {
        match value {
            false => TOS3R::_0,
            true => TOS3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS3R::_1
    }
}
#[doc = "Possible values of the field `TOS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS4R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS4R {
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
            TOS4R::_0 => false,
            TOS4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS4R {
        match value {
            false => TOS4R::_0,
            true => TOS4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS4R::_1
    }
}
#[doc = "Possible values of the field `TOS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS5R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS5R {
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
            TOS5R::_0 => false,
            TOS5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS5R {
        match value {
            false => TOS5R::_0,
            true => TOS5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS5R::_1
    }
}
#[doc = "Possible values of the field `TOS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS6R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS6R {
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
            TOS6R::_0 => false,
            TOS6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS6R {
        match value {
            false => TOS6R::_0,
            true => TOS6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS6R::_1
    }
}
#[doc = "Possible values of the field `TOS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOS7R {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS7R {
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
            TOS7R::_0 => false,
            TOS7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOS7R {
        match value {
            false => TOS7R::_0,
            true => TOS7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOS7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOS7R::_1
    }
}
#[doc = "Possible values of the field `BB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB0R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB0R {
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
            BB0R::_0 => false,
            BB0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB0R {
        match value {
            false => BB0R::_0,
            true => BB0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB0R::_1
    }
}
#[doc = "Possible values of the field `BB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB1R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB1R {
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
            BB1R::_0 => false,
            BB1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB1R {
        match value {
            false => BB1R::_0,
            true => BB1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB1R::_1
    }
}
#[doc = "Possible values of the field `BB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB2R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB2R {
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
            BB2R::_0 => false,
            BB2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB2R {
        match value {
            false => BB2R::_0,
            true => BB2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB2R::_1
    }
}
#[doc = "Possible values of the field `BB3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB3R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB3R {
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
            BB3R::_0 => false,
            BB3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB3R {
        match value {
            false => BB3R::_0,
            true => BB3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB3R::_1
    }
}
#[doc = "Possible values of the field `BB4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB4R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB4R {
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
            BB4R::_0 => false,
            BB4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB4R {
        match value {
            false => BB4R::_0,
            true => BB4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB4R::_1
    }
}
#[doc = "Possible values of the field `BB5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB5R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB5R {
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
            BB5R::_0 => false,
            BB5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB5R {
        match value {
            false => BB5R::_0,
            true => BB5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB5R::_1
    }
}
#[doc = "Possible values of the field `BB6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB6R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB6R {
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
            BB6R::_0 => false,
            BB6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB6R {
        match value {
            false => BB6R::_0,
            true => BB6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB6R::_1
    }
}
#[doc = "Possible values of the field `BB7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB7R {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB7R {
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
            BB7R::_0 => false,
            BB7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB7R {
        match value {
            false => BB7R::_0,
            true => BB7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BB7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BB7R::_1
    }
}
#[doc = "Values that can be written to the field `EN0`"]
pub enum EN0W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN0W::_0 => false,
            EN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN0W<'a> {
    w: &'a mut W,
}
impl<'a> _EN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN0W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN0W::_1)
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
#[doc = "Values that can be written to the field `EN1`"]
pub enum EN1W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN1W::_0 => false,
            EN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN1W<'a> {
    w: &'a mut W,
}
impl<'a> _EN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1W::_1)
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
#[doc = "Values that can be written to the field `EN2`"]
pub enum EN2W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN2W::_0 => false,
            EN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN2W<'a> {
    w: &'a mut W,
}
impl<'a> _EN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN2W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN2W::_1)
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
#[doc = "Values that can be written to the field `EN3`"]
pub enum EN3W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN3W::_0 => false,
            EN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN3W<'a> {
    w: &'a mut W,
}
impl<'a> _EN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN3W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN3W::_1)
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
#[doc = "Values that can be written to the field `EN4`"]
pub enum EN4W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN4W::_0 => false,
            EN4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN4W<'a> {
    w: &'a mut W,
}
impl<'a> _EN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN4W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN4W::_1)
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
#[doc = "Values that can be written to the field `EN5`"]
pub enum EN5W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN5W::_0 => false,
            EN5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN5W<'a> {
    w: &'a mut W,
}
impl<'a> _EN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN5W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN5W::_1)
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
#[doc = "Values that can be written to the field `EN6`"]
pub enum EN6W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN6W::_0 => false,
            EN6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN6W<'a> {
    w: &'a mut W,
}
impl<'a> _EN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN6W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN6W::_1)
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
#[doc = "Values that can be written to the field `EN7`"]
pub enum EN7W {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl EN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN7W::_0 => false,
            EN7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN7W<'a> {
    w: &'a mut W,
}
impl<'a> _EN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN7W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN7W::_1)
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
#[doc = "Values that can be written to the field `TOS0`"]
pub enum TOS0W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS0W::_0 => false,
            TOS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS0W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS0W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS0W::_1)
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
#[doc = "Values that can be written to the field `TOS1`"]
pub enum TOS1W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS1W::_0 => false,
            TOS1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS1W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS1W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS1W::_1)
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
#[doc = "Values that can be written to the field `TOS2`"]
pub enum TOS2W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS2W::_0 => false,
            TOS2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS2W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS2W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS2W::_1)
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
#[doc = "Values that can be written to the field `TOS3`"]
pub enum TOS3W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS3W::_0 => false,
            TOS3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS3W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS3W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS3W::_1)
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
#[doc = "Values that can be written to the field `TOS4`"]
pub enum TOS4W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS4W::_0 => false,
            TOS4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS4W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS4W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS4W::_1)
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
#[doc = "Values that can be written to the field `TOS5`"]
pub enum TOS5W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS5W::_0 => false,
            TOS5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS5W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS5W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS5W::_1)
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
#[doc = "Values that can be written to the field `TOS6`"]
pub enum TOS6W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS6W::_0 => false,
            TOS6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS6W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS6W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS6W::_1)
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
#[doc = "Values that can be written to the field `TOS7`"]
pub enum TOS7W {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOS7W::_0 => false,
            TOS7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOS7W<'a> {
    w: &'a mut W,
}
impl<'a> _TOS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOS7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS7W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS7W::_1)
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
#[doc = "Values that can be written to the field `BB0`"]
pub enum BB0W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB0W::_0 => false,
            BB0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB0W<'a> {
    w: &'a mut W,
}
impl<'a> _BB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB0W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB0W::_1)
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
#[doc = "Values that can be written to the field `BB1`"]
pub enum BB1W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB1W::_0 => false,
            BB1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB1W<'a> {
    w: &'a mut W,
}
impl<'a> _BB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB1W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB1W::_1)
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
#[doc = "Values that can be written to the field `BB2`"]
pub enum BB2W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB2W::_0 => false,
            BB2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB2W<'a> {
    w: &'a mut W,
}
impl<'a> _BB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB2W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB2W::_1)
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
#[doc = "Values that can be written to the field `BB3`"]
pub enum BB3W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB3W::_0 => false,
            BB3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB3W<'a> {
    w: &'a mut W,
}
impl<'a> _BB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB3W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB3W::_1)
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
#[doc = "Values that can be written to the field `BB4`"]
pub enum BB4W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB4W::_0 => false,
            BB4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB4W<'a> {
    w: &'a mut W,
}
impl<'a> _BB4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB4W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB4W::_1)
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
#[doc = "Values that can be written to the field `BB5`"]
pub enum BB5W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB5W::_0 => false,
            BB5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB5W<'a> {
    w: &'a mut W,
}
impl<'a> _BB5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB5W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB5W::_1)
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
#[doc = "Values that can be written to the field `BB6`"]
pub enum BB6W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB6W::_0 => false,
            BB6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB6W<'a> {
    w: &'a mut W,
}
impl<'a> _BB6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB6W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB6W::_1)
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
#[doc = "Values that can be written to the field `BB7`"]
pub enum BB7W {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BB7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BB7W::_0 => false,
            BB7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BB7W<'a> {
    w: &'a mut W,
}
impl<'a> _BB7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB7W::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB7W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en0(&self) -> EN0R {
        EN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en1(&self) -> EN1R {
        EN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en2(&self) -> EN2R {
        EN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en3(&self) -> EN3R {
        EN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en4(&self) -> EN4R {
        EN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en5(&self) -> EN5R {
        EN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en6(&self) -> EN6R {
        EN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en7(&self) -> EN7R {
        EN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos0(&self) -> TOS0R {
        TOS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos1(&self) -> TOS1R {
        TOS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos2(&self) -> TOS2R {
        TOS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos3(&self) -> TOS3R {
        TOS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos4(&self) -> TOS4R {
        TOS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos5(&self) -> TOS5R {
        TOS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos6(&self) -> TOS6R {
        TOS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos7(&self) -> TOS7R {
        TOS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb0(&self) -> BB0R {
        BB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb1(&self) -> BB1R {
        BB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb2(&self) -> BB2R {
        BB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb3(&self) -> BB3R {
        BB3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb4(&self) -> BB4R {
        BB4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb5(&self) -> BB5R {
        BB5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb6(&self) -> BB6R {
        BB6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb7(&self) -> BB7R {
        BB7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en0(&mut self) -> _EN0W {
        _EN0W { w: self }
    }
    #[doc = "Bit 1 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en1(&mut self) -> _EN1W {
        _EN1W { w: self }
    }
    #[doc = "Bit 2 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en2(&mut self) -> _EN2W {
        _EN2W { w: self }
    }
    #[doc = "Bit 3 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en3(&mut self) -> _EN3W {
        _EN3W { w: self }
    }
    #[doc = "Bit 4 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en4(&mut self) -> _EN4W {
        _EN4W { w: self }
    }
    #[doc = "Bit 5 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en5(&mut self) -> _EN5W {
        _EN5W { w: self }
    }
    #[doc = "Bit 6 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en6(&mut self) -> _EN6W {
        _EN6W { w: self }
    }
    #[doc = "Bit 7 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en7(&mut self) -> _EN7W {
        _EN7W { w: self }
    }
    #[doc = "Bit 8 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos0(&mut self) -> _TOS0W {
        _TOS0W { w: self }
    }
    #[doc = "Bit 9 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos1(&mut self) -> _TOS1W {
        _TOS1W { w: self }
    }
    #[doc = "Bit 10 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos2(&mut self) -> _TOS2W {
        _TOS2W { w: self }
    }
    #[doc = "Bit 11 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos3(&mut self) -> _TOS3W {
        _TOS3W { w: self }
    }
    #[doc = "Bit 12 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos4(&mut self) -> _TOS4W {
        _TOS4W { w: self }
    }
    #[doc = "Bit 13 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos5(&mut self) -> _TOS5W {
        _TOS5W { w: self }
    }
    #[doc = "Bit 14 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos6(&mut self) -> _TOS6W {
        _TOS6W { w: self }
    }
    #[doc = "Bit 15 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos7(&mut self) -> _TOS7W {
        _TOS7W { w: self }
    }
    #[doc = "Bit 16 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb0(&mut self) -> _BB0W {
        _BB0W { w: self }
    }
    #[doc = "Bit 17 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb1(&mut self) -> _BB1W {
        _BB1W { w: self }
    }
    #[doc = "Bit 18 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb2(&mut self) -> _BB2W {
        _BB2W { w: self }
    }
    #[doc = "Bit 19 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb3(&mut self) -> _BB3W {
        _BB3W { w: self }
    }
    #[doc = "Bit 20 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb4(&mut self) -> _BB4W {
        _BB4W { w: self }
    }
    #[doc = "Bit 21 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb5(&mut self) -> _BB5W {
        _BB5W { w: self }
    }
    #[doc = "Bit 22 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb6(&mut self) -> _BB6W {
        _BB6W { w: self }
    }
    #[doc = "Bit 23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb7(&mut self) -> _BB7W {
        _BB7W { w: self }
    }
}
