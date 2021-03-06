#[doc = "Reader of register P3SEL1"]
pub type R = crate::R<u8, super::P3SEL1>;
#[doc = "Writer for register P3SEL1"]
pub type W = crate::W<u8, super::P3SEL1>;
#[doc = "Register P3SEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::P3SEL1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P3SEL1_0`"]
pub type P3SEL1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_0`"]
pub struct P3SEL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_0_W<'a> {
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
#[doc = "Reader of field `P3SEL1_1`"]
pub type P3SEL1_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_1`"]
pub struct P3SEL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_1_W<'a> {
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
#[doc = "Reader of field `P3SEL1_2`"]
pub type P3SEL1_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_2`"]
pub struct P3SEL1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_2_W<'a> {
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
#[doc = "Reader of field `P3SEL1_3`"]
pub type P3SEL1_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_3`"]
pub struct P3SEL1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_3_W<'a> {
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
#[doc = "Reader of field `P3SEL1_4`"]
pub type P3SEL1_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_4`"]
pub struct P3SEL1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_4_W<'a> {
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
#[doc = "Reader of field `P3SEL1_5`"]
pub type P3SEL1_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_5`"]
pub struct P3SEL1_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_5_W<'a> {
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
#[doc = "Reader of field `P3SEL1_6`"]
pub type P3SEL1_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_6`"]
pub struct P3SEL1_6_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `P3SEL1_7`"]
pub type P3SEL1_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P3SEL1_7`"]
pub struct P3SEL1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SEL1_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P3SEL1_0"]
    #[inline(always)]
    pub fn p3sel1_0(&self) -> P3SEL1_0_R {
        P3SEL1_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P3SEL1_1"]
    #[inline(always)]
    pub fn p3sel1_1(&self) -> P3SEL1_1_R {
        P3SEL1_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P3SEL1_2"]
    #[inline(always)]
    pub fn p3sel1_2(&self) -> P3SEL1_2_R {
        P3SEL1_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P3SEL1_3"]
    #[inline(always)]
    pub fn p3sel1_3(&self) -> P3SEL1_3_R {
        P3SEL1_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P3SEL1_4"]
    #[inline(always)]
    pub fn p3sel1_4(&self) -> P3SEL1_4_R {
        P3SEL1_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P3SEL1_5"]
    #[inline(always)]
    pub fn p3sel1_5(&self) -> P3SEL1_5_R {
        P3SEL1_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P3SEL1_6"]
    #[inline(always)]
    pub fn p3sel1_6(&self) -> P3SEL1_6_R {
        P3SEL1_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P3SEL1_7"]
    #[inline(always)]
    pub fn p3sel1_7(&self) -> P3SEL1_7_R {
        P3SEL1_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3SEL1_0"]
    #[inline(always)]
    pub fn p3sel1_0(&mut self) -> P3SEL1_0_W {
        P3SEL1_0_W { w: self }
    }
    #[doc = "Bit 1 - P3SEL1_1"]
    #[inline(always)]
    pub fn p3sel1_1(&mut self) -> P3SEL1_1_W {
        P3SEL1_1_W { w: self }
    }
    #[doc = "Bit 2 - P3SEL1_2"]
    #[inline(always)]
    pub fn p3sel1_2(&mut self) -> P3SEL1_2_W {
        P3SEL1_2_W { w: self }
    }
    #[doc = "Bit 3 - P3SEL1_3"]
    #[inline(always)]
    pub fn p3sel1_3(&mut self) -> P3SEL1_3_W {
        P3SEL1_3_W { w: self }
    }
    #[doc = "Bit 4 - P3SEL1_4"]
    #[inline(always)]
    pub fn p3sel1_4(&mut self) -> P3SEL1_4_W {
        P3SEL1_4_W { w: self }
    }
    #[doc = "Bit 5 - P3SEL1_5"]
    #[inline(always)]
    pub fn p3sel1_5(&mut self) -> P3SEL1_5_W {
        P3SEL1_5_W { w: self }
    }
    #[doc = "Bit 6 - P3SEL1_6"]
    #[inline(always)]
    pub fn p3sel1_6(&mut self) -> P3SEL1_6_W {
        P3SEL1_6_W { w: self }
    }
    #[doc = "Bit 7 - P3SEL1_7"]
    #[inline(always)]
    pub fn p3sel1_7(&mut self) -> P3SEL1_7_W {
        P3SEL1_7_W { w: self }
    }
}
