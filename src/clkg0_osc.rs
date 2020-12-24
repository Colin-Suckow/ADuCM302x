#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Key Protection for CLKG_OSC_CTL"]
    pub key: KEY,
    #[doc = "0x10 - Oscillator Control"]
    pub ctl: CTL,
}
#[doc = "Key Protection for CLKG_OSC_CTL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "Key Protection for CLKG_OSC_CTL"]
pub mod key;
#[doc = "Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Oscillator Control"]
pub mod ctl;
