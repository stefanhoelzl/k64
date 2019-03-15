#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PTOR {
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
#[doc = "Values that can be written to the field `PTTO0`"]
pub enum PTTO0W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO0W::_0 => false,
            PTTO0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO0W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO0W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO0W::_1)
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
#[doc = "Values that can be written to the field `PTTO1`"]
pub enum PTTO1W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO1W::_0 => false,
            PTTO1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO1W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO1W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO1W::_1)
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
#[doc = "Values that can be written to the field `PTTO2`"]
pub enum PTTO2W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO2W::_0 => false,
            PTTO2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO2W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO2W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO2W::_1)
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
#[doc = "Values that can be written to the field `PTTO3`"]
pub enum PTTO3W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO3W::_0 => false,
            PTTO3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO3W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO3W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO3W::_1)
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
#[doc = "Values that can be written to the field `PTTO4`"]
pub enum PTTO4W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO4W::_0 => false,
            PTTO4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO4W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO4W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO4W::_1)
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
#[doc = "Values that can be written to the field `PTTO5`"]
pub enum PTTO5W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO5W::_0 => false,
            PTTO5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO5W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO5W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO5W::_1)
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
#[doc = "Values that can be written to the field `PTTO6`"]
pub enum PTTO6W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO6W::_0 => false,
            PTTO6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO6W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO6W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO6W::_1)
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
#[doc = "Values that can be written to the field `PTTO7`"]
pub enum PTTO7W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO7W::_0 => false,
            PTTO7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO7W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO7W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO7W::_1)
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
#[doc = "Values that can be written to the field `PTTO8`"]
pub enum PTTO8W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO8W::_0 => false,
            PTTO8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO8W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO8W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO8W::_1)
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
#[doc = "Values that can be written to the field `PTTO9`"]
pub enum PTTO9W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO9W::_0 => false,
            PTTO9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO9W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO9W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO9W::_1)
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
#[doc = "Values that can be written to the field `PTTO10`"]
pub enum PTTO10W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO10W::_0 => false,
            PTTO10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO10W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO10W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO10W::_1)
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
#[doc = "Values that can be written to the field `PTTO11`"]
pub enum PTTO11W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO11W::_0 => false,
            PTTO11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO11W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO11W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO11W::_1)
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
#[doc = "Values that can be written to the field `PTTO12`"]
pub enum PTTO12W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO12W::_0 => false,
            PTTO12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO12W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO12W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO12W::_1)
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
#[doc = "Values that can be written to the field `PTTO13`"]
pub enum PTTO13W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO13W::_0 => false,
            PTTO13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO13W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO13W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO13W::_1)
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
#[doc = "Values that can be written to the field `PTTO14`"]
pub enum PTTO14W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO14W::_0 => false,
            PTTO14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO14W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO14W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO14W::_1)
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
#[doc = "Values that can be written to the field `PTTO15`"]
pub enum PTTO15W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO15W::_0 => false,
            PTTO15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO15W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO15W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO15W::_1)
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
#[doc = "Values that can be written to the field `PTTO16`"]
pub enum PTTO16W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO16W::_0 => false,
            PTTO16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO16W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO16W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO16W::_1)
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
#[doc = "Values that can be written to the field `PTTO17`"]
pub enum PTTO17W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO17W::_0 => false,
            PTTO17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO17W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO17W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO17W::_1)
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
#[doc = "Values that can be written to the field `PTTO18`"]
pub enum PTTO18W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO18W::_0 => false,
            PTTO18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO18W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO18W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO18W::_1)
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
#[doc = "Values that can be written to the field `PTTO19`"]
pub enum PTTO19W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO19W::_0 => false,
            PTTO19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO19W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO19W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO19W::_1)
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
#[doc = "Values that can be written to the field `PTTO20`"]
pub enum PTTO20W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO20W::_0 => false,
            PTTO20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO20W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO20W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO20W::_1)
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
#[doc = "Values that can be written to the field `PTTO21`"]
pub enum PTTO21W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO21W::_0 => false,
            PTTO21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO21W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO21W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO21W::_1)
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
#[doc = "Values that can be written to the field `PTTO22`"]
pub enum PTTO22W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO22W::_0 => false,
            PTTO22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO22W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO22W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO22W::_1)
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
#[doc = "Values that can be written to the field `PTTO23`"]
pub enum PTTO23W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO23W::_0 => false,
            PTTO23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO23W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO23W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO23W::_1)
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
#[doc = "Values that can be written to the field `PTTO24`"]
pub enum PTTO24W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO24W::_0 => false,
            PTTO24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO24W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO24W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO24W::_1)
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
#[doc = "Values that can be written to the field `PTTO25`"]
pub enum PTTO25W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO25W::_0 => false,
            PTTO25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO25W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO25W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO25W::_1)
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
#[doc = "Values that can be written to the field `PTTO26`"]
pub enum PTTO26W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO26W::_0 => false,
            PTTO26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO26W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO26W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO26W::_1)
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
#[doc = "Values that can be written to the field `PTTO27`"]
pub enum PTTO27W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO27W::_0 => false,
            PTTO27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO27W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO27W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO27W::_1)
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
#[doc = "Values that can be written to the field `PTTO28`"]
pub enum PTTO28W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO28W::_0 => false,
            PTTO28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO28W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO28W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO28W::_1)
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
#[doc = "Values that can be written to the field `PTTO29`"]
pub enum PTTO29W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO29W::_0 => false,
            PTTO29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO29W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO29W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO29W::_1)
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
#[doc = "Values that can be written to the field `PTTO30`"]
pub enum PTTO30W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO30W::_0 => false,
            PTTO30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO30W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO30W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO30W::_1)
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
#[doc = "Values that can be written to the field `PTTO31`"]
pub enum PTTO31W {
    #[doc = "Corresponding bit in PDORn does not change."]
    _0,
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1,
}
impl PTTO31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTTO31W::_0 => false,
            PTTO31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTTO31W<'a> {
    w: &'a mut W,
}
impl<'a> _PTTO31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTTO31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO31W::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO31W::_1)
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
    #[doc = "Bit 0 - Port Toggle Output"]
    #[inline]
    pub fn ptto0(&mut self) -> _PTTO0W {
        _PTTO0W { w: self }
    }
    #[doc = "Bit 1 - Port Toggle Output"]
    #[inline]
    pub fn ptto1(&mut self) -> _PTTO1W {
        _PTTO1W { w: self }
    }
    #[doc = "Bit 2 - Port Toggle Output"]
    #[inline]
    pub fn ptto2(&mut self) -> _PTTO2W {
        _PTTO2W { w: self }
    }
    #[doc = "Bit 3 - Port Toggle Output"]
    #[inline]
    pub fn ptto3(&mut self) -> _PTTO3W {
        _PTTO3W { w: self }
    }
    #[doc = "Bit 4 - Port Toggle Output"]
    #[inline]
    pub fn ptto4(&mut self) -> _PTTO4W {
        _PTTO4W { w: self }
    }
    #[doc = "Bit 5 - Port Toggle Output"]
    #[inline]
    pub fn ptto5(&mut self) -> _PTTO5W {
        _PTTO5W { w: self }
    }
    #[doc = "Bit 6 - Port Toggle Output"]
    #[inline]
    pub fn ptto6(&mut self) -> _PTTO6W {
        _PTTO6W { w: self }
    }
    #[doc = "Bit 7 - Port Toggle Output"]
    #[inline]
    pub fn ptto7(&mut self) -> _PTTO7W {
        _PTTO7W { w: self }
    }
    #[doc = "Bit 8 - Port Toggle Output"]
    #[inline]
    pub fn ptto8(&mut self) -> _PTTO8W {
        _PTTO8W { w: self }
    }
    #[doc = "Bit 9 - Port Toggle Output"]
    #[inline]
    pub fn ptto9(&mut self) -> _PTTO9W {
        _PTTO9W { w: self }
    }
    #[doc = "Bit 10 - Port Toggle Output"]
    #[inline]
    pub fn ptto10(&mut self) -> _PTTO10W {
        _PTTO10W { w: self }
    }
    #[doc = "Bit 11 - Port Toggle Output"]
    #[inline]
    pub fn ptto11(&mut self) -> _PTTO11W {
        _PTTO11W { w: self }
    }
    #[doc = "Bit 12 - Port Toggle Output"]
    #[inline]
    pub fn ptto12(&mut self) -> _PTTO12W {
        _PTTO12W { w: self }
    }
    #[doc = "Bit 13 - Port Toggle Output"]
    #[inline]
    pub fn ptto13(&mut self) -> _PTTO13W {
        _PTTO13W { w: self }
    }
    #[doc = "Bit 14 - Port Toggle Output"]
    #[inline]
    pub fn ptto14(&mut self) -> _PTTO14W {
        _PTTO14W { w: self }
    }
    #[doc = "Bit 15 - Port Toggle Output"]
    #[inline]
    pub fn ptto15(&mut self) -> _PTTO15W {
        _PTTO15W { w: self }
    }
    #[doc = "Bit 16 - Port Toggle Output"]
    #[inline]
    pub fn ptto16(&mut self) -> _PTTO16W {
        _PTTO16W { w: self }
    }
    #[doc = "Bit 17 - Port Toggle Output"]
    #[inline]
    pub fn ptto17(&mut self) -> _PTTO17W {
        _PTTO17W { w: self }
    }
    #[doc = "Bit 18 - Port Toggle Output"]
    #[inline]
    pub fn ptto18(&mut self) -> _PTTO18W {
        _PTTO18W { w: self }
    }
    #[doc = "Bit 19 - Port Toggle Output"]
    #[inline]
    pub fn ptto19(&mut self) -> _PTTO19W {
        _PTTO19W { w: self }
    }
    #[doc = "Bit 20 - Port Toggle Output"]
    #[inline]
    pub fn ptto20(&mut self) -> _PTTO20W {
        _PTTO20W { w: self }
    }
    #[doc = "Bit 21 - Port Toggle Output"]
    #[inline]
    pub fn ptto21(&mut self) -> _PTTO21W {
        _PTTO21W { w: self }
    }
    #[doc = "Bit 22 - Port Toggle Output"]
    #[inline]
    pub fn ptto22(&mut self) -> _PTTO22W {
        _PTTO22W { w: self }
    }
    #[doc = "Bit 23 - Port Toggle Output"]
    #[inline]
    pub fn ptto23(&mut self) -> _PTTO23W {
        _PTTO23W { w: self }
    }
    #[doc = "Bit 24 - Port Toggle Output"]
    #[inline]
    pub fn ptto24(&mut self) -> _PTTO24W {
        _PTTO24W { w: self }
    }
    #[doc = "Bit 25 - Port Toggle Output"]
    #[inline]
    pub fn ptto25(&mut self) -> _PTTO25W {
        _PTTO25W { w: self }
    }
    #[doc = "Bit 26 - Port Toggle Output"]
    #[inline]
    pub fn ptto26(&mut self) -> _PTTO26W {
        _PTTO26W { w: self }
    }
    #[doc = "Bit 27 - Port Toggle Output"]
    #[inline]
    pub fn ptto27(&mut self) -> _PTTO27W {
        _PTTO27W { w: self }
    }
    #[doc = "Bit 28 - Port Toggle Output"]
    #[inline]
    pub fn ptto28(&mut self) -> _PTTO28W {
        _PTTO28W { w: self }
    }
    #[doc = "Bit 29 - Port Toggle Output"]
    #[inline]
    pub fn ptto29(&mut self) -> _PTTO29W {
        _PTTO29W { w: self }
    }
    #[doc = "Bit 30 - Port Toggle Output"]
    #[inline]
    pub fn ptto30(&mut self) -> _PTTO30W {
        _PTTO30W { w: self }
    }
    #[doc = "Bit 31 - Port Toggle Output"]
    #[inline]
    pub fn ptto31(&mut self) -> _PTTO31W {
        _PTTO31W { w: self }
    }
}
