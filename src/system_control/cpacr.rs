#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPACR {
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
#[doc = "Possible values of the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    _00,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    _01,
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    _10,
    #[doc = "Full access."]
    _11,
}
impl CP10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP10R::_00 => 0,
            CP10R::_01 => 1,
            CP10R::_10 => 2,
            CP10R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP10R {
        match value {
            0 => CP10R::_00,
            1 => CP10R::_01,
            2 => CP10R::_10,
            3 => CP10R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CP10R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CP10R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CP10R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CP10R::_11
    }
}
#[doc = "Possible values of the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    _00,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    _01,
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    _10,
    #[doc = "Full access."]
    _11,
}
impl CP11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP11R::_00 => 0,
            CP11R::_01 => 1,
            CP11R::_10 => 2,
            CP11R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP11R {
        match value {
            0 => CP11R::_00,
            1 => CP11R::_01,
            2 => CP11R::_10,
            3 => CP11R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CP11R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CP11R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CP11R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CP11R::_11
    }
}
#[doc = "Values that can be written to the field `CP10`"]
pub enum CP10W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    _00,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    _01,
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    _10,
    #[doc = "Full access."]
    _11,
}
impl CP10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP10W::_00 => 0,
            CP10W::_01 => 1,
            CP10W::_10 => 2,
            CP10W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP10W<'a> {
    w: &'a mut W,
}
impl<'a> _CP10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CP10W::_00)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CP10W::_01)
    }
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CP10W::_10)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CP10W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP11`"]
pub enum CP11W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    _00,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    _01,
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    _10,
    #[doc = "Full access."]
    _11,
}
impl CP11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP11W::_00 => 0,
            CP11W::_01 => 1,
            CP11W::_10 => 2,
            CP11W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP11W<'a> {
    w: &'a mut W,
}
impl<'a> _CP11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CP11W::_00)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CP11W::_01)
    }
    #[doc = "Reserved. The result of any access is UNPREDICTABLE."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CP11W::_10)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CP11W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline]
    pub fn cp10(&self) -> CP10R {
        CP10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline]
    pub fn cp11(&self) -> CP11R {
        CP11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline]
    pub fn cp10(&mut self) -> _CP10W {
        _CP10W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline]
    pub fn cp11(&mut self) -> _CP11W {
        _CP11W { w: self }
    }
}
