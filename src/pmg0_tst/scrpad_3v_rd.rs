#[doc = "Reader of register SCRPAD_3V_RD"]
pub type R = crate::R<u32, super::SCRPAD_3V_RD>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reading the Scratch Pad Stored in Shutdown Mode"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
