#[doc = "Reader of register UCA1CTL0"]
pub type R = crate::R<u8, super::UCA1CTL0>;
#[doc = "Writer for register UCA1CTL0"]
pub type W = crate::W<u8, super::UCA1CTL0>;
#[doc = "Register UCA1CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA1CTL0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCSSELx`"]
pub type UCSSELX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCSSELx`"]
pub struct UCSSELX_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSSELX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `UCRXEIE`"]
pub type UCRXEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCRXEIE`"]
pub struct UCRXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UCBRKIE`"]
pub type UCBRKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCBRKIE`"]
pub struct UCBRKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRKIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UCDORM`"]
pub type UCDORM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCDORM`"]
pub struct UCDORM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDORM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UCTXADDR`"]
pub type UCTXADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXADDR`"]
pub struct UCTXADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXADDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UCTXBRK`"]
pub type UCTXBRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCTXBRK`"]
pub struct UCTXBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXBRK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UCSWRST`"]
pub type UCSWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCSWRST`"]
pub struct UCSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - BRCLK clock source"]
    #[inline(always)]
    pub fn ucsselx(&self) -> UCSSELX_R {
        UCSSELX_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Receive err char interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UCRXEIE_R {
        UCRXEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive break char int enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UCBRKIE_R {
        UCBRKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - puts eUSCI in sleep mode"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UCDORM_R {
        UCDORM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit address, the next frame is marked as address"]
    #[inline(always)]
    pub fn uctxaddr(&self) -> UCTXADDR_R {
        UCTXADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&self) -> UCTXBRK_R {
        UCTXBRK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - BRCLK clock source"]
    #[inline(always)]
    pub fn ucsselx(&mut self) -> UCSSELX_W {
        UCSSELX_W { w: self }
    }
    #[doc = "Bit 5 - Receive err char interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UCRXEIE_W {
        UCRXEIE_W { w: self }
    }
    #[doc = "Bit 4 - Receive break char int enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UCBRKIE_W {
        UCBRKIE_W { w: self }
    }
    #[doc = "Bit 3 - puts eUSCI in sleep mode"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UCDORM_W {
        UCDORM_W { w: self }
    }
    #[doc = "Bit 2 - Transmit address, the next frame is marked as address"]
    #[inline(always)]
    pub fn uctxaddr(&mut self) -> UCTXADDR_W {
        UCTXADDR_W { w: self }
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&mut self) -> UCTXBRK_W {
        UCTXBRK_W { w: self }
    }
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W {
        UCSWRST_W { w: self }
    }
}
