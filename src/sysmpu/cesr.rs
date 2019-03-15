#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CESR {
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
#[doc = "Possible values of the field `VLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLDR {
    #[doc = "MPU is disabled. All accesses from all bus masters are allowed."]
    _0,
    #[doc = "MPU is enabled"]
    _1,
}
impl VLDR {
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
            VLDR::_0 => false,
            VLDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLDR {
        match value {
            false => VLDR::_0,
            true => VLDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VLDR::_1
    }
}
#[doc = "Possible values of the field `NRGD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRGDR {
    #[doc = "8 region descriptors"]
    _0000,
    #[doc = "12 region descriptors"]
    _0001,
    #[doc = "16 region descriptors"]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NRGDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NRGDR::_0000 => 0,
            NRGDR::_0001 => 1,
            NRGDR::_0010 => 2,
            NRGDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NRGDR {
        match value {
            0 => NRGDR::_0000,
            1 => NRGDR::_0001,
            2 => NRGDR::_0010,
            i => NRGDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == NRGDR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == NRGDR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == NRGDR::_0010
    }
}
#[doc = r" Value of the field"]
pub struct NSPR {
    bits: u8,
}
impl NSPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HRLR {
    bits: u8,
}
impl HRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPERRR {
    #[doc = "No error has occurred for slave port n."]
    _0,
    #[doc = "An error has occurred for slave port n."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPERRR::_0 => 0,
            SPERRR::_1 => 1,
            SPERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPERRR {
        match value {
            0 => SPERRR::_0,
            1 => SPERRR::_1,
            i => SPERRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPERRR::_1
    }
}
#[doc = "Values that can be written to the field `VLD`"]
pub enum VLDW {
    #[doc = "MPU is disabled. All accesses from all bus masters are allowed."]
    _0,
    #[doc = "MPU is enabled"]
    _1,
}
impl VLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VLDW::_0 => false,
            VLDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLDW<'a> {
    w: &'a mut W,
}
impl<'a> _VLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MPU is disabled. All accesses from all bus masters are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VLDW::_0)
    }
    #[doc = "MPU is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VLDW::_1)
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
#[doc = "Values that can be written to the field `SPERR`"]
pub enum SPERRW {
    #[doc = "No error has occurred for slave port n."]
    _0,
    #[doc = "An error has occurred for slave port n."]
    _1,
}
impl SPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPERRW::_0 => 0,
            SPERRW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPERRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No error has occurred for slave port n."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPERRW::_0)
    }
    #[doc = "An error has occurred for slave port n."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPERRW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Valid"]
    #[inline]
    pub fn vld(&self) -> VLDR {
        VLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Number Of Region Descriptors"]
    #[inline]
    pub fn nrgd(&self) -> NRGDR {
        NRGDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Number Of Slave Ports"]
    #[inline]
    pub fn nsp(&self) -> NSPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NSPR { bits }
    }
    #[doc = "Bits 16:19 - Hardware Revision Level"]
    #[inline]
    pub fn hrl(&self) -> HRLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HRLR { bits }
    }
    #[doc = "Bits 27:31 - Slave Port n Error"]
    #[inline]
    pub fn sperr(&self) -> SPERRR {
        SPERRR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8474881 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Valid"]
    #[inline]
    pub fn vld(&mut self) -> _VLDW {
        _VLDW { w: self }
    }
    #[doc = "Bits 27:31 - Slave Port n Error"]
    #[inline]
    pub fn sperr(&mut self) -> _SPERRW {
        _SPERRW { w: self }
    }
}
