#[doc = "Reader of register UCA1CTLW1"]
pub type R = crate::R<u16, super::UCA1CTLW1>;
#[doc = "Writer for register UCA1CTLW1"]
pub type W = crate::W<u16, super::UCA1CTLW1>;
#[doc = "Register UCA1CTLW1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA1CTLW1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USCI Deglitch Time Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCGLIT_A {
    #[doc = "0: USCI Deglitch time: 0"]
    UCGLIT_0 = 0,
    #[doc = "1: USCI Deglitch time: 1"]
    UCGLIT_1 = 1,
    #[doc = "2: USCI Deglitch time: 2"]
    UCGLIT_2 = 2,
    #[doc = "3: USCI Deglitch time: 3"]
    UCGLIT_3 = 3,
}
impl From<UCGLIT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCGLIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UCGLIT`"]
pub type UCGLIT_R = crate::R<u8, UCGLIT_A>;
impl UCGLIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGLIT_A {
        match self.bits {
            0 => UCGLIT_A::UCGLIT_0,
            1 => UCGLIT_A::UCGLIT_1,
            2 => UCGLIT_A::UCGLIT_2,
            3 => UCGLIT_A::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_3
    }
}
#[doc = "Write proxy for field `UCGLIT`"]
pub struct UCGLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGLIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCGLIT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USCI Deglitch time: 0"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_0)
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_1)
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_2)
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UCGLIT_W {
        UCGLIT_W { w: self }
    }
}
