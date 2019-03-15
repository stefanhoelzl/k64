#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RGDAAC {
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
pub struct M0UMR {
    bits: u8,
}
impl M0UMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M0SMR {
    bits: u8,
}
impl M0SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M0PER {
    bits: bool,
}
impl M0PER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct M1UMR {
    bits: u8,
}
impl M1UMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M1SMR {
    bits: u8,
}
impl M1SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M1PER {
    bits: bool,
}
impl M1PER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct M2UMR {
    bits: u8,
}
impl M2UMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M2SMR {
    bits: u8,
}
impl M2SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M2PER {
    bits: bool,
}
impl M2PER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `M3UM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3UMR {
    #[doc = "An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    _0,
    #[doc = "Allows the given access type to occur"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl M3UMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M3UMR::_0 => 0,
            M3UMR::_1 => 1,
            M3UMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M3UMR {
        match value {
            0 => M3UMR::_0,
            1 => M3UMR::_1,
            i => M3UMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M3UMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M3UMR::_1
    }
}
#[doc = "Possible values of the field `M3SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3SMR {
    #[doc = "r/w/x; read, write and execute allowed"]
    _00,
    #[doc = "r/x; read and execute allowed, but no write"]
    _01,
    #[doc = "r/w; read and write allowed, but no execute"]
    _10,
    #[doc = "Same as User mode defined in M3UM"]
    _11,
}
impl M3SMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M3SMR::_00 => 0,
            M3SMR::_01 => 1,
            M3SMR::_10 => 2,
            M3SMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M3SMR {
        match value {
            0 => M3SMR::_00,
            1 => M3SMR::_01,
            2 => M3SMR::_10,
            3 => M3SMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M3SMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M3SMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M3SMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M3SMR::_11
    }
}
#[doc = "Possible values of the field `M3PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3PER {
    #[doc = "Do not include the process identifier in the evaluation"]
    _0,
    #[doc = "Include the process identifier and mask (RGDn.RGDAAC) in the region hit evaluation"]
    _1,
}
impl M3PER {
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
            M3PER::_0 => false,
            M3PER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M3PER {
        match value {
            false => M3PER::_0,
            true => M3PER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M3PER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M3PER::_1
    }
}
#[doc = "Possible values of the field `M4WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4WER {
    #[doc = "Bus master 4 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 4 writes allowed"]
    _1,
}
impl M4WER {
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
            M4WER::_0 => false,
            M4WER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4WER {
        match value {
            false => M4WER::_0,
            true => M4WER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M4WER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M4WER::_1
    }
}
#[doc = "Possible values of the field `M4RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4RER {
    #[doc = "Bus master 4 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 4 reads allowed"]
    _1,
}
impl M4RER {
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
            M4RER::_0 => false,
            M4RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4RER {
        match value {
            false => M4RER::_0,
            true => M4RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M4RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M4RER::_1
    }
}
#[doc = "Possible values of the field `M5WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5WER {
    #[doc = "Bus master 5 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 5 writes allowed"]
    _1,
}
impl M5WER {
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
            M5WER::_0 => false,
            M5WER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M5WER {
        match value {
            false => M5WER::_0,
            true => M5WER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M5WER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M5WER::_1
    }
}
#[doc = "Possible values of the field `M5RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5RER {
    #[doc = "Bus master 5 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 5 reads allowed"]
    _1,
}
impl M5RER {
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
            M5RER::_0 => false,
            M5RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M5RER {
        match value {
            false => M5RER::_0,
            true => M5RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M5RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M5RER::_1
    }
}
#[doc = "Possible values of the field `M6WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6WER {
    #[doc = "Bus master 6 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 6 writes allowed"]
    _1,
}
impl M6WER {
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
            M6WER::_0 => false,
            M6WER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M6WER {
        match value {
            false => M6WER::_0,
            true => M6WER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M6WER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M6WER::_1
    }
}
#[doc = "Possible values of the field `M6RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6RER {
    #[doc = "Bus master 6 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 6 reads allowed"]
    _1,
}
impl M6RER {
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
            M6RER::_0 => false,
            M6RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M6RER {
        match value {
            false => M6RER::_0,
            true => M6RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M6RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M6RER::_1
    }
}
#[doc = "Possible values of the field `M7WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7WER {
    #[doc = "Bus master 7 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 7 writes allowed"]
    _1,
}
impl M7WER {
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
            M7WER::_0 => false,
            M7WER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M7WER {
        match value {
            false => M7WER::_0,
            true => M7WER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M7WER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M7WER::_1
    }
}
#[doc = "Possible values of the field `M7RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7RER {
    #[doc = "Bus master 7 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 7 reads allowed"]
    _1,
}
impl M7RER {
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
            M7RER::_0 => false,
            M7RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M7RER {
        match value {
            false => M7RER::_0,
            true => M7RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M7RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M7RER::_1
    }
}
#[doc = r" Proxy"]
pub struct _M0UMW<'a> {
    w: &'a mut W,
}
impl<'a> _M0UMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M0SMW<'a> {
    w: &'a mut W,
}
impl<'a> _M0SMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M0PEW<'a> {
    w: &'a mut W,
}
impl<'a> _M0PEW<'a> {
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
#[doc = r" Proxy"]
pub struct _M1UMW<'a> {
    w: &'a mut W,
}
impl<'a> _M1UMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M1SMW<'a> {
    w: &'a mut W,
}
impl<'a> _M1SMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M1PEW<'a> {
    w: &'a mut W,
}
impl<'a> _M1PEW<'a> {
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
#[doc = r" Proxy"]
pub struct _M2UMW<'a> {
    w: &'a mut W,
}
impl<'a> _M2UMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M2SMW<'a> {
    w: &'a mut W,
}
impl<'a> _M2SMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M2PEW<'a> {
    w: &'a mut W,
}
impl<'a> _M2PEW<'a> {
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
#[doc = "Values that can be written to the field `M3UM`"]
pub enum M3UMW {
    #[doc = "An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    _0,
    #[doc = "Allows the given access type to occur"]
    _1,
}
impl M3UMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M3UMW::_0 => 0,
            M3UMW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M3UMW<'a> {
    w: &'a mut W,
}
impl<'a> _M3UMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M3UMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3UMW::_0)
    }
    #[doc = "Allows the given access type to occur"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3UMW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M3SM`"]
pub enum M3SMW {
    #[doc = "r/w/x; read, write and execute allowed"]
    _00,
    #[doc = "r/x; read and execute allowed, but no write"]
    _01,
    #[doc = "r/w; read and write allowed, but no execute"]
    _10,
    #[doc = "Same as User mode defined in M3UM"]
    _11,
}
impl M3SMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M3SMW::_00 => 0,
            M3SMW::_01 => 1,
            M3SMW::_10 => 2,
            M3SMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M3SMW<'a> {
    w: &'a mut W,
}
impl<'a> _M3SMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M3SMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "r/w/x; read, write and execute allowed"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M3SMW::_00)
    }
    #[doc = "r/x; read and execute allowed, but no write"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M3SMW::_01)
    }
    #[doc = "r/w; read and write allowed, but no execute"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M3SMW::_10)
    }
    #[doc = "Same as User mode defined in M3UM"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M3SMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M3PE`"]
pub enum M3PEW {
    #[doc = "Do not include the process identifier in the evaluation"]
    _0,
    #[doc = "Include the process identifier and mask (RGDn.RGDAAC) in the region hit evaluation"]
    _1,
}
impl M3PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M3PEW::_0 => false,
            M3PEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M3PEW<'a> {
    w: &'a mut W,
}
impl<'a> _M3PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M3PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not include the process identifier in the evaluation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3PEW::_0)
    }
    #[doc = "Include the process identifier and mask (RGDn.RGDAAC) in the region hit evaluation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3PEW::_1)
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
#[doc = "Values that can be written to the field `M4WE`"]
pub enum M4WEW {
    #[doc = "Bus master 4 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 4 writes allowed"]
    _1,
}
impl M4WEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4WEW::_0 => false,
            M4WEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4WEW<'a> {
    w: &'a mut W,
}
impl<'a> _M4WEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4WEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 4 writes terminate with an access error and the write is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4WEW::_0)
    }
    #[doc = "Bus master 4 writes allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4WEW::_1)
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
#[doc = "Values that can be written to the field `M4RE`"]
pub enum M4REW {
    #[doc = "Bus master 4 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 4 reads allowed"]
    _1,
}
impl M4REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4REW::_0 => false,
            M4REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4REW<'a> {
    w: &'a mut W,
}
impl<'a> _M4REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 4 reads terminate with an access error and the read is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4REW::_0)
    }
    #[doc = "Bus master 4 reads allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4REW::_1)
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
#[doc = "Values that can be written to the field `M5WE`"]
pub enum M5WEW {
    #[doc = "Bus master 5 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 5 writes allowed"]
    _1,
}
impl M5WEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M5WEW::_0 => false,
            M5WEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M5WEW<'a> {
    w: &'a mut W,
}
impl<'a> _M5WEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M5WEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 5 writes terminate with an access error and the write is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5WEW::_0)
    }
    #[doc = "Bus master 5 writes allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5WEW::_1)
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
#[doc = "Values that can be written to the field `M5RE`"]
pub enum M5REW {
    #[doc = "Bus master 5 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 5 reads allowed"]
    _1,
}
impl M5REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M5REW::_0 => false,
            M5REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M5REW<'a> {
    w: &'a mut W,
}
impl<'a> _M5REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M5REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 5 reads terminate with an access error and the read is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5REW::_0)
    }
    #[doc = "Bus master 5 reads allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5REW::_1)
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
#[doc = "Values that can be written to the field `M6WE`"]
pub enum M6WEW {
    #[doc = "Bus master 6 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 6 writes allowed"]
    _1,
}
impl M6WEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M6WEW::_0 => false,
            M6WEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M6WEW<'a> {
    w: &'a mut W,
}
impl<'a> _M6WEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M6WEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 6 writes terminate with an access error and the write is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6WEW::_0)
    }
    #[doc = "Bus master 6 writes allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6WEW::_1)
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
#[doc = "Values that can be written to the field `M6RE`"]
pub enum M6REW {
    #[doc = "Bus master 6 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 6 reads allowed"]
    _1,
}
impl M6REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M6REW::_0 => false,
            M6REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M6REW<'a> {
    w: &'a mut W,
}
impl<'a> _M6REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M6REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 6 reads terminate with an access error and the read is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6REW::_0)
    }
    #[doc = "Bus master 6 reads allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6REW::_1)
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
#[doc = "Values that can be written to the field `M7WE`"]
pub enum M7WEW {
    #[doc = "Bus master 7 writes terminate with an access error and the write is not performed"]
    _0,
    #[doc = "Bus master 7 writes allowed"]
    _1,
}
impl M7WEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M7WEW::_0 => false,
            M7WEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7WEW<'a> {
    w: &'a mut W,
}
impl<'a> _M7WEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7WEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 7 writes terminate with an access error and the write is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7WEW::_0)
    }
    #[doc = "Bus master 7 writes allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7WEW::_1)
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
#[doc = "Values that can be written to the field `M7RE`"]
pub enum M7REW {
    #[doc = "Bus master 7 reads terminate with an access error and the read is not performed"]
    _0,
    #[doc = "Bus master 7 reads allowed"]
    _1,
}
impl M7REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M7REW::_0 => false,
            M7REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7REW<'a> {
    w: &'a mut W,
}
impl<'a> _M7REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master 7 reads terminate with an access error and the read is not performed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7REW::_0)
    }
    #[doc = "Bus master 7 reads allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7REW::_1)
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
    #[doc = "Bits 0:2 - Bus Master 0 User Mode Access Control"]
    #[inline]
    pub fn m0um(&self) -> M0UMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M0UMR { bits }
    }
    #[doc = "Bits 3:4 - Bus Master 0 Supervisor Mode Access Control"]
    #[inline]
    pub fn m0sm(&self) -> M0SMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M0SMR { bits }
    }
    #[doc = "Bit 5 - Bus Master 0 Process Identifier Enable"]
    #[inline]
    pub fn m0pe(&self) -> M0PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M0PER { bits }
    }
    #[doc = "Bits 6:8 - Bus Master 1 User Mode Access Control"]
    #[inline]
    pub fn m1um(&self) -> M1UMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M1UMR { bits }
    }
    #[doc = "Bits 9:10 - Bus Master 1 Supervisor Mode Access Control"]
    #[inline]
    pub fn m1sm(&self) -> M1SMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M1SMR { bits }
    }
    #[doc = "Bit 11 - Bus Master 1 Process Identifier Enable"]
    #[inline]
    pub fn m1pe(&self) -> M1PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M1PER { bits }
    }
    #[doc = "Bits 12:14 - Bus Master 2 User Mode Access Control"]
    #[inline]
    pub fn m2um(&self) -> M2UMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M2UMR { bits }
    }
    #[doc = "Bits 15:16 - Bus Master 2 Supervisor Mode Access Control"]
    #[inline]
    pub fn m2sm(&self) -> M2SMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M2SMR { bits }
    }
    #[doc = "Bit 17 - Bus Master 2 Process Identifier Enable"]
    #[inline]
    pub fn m2pe(&self) -> M2PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M2PER { bits }
    }
    #[doc = "Bits 18:20 - Bus Master 3 User Mode Access Control"]
    #[inline]
    pub fn m3um(&self) -> M3UMR {
        M3UMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:22 - Bus Master 3 Supervisor Mode Access Control"]
    #[inline]
    pub fn m3sm(&self) -> M3SMR {
        M3SMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Bus Master 3 Process Identifier Enable"]
    #[inline]
    pub fn m3pe(&self) -> M3PER {
        M3PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Bus Master 4 Write Enable"]
    #[inline]
    pub fn m4we(&self) -> M4WER {
        M4WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Bus Master 4 Read Enable"]
    #[inline]
    pub fn m4re(&self) -> M4RER {
        M4RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Bus Master 5 Write Enable"]
    #[inline]
    pub fn m5we(&self) -> M5WER {
        M5WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Bus Master 5 Read Enable"]
    #[inline]
    pub fn m5re(&self) -> M5RER {
        M5RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Bus Master 6 Write Enable"]
    #[inline]
    pub fn m6we(&self) -> M6WER {
        M6WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Bus Master 6 Read Enable"]
    #[inline]
    pub fn m6re(&self) -> M6RER {
        M6RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Bus Master 7 Write Enable"]
    #[inline]
    pub fn m7we(&self) -> M7WER {
        M7WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Bus Master 7 Read Enable"]
    #[inline]
    pub fn m7re(&self) -> M7RER {
        M7RER::_from({
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
        W { bits: 6420447 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Bus Master 0 User Mode Access Control"]
    #[inline]
    pub fn m0um(&mut self) -> _M0UMW {
        _M0UMW { w: self }
    }
    #[doc = "Bits 3:4 - Bus Master 0 Supervisor Mode Access Control"]
    #[inline]
    pub fn m0sm(&mut self) -> _M0SMW {
        _M0SMW { w: self }
    }
    #[doc = "Bit 5 - Bus Master 0 Process Identifier Enable"]
    #[inline]
    pub fn m0pe(&mut self) -> _M0PEW {
        _M0PEW { w: self }
    }
    #[doc = "Bits 6:8 - Bus Master 1 User Mode Access Control"]
    #[inline]
    pub fn m1um(&mut self) -> _M1UMW {
        _M1UMW { w: self }
    }
    #[doc = "Bits 9:10 - Bus Master 1 Supervisor Mode Access Control"]
    #[inline]
    pub fn m1sm(&mut self) -> _M1SMW {
        _M1SMW { w: self }
    }
    #[doc = "Bit 11 - Bus Master 1 Process Identifier Enable"]
    #[inline]
    pub fn m1pe(&mut self) -> _M1PEW {
        _M1PEW { w: self }
    }
    #[doc = "Bits 12:14 - Bus Master 2 User Mode Access Control"]
    #[inline]
    pub fn m2um(&mut self) -> _M2UMW {
        _M2UMW { w: self }
    }
    #[doc = "Bits 15:16 - Bus Master 2 Supervisor Mode Access Control"]
    #[inline]
    pub fn m2sm(&mut self) -> _M2SMW {
        _M2SMW { w: self }
    }
    #[doc = "Bit 17 - Bus Master 2 Process Identifier Enable"]
    #[inline]
    pub fn m2pe(&mut self) -> _M2PEW {
        _M2PEW { w: self }
    }
    #[doc = "Bits 18:20 - Bus Master 3 User Mode Access Control"]
    #[inline]
    pub fn m3um(&mut self) -> _M3UMW {
        _M3UMW { w: self }
    }
    #[doc = "Bits 21:22 - Bus Master 3 Supervisor Mode Access Control"]
    #[inline]
    pub fn m3sm(&mut self) -> _M3SMW {
        _M3SMW { w: self }
    }
    #[doc = "Bit 23 - Bus Master 3 Process Identifier Enable"]
    #[inline]
    pub fn m3pe(&mut self) -> _M3PEW {
        _M3PEW { w: self }
    }
    #[doc = "Bit 24 - Bus Master 4 Write Enable"]
    #[inline]
    pub fn m4we(&mut self) -> _M4WEW {
        _M4WEW { w: self }
    }
    #[doc = "Bit 25 - Bus Master 4 Read Enable"]
    #[inline]
    pub fn m4re(&mut self) -> _M4REW {
        _M4REW { w: self }
    }
    #[doc = "Bit 26 - Bus Master 5 Write Enable"]
    #[inline]
    pub fn m5we(&mut self) -> _M5WEW {
        _M5WEW { w: self }
    }
    #[doc = "Bit 27 - Bus Master 5 Read Enable"]
    #[inline]
    pub fn m5re(&mut self) -> _M5REW {
        _M5REW { w: self }
    }
    #[doc = "Bit 28 - Bus Master 6 Write Enable"]
    #[inline]
    pub fn m6we(&mut self) -> _M6WEW {
        _M6WEW { w: self }
    }
    #[doc = "Bit 29 - Bus Master 6 Read Enable"]
    #[inline]
    pub fn m6re(&mut self) -> _M6REW {
        _M6REW { w: self }
    }
    #[doc = "Bit 30 - Bus Master 7 Write Enable"]
    #[inline]
    pub fn m7we(&mut self) -> _M7WEW {
        _M7WEW { w: self }
    }
    #[doc = "Bit 31 - Bus Master 7 Read Enable"]
    #[inline]
    pub fn m7re(&mut self) -> _M7REW {
        _M7REW { w: self }
    }
}
