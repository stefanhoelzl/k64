#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C6 {
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
#[doc = "Possible values of the field `VDIV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDIV0R {
    #[doc = "Multiply Factor is 24"]
    _0,
    #[doc = "Multiply Factor is 25"]
    _1,
    #[doc = "Multiply Factor is 26"]
    _2,
    #[doc = "Multiply Factor is 27"]
    _3,
    #[doc = "Multiply Factor is 28"]
    _4,
    #[doc = "Multiply Factor is 29"]
    _5,
    #[doc = "Multiply Factor is 30"]
    _6,
    #[doc = "Multiply Factor is 31"]
    _7,
    #[doc = "Multiply Factor is 32"]
    _8,
    #[doc = "Multiply Factor is 33"]
    _9,
    #[doc = "Multiply Factor is 34"]
    _10,
    #[doc = "Multiply Factor is 35"]
    _11,
    #[doc = "Multiply Factor is 36"]
    _12,
    #[doc = "Multiply Factor is 37"]
    _13,
    #[doc = "Multiply Factor is 38"]
    _14,
    #[doc = "Multiply Factor is 39"]
    _15,
    #[doc = "Multiply Factor is 40"]
    _16,
    #[doc = "Multiply Factor is 41"]
    _17,
    #[doc = "Multiply Factor is 42"]
    _18,
    #[doc = "Multiply Factor is 43"]
    _19,
    #[doc = "Multiply Factor is 44"]
    _20,
    #[doc = "Multiply Factor is 45"]
    _21,
    #[doc = "Multiply Factor is 46"]
    _22,
    #[doc = "Multiply Factor is 47"]
    _23,
    #[doc = "Multiply Factor is 48"]
    _24,
    #[doc = "Multiply Factor is 49"]
    _25,
    #[doc = "Multiply Factor is 50"]
    _26,
    #[doc = "Multiply Factor is 51"]
    _27,
    #[doc = "Multiply Factor is 52"]
    _28,
    #[doc = "Multiply Factor is 53"]
    _29,
    #[doc = "Multiply Factor is 54"]
    _30,
    #[doc = "Multiply Factor is 55"]
    _31,
}
impl VDIV0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDIV0R::_0 => 0,
            VDIV0R::_1 => 1,
            VDIV0R::_2 => 2,
            VDIV0R::_3 => 3,
            VDIV0R::_4 => 4,
            VDIV0R::_5 => 5,
            VDIV0R::_6 => 6,
            VDIV0R::_7 => 7,
            VDIV0R::_8 => 8,
            VDIV0R::_9 => 9,
            VDIV0R::_10 => 10,
            VDIV0R::_11 => 11,
            VDIV0R::_12 => 12,
            VDIV0R::_13 => 13,
            VDIV0R::_14 => 14,
            VDIV0R::_15 => 15,
            VDIV0R::_16 => 16,
            VDIV0R::_17 => 17,
            VDIV0R::_18 => 18,
            VDIV0R::_19 => 19,
            VDIV0R::_20 => 20,
            VDIV0R::_21 => 21,
            VDIV0R::_22 => 22,
            VDIV0R::_23 => 23,
            VDIV0R::_24 => 24,
            VDIV0R::_25 => 25,
            VDIV0R::_26 => 26,
            VDIV0R::_27 => 27,
            VDIV0R::_28 => 28,
            VDIV0R::_29 => 29,
            VDIV0R::_30 => 30,
            VDIV0R::_31 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDIV0R {
        match value {
            0 => VDIV0R::_0,
            1 => VDIV0R::_1,
            2 => VDIV0R::_2,
            3 => VDIV0R::_3,
            4 => VDIV0R::_4,
            5 => VDIV0R::_5,
            6 => VDIV0R::_6,
            7 => VDIV0R::_7,
            8 => VDIV0R::_8,
            9 => VDIV0R::_9,
            10 => VDIV0R::_10,
            11 => VDIV0R::_11,
            12 => VDIV0R::_12,
            13 => VDIV0R::_13,
            14 => VDIV0R::_14,
            15 => VDIV0R::_15,
            16 => VDIV0R::_16,
            17 => VDIV0R::_17,
            18 => VDIV0R::_18,
            19 => VDIV0R::_19,
            20 => VDIV0R::_20,
            21 => VDIV0R::_21,
            22 => VDIV0R::_22,
            23 => VDIV0R::_23,
            24 => VDIV0R::_24,
            25 => VDIV0R::_25,
            26 => VDIV0R::_26,
            27 => VDIV0R::_27,
            28 => VDIV0R::_28,
            29 => VDIV0R::_29,
            30 => VDIV0R::_30,
            31 => VDIV0R::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VDIV0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VDIV0R::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == VDIV0R::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == VDIV0R::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == VDIV0R::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline]
    pub fn is_5(&self) -> bool {
        *self == VDIV0R::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == VDIV0R::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == VDIV0R::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == VDIV0R::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline]
    pub fn is_9(&self) -> bool {
        *self == VDIV0R::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == VDIV0R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == VDIV0R::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline]
    pub fn is_12(&self) -> bool {
        *self == VDIV0R::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline]
    pub fn is_13(&self) -> bool {
        *self == VDIV0R::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline]
    pub fn is_14(&self) -> bool {
        *self == VDIV0R::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline]
    pub fn is_15(&self) -> bool {
        *self == VDIV0R::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == VDIV0R::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline]
    pub fn is_17(&self) -> bool {
        *self == VDIV0R::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline]
    pub fn is_18(&self) -> bool {
        *self == VDIV0R::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline]
    pub fn is_19(&self) -> bool {
        *self == VDIV0R::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline]
    pub fn is_20(&self) -> bool {
        *self == VDIV0R::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline]
    pub fn is_21(&self) -> bool {
        *self == VDIV0R::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline]
    pub fn is_22(&self) -> bool {
        *self == VDIV0R::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline]
    pub fn is_23(&self) -> bool {
        *self == VDIV0R::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == VDIV0R::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline]
    pub fn is_25(&self) -> bool {
        *self == VDIV0R::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline]
    pub fn is_26(&self) -> bool {
        *self == VDIV0R::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline]
    pub fn is_27(&self) -> bool {
        *self == VDIV0R::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline]
    pub fn is_28(&self) -> bool {
        *self == VDIV0R::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline]
    pub fn is_29(&self) -> bool {
        *self == VDIV0R::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == VDIV0R::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline]
    pub fn is_31(&self) -> bool {
        *self == VDIV0R::_31
    }
}
#[doc = "Possible values of the field `CME0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME0R {
    #[doc = "External clock monitor is disabled for OSC0."]
    _0,
    #[doc = "External clock monitor is enabled for OSC0."]
    _1,
}
impl CME0R {
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
            CME0R::_0 => false,
            CME0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CME0R {
        match value {
            false => CME0R::_0,
            true => CME0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CME0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CME0R::_1
    }
}
#[doc = "Possible values of the field `PLLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSR {
    #[doc = "FLL is selected."]
    _0,
    #[doc = "PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    _1,
}
impl PLLSR {
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
            PLLSR::_0 => false,
            PLLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSR {
        match value {
            false => PLLSR::_0,
            true => PLLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLSR::_1
    }
}
#[doc = "Possible values of the field `LOLIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLIE0R {
    #[doc = "No interrupt request is generated on loss of lock."]
    _0,
    #[doc = "Generate an interrupt request on loss of lock."]
    _1,
}
impl LOLIE0R {
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
            LOLIE0R::_0 => false,
            LOLIE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOLIE0R {
        match value {
            false => LOLIE0R::_0,
            true => LOLIE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOLIE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOLIE0R::_1
    }
}
#[doc = "Values that can be written to the field `VDIV0`"]
pub enum VDIV0W {
    #[doc = "Multiply Factor is 24"]
    _0,
    #[doc = "Multiply Factor is 25"]
    _1,
    #[doc = "Multiply Factor is 26"]
    _2,
    #[doc = "Multiply Factor is 27"]
    _3,
    #[doc = "Multiply Factor is 28"]
    _4,
    #[doc = "Multiply Factor is 29"]
    _5,
    #[doc = "Multiply Factor is 30"]
    _6,
    #[doc = "Multiply Factor is 31"]
    _7,
    #[doc = "Multiply Factor is 32"]
    _8,
    #[doc = "Multiply Factor is 33"]
    _9,
    #[doc = "Multiply Factor is 34"]
    _10,
    #[doc = "Multiply Factor is 35"]
    _11,
    #[doc = "Multiply Factor is 36"]
    _12,
    #[doc = "Multiply Factor is 37"]
    _13,
    #[doc = "Multiply Factor is 38"]
    _14,
    #[doc = "Multiply Factor is 39"]
    _15,
    #[doc = "Multiply Factor is 40"]
    _16,
    #[doc = "Multiply Factor is 41"]
    _17,
    #[doc = "Multiply Factor is 42"]
    _18,
    #[doc = "Multiply Factor is 43"]
    _19,
    #[doc = "Multiply Factor is 44"]
    _20,
    #[doc = "Multiply Factor is 45"]
    _21,
    #[doc = "Multiply Factor is 46"]
    _22,
    #[doc = "Multiply Factor is 47"]
    _23,
    #[doc = "Multiply Factor is 48"]
    _24,
    #[doc = "Multiply Factor is 49"]
    _25,
    #[doc = "Multiply Factor is 50"]
    _26,
    #[doc = "Multiply Factor is 51"]
    _27,
    #[doc = "Multiply Factor is 52"]
    _28,
    #[doc = "Multiply Factor is 53"]
    _29,
    #[doc = "Multiply Factor is 54"]
    _30,
    #[doc = "Multiply Factor is 55"]
    _31,
}
impl VDIV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VDIV0W::_0 => 0,
            VDIV0W::_1 => 1,
            VDIV0W::_2 => 2,
            VDIV0W::_3 => 3,
            VDIV0W::_4 => 4,
            VDIV0W::_5 => 5,
            VDIV0W::_6 => 6,
            VDIV0W::_7 => 7,
            VDIV0W::_8 => 8,
            VDIV0W::_9 => 9,
            VDIV0W::_10 => 10,
            VDIV0W::_11 => 11,
            VDIV0W::_12 => 12,
            VDIV0W::_13 => 13,
            VDIV0W::_14 => 14,
            VDIV0W::_15 => 15,
            VDIV0W::_16 => 16,
            VDIV0W::_17 => 17,
            VDIV0W::_18 => 18,
            VDIV0W::_19 => 19,
            VDIV0W::_20 => 20,
            VDIV0W::_21 => 21,
            VDIV0W::_22 => 22,
            VDIV0W::_23 => 23,
            VDIV0W::_24 => 24,
            VDIV0W::_25 => 25,
            VDIV0W::_26 => 26,
            VDIV0W::_27 => 27,
            VDIV0W::_28 => 28,
            VDIV0W::_29 => 29,
            VDIV0W::_30 => 30,
            VDIV0W::_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDIV0W<'a> {
    w: &'a mut W,
}
impl<'a> _VDIV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDIV0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Multiply Factor is 24"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDIV0W::_0)
    }
    #[doc = "Multiply Factor is 25"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDIV0W::_1)
    }
    #[doc = "Multiply Factor is 26"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(VDIV0W::_2)
    }
    #[doc = "Multiply Factor is 27"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(VDIV0W::_3)
    }
    #[doc = "Multiply Factor is 28"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(VDIV0W::_4)
    }
    #[doc = "Multiply Factor is 29"]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(VDIV0W::_5)
    }
    #[doc = "Multiply Factor is 30"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(VDIV0W::_6)
    }
    #[doc = "Multiply Factor is 31"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(VDIV0W::_7)
    }
    #[doc = "Multiply Factor is 32"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(VDIV0W::_8)
    }
    #[doc = "Multiply Factor is 33"]
    #[inline]
    pub fn _9(self) -> &'a mut W {
        self.variant(VDIV0W::_9)
    }
    #[doc = "Multiply Factor is 34"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(VDIV0W::_10)
    }
    #[doc = "Multiply Factor is 35"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(VDIV0W::_11)
    }
    #[doc = "Multiply Factor is 36"]
    #[inline]
    pub fn _12(self) -> &'a mut W {
        self.variant(VDIV0W::_12)
    }
    #[doc = "Multiply Factor is 37"]
    #[inline]
    pub fn _13(self) -> &'a mut W {
        self.variant(VDIV0W::_13)
    }
    #[doc = "Multiply Factor is 38"]
    #[inline]
    pub fn _14(self) -> &'a mut W {
        self.variant(VDIV0W::_14)
    }
    #[doc = "Multiply Factor is 39"]
    #[inline]
    pub fn _15(self) -> &'a mut W {
        self.variant(VDIV0W::_15)
    }
    #[doc = "Multiply Factor is 40"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(VDIV0W::_16)
    }
    #[doc = "Multiply Factor is 41"]
    #[inline]
    pub fn _17(self) -> &'a mut W {
        self.variant(VDIV0W::_17)
    }
    #[doc = "Multiply Factor is 42"]
    #[inline]
    pub fn _18(self) -> &'a mut W {
        self.variant(VDIV0W::_18)
    }
    #[doc = "Multiply Factor is 43"]
    #[inline]
    pub fn _19(self) -> &'a mut W {
        self.variant(VDIV0W::_19)
    }
    #[doc = "Multiply Factor is 44"]
    #[inline]
    pub fn _20(self) -> &'a mut W {
        self.variant(VDIV0W::_20)
    }
    #[doc = "Multiply Factor is 45"]
    #[inline]
    pub fn _21(self) -> &'a mut W {
        self.variant(VDIV0W::_21)
    }
    #[doc = "Multiply Factor is 46"]
    #[inline]
    pub fn _22(self) -> &'a mut W {
        self.variant(VDIV0W::_22)
    }
    #[doc = "Multiply Factor is 47"]
    #[inline]
    pub fn _23(self) -> &'a mut W {
        self.variant(VDIV0W::_23)
    }
    #[doc = "Multiply Factor is 48"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(VDIV0W::_24)
    }
    #[doc = "Multiply Factor is 49"]
    #[inline]
    pub fn _25(self) -> &'a mut W {
        self.variant(VDIV0W::_25)
    }
    #[doc = "Multiply Factor is 50"]
    #[inline]
    pub fn _26(self) -> &'a mut W {
        self.variant(VDIV0W::_26)
    }
    #[doc = "Multiply Factor is 51"]
    #[inline]
    pub fn _27(self) -> &'a mut W {
        self.variant(VDIV0W::_27)
    }
    #[doc = "Multiply Factor is 52"]
    #[inline]
    pub fn _28(self) -> &'a mut W {
        self.variant(VDIV0W::_28)
    }
    #[doc = "Multiply Factor is 53"]
    #[inline]
    pub fn _29(self) -> &'a mut W {
        self.variant(VDIV0W::_29)
    }
    #[doc = "Multiply Factor is 54"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(VDIV0W::_30)
    }
    #[doc = "Multiply Factor is 55"]
    #[inline]
    pub fn _31(self) -> &'a mut W {
        self.variant(VDIV0W::_31)
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
#[doc = "Values that can be written to the field `CME0`"]
pub enum CME0W {
    #[doc = "External clock monitor is disabled for OSC0."]
    _0,
    #[doc = "External clock monitor is enabled for OSC0."]
    _1,
}
impl CME0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CME0W::_0 => false,
            CME0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CME0W<'a> {
    w: &'a mut W,
}
impl<'a> _CME0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CME0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External clock monitor is disabled for OSC0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME0W::_0)
    }
    #[doc = "External clock monitor is enabled for OSC0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME0W::_1)
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
#[doc = "Values that can be written to the field `PLLS`"]
pub enum PLLSW {
    #[doc = "FLL is selected."]
    _0,
    #[doc = "PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    _1,
}
impl PLLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSW::_0 => false,
            PLLSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FLL is selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSW::_0)
    }
    #[doc = "PLL is selected (PRDIV 0 need to be programmed to the correct divider to generate a PLL reference clock in the range of 2-4 MHz prior to setting the PLLS bit)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSW::_1)
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
#[doc = "Values that can be written to the field `LOLIE0`"]
pub enum LOLIE0W {
    #[doc = "No interrupt request is generated on loss of lock."]
    _0,
    #[doc = "Generate an interrupt request on loss of lock."]
    _1,
}
impl LOLIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOLIE0W::_0 => false,
            LOLIE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOLIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOLIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOLIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt request is generated on loss of lock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLIE0W::_0)
    }
    #[doc = "Generate an interrupt request on loss of lock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLIE0W::_1)
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
    #[doc = "Bits 0:4 - VCO 0 Divider"]
    #[inline]
    pub fn vdiv0(&self) -> VDIV0R {
        VDIV0R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline]
    pub fn cme0(&self) -> CME0R {
        CME0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline]
    pub fn plls(&self) -> PLLSR {
        PLLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline]
    pub fn lolie0(&self) -> LOLIE0R {
        LOLIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:4 - VCO 0 Divider"]
    #[inline]
    pub fn vdiv0(&mut self) -> _VDIV0W {
        _VDIV0W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline]
    pub fn cme0(&mut self) -> _CME0W {
        _CME0W { w: self }
    }
    #[doc = "Bit 6 - PLL Select"]
    #[inline]
    pub fn plls(&mut self) -> _PLLSW {
        _PLLSW { w: self }
    }
    #[doc = "Bit 7 - Loss of Lock Interrrupt Enable"]
    #[inline]
    pub fn lolie0(&mut self) -> _LOLIE0W {
        _LOLIE0W { w: self }
    }
}
