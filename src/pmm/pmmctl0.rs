#[doc = "Reader of register PMMCTL0"]
pub type R = crate::R<u16, super::PMMCTL0>;
#[doc = "Writer for register PMMCTL0"]
pub type W = crate::W<u16, super::PMMCTL0>;
#[doc = "Register PMMCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PMMSWBOR`"]
pub type PMMSWBOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMSWBOR`"]
pub struct PMMSWBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWBOR_W<'a> {
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
#[doc = "Reader of field `PMMSWPOR`"]
pub type PMMSWPOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMSWPOR`"]
pub struct PMMSWPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMSWPOR_W<'a> {
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
#[doc = "Reader of field `PMMREGOFF`"]
pub type PMMREGOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMREGOFF`"]
pub struct PMMREGOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMREGOFF_W<'a> {
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
#[doc = "Reader of field `SVSHE`"]
pub type SVSHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SVSHE`"]
pub struct SVSHE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSHE_W<'a> {
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
#[doc = "Reader of field `PMMLPRST`"]
pub type PMMLPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMMLPRST`"]
pub struct PMMLPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PMMLPRST_W<'a> {
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
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMM Low-Power Reset Enable"]
    #[inline(always)]
    pub fn pmmlprst(&self) -> PMMLPRST_R {
        PMMLPRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W {
        PMMSWBOR_W { w: self }
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W {
        PMMSWPOR_W { w: self }
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W {
        PMMREGOFF_W { w: self }
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W {
        SVSHE_W { w: self }
    }
    #[doc = "Bit 7 - PMM Low-Power Reset Enable"]
    #[inline(always)]
    pub fn pmmlprst(&mut self) -> PMMLPRST_W {
        PMMLPRST_W { w: self }
    }
}
