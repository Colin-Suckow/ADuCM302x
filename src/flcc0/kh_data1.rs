#[doc = "Reader of register KH_DATA1"]
pub type R = crate::R<u32, super::KH_DATA1>;
#[doc = "Writer for register KH_DATA1"]
pub type W = crate::W<u32, super::KH_DATA1>;
#[doc = "Register KH_DATA1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::KH_DATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
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
    #[doc = "Bits 0:31 - Upper Half of 64-bit Dualword Data to Be Written"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper Half of 64-bit Dualword Data to Be Written"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
