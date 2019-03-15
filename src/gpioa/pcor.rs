#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCOR {
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
}
#[doc = "Values that can be written to the field `PTCO0`"]
pub enum PTCO0W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO0W::_0 => false,
            PTCO0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO0W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO0W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO0W::_1)
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
#[doc = "Values that can be written to the field `PTCO1`"]
pub enum PTCO1W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO1W::_0 => false,
            PTCO1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO1W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO1W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO1W::_1)
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
#[doc = "Values that can be written to the field `PTCO2`"]
pub enum PTCO2W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO2W::_0 => false,
            PTCO2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO2W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO2W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO2W::_1)
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
#[doc = "Values that can be written to the field `PTCO3`"]
pub enum PTCO3W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO3W::_0 => false,
            PTCO3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO3W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO3W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO3W::_1)
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
#[doc = "Values that can be written to the field `PTCO4`"]
pub enum PTCO4W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO4W::_0 => false,
            PTCO4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO4W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO4W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO4W::_1)
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
#[doc = "Values that can be written to the field `PTCO5`"]
pub enum PTCO5W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO5W::_0 => false,
            PTCO5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO5W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO5W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO5W::_1)
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
#[doc = "Values that can be written to the field `PTCO6`"]
pub enum PTCO6W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO6W::_0 => false,
            PTCO6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO6W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO6W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO6W::_1)
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
#[doc = "Values that can be written to the field `PTCO7`"]
pub enum PTCO7W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO7W::_0 => false,
            PTCO7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO7W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO7W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO7W::_1)
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
#[doc = "Values that can be written to the field `PTCO8`"]
pub enum PTCO8W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO8W::_0 => false,
            PTCO8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO8W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO8W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO8W::_1)
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
#[doc = "Values that can be written to the field `PTCO9`"]
pub enum PTCO9W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO9W::_0 => false,
            PTCO9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO9W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO9W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO9W::_1)
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
#[doc = "Values that can be written to the field `PTCO10`"]
pub enum PTCO10W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO10W::_0 => false,
            PTCO10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO10W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO10W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO10W::_1)
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
#[doc = "Values that can be written to the field `PTCO11`"]
pub enum PTCO11W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO11W::_0 => false,
            PTCO11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO11W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO11W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO11W::_1)
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
#[doc = "Values that can be written to the field `PTCO12`"]
pub enum PTCO12W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO12W::_0 => false,
            PTCO12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO12W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO12W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO12W::_1)
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
#[doc = "Values that can be written to the field `PTCO13`"]
pub enum PTCO13W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO13W::_0 => false,
            PTCO13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO13W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO13W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO13W::_1)
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
#[doc = "Values that can be written to the field `PTCO14`"]
pub enum PTCO14W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO14W::_0 => false,
            PTCO14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO14W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO14W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO14W::_1)
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
#[doc = "Values that can be written to the field `PTCO15`"]
pub enum PTCO15W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO15W::_0 => false,
            PTCO15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO15W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO15W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO15W::_1)
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
#[doc = "Values that can be written to the field `PTCO16`"]
pub enum PTCO16W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO16W::_0 => false,
            PTCO16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO16W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO16W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO16W::_1)
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
#[doc = "Values that can be written to the field `PTCO17`"]
pub enum PTCO17W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO17W::_0 => false,
            PTCO17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO17W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO17W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO17W::_1)
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
#[doc = "Values that can be written to the field `PTCO18`"]
pub enum PTCO18W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO18W::_0 => false,
            PTCO18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO18W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO18W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO18W::_1)
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
#[doc = "Values that can be written to the field `PTCO19`"]
pub enum PTCO19W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO19W::_0 => false,
            PTCO19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO19W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO19W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO19W::_1)
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
#[doc = "Values that can be written to the field `PTCO20`"]
pub enum PTCO20W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO20W::_0 => false,
            PTCO20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO20W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO20W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO20W::_1)
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
#[doc = "Values that can be written to the field `PTCO21`"]
pub enum PTCO21W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO21W::_0 => false,
            PTCO21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO21W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO21W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO21W::_1)
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
#[doc = "Values that can be written to the field `PTCO22`"]
pub enum PTCO22W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO22W::_0 => false,
            PTCO22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO22W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO22W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO22W::_1)
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
#[doc = "Values that can be written to the field `PTCO23`"]
pub enum PTCO23W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO23W::_0 => false,
            PTCO23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO23W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO23W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO23W::_1)
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
#[doc = "Values that can be written to the field `PTCO24`"]
pub enum PTCO24W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO24W::_0 => false,
            PTCO24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO24W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO24W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO24W::_1)
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
#[doc = "Values that can be written to the field `PTCO25`"]
pub enum PTCO25W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO25W::_0 => false,
            PTCO25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO25W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO25W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO25W::_1)
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
#[doc = "Values that can be written to the field `PTCO26`"]
pub enum PTCO26W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO26W::_0 => false,
            PTCO26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO26W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO26W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO26W::_1)
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
#[doc = "Values that can be written to the field `PTCO27`"]
pub enum PTCO27W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO27W::_0 => false,
            PTCO27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO27W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO27W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO27W::_1)
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
#[doc = "Values that can be written to the field `PTCO28`"]
pub enum PTCO28W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO28W::_0 => false,
            PTCO28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO28W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO28W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO28W::_1)
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
#[doc = "Values that can be written to the field `PTCO29`"]
pub enum PTCO29W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO29W::_0 => false,
            PTCO29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO29W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO29W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO29W::_1)
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
#[doc = "Values that can be written to the field `PTCO30`"]
pub enum PTCO30W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO30W::_0 => false,
            PTCO30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO30W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO30W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO30W::_1)
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
#[doc = "Values that can be written to the field `PTCO31`"]
pub enum PTCO31W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    _1,
}
impl PTCO31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTCO31W::_0 => false,
            PTCO31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCO31W<'a> {
    w: &'a mut W,
}
impl<'a> _PTCO31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCO31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO31W::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO31W::_1)
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
    #[doc = "Bit 0 - Port Clear Output"]
    #[inline]
    pub fn ptco0(&mut self) -> _PTCO0W {
        _PTCO0W { w: self }
    }
    #[doc = "Bit 1 - Port Clear Output"]
    #[inline]
    pub fn ptco1(&mut self) -> _PTCO1W {
        _PTCO1W { w: self }
    }
    #[doc = "Bit 2 - Port Clear Output"]
    #[inline]
    pub fn ptco2(&mut self) -> _PTCO2W {
        _PTCO2W { w: self }
    }
    #[doc = "Bit 3 - Port Clear Output"]
    #[inline]
    pub fn ptco3(&mut self) -> _PTCO3W {
        _PTCO3W { w: self }
    }
    #[doc = "Bit 4 - Port Clear Output"]
    #[inline]
    pub fn ptco4(&mut self) -> _PTCO4W {
        _PTCO4W { w: self }
    }
    #[doc = "Bit 5 - Port Clear Output"]
    #[inline]
    pub fn ptco5(&mut self) -> _PTCO5W {
        _PTCO5W { w: self }
    }
    #[doc = "Bit 6 - Port Clear Output"]
    #[inline]
    pub fn ptco6(&mut self) -> _PTCO6W {
        _PTCO6W { w: self }
    }
    #[doc = "Bit 7 - Port Clear Output"]
    #[inline]
    pub fn ptco7(&mut self) -> _PTCO7W {
        _PTCO7W { w: self }
    }
    #[doc = "Bit 8 - Port Clear Output"]
    #[inline]
    pub fn ptco8(&mut self) -> _PTCO8W {
        _PTCO8W { w: self }
    }
    #[doc = "Bit 9 - Port Clear Output"]
    #[inline]
    pub fn ptco9(&mut self) -> _PTCO9W {
        _PTCO9W { w: self }
    }
    #[doc = "Bit 10 - Port Clear Output"]
    #[inline]
    pub fn ptco10(&mut self) -> _PTCO10W {
        _PTCO10W { w: self }
    }
    #[doc = "Bit 11 - Port Clear Output"]
    #[inline]
    pub fn ptco11(&mut self) -> _PTCO11W {
        _PTCO11W { w: self }
    }
    #[doc = "Bit 12 - Port Clear Output"]
    #[inline]
    pub fn ptco12(&mut self) -> _PTCO12W {
        _PTCO12W { w: self }
    }
    #[doc = "Bit 13 - Port Clear Output"]
    #[inline]
    pub fn ptco13(&mut self) -> _PTCO13W {
        _PTCO13W { w: self }
    }
    #[doc = "Bit 14 - Port Clear Output"]
    #[inline]
    pub fn ptco14(&mut self) -> _PTCO14W {
        _PTCO14W { w: self }
    }
    #[doc = "Bit 15 - Port Clear Output"]
    #[inline]
    pub fn ptco15(&mut self) -> _PTCO15W {
        _PTCO15W { w: self }
    }
    #[doc = "Bit 16 - Port Clear Output"]
    #[inline]
    pub fn ptco16(&mut self) -> _PTCO16W {
        _PTCO16W { w: self }
    }
    #[doc = "Bit 17 - Port Clear Output"]
    #[inline]
    pub fn ptco17(&mut self) -> _PTCO17W {
        _PTCO17W { w: self }
    }
    #[doc = "Bit 18 - Port Clear Output"]
    #[inline]
    pub fn ptco18(&mut self) -> _PTCO18W {
        _PTCO18W { w: self }
    }
    #[doc = "Bit 19 - Port Clear Output"]
    #[inline]
    pub fn ptco19(&mut self) -> _PTCO19W {
        _PTCO19W { w: self }
    }
    #[doc = "Bit 20 - Port Clear Output"]
    #[inline]
    pub fn ptco20(&mut self) -> _PTCO20W {
        _PTCO20W { w: self }
    }
    #[doc = "Bit 21 - Port Clear Output"]
    #[inline]
    pub fn ptco21(&mut self) -> _PTCO21W {
        _PTCO21W { w: self }
    }
    #[doc = "Bit 22 - Port Clear Output"]
    #[inline]
    pub fn ptco22(&mut self) -> _PTCO22W {
        _PTCO22W { w: self }
    }
    #[doc = "Bit 23 - Port Clear Output"]
    #[inline]
    pub fn ptco23(&mut self) -> _PTCO23W {
        _PTCO23W { w: self }
    }
    #[doc = "Bit 24 - Port Clear Output"]
    #[inline]
    pub fn ptco24(&mut self) -> _PTCO24W {
        _PTCO24W { w: self }
    }
    #[doc = "Bit 25 - Port Clear Output"]
    #[inline]
    pub fn ptco25(&mut self) -> _PTCO25W {
        _PTCO25W { w: self }
    }
    #[doc = "Bit 26 - Port Clear Output"]
    #[inline]
    pub fn ptco26(&mut self) -> _PTCO26W {
        _PTCO26W { w: self }
    }
    #[doc = "Bit 27 - Port Clear Output"]
    #[inline]
    pub fn ptco27(&mut self) -> _PTCO27W {
        _PTCO27W { w: self }
    }
    #[doc = "Bit 28 - Port Clear Output"]
    #[inline]
    pub fn ptco28(&mut self) -> _PTCO28W {
        _PTCO28W { w: self }
    }
    #[doc = "Bit 29 - Port Clear Output"]
    #[inline]
    pub fn ptco29(&mut self) -> _PTCO29W {
        _PTCO29W { w: self }
    }
    #[doc = "Bit 30 - Port Clear Output"]
    #[inline]
    pub fn ptco30(&mut self) -> _PTCO30W {
        _PTCO30W { w: self }
    }
    #[doc = "Bit 31 - Port Clear Output"]
    #[inline]
    pub fn ptco31(&mut self) -> _PTCO31W {
        _PTCO31W { w: self }
    }
}
