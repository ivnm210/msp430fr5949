#[doc = "Reader of register ADC12CTL0"]
pub type R = crate::R<u16, super::ADC12CTL0>;
#[doc = "Writer for register ADC12CTL0"]
pub type W = crate::W<u16, super::ADC12CTL0>;
#[doc = "Register ADC12CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC12CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC12SC`"]
pub type ADC12SC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12SC`"]
pub struct ADC12SC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADC12ENC`"]
pub type ADC12ENC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ENC`"]
pub struct ADC12ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ENC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC12ON`"]
pub type ADC12ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12ON`"]
pub struct ADC12ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC12MSC`"]
pub type ADC12MSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC12MSC`"]
pub struct ADC12MSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12MSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "ADC12 Sample Hold 0 Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHT0_A {
    #[doc = "0: ADC12 Sample Hold 0 Select Bit: 0"]
    ADC12SHT0_0 = 0,
    #[doc = "1: ADC12 Sample Hold 0 Select Bit: 1"]
    ADC12SHT0_1 = 1,
    #[doc = "2: ADC12 Sample Hold 0 Select Bit: 2"]
    ADC12SHT0_2 = 2,
    #[doc = "3: ADC12 Sample Hold 0 Select Bit: 3"]
    ADC12SHT0_3 = 3,
    #[doc = "4: ADC12 Sample Hold 0 Select Bit: 4"]
    ADC12SHT0_4 = 4,
    #[doc = "5: ADC12 Sample Hold 0 Select Bit: 5"]
    ADC12SHT0_5 = 5,
    #[doc = "6: ADC12 Sample Hold 0 Select Bit: 6"]
    ADC12SHT0_6 = 6,
    #[doc = "7: ADC12 Sample Hold 0 Select Bit: 7"]
    ADC12SHT0_7 = 7,
    #[doc = "8: ADC12 Sample Hold 0 Select Bit: 8"]
    ADC12SHT0_8 = 8,
    #[doc = "9: ADC12 Sample Hold 0 Select Bit: 9"]
    ADC12SHT0_9 = 9,
    #[doc = "10: ADC12 Sample Hold 0 Select Bit: 10"]
    ADC12SHT0_10 = 10,
    #[doc = "11: ADC12 Sample Hold 0 Select Bit: 11"]
    ADC12SHT0_11 = 11,
    #[doc = "12: ADC12 Sample Hold 0 Select Bit: 12"]
    ADC12SHT0_12 = 12,
    #[doc = "13: ADC12 Sample Hold 0 Select Bit: 13"]
    ADC12SHT0_13 = 13,
    #[doc = "14: ADC12 Sample Hold 0 Select Bit: 14"]
    ADC12SHT0_14 = 14,
    #[doc = "15: ADC12 Sample Hold 0 Select Bit: 15"]
    ADC12SHT0_15 = 15,
}
impl From<ADC12SHT0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SHT0`"]
pub type ADC12SHT0_R = crate::R<u8, ADC12SHT0_A>;
impl ADC12SHT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHT0_A {
        match self.bits {
            0 => ADC12SHT0_A::ADC12SHT0_0,
            1 => ADC12SHT0_A::ADC12SHT0_1,
            2 => ADC12SHT0_A::ADC12SHT0_2,
            3 => ADC12SHT0_A::ADC12SHT0_3,
            4 => ADC12SHT0_A::ADC12SHT0_4,
            5 => ADC12SHT0_A::ADC12SHT0_5,
            6 => ADC12SHT0_A::ADC12SHT0_6,
            7 => ADC12SHT0_A::ADC12SHT0_7,
            8 => ADC12SHT0_A::ADC12SHT0_8,
            9 => ADC12SHT0_A::ADC12SHT0_9,
            10 => ADC12SHT0_A::ADC12SHT0_10,
            11 => ADC12SHT0_A::ADC12SHT0_11,
            12 => ADC12SHT0_A::ADC12SHT0_12,
            13 => ADC12SHT0_A::ADC12SHT0_13,
            14 => ADC12SHT0_A::ADC12SHT0_14,
            15 => ADC12SHT0_A::ADC12SHT0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_0`"]
    #[inline(always)]
    pub fn is_adc12sht0_0(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_1`"]
    #[inline(always)]
    pub fn is_adc12sht0_1(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_2`"]
    #[inline(always)]
    pub fn is_adc12sht0_2(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_3`"]
    #[inline(always)]
    pub fn is_adc12sht0_3(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_4`"]
    #[inline(always)]
    pub fn is_adc12sht0_4(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_5`"]
    #[inline(always)]
    pub fn is_adc12sht0_5(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_6`"]
    #[inline(always)]
    pub fn is_adc12sht0_6(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_7`"]
    #[inline(always)]
    pub fn is_adc12sht0_7(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_7
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_8`"]
    #[inline(always)]
    pub fn is_adc12sht0_8(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_8
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_9`"]
    #[inline(always)]
    pub fn is_adc12sht0_9(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_9
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_10`"]
    #[inline(always)]
    pub fn is_adc12sht0_10(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_10
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_11`"]
    #[inline(always)]
    pub fn is_adc12sht0_11(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_11
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_12`"]
    #[inline(always)]
    pub fn is_adc12sht0_12(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_12
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_13`"]
    #[inline(always)]
    pub fn is_adc12sht0_13(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_13
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_14`"]
    #[inline(always)]
    pub fn is_adc12sht0_14(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_14
    }
    #[doc = "Checks if the value of the field is `ADC12SHT0_15`"]
    #[inline(always)]
    pub fn is_adc12sht0_15(&self) -> bool {
        *self == ADC12SHT0_A::ADC12SHT0_15
    }
}
#[doc = "Write proxy for field `ADC12SHT0`"]
pub struct ADC12SHT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHT0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0_0(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_0)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 1"]
    #[inline(always)]
    pub fn adc12sht0_1(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_1)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 2"]
    #[inline(always)]
    pub fn adc12sht0_2(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_2)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 3"]
    #[inline(always)]
    pub fn adc12sht0_3(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_3)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 4"]
    #[inline(always)]
    pub fn adc12sht0_4(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_4)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 5"]
    #[inline(always)]
    pub fn adc12sht0_5(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_5)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 6"]
    #[inline(always)]
    pub fn adc12sht0_6(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_6)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 7"]
    #[inline(always)]
    pub fn adc12sht0_7(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_7)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 8"]
    #[inline(always)]
    pub fn adc12sht0_8(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_8)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 9"]
    #[inline(always)]
    pub fn adc12sht0_9(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_9)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 10"]
    #[inline(always)]
    pub fn adc12sht0_10(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_10)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 11"]
    #[inline(always)]
    pub fn adc12sht0_11(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_11)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 12"]
    #[inline(always)]
    pub fn adc12sht0_12(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_12)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 13"]
    #[inline(always)]
    pub fn adc12sht0_13(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_13)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 14"]
    #[inline(always)]
    pub fn adc12sht0_14(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_14)
    }
    #[doc = "ADC12 Sample Hold 0 Select Bit: 15"]
    #[inline(always)]
    pub fn adc12sht0_15(self) -> &'a mut W {
        self.variant(ADC12SHT0_A::ADC12SHT0_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "ADC12 Sample Hold 1 Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SHT1_A {
    #[doc = "0: ADC12 Sample Hold 1 Select Bit: 0"]
    ADC12SHT1_0 = 0,
    #[doc = "1: ADC12 Sample Hold 1 Select Bit: 1"]
    ADC12SHT1_1 = 1,
    #[doc = "2: ADC12 Sample Hold 1 Select Bit: 2"]
    ADC12SHT1_2 = 2,
    #[doc = "3: ADC12 Sample Hold 1 Select Bit: 3"]
    ADC12SHT1_3 = 3,
    #[doc = "4: ADC12 Sample Hold 1 Select Bit: 4"]
    ADC12SHT1_4 = 4,
    #[doc = "5: ADC12 Sample Hold 1 Select Bit: 5"]
    ADC12SHT1_5 = 5,
    #[doc = "6: ADC12 Sample Hold 1 Select Bit: 6"]
    ADC12SHT1_6 = 6,
    #[doc = "7: ADC12 Sample Hold 1 Select Bit: 7"]
    ADC12SHT1_7 = 7,
    #[doc = "8: ADC12 Sample Hold 1 Select Bit: 8"]
    ADC12SHT1_8 = 8,
    #[doc = "9: ADC12 Sample Hold 1 Select Bit: 9"]
    ADC12SHT1_9 = 9,
    #[doc = "10: ADC12 Sample Hold 1 Select Bit: 10"]
    ADC12SHT1_10 = 10,
    #[doc = "11: ADC12 Sample Hold 1 Select Bit: 11"]
    ADC12SHT1_11 = 11,
    #[doc = "12: ADC12 Sample Hold 1 Select Bit: 12"]
    ADC12SHT1_12 = 12,
    #[doc = "13: ADC12 Sample Hold 1 Select Bit: 13"]
    ADC12SHT1_13 = 13,
    #[doc = "14: ADC12 Sample Hold 1 Select Bit: 14"]
    ADC12SHT1_14 = 14,
    #[doc = "15: ADC12 Sample Hold 1 Select Bit: 15"]
    ADC12SHT1_15 = 15,
}
impl From<ADC12SHT1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SHT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC12SHT1`"]
pub type ADC12SHT1_R = crate::R<u8, ADC12SHT1_A>;
impl ADC12SHT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12SHT1_A {
        match self.bits {
            0 => ADC12SHT1_A::ADC12SHT1_0,
            1 => ADC12SHT1_A::ADC12SHT1_1,
            2 => ADC12SHT1_A::ADC12SHT1_2,
            3 => ADC12SHT1_A::ADC12SHT1_3,
            4 => ADC12SHT1_A::ADC12SHT1_4,
            5 => ADC12SHT1_A::ADC12SHT1_5,
            6 => ADC12SHT1_A::ADC12SHT1_6,
            7 => ADC12SHT1_A::ADC12SHT1_7,
            8 => ADC12SHT1_A::ADC12SHT1_8,
            9 => ADC12SHT1_A::ADC12SHT1_9,
            10 => ADC12SHT1_A::ADC12SHT1_10,
            11 => ADC12SHT1_A::ADC12SHT1_11,
            12 => ADC12SHT1_A::ADC12SHT1_12,
            13 => ADC12SHT1_A::ADC12SHT1_13,
            14 => ADC12SHT1_A::ADC12SHT1_14,
            15 => ADC12SHT1_A::ADC12SHT1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_0`"]
    #[inline(always)]
    pub fn is_adc12sht1_0(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_0
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_1`"]
    #[inline(always)]
    pub fn is_adc12sht1_1(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_1
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_2`"]
    #[inline(always)]
    pub fn is_adc12sht1_2(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_2
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_3`"]
    #[inline(always)]
    pub fn is_adc12sht1_3(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_3
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_4`"]
    #[inline(always)]
    pub fn is_adc12sht1_4(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_4
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_5`"]
    #[inline(always)]
    pub fn is_adc12sht1_5(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_5
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_6`"]
    #[inline(always)]
    pub fn is_adc12sht1_6(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_6
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_7`"]
    #[inline(always)]
    pub fn is_adc12sht1_7(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_7
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_8`"]
    #[inline(always)]
    pub fn is_adc12sht1_8(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_8
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_9`"]
    #[inline(always)]
    pub fn is_adc12sht1_9(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_9
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_10`"]
    #[inline(always)]
    pub fn is_adc12sht1_10(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_10
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_11`"]
    #[inline(always)]
    pub fn is_adc12sht1_11(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_11
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_12`"]
    #[inline(always)]
    pub fn is_adc12sht1_12(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_12
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_13`"]
    #[inline(always)]
    pub fn is_adc12sht1_13(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_13
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_14`"]
    #[inline(always)]
    pub fn is_adc12sht1_14(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_14
    }
    #[doc = "Checks if the value of the field is `ADC12SHT1_15`"]
    #[inline(always)]
    pub fn is_adc12sht1_15(&self) -> bool {
        *self == ADC12SHT1_A::ADC12SHT1_15
    }
}
#[doc = "Write proxy for field `ADC12SHT1`"]
pub struct ADC12SHT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SHT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SHT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1_0(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_0)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 1"]
    #[inline(always)]
    pub fn adc12sht1_1(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_1)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 2"]
    #[inline(always)]
    pub fn adc12sht1_2(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_2)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 3"]
    #[inline(always)]
    pub fn adc12sht1_3(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_3)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 4"]
    #[inline(always)]
    pub fn adc12sht1_4(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_4)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 5"]
    #[inline(always)]
    pub fn adc12sht1_5(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_5)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 6"]
    #[inline(always)]
    pub fn adc12sht1_6(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_6)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 7"]
    #[inline(always)]
    pub fn adc12sht1_7(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_7)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 8"]
    #[inline(always)]
    pub fn adc12sht1_8(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_8)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 9"]
    #[inline(always)]
    pub fn adc12sht1_9(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_9)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 10"]
    #[inline(always)]
    pub fn adc12sht1_10(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_10)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 11"]
    #[inline(always)]
    pub fn adc12sht1_11(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_11)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 12"]
    #[inline(always)]
    pub fn adc12sht1_12(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_12)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 13"]
    #[inline(always)]
    pub fn adc12sht1_13(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_13)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 14"]
    #[inline(always)]
    pub fn adc12sht1_14(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_14)
    }
    #[doc = "ADC12 Sample Hold 1 Select Bit: 15"]
    #[inline(always)]
    pub fn adc12sht1_15(self) -> &'a mut W {
        self.variant(ADC12SHT1_A::ADC12SHT1_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&self) -> ADC12SC_R {
        ADC12SC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn adc12enc(&self) -> ADC12ENC_R {
        ADC12ENC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&self) -> ADC12ON_R {
        ADC12ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc12msc(&self) -> ADC12MSC_R {
        ADC12MSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0(&self) -> ADC12SHT0_R {
        ADC12SHT0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1(&self) -> ADC12SHT1_R {
        ADC12SHT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC12 Start Conversion"]
    #[inline(always)]
    pub fn adc12sc(&mut self) -> ADC12SC_W {
        ADC12SC_W { w: self }
    }
    #[doc = "Bit 1 - ADC12 Enable Conversion"]
    #[inline(always)]
    pub fn adc12enc(&mut self) -> ADC12ENC_W {
        ADC12ENC_W { w: self }
    }
    #[doc = "Bit 4 - ADC12 On/enable"]
    #[inline(always)]
    pub fn adc12on(&mut self) -> ADC12ON_W {
        ADC12ON_W { w: self }
    }
    #[doc = "Bit 7 - ADC12 Multiple SampleConversion"]
    #[inline(always)]
    pub fn adc12msc(&mut self) -> ADC12MSC_W {
        ADC12MSC_W { w: self }
    }
    #[doc = "Bits 8:11 - ADC12 Sample Hold 0 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht0(&mut self) -> ADC12SHT0_W {
        ADC12SHT0_W { w: self }
    }
    #[doc = "Bits 12:15 - ADC12 Sample Hold 1 Select Bit: 0"]
    #[inline(always)]
    pub fn adc12sht1(&mut self) -> ADC12SHT1_W {
        ADC12SHT1_W { w: self }
    }
}
