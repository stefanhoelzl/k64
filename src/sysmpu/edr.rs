#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ERW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERWR {
    #[doc = "Read"]
    _0,
    #[doc = "Write"]
    _1,
}
impl ERWR {
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
            ERWR::_0 => false,
            ERWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERWR {
        match value {
            false => ERWR::_0,
            true => ERWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERWR::_1
    }
}
#[doc = "Possible values of the field `EATTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EATTRR {
    #[doc = "User mode, instruction access"]
    _000,
    #[doc = "User mode, data access"]
    _001,
    #[doc = "Supervisor mode, instruction access"]
    _010,
    #[doc = "Supervisor mode, data access"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EATTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EATTRR::_000 => 0,
            EATTRR::_001 => 1,
            EATTRR::_010 => 2,
            EATTRR::_011 => 3,
            EATTRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EATTRR {
        match value {
            0 => EATTRR::_000,
            1 => EATTRR::_001,
            2 => EATTRR::_010,
            3 => EATTRR::_011,
            i => EATTRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == EATTRR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == EATTRR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == EATTRR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == EATTRR::_011
    }
}
#[doc = r" Value of the field"]
pub struct EMNR {
    bits: u8,
}
impl EMNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPIDR {
    bits: u8,
}
impl EPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EACDR {
    bits: u16,
}
impl EACDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Error Read/Write"]
    #[inline]
    pub fn erw(&self) -> ERWR {
        ERWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Error Attributes"]
    #[inline]
    pub fn eattr(&self) -> EATTRR {
        EATTRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Error Master Number"]
    #[inline]
    pub fn emn(&self) -> EMNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EMNR { bits }
    }
    #[doc = "Bits 8:15 - Error Process Identification"]
    #[inline]
    pub fn epid(&self) -> EPIDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPIDR { bits }
    }
    #[doc = "Bits 16:31 - Error Access Control Detail"]
    #[inline]
    pub fn eacd(&self) -> EACDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EACDR { bits }
    }
}
