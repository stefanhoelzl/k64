#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C5 {
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
#[doc = "Possible values of the field `PRDIV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRDIV0R {
    #[doc = "Divide Factor is 1"]
    _0,
    #[doc = "Divide Factor is 2"]
    _1,
    #[doc = "Divide Factor is 3"]
    _2,
    #[doc = "Divide Factor is 4"]
    _3,
    #[doc = "Divide Factor is 5"]
    _4,
    #[doc = "Divide Factor is 6"]
    _5,
    #[doc = "Divide Factor is 7"]
    _6,
    #[doc = "Divide Factor is 8"]
    _7,
    #[doc = "Divide Factor is 9"]
    _8,
    #[doc = "Divide Factor is 10"]
    _9,
    #[doc = "Divide Factor is 11"]
    _10,
    #[doc = "Divide Factor is 12"]
    _11,
    #[doc = "Divide Factor is 13"]
    _12,
    #[doc = "Divide Factor is 14"]
    _13,
    #[doc = "Divide Factor is 15"]
    _14,
    #[doc = "Divide Factor is 16"]
    _15,
    #[doc = "Divide Factor is 17"]
    _16,
    #[doc = "Divide Factor is 18"]
    _17,
    #[doc = "Divide Factor is 19"]
    _18,
    #[doc = "Divide Factor is 20"]
    _19,
    #[doc = "Divide Factor is 21"]
    _20,
    #[doc = "Divide Factor is 22"]
    _21,
    #[doc = "Divide Factor is 23"]
    _22,
    #[doc = "Divide Factor is 24"]
    _23,
    #[doc = "Divide Factor is 25"]
    _24,
    #[doc = "Divide Factor is 26"]
    _25,
    #[doc = "Divide Factor is 27"]
    _26,
    #[doc = "Divide Factor is 28"]
    _27,
    #[doc = "Divide Factor is 29"]
    _28,
    #[doc = "Divide Factor is 30"]
    _29,
    #[doc = "Divide Factor is 31"]
    _30,
    #[doc = "Divide Factor is 32"]
    _31,
}
impl PRDIV0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRDIV0R::_0 => 0,
            PRDIV0R::_1 => 1,
            PRDIV0R::_2 => 2,
            PRDIV0R::_3 => 3,
            PRDIV0R::_4 => 4,
            PRDIV0R::_5 => 5,
            PRDIV0R::_6 => 6,
            PRDIV0R::_7 => 7,
            PRDIV0R::_8 => 8,
            PRDIV0R::_9 => 9,
            PRDIV0R::_10 => 10,
            PRDIV0R::_11 => 11,
            PRDIV0R::_12 => 12,
            PRDIV0R::_13 => 13,
            PRDIV0R::_14 => 14,
            PRDIV0R::_15 => 15,
            PRDIV0R::_16 => 16,
            PRDIV0R::_17 => 17,
            PRDIV0R::_18 => 18,
            PRDIV0R::_19 => 19,
            PRDIV0R::_20 => 20,
            PRDIV0R::_21 => 21,
            PRDIV0R::_22 => 22,
            PRDIV0R::_23 => 23,
            PRDIV0R::_24 => 24,
            PRDIV0R::_25 => 25,
            PRDIV0R::_26 => 26,
            PRDIV0R::_27 => 27,
            PRDIV0R::_28 => 28,
            PRDIV0R::_29 => 29,
            PRDIV0R::_30 => 30,
            PRDIV0R::_31 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRDIV0R {
        match value {
            0 => PRDIV0R::_0,
            1 => PRDIV0R::_1,
            2 => PRDIV0R::_2,
            3 => PRDIV0R::_3,
            4 => PRDIV0R::_4,
            5 => PRDIV0R::_5,
            6 => PRDIV0R::_6,
            7 => PRDIV0R::_7,
            8 => PRDIV0R::_8,
            9 => PRDIV0R::_9,
            10 => PRDIV0R::_10,
            11 => PRDIV0R::_11,
            12 => PRDIV0R::_12,
            13 => PRDIV0R::_13,
            14 => PRDIV0R::_14,
            15 => PRDIV0R::_15,
            16 => PRDIV0R::_16,
            17 => PRDIV0R::_17,
            18 => PRDIV0R::_18,
            19 => PRDIV0R::_19,
            20 => PRDIV0R::_20,
            21 => PRDIV0R::_21,
            22 => PRDIV0R::_22,
            23 => PRDIV0R::_23,
            24 => PRDIV0R::_24,
            25 => PRDIV0R::_25,
            26 => PRDIV0R::_26,
            27 => PRDIV0R::_27,
            28 => PRDIV0R::_28,
            29 => PRDIV0R::_29,
            30 => PRDIV0R::_30,
            31 => PRDIV0R::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRDIV0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRDIV0R::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PRDIV0R::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PRDIV0R::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == PRDIV0R::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == PRDIV0R::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == PRDIV0R::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == PRDIV0R::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == PRDIV0R::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == PRDIV0R::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PRDIV0R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PRDIV0R::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == PRDIV0R::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline]
    pub fn is_13(&self) -> bool {
        *self == PRDIV0R::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == PRDIV0R::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == PRDIV0R::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == PRDIV0R::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline]
    pub fn is_17(&self) -> bool {
        *self == PRDIV0R::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == PRDIV0R::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline]
    pub fn is_19(&self) -> bool {
        *self == PRDIV0R::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == PRDIV0R::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline]
    pub fn is_21(&self) -> bool {
        *self == PRDIV0R::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline]
    pub fn is_22(&self) -> bool {
        *self == PRDIV0R::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline]
    pub fn is_23(&self) -> bool {
        *self == PRDIV0R::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == PRDIV0R::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline]
    pub fn is_25(&self) -> bool {
        *self == PRDIV0R::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline]
    pub fn is_26(&self) -> bool {
        *self == PRDIV0R::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline]
    pub fn is_27(&self) -> bool {
        *self == PRDIV0R::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline]
    pub fn is_28(&self) -> bool {
        *self == PRDIV0R::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline]
    pub fn is_29(&self) -> bool {
        *self == PRDIV0R::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == PRDIV0R::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline]
    pub fn is_31(&self) -> bool {
        *self == PRDIV0R::_31
    }
}
#[doc = "Possible values of the field `PLLSTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTEN0R {
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    _0,
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1,
}
impl PLLSTEN0R {
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
            PLLSTEN0R::_0 => false,
            PLLSTEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSTEN0R {
        match value {
            false => PLLSTEN0R::_0,
            true => PLLSTEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLSTEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLSTEN0R::_1
    }
}
#[doc = "Possible values of the field `PLLCLKEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLCLKEN0R {
    #[doc = "MCGPLLCLK is inactive."]
    _0,
    #[doc = "MCGPLLCLK is active."]
    _1,
}
impl PLLCLKEN0R {
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
            PLLCLKEN0R::_0 => false,
            PLLCLKEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLCLKEN0R {
        match value {
            false => PLLCLKEN0R::_0,
            true => PLLCLKEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKEN0R::_1
    }
}
#[doc = "Values that can be written to the field `PRDIV0`"]
pub enum PRDIV0W {
    #[doc = "Divide Factor is 1"]
    _0,
    #[doc = "Divide Factor is 2"]
    _1,
    #[doc = "Divide Factor is 3"]
    _2,
    #[doc = "Divide Factor is 4"]
    _3,
    #[doc = "Divide Factor is 5"]
    _4,
    #[doc = "Divide Factor is 6"]
    _5,
    #[doc = "Divide Factor is 7"]
    _6,
    #[doc = "Divide Factor is 8"]
    _7,
    #[doc = "Divide Factor is 9"]
    _8,
    #[doc = "Divide Factor is 10"]
    _9,
    #[doc = "Divide Factor is 11"]
    _10,
    #[doc = "Divide Factor is 12"]
    _11,
    #[doc = "Divide Factor is 13"]
    _12,
    #[doc = "Divide Factor is 14"]
    _13,
    #[doc = "Divide Factor is 15"]
    _14,
    #[doc = "Divide Factor is 16"]
    _15,
    #[doc = "Divide Factor is 17"]
    _16,
    #[doc = "Divide Factor is 18"]
    _17,
    #[doc = "Divide Factor is 19"]
    _18,
    #[doc = "Divide Factor is 20"]
    _19,
    #[doc = "Divide Factor is 21"]
    _20,
    #[doc = "Divide Factor is 22"]
    _21,
    #[doc = "Divide Factor is 23"]
    _22,
    #[doc = "Divide Factor is 24"]
    _23,
    #[doc = "Divide Factor is 25"]
    _24,
    #[doc = "Divide Factor is 26"]
    _25,
    #[doc = "Divide Factor is 27"]
    _26,
    #[doc = "Divide Factor is 28"]
    _27,
    #[doc = "Divide Factor is 29"]
    _28,
    #[doc = "Divide Factor is 30"]
    _29,
    #[doc = "Divide Factor is 31"]
    _30,
    #[doc = "Divide Factor is 32"]
    _31,
}
impl PRDIV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRDIV0W::_0 => 0,
            PRDIV0W::_1 => 1,
            PRDIV0W::_2 => 2,
            PRDIV0W::_3 => 3,
            PRDIV0W::_4 => 4,
            PRDIV0W::_5 => 5,
            PRDIV0W::_6 => 6,
            PRDIV0W::_7 => 7,
            PRDIV0W::_8 => 8,
            PRDIV0W::_9 => 9,
            PRDIV0W::_10 => 10,
            PRDIV0W::_11 => 11,
            PRDIV0W::_12 => 12,
            PRDIV0W::_13 => 13,
            PRDIV0W::_14 => 14,
            PRDIV0W::_15 => 15,
            PRDIV0W::_16 => 16,
            PRDIV0W::_17 => 17,
            PRDIV0W::_18 => 18,
            PRDIV0W::_19 => 19,
            PRDIV0W::_20 => 20,
            PRDIV0W::_21 => 21,
            PRDIV0W::_22 => 22,
            PRDIV0W::_23 => 23,
            PRDIV0W::_24 => 24,
            PRDIV0W::_25 => 25,
            PRDIV0W::_26 => 26,
            PRDIV0W::_27 => 27,
            PRDIV0W::_28 => 28,
            PRDIV0W::_29 => 29,
            PRDIV0W::_30 => 30,
            PRDIV0W::_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRDIV0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRDIV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRDIV0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide Factor is 1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDIV0W::_0)
    }
    #[doc = "Divide Factor is 2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDIV0W::_1)
    }
    #[doc = "Divide Factor is 3"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDIV0W::_2)
    }
    #[doc = "Divide Factor is 4"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDIV0W::_3)
    }
    #[doc = "Divide Factor is 5"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(PRDIV0W::_4)
    }
    #[doc = "Divide Factor is 6"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(PRDIV0W::_5)
    }
    #[doc = "Divide Factor is 7"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(PRDIV0W::_6)
    }
    #[doc = "Divide Factor is 8"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(PRDIV0W::_7)
    }
    #[doc = "Divide Factor is 9"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(PRDIV0W::_8)
    }
    #[doc = "Divide Factor is 10"]
    #[inline]
    pub fn _9(self) -> &'a mut W {
        self.variant(PRDIV0W::_9)
    }
    #[doc = "Divide Factor is 11"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PRDIV0W::_10)
    }
    #[doc = "Divide Factor is 12"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PRDIV0W::_11)
    }
    #[doc = "Divide Factor is 13"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(PRDIV0W::_12)
    }
    #[doc = "Divide Factor is 14"]
    #[inline]
    pub fn _13(self) -> &'a mut W {
        self.variant(PRDIV0W::_13)
    }
    #[doc = "Divide Factor is 15"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(PRDIV0W::_14)
    }
    #[doc = "Divide Factor is 16"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(PRDIV0W::_15)
    }
    #[doc = "Divide Factor is 17"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(PRDIV0W::_16)
    }
    #[doc = "Divide Factor is 18"]
    #[inline]
    pub fn _17(self) -> &'a mut W {
        self.variant(PRDIV0W::_17)
    }
    #[doc = "Divide Factor is 19"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(PRDIV0W::_18)
    }
    #[doc = "Divide Factor is 20"]
    #[inline]
    pub fn _19(self) -> &'a mut W {
        self.variant(PRDIV0W::_19)
    }
    #[doc = "Divide Factor is 21"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(PRDIV0W::_20)
    }
    #[doc = "Divide Factor is 22"]
    #[inline]
    pub fn _21(self) -> &'a mut W {
        self.variant(PRDIV0W::_21)
    }
    #[doc = "Divide Factor is 23"]
    #[inline]
    pub fn _22(self) -> &'a mut W {
        self.variant(PRDIV0W::_22)
    }
    #[doc = "Divide Factor is 24"]
    #[inline]
    pub fn _23(self) -> &'a mut W {
        self.variant(PRDIV0W::_23)
    }
    #[doc = "Divide Factor is 25"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(PRDIV0W::_24)
    }
    #[doc = "Divide Factor is 26"]
    #[inline]
    pub fn _25(self) -> &'a mut W {
        self.variant(PRDIV0W::_25)
    }
    #[doc = "Divide Factor is 27"]
    #[inline]
    pub fn _26(self) -> &'a mut W {
        self.variant(PRDIV0W::_26)
    }
    #[doc = "Divide Factor is 28"]
    #[inline]
    pub fn _27(self) -> &'a mut W {
        self.variant(PRDIV0W::_27)
    }
    #[doc = "Divide Factor is 29"]
    #[inline]
    pub fn _28(self) -> &'a mut W {
        self.variant(PRDIV0W::_28)
    }
    #[doc = "Divide Factor is 30"]
    #[inline]
    pub fn _29(self) -> &'a mut W {
        self.variant(PRDIV0W::_29)
    }
    #[doc = "Divide Factor is 31"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(PRDIV0W::_30)
    }
    #[doc = "Divide Factor is 32"]
    #[inline]
    pub fn _31(self) -> &'a mut W {
        self.variant(PRDIV0W::_31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSTEN0`"]
pub enum PLLSTEN0W {
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    _0,
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1,
}
impl PLLSTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSTEN0W::_0 => false,
            PLLSTEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN0W::_0)
    }
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN0W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLCLKEN0`"]
pub enum PLLCLKEN0W {
    #[doc = "MCGPLLCLK is inactive."]
    _0,
    #[doc = "MCGPLLCLK is active."]
    _1,
}
impl PLLCLKEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLCLKEN0W::_0 => false,
            PLLCLKEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLCLKEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PLLCLKEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLCLKEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MCGPLLCLK is inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN0W::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN0W::_1)
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
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline]
    pub fn prdiv0(&self) -> PRDIV0R {
        PRDIV0R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline]
    pub fn pllsten0(&self) -> PLLSTEN0R {
        PLLSTEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline]
    pub fn pllclken0(&self) -> PLLCLKEN0R {
        PLLCLKEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline]
    pub fn prdiv0(&mut self) -> _PRDIV0W {
        _PRDIV0W { w: self }
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline]
    pub fn pllsten0(&mut self) -> _PLLSTEN0W {
        _PLLSTEN0W { w: self }
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline]
    pub fn pllclken0(&mut self) -> _PLLCLKEN0W {
        _PLLCLKEN0W { w: self }
    }
}
