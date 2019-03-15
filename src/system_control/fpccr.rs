#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FPCCR {
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
#[doc = "Possible values of the field `LSPACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPACTR {
    #[doc = "Lazy state preservation is not active."]
    _0,
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    _1,
}
impl LSPACTR {
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
            LSPACTR::_0 => false,
            LSPACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPACTR {
        match value {
            false => LSPACTR::_0,
            true => LSPACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LSPACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LSPACTR::_1
    }
}
#[doc = "Possible values of the field `USER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERR {
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    _0,
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    _1,
}
impl USERR {
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
            USERR::_0 => false,
            USERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USERR {
        match value {
            false => USERR::_0,
            true => USERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USERR::_1
    }
}
#[doc = "Possible values of the field `THREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREADR {
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    _0,
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    _1,
}
impl THREADR {
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
            THREADR::_0 => false,
            THREADR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> THREADR {
        match value {
            false => THREADR::_0,
            true => THREADR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == THREADR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == THREADR::_1
    }
}
#[doc = "Possible values of the field `HFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFRDYR {
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    _0,
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    _1,
}
impl HFRDYR {
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
            HFRDYR::_0 => false,
            HFRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFRDYR {
        match value {
            false => HFRDYR::_0,
            true => HFRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HFRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HFRDYR::_1
    }
}
#[doc = "Possible values of the field `MMRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRDYR {
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    _0,
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    _1,
}
impl MMRDYR {
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
            MMRDYR::_0 => false,
            MMRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMRDYR {
        match value {
            false => MMRDYR::_0,
            true => MMRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MMRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MMRDYR::_1
    }
}
#[doc = "Possible values of the field `BFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFRDYR {
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    _0,
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    _1,
}
impl BFRDYR {
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
            BFRDYR::_0 => false,
            BFRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFRDYR {
        match value {
            false => BFRDYR::_0,
            true => BFRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BFRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BFRDYR::_1
    }
}
#[doc = "Possible values of the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYR {
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    _0,
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    _1,
}
impl MONRDYR {
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
            MONRDYR::_0 => false,
            MONRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONRDYR {
        match value {
            false => MONRDYR::_0,
            true => MONRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MONRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MONRDYR::_1
    }
}
#[doc = "Possible values of the field `LSPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPENR {
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    _0,
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    _1,
}
impl LSPENR {
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
            LSPENR::_0 => false,
            LSPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPENR {
        match value {
            false => LSPENR::_0,
            true => LSPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LSPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LSPENR::_1
    }
}
#[doc = "Possible values of the field `ASPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPENR {
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    _0,
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    _1,
}
impl ASPENR {
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
            ASPENR::_0 => false,
            ASPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASPENR {
        match value {
            false => ASPENR::_0,
            true => ASPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASPENR::_1
    }
}
#[doc = "Values that can be written to the field `LSPACT`"]
pub enum LSPACTW {
    #[doc = "Lazy state preservation is not active."]
    _0,
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    _1,
}
impl LSPACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPACTW::_0 => false,
            LSPACTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPACTW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lazy state preservation is not active."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSPACTW::_0)
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSPACTW::_1)
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
#[doc = "Values that can be written to the field `USER`"]
pub enum USERW {
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    _0,
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    _1,
}
impl USERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USERW::_0 => false,
            USERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USERW<'a> {
    w: &'a mut W,
}
impl<'a> _USERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USERW::_0)
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USERW::_1)
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
#[doc = "Values that can be written to the field `THREAD`"]
pub enum THREADW {
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    _0,
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    _1,
}
impl THREADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            THREADW::_0 => false,
            THREADW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THREADW<'a> {
    w: &'a mut W,
}
impl<'a> _THREADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THREADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(THREADW::_0)
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(THREADW::_1)
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
#[doc = "Values that can be written to the field `HFRDY`"]
pub enum HFRDYW {
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    _0,
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    _1,
}
impl HFRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFRDYW::_0 => false,
            HFRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _HFRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HFRDYW::_0)
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HFRDYW::_1)
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
#[doc = "Values that can be written to the field `MMRDY`"]
pub enum MMRDYW {
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    _0,
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    _1,
}
impl MMRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMRDYW::_0 => false,
            MMRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MMRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MMRDYW::_0)
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MMRDYW::_1)
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
#[doc = "Values that can be written to the field `BFRDY`"]
pub enum BFRDYW {
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    _0,
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    _1,
}
impl BFRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFRDYW::_0 => false,
            BFRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BFRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFRDYW::_0)
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFRDYW::_1)
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
#[doc = "Values that can be written to the field `MONRDY`"]
pub enum MONRDYW {
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    _0,
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    _1,
}
impl MONRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYW::_0 => false,
            MONRDYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MONRDYW::_0)
    }
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MONRDYW::_1)
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
#[doc = "Values that can be written to the field `LSPEN`"]
pub enum LSPENW {
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    _0,
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    _1,
}
impl LSPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPENW::_0 => false,
            LSPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSPENW::_0)
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSPENW::_1)
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
#[doc = "Values that can be written to the field `ASPEN`"]
pub enum ASPENW {
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    _0,
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    _1,
}
impl ASPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASPENW::_0 => false,
            ASPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASPENW::_0)
    }
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASPENW::_1)
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
    #[doc = "Bit 0 - Lazy state preservation."]
    #[inline]
    pub fn lspact(&self) -> LSPACTR {
        LSPACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Privilege level when the floating-point stack frame was allocated."]
    #[inline]
    pub fn user(&self) -> USERR {
        USERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Mode when the floating-point stack frame was allocated."]
    #[inline]
    pub fn thread(&self) -> THREADR {
        THREADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn hfrdy(&self) -> HFRDYR {
        HFRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn mmrdy(&self) -> MMRDYR {
        MMRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn bfrdy(&self) -> BFRDYR {
        BFRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Permission to set the MON_PEND when the floating-point stack frame was allocated."]
    #[inline]
    pub fn monrdy(&self) -> MONRDYR {
        MONRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Lazy state preservation for floating-point context."]
    #[inline]
    pub fn lspen(&self) -> LSPENR {
        LSPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit."]
    #[inline]
    pub fn aspen(&self) -> ASPENR {
        ASPENR::_from({
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
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Lazy state preservation."]
    #[inline]
    pub fn lspact(&mut self) -> _LSPACTW {
        _LSPACTW { w: self }
    }
    #[doc = "Bit 1 - Privilege level when the floating-point stack frame was allocated."]
    #[inline]
    pub fn user(&mut self) -> _USERW {
        _USERW { w: self }
    }
    #[doc = "Bit 3 - Mode when the floating-point stack frame was allocated."]
    #[inline]
    pub fn thread(&mut self) -> _THREADW {
        _THREADW { w: self }
    }
    #[doc = "Bit 4 - Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn hfrdy(&mut self) -> _HFRDYW {
        _HFRDYW { w: self }
    }
    #[doc = "Bit 5 - Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn mmrdy(&mut self) -> _MMRDYW {
        _MMRDYW { w: self }
    }
    #[doc = "Bit 6 - Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline]
    pub fn bfrdy(&mut self) -> _BFRDYW {
        _BFRDYW { w: self }
    }
    #[doc = "Bit 8 - Permission to set the MON_PEND when the floating-point stack frame was allocated."]
    #[inline]
    pub fn monrdy(&mut self) -> _MONRDYW {
        _MONRDYW { w: self }
    }
    #[doc = "Bit 30 - Lazy state preservation for floating-point context."]
    #[inline]
    pub fn lspen(&mut self) -> _LSPENW {
        _LSPENW { w: self }
    }
    #[doc = "Bit 31 - Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit."]
    #[inline]
    pub fn aspen(&mut self) -> _ASPENW {
        _ASPENW { w: self }
    }
}
