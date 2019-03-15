#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPCHR {
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
#[doc = r" Proxy"]
pub struct _GPWDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPWDW<'a> {
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
#[doc = "Values that can be written to the field `GPWE0`"]
pub enum GPWE0W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE0W::_0 => false,
            GPWE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE0W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE0W::_1)
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
#[doc = "Values that can be written to the field `GPWE1`"]
pub enum GPWE1W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE1W::_0 => false,
            GPWE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE1W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE1W::_1)
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
#[doc = "Values that can be written to the field `GPWE2`"]
pub enum GPWE2W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE2W::_0 => false,
            GPWE2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE2W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE2W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE2W::_1)
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
#[doc = "Values that can be written to the field `GPWE3`"]
pub enum GPWE3W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE3W::_0 => false,
            GPWE3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE3W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE3W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE3W::_1)
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
#[doc = "Values that can be written to the field `GPWE4`"]
pub enum GPWE4W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE4W::_0 => false,
            GPWE4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE4W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE4W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE4W::_1)
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
#[doc = "Values that can be written to the field `GPWE5`"]
pub enum GPWE5W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE5W::_0 => false,
            GPWE5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE5W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE5W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE5W::_1)
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
#[doc = "Values that can be written to the field `GPWE6`"]
pub enum GPWE6W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE6W::_0 => false,
            GPWE6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE6W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE6W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE6W::_1)
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
#[doc = "Values that can be written to the field `GPWE7`"]
pub enum GPWE7W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE7W::_0 => false,
            GPWE7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE7W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE7W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE7W::_1)
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
#[doc = "Values that can be written to the field `GPWE8`"]
pub enum GPWE8W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE8W::_0 => false,
            GPWE8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE8W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE8W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE8W::_1)
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
#[doc = "Values that can be written to the field `GPWE9`"]
pub enum GPWE9W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE9W::_0 => false,
            GPWE9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE9W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE9W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE9W::_1)
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
#[doc = "Values that can be written to the field `GPWE10`"]
pub enum GPWE10W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE10W::_0 => false,
            GPWE10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE10W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE10W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE10W::_1)
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
#[doc = "Values that can be written to the field `GPWE11`"]
pub enum GPWE11W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE11W::_0 => false,
            GPWE11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE11W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE11W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE11W::_1)
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
#[doc = "Values that can be written to the field `GPWE12`"]
pub enum GPWE12W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE12W::_0 => false,
            GPWE12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE12W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE12W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE12W::_1)
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
#[doc = "Values that can be written to the field `GPWE13`"]
pub enum GPWE13W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE13W::_0 => false,
            GPWE13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE13W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE13W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE13W::_1)
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
#[doc = "Values that can be written to the field `GPWE14`"]
pub enum GPWE14W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE14W::_0 => false,
            GPWE14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE14W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE14W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE14W::_1)
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
#[doc = "Values that can be written to the field `GPWE15`"]
pub enum GPWE15W {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GPWE15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPWE15W::_0 => false,
            GPWE15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPWE15W<'a> {
    w: &'a mut W,
}
impl<'a> _GPWE15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPWE15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE15W::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE15W::_1)
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
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline]
    pub fn gpwd(&mut self) -> _GPWDW {
        _GPWDW { w: self }
    }
    #[doc = "Bit 16 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe0(&mut self) -> _GPWE0W {
        _GPWE0W { w: self }
    }
    #[doc = "Bit 17 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe1(&mut self) -> _GPWE1W {
        _GPWE1W { w: self }
    }
    #[doc = "Bit 18 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe2(&mut self) -> _GPWE2W {
        _GPWE2W { w: self }
    }
    #[doc = "Bit 19 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe3(&mut self) -> _GPWE3W {
        _GPWE3W { w: self }
    }
    #[doc = "Bit 20 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe4(&mut self) -> _GPWE4W {
        _GPWE4W { w: self }
    }
    #[doc = "Bit 21 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe5(&mut self) -> _GPWE5W {
        _GPWE5W { w: self }
    }
    #[doc = "Bit 22 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe6(&mut self) -> _GPWE6W {
        _GPWE6W { w: self }
    }
    #[doc = "Bit 23 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe7(&mut self) -> _GPWE7W {
        _GPWE7W { w: self }
    }
    #[doc = "Bit 24 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe8(&mut self) -> _GPWE8W {
        _GPWE8W { w: self }
    }
    #[doc = "Bit 25 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe9(&mut self) -> _GPWE9W {
        _GPWE9W { w: self }
    }
    #[doc = "Bit 26 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe10(&mut self) -> _GPWE10W {
        _GPWE10W { w: self }
    }
    #[doc = "Bit 27 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe11(&mut self) -> _GPWE11W {
        _GPWE11W { w: self }
    }
    #[doc = "Bit 28 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe12(&mut self) -> _GPWE12W {
        _GPWE12W { w: self }
    }
    #[doc = "Bit 29 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe13(&mut self) -> _GPWE13W {
        _GPWE13W { w: self }
    }
    #[doc = "Bit 30 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe14(&mut self) -> _GPWE14W {
        _GPWE14W { w: self }
    }
    #[doc = "Bit 31 - Global Pin Write Enable"]
    #[inline]
    pub fn gpwe15(&mut self) -> _GPWE15W {
        _GPWE15W { w: self }
    }
}
