#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSOR {
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
#[doc = "Values that can be written to the field `PTSO0`"]
pub enum PTSO0W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO0W::_0 => false,
            PTSO0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO0W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO0W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO0W::_1)
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
#[doc = "Values that can be written to the field `PTSO1`"]
pub enum PTSO1W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO1W::_0 => false,
            PTSO1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO1W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO1W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO1W::_1)
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
#[doc = "Values that can be written to the field `PTSO2`"]
pub enum PTSO2W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO2W::_0 => false,
            PTSO2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO2W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO2W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO2W::_1)
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
#[doc = "Values that can be written to the field `PTSO3`"]
pub enum PTSO3W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO3W::_0 => false,
            PTSO3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO3W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO3W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO3W::_1)
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
#[doc = "Values that can be written to the field `PTSO4`"]
pub enum PTSO4W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO4W::_0 => false,
            PTSO4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO4W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO4W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO4W::_1)
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
#[doc = "Values that can be written to the field `PTSO5`"]
pub enum PTSO5W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO5W::_0 => false,
            PTSO5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO5W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO5W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO5W::_1)
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
#[doc = "Values that can be written to the field `PTSO6`"]
pub enum PTSO6W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO6W::_0 => false,
            PTSO6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO6W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO6W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO6W::_1)
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
#[doc = "Values that can be written to the field `PTSO7`"]
pub enum PTSO7W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO7W::_0 => false,
            PTSO7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO7W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO7W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO7W::_1)
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
#[doc = "Values that can be written to the field `PTSO8`"]
pub enum PTSO8W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO8W::_0 => false,
            PTSO8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO8W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO8W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO8W::_1)
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
#[doc = "Values that can be written to the field `PTSO9`"]
pub enum PTSO9W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO9W::_0 => false,
            PTSO9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO9W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO9W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO9W::_1)
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
#[doc = "Values that can be written to the field `PTSO10`"]
pub enum PTSO10W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO10W::_0 => false,
            PTSO10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO10W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO10W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO10W::_1)
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
#[doc = "Values that can be written to the field `PTSO11`"]
pub enum PTSO11W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO11W::_0 => false,
            PTSO11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO11W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO11W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO11W::_1)
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
#[doc = "Values that can be written to the field `PTSO12`"]
pub enum PTSO12W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO12W::_0 => false,
            PTSO12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO12W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO12W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO12W::_1)
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
#[doc = "Values that can be written to the field `PTSO13`"]
pub enum PTSO13W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO13W::_0 => false,
            PTSO13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO13W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO13W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO13W::_1)
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
#[doc = "Values that can be written to the field `PTSO14`"]
pub enum PTSO14W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO14W::_0 => false,
            PTSO14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO14W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO14W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO14W::_1)
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
#[doc = "Values that can be written to the field `PTSO15`"]
pub enum PTSO15W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO15W::_0 => false,
            PTSO15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO15W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO15W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO15W::_1)
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
#[doc = "Values that can be written to the field `PTSO16`"]
pub enum PTSO16W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO16W::_0 => false,
            PTSO16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO16W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO16W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO16W::_1)
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
#[doc = "Values that can be written to the field `PTSO17`"]
pub enum PTSO17W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO17W::_0 => false,
            PTSO17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO17W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO17W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO17W::_1)
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
#[doc = "Values that can be written to the field `PTSO18`"]
pub enum PTSO18W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO18W::_0 => false,
            PTSO18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO18W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO18W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO18W::_1)
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
#[doc = "Values that can be written to the field `PTSO19`"]
pub enum PTSO19W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO19W::_0 => false,
            PTSO19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO19W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO19W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO19W::_1)
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
#[doc = "Values that can be written to the field `PTSO20`"]
pub enum PTSO20W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO20W::_0 => false,
            PTSO20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO20W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO20W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO20W::_1)
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
#[doc = "Values that can be written to the field `PTSO21`"]
pub enum PTSO21W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO21W::_0 => false,
            PTSO21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO21W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO21W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO21W::_1)
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
#[doc = "Values that can be written to the field `PTSO22`"]
pub enum PTSO22W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO22W::_0 => false,
            PTSO22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO22W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO22W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO22W::_1)
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
#[doc = "Values that can be written to the field `PTSO23`"]
pub enum PTSO23W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO23W::_0 => false,
            PTSO23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO23W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO23W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO23W::_1)
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
#[doc = "Values that can be written to the field `PTSO24`"]
pub enum PTSO24W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO24W::_0 => false,
            PTSO24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO24W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO24W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO24W::_1)
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
#[doc = "Values that can be written to the field `PTSO25`"]
pub enum PTSO25W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO25W::_0 => false,
            PTSO25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO25W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO25W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO25W::_1)
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
#[doc = "Values that can be written to the field `PTSO26`"]
pub enum PTSO26W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO26W::_0 => false,
            PTSO26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO26W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO26W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO26W::_1)
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
#[doc = "Values that can be written to the field `PTSO27`"]
pub enum PTSO27W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO27W::_0 => false,
            PTSO27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO27W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO27W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO27W::_1)
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
#[doc = "Values that can be written to the field `PTSO28`"]
pub enum PTSO28W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO28W::_0 => false,
            PTSO28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO28W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO28W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO28W::_1)
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
#[doc = "Values that can be written to the field `PTSO29`"]
pub enum PTSO29W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO29W::_0 => false,
            PTSO29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO29W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO29W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO29W::_1)
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
#[doc = "Values that can be written to the field `PTSO30`"]
pub enum PTSO30W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO30W::_0 => false,
            PTSO30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO30W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO30W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO30W::_1)
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
#[doc = "Values that can be written to the field `PTSO31`"]
pub enum PTSO31W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    _1,
}
impl PTSO31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTSO31W::_0 => false,
            PTSO31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTSO31W<'a> {
    w: &'a mut W,
}
impl<'a> _PTSO31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTSO31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO31W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO31W::_1)
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
    #[doc = "Bit 0 - Port Set Output"]
    #[inline]
    pub fn ptso0(&mut self) -> _PTSO0W {
        _PTSO0W { w: self }
    }
    #[doc = "Bit 1 - Port Set Output"]
    #[inline]
    pub fn ptso1(&mut self) -> _PTSO1W {
        _PTSO1W { w: self }
    }
    #[doc = "Bit 2 - Port Set Output"]
    #[inline]
    pub fn ptso2(&mut self) -> _PTSO2W {
        _PTSO2W { w: self }
    }
    #[doc = "Bit 3 - Port Set Output"]
    #[inline]
    pub fn ptso3(&mut self) -> _PTSO3W {
        _PTSO3W { w: self }
    }
    #[doc = "Bit 4 - Port Set Output"]
    #[inline]
    pub fn ptso4(&mut self) -> _PTSO4W {
        _PTSO4W { w: self }
    }
    #[doc = "Bit 5 - Port Set Output"]
    #[inline]
    pub fn ptso5(&mut self) -> _PTSO5W {
        _PTSO5W { w: self }
    }
    #[doc = "Bit 6 - Port Set Output"]
    #[inline]
    pub fn ptso6(&mut self) -> _PTSO6W {
        _PTSO6W { w: self }
    }
    #[doc = "Bit 7 - Port Set Output"]
    #[inline]
    pub fn ptso7(&mut self) -> _PTSO7W {
        _PTSO7W { w: self }
    }
    #[doc = "Bit 8 - Port Set Output"]
    #[inline]
    pub fn ptso8(&mut self) -> _PTSO8W {
        _PTSO8W { w: self }
    }
    #[doc = "Bit 9 - Port Set Output"]
    #[inline]
    pub fn ptso9(&mut self) -> _PTSO9W {
        _PTSO9W { w: self }
    }
    #[doc = "Bit 10 - Port Set Output"]
    #[inline]
    pub fn ptso10(&mut self) -> _PTSO10W {
        _PTSO10W { w: self }
    }
    #[doc = "Bit 11 - Port Set Output"]
    #[inline]
    pub fn ptso11(&mut self) -> _PTSO11W {
        _PTSO11W { w: self }
    }
    #[doc = "Bit 12 - Port Set Output"]
    #[inline]
    pub fn ptso12(&mut self) -> _PTSO12W {
        _PTSO12W { w: self }
    }
    #[doc = "Bit 13 - Port Set Output"]
    #[inline]
    pub fn ptso13(&mut self) -> _PTSO13W {
        _PTSO13W { w: self }
    }
    #[doc = "Bit 14 - Port Set Output"]
    #[inline]
    pub fn ptso14(&mut self) -> _PTSO14W {
        _PTSO14W { w: self }
    }
    #[doc = "Bit 15 - Port Set Output"]
    #[inline]
    pub fn ptso15(&mut self) -> _PTSO15W {
        _PTSO15W { w: self }
    }
    #[doc = "Bit 16 - Port Set Output"]
    #[inline]
    pub fn ptso16(&mut self) -> _PTSO16W {
        _PTSO16W { w: self }
    }
    #[doc = "Bit 17 - Port Set Output"]
    #[inline]
    pub fn ptso17(&mut self) -> _PTSO17W {
        _PTSO17W { w: self }
    }
    #[doc = "Bit 18 - Port Set Output"]
    #[inline]
    pub fn ptso18(&mut self) -> _PTSO18W {
        _PTSO18W { w: self }
    }
    #[doc = "Bit 19 - Port Set Output"]
    #[inline]
    pub fn ptso19(&mut self) -> _PTSO19W {
        _PTSO19W { w: self }
    }
    #[doc = "Bit 20 - Port Set Output"]
    #[inline]
    pub fn ptso20(&mut self) -> _PTSO20W {
        _PTSO20W { w: self }
    }
    #[doc = "Bit 21 - Port Set Output"]
    #[inline]
    pub fn ptso21(&mut self) -> _PTSO21W {
        _PTSO21W { w: self }
    }
    #[doc = "Bit 22 - Port Set Output"]
    #[inline]
    pub fn ptso22(&mut self) -> _PTSO22W {
        _PTSO22W { w: self }
    }
    #[doc = "Bit 23 - Port Set Output"]
    #[inline]
    pub fn ptso23(&mut self) -> _PTSO23W {
        _PTSO23W { w: self }
    }
    #[doc = "Bit 24 - Port Set Output"]
    #[inline]
    pub fn ptso24(&mut self) -> _PTSO24W {
        _PTSO24W { w: self }
    }
    #[doc = "Bit 25 - Port Set Output"]
    #[inline]
    pub fn ptso25(&mut self) -> _PTSO25W {
        _PTSO25W { w: self }
    }
    #[doc = "Bit 26 - Port Set Output"]
    #[inline]
    pub fn ptso26(&mut self) -> _PTSO26W {
        _PTSO26W { w: self }
    }
    #[doc = "Bit 27 - Port Set Output"]
    #[inline]
    pub fn ptso27(&mut self) -> _PTSO27W {
        _PTSO27W { w: self }
    }
    #[doc = "Bit 28 - Port Set Output"]
    #[inline]
    pub fn ptso28(&mut self) -> _PTSO28W {
        _PTSO28W { w: self }
    }
    #[doc = "Bit 29 - Port Set Output"]
    #[inline]
    pub fn ptso29(&mut self) -> _PTSO29W {
        _PTSO29W { w: self }
    }
    #[doc = "Bit 30 - Port Set Output"]
    #[inline]
    pub fn ptso30(&mut self) -> _PTSO30W {
        _PTSO30W { w: self }
    }
    #[doc = "Bit 31 - Port Set Output"]
    #[inline]
    pub fn ptso31(&mut self) -> _PTSO31W {
        _PTSO31W { w: self }
    }
}
