#[doc = "Reader of register POLY"]
pub type R = crate::R<u32, super::POLY>;
#[doc = "Writer for register POLY"]
pub type W = crate::W<u32, super::POLY>;
#[doc = "Register POLY `reset()`'s with value 0x04c1_1db7"]
impl crate::ResetValue for super::POLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04c1_1db7
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Reduction Polynomial"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Reduction Polynomial"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
