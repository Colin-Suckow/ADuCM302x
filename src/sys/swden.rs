#[doc = "Writer for register SWDEN"]
pub type W = crate::W<u16, super::SWDEN>;
#[doc = "Register SWDEN `reset()`'s with value 0x7072"]
impl crate::ResetValue for super::SWDEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7072
    }
}
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
impl W {
    #[doc = "Bits 0:15 - SWD Interface Enable"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
