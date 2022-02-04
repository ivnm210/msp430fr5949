#[doc = "Reader of register PJOUT"]
pub type R = crate::R<u16, super::PJOUT>;
#[doc = "Writer for register PJOUT"]
pub type W = crate::W<u16, super::PJOUT>;
#[doc = "Register PJOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::PJOUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PJOUT0`"]
pub type PJOUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT0`"]
pub struct PJOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT0_W<'a> {
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
#[doc = "Reader of field `PJOUT1`"]
pub type PJOUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT1`"]
pub struct PJOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT1_W<'a> {
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
#[doc = "Reader of field `PJOUT2`"]
pub type PJOUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT2`"]
pub struct PJOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PJOUT3`"]
pub type PJOUT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT3`"]
pub struct PJOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PJOUT4`"]
pub type PJOUT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT4`"]
pub struct PJOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT4_W<'a> {
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
#[doc = "Reader of field `PJOUT5`"]
pub type PJOUT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT5`"]
pub struct PJOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PJOUT6`"]
pub type PJOUT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT6`"]
pub struct PJOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PJOUT7`"]
pub type PJOUT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PJOUT7`"]
pub struct PJOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PJOUT7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&self) -> PJOUT0_R {
        PJOUT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&self) -> PJOUT1_R {
        PJOUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&self) -> PJOUT2_R {
        PJOUT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&self) -> PJOUT3_R {
        PJOUT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PJOUT4"]
    #[inline(always)]
    pub fn pjout4(&self) -> PJOUT4_R {
        PJOUT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PJOUT5"]
    #[inline(always)]
    pub fn pjout5(&self) -> PJOUT5_R {
        PJOUT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PJOUT6"]
    #[inline(always)]
    pub fn pjout6(&self) -> PJOUT6_R {
        PJOUT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PJOUT7"]
    #[inline(always)]
    pub fn pjout7(&self) -> PJOUT7_R {
        PJOUT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PJOUT0"]
    #[inline(always)]
    pub fn pjout0(&mut self) -> PJOUT0_W {
        PJOUT0_W { w: self }
    }
    #[doc = "Bit 1 - PJOUT1"]
    #[inline(always)]
    pub fn pjout1(&mut self) -> PJOUT1_W {
        PJOUT1_W { w: self }
    }
    #[doc = "Bit 2 - PJOUT2"]
    #[inline(always)]
    pub fn pjout2(&mut self) -> PJOUT2_W {
        PJOUT2_W { w: self }
    }
    #[doc = "Bit 3 - PJOUT3"]
    #[inline(always)]
    pub fn pjout3(&mut self) -> PJOUT3_W {
        PJOUT3_W { w: self }
    }
    #[doc = "Bit 4 - PJOUT4"]
    #[inline(always)]
    pub fn pjout4(&mut self) -> PJOUT4_W {
        PJOUT4_W { w: self }
    }
    #[doc = "Bit 5 - PJOUT5"]
    #[inline(always)]
    pub fn pjout5(&mut self) -> PJOUT5_W {
        PJOUT5_W { w: self }
    }
    #[doc = "Bit 6 - PJOUT6"]
    #[inline(always)]
    pub fn pjout6(&mut self) -> PJOUT6_W {
        PJOUT6_W { w: self }
    }
    #[doc = "Bit 7 - PJOUT7"]
    #[inline(always)]
    pub fn pjout7(&mut self) -> PJOUT7_W {
        PJOUT7_W { w: self }
    }
}
