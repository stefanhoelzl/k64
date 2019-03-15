#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMFR {
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
pub struct DATAR {
    bits: u16,
}
impl DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TAR {
    bits: u8,
}
impl TAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RAR {
    bits: u8,
}
impl RAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAR {
    bits: u8,
}
impl PAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPR {
    #[doc = "Write frame operation, but not MII compliant."]
    _00,
    #[doc = "Write frame operation for a valid MII management frame."]
    _01,
    #[doc = "Read frame operation for a valid MII management frame."]
    _10,
    #[doc = "Read frame operation, but not MII compliant."]
    _11,
}
impl OPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPR::_00 => 0,
            OPR::_01 => 1,
            OPR::_10 => 2,
            OPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPR {
        match value {
            0 => OPR::_00,
            1 => OPR::_01,
            2 => OPR::_10,
            3 => OPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == OPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == OPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == OPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == OPR::_11
    }
}
#[doc = r" Value of the field"]
pub struct STR {
    bits: u8,
}
impl STR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAW<'a> {
    w: &'a mut W,
}
impl<'a> _TAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAW<'a> {
    w: &'a mut W,
}
impl<'a> _PAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OP`"]
pub enum OPW {
    #[doc = "Write frame operation, but not MII compliant."]
    _00,
    #[doc = "Write frame operation for a valid MII management frame."]
    _01,
    #[doc = "Read frame operation for a valid MII management frame."]
    _10,
    #[doc = "Read frame operation, but not MII compliant."]
    _11,
}
impl OPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPW::_00 => 0,
            OPW::_01 => 1,
            OPW::_10 => 2,
            OPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPW<'a> {
    w: &'a mut W,
}
impl<'a> _OPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Write frame operation, but not MII compliant."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(OPW::_00)
    }
    #[doc = "Write frame operation for a valid MII management frame."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(OPW::_01)
    }
    #[doc = "Read frame operation for a valid MII management frame."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(OPW::_10)
    }
    #[doc = "Read frame operation, but not MII compliant."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(OPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STW<'a> {
    w: &'a mut W,
}
impl<'a> _STW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline]
    pub fn data(&self) -> DATAR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATAR { bits }
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline]
    pub fn ta(&self) -> TAR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TAR { bits }
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline]
    pub fn ra(&self) -> RAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RAR { bits }
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline]
    pub fn pa(&self) -> PAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAR { bits }
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline]
    pub fn op(&self) -> OPR {
        OPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline]
    pub fn st(&self) -> STR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STR { bits }
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
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline]
    pub fn data(&mut self) -> _DATAW {
        _DATAW { w: self }
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline]
    pub fn ta(&mut self) -> _TAW {
        _TAW { w: self }
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline]
    pub fn ra(&mut self) -> _RAW {
        _RAW { w: self }
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline]
    pub fn pa(&mut self) -> _PAW {
        _PAW { w: self }
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline]
    pub fn op(&mut self) -> _OPW {
        _OPW { w: self }
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline]
    pub fn st(&mut self) -> _STW {
        _STW { w: self }
    }
}
