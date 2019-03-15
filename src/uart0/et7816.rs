#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ET7816 {
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
pub struct RXTHRESHOLDR {
    bits: u8,
}
impl RXTHRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXTHRESHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTHRESHOLDR {
    #[doc = "TXT asserts on the first NACK that is received."]
    _0,
    #[doc = "TXT asserts on the second NACK that is received."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXTHRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXTHRESHOLDR::_0 => 0,
            TXTHRESHOLDR::_1 => 1,
            TXTHRESHOLDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXTHRESHOLDR {
        match value {
            0 => TXTHRESHOLDR::_0,
            1 => TXTHRESHOLDR::_1,
            i => TXTHRESHOLDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXTHRESHOLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXTHRESHOLDR::_1
    }
}
#[doc = r" Proxy"]
pub struct _RXTHRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTHRESHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXTHRESHOLD`"]
pub enum TXTHRESHOLDW {
    #[doc = "TXT asserts on the first NACK that is received."]
    _0,
    #[doc = "TXT asserts on the second NACK that is received."]
    _1,
}
impl TXTHRESHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXTHRESHOLDW::_0 => 0,
            TXTHRESHOLDW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXTHRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTHRESHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXTHRESHOLDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TXT asserts on the first NACK that is received."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTHRESHOLDW::_0)
    }
    #[doc = "TXT asserts on the second NACK that is received."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTHRESHOLDW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline]
    pub fn rxthreshold(&self) -> RXTHRESHOLDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        RXTHRESHOLDR { bits }
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline]
    pub fn txthreshold(&self) -> TXTHRESHOLDR {
        TXTHRESHOLDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline]
    pub fn rxthreshold(&mut self) -> _RXTHRESHOLDW {
        _RXTHRESHOLDW { w: self }
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline]
    pub fn txthreshold(&mut self) -> _TXTHRESHOLDW {
        _TXTHRESHOLDW { w: self }
    }
}
