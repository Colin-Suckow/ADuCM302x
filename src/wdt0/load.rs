#[doc = "Reader of register LOAD"]
pub type R = crate::R<u16, super::LOAD>;
#[doc = "Writer for register LOAD"]
pub type W = crate::W<u16, super::LOAD>;
#[doc = "Register LOAD `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::LOAD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Load Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Load Value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
