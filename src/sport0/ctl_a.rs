#[doc = "Reader of register CTL_A"]
pub type R = crate::R<u32, super::CTL_A>;
#[doc = "Writer for register CTL_A"]
pub type W = crate::W<u32, super::CTL_A>;
#[doc = "Register CTL_A `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL_A {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Serial Port Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEN_A {
    #[doc = "0: Disable"]
    CTL_DIS = 0,
    #[doc = "1: Enable"]
    CTL_EN = 1,
}
impl From<SPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPEN`"]
pub type SPEN_R = crate::R<bool, SPEN_A>;
impl SPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEN_A {
        match self.bits {
            false => SPEN_A::CTL_DIS,
            true => SPEN_A::CTL_EN,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_DIS`"]
    #[inline(always)]
    pub fn is_ctl_dis(&self) -> bool {
        *self == SPEN_A::CTL_DIS
    }
    #[doc = "Checks if the value of the field is `CTL_EN`"]
    #[inline(always)]
    pub fn is_ctl_en(&self) -> bool {
        *self == SPEN_A::CTL_EN
    }
}
#[doc = "Write proxy for field `SPEN`"]
pub struct SPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ctl_dis(self) -> &'a mut W {
        self.variant(SPEN_A::CTL_DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ctl_en(self) -> &'a mut W {
        self.variant(SPEN_A::CTL_EN)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Frame Sync Multiplexer Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMUXSEL_A {
    #[doc = "0: Disable frame sync multiplexing"]
    CTL_FS_MUX_DIS = 0,
    #[doc = "1: Enable frame sync multiplexing"]
    CTL_FS_MUX_EN = 1,
}
impl From<FSMUXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FSMUXSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSMUXSEL`"]
pub type FSMUXSEL_R = crate::R<bool, FSMUXSEL_A>;
impl FSMUXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSMUXSEL_A {
        match self.bits {
            false => FSMUXSEL_A::CTL_FS_MUX_DIS,
            true => FSMUXSEL_A::CTL_FS_MUX_EN,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_FS_MUX_DIS`"]
    #[inline(always)]
    pub fn is_ctl_fs_mux_dis(&self) -> bool {
        *self == FSMUXSEL_A::CTL_FS_MUX_DIS
    }
    #[doc = "Checks if the value of the field is `CTL_FS_MUX_EN`"]
    #[inline(always)]
    pub fn is_ctl_fs_mux_en(&self) -> bool {
        *self == FSMUXSEL_A::CTL_FS_MUX_EN
    }
}
#[doc = "Write proxy for field `FSMUXSEL`"]
pub struct FSMUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMUXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSMUXSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable frame sync multiplexing"]
    #[inline(always)]
    pub fn ctl_fs_mux_dis(self) -> &'a mut W {
        self.variant(FSMUXSEL_A::CTL_FS_MUX_DIS)
    }
    #[doc = "Enable frame sync multiplexing"]
    #[inline(always)]
    pub fn ctl_fs_mux_en(self) -> &'a mut W {
        self.variant(FSMUXSEL_A::CTL_FS_MUX_EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clock Multiplexer Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKMUXSEL_A {
    #[doc = "0: Disable serial clock multiplexing"]
    CTL_CLK_MUX_DIS = 0,
    #[doc = "1: Enable serial clock multiplexing"]
    CTL_CLK_MUX_EN = 1,
}
impl From<CKMUXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKMUXSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKMUXSEL`"]
pub type CKMUXSEL_R = crate::R<bool, CKMUXSEL_A>;
impl CKMUXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMUXSEL_A {
        match self.bits {
            false => CKMUXSEL_A::CTL_CLK_MUX_DIS,
            true => CKMUXSEL_A::CTL_CLK_MUX_EN,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_CLK_MUX_DIS`"]
    #[inline(always)]
    pub fn is_ctl_clk_mux_dis(&self) -> bool {
        *self == CKMUXSEL_A::CTL_CLK_MUX_DIS
    }
    #[doc = "Checks if the value of the field is `CTL_CLK_MUX_EN`"]
    #[inline(always)]
    pub fn is_ctl_clk_mux_en(&self) -> bool {
        *self == CKMUXSEL_A::CTL_CLK_MUX_EN
    }
}
#[doc = "Write proxy for field `CKMUXSEL`"]
pub struct CKMUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMUXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKMUXSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable serial clock multiplexing"]
    #[inline(always)]
    pub fn ctl_clk_mux_dis(self) -> &'a mut W {
        self.variant(CKMUXSEL_A::CTL_CLK_MUX_DIS)
    }
    #[doc = "Enable serial clock multiplexing"]
    #[inline(always)]
    pub fn ctl_clk_mux_en(self) -> &'a mut W {
        self.variant(CKMUXSEL_A::CTL_CLK_MUX_EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Least-Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBF_A {
    #[doc = "0: MSB first sent/received"]
    CTL_MSB_FIRST = 0,
    #[doc = "1: LSB first sent/received"]
    CTL_LSB_FIRST = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSBF`"]
pub type LSBF_R = crate::R<bool, LSBF_A>;
impl LSBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::CTL_MSB_FIRST,
            true => LSBF_A::CTL_LSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_MSB_FIRST`"]
    #[inline(always)]
    pub fn is_ctl_msb_first(&self) -> bool {
        *self == LSBF_A::CTL_MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `CTL_LSB_FIRST`"]
    #[inline(always)]
    pub fn is_ctl_lsb_first(&self) -> bool {
        *self == LSBF_A::CTL_LSB_FIRST
    }
}
#[doc = "Write proxy for field `LSBF`"]
pub struct LSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MSB first sent/received"]
    #[inline(always)]
    pub fn ctl_msb_first(self) -> &'a mut W {
        self.variant(LSBF_A::CTL_MSB_FIRST)
    }
    #[doc = "LSB first sent/received"]
    #[inline(always)]
    pub fn ctl_lsb_first(self) -> &'a mut W {
        self.variant(LSBF_A::CTL_LSB_FIRST)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SLEN`"]
pub type SLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLEN`"]
pub struct SLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Internal Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICLK_A {
    #[doc = "0: External clock"]
    CTL_EXTERNAL_CLK = 0,
    #[doc = "1: Internal clock"]
    CTL_INTERNAL_CLK = 1,
}
impl From<ICLK_A> for bool {
    #[inline(always)]
    fn from(variant: ICLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICLK`"]
pub type ICLK_R = crate::R<bool, ICLK_A>;
impl ICLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLK_A {
        match self.bits {
            false => ICLK_A::CTL_EXTERNAL_CLK,
            true => ICLK_A::CTL_INTERNAL_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_EXTERNAL_CLK`"]
    #[inline(always)]
    pub fn is_ctl_external_clk(&self) -> bool {
        *self == ICLK_A::CTL_EXTERNAL_CLK
    }
    #[doc = "Checks if the value of the field is `CTL_INTERNAL_CLK`"]
    #[inline(always)]
    pub fn is_ctl_internal_clk(&self) -> bool {
        *self == ICLK_A::CTL_INTERNAL_CLK
    }
}
#[doc = "Write proxy for field `ICLK`"]
pub struct ICLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ICLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn ctl_external_clk(self) -> &'a mut W {
        self.variant(ICLK_A::CTL_EXTERNAL_CLK)
    }
    #[doc = "Internal clock"]
    #[inline(always)]
    pub fn ctl_internal_clk(self) -> &'a mut W {
        self.variant(ICLK_A::CTL_INTERNAL_CLK)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPMODE_A {
    #[doc = "0: DSP standard"]
    CTL_SERIAL = 0,
    #[doc = "1: Timer_enable mode"]
    CTL_TIMER_EN_MODE = 1,
}
impl From<OPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OPMODE`"]
pub type OPMODE_R = crate::R<bool, OPMODE_A>;
impl OPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPMODE_A {
        match self.bits {
            false => OPMODE_A::CTL_SERIAL,
            true => OPMODE_A::CTL_TIMER_EN_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_SERIAL`"]
    #[inline(always)]
    pub fn is_ctl_serial(&self) -> bool {
        *self == OPMODE_A::CTL_SERIAL
    }
    #[doc = "Checks if the value of the field is `CTL_TIMER_EN_MODE`"]
    #[inline(always)]
    pub fn is_ctl_timer_en_mode(&self) -> bool {
        *self == OPMODE_A::CTL_TIMER_EN_MODE
    }
}
#[doc = "Write proxy for field `OPMODE`"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSP standard"]
    #[inline(always)]
    pub fn ctl_serial(self) -> &'a mut W {
        self.variant(OPMODE_A::CTL_SERIAL)
    }
    #[doc = "Timer_enable mode"]
    #[inline(always)]
    pub fn ctl_timer_en_mode(self) -> &'a mut W {
        self.variant(OPMODE_A::CTL_TIMER_EN_MODE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Clock Rising Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKRE_A {
    #[doc = "0: Clock falling edge"]
    CTL_CLK_FALL_EDGE = 0,
    #[doc = "1: Clock rising edge"]
    CTL_CLK_RISE_EDGE = 1,
}
impl From<CKRE_A> for bool {
    #[inline(always)]
    fn from(variant: CKRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKRE`"]
pub type CKRE_R = crate::R<bool, CKRE_A>;
impl CKRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKRE_A {
        match self.bits {
            false => CKRE_A::CTL_CLK_FALL_EDGE,
            true => CKRE_A::CTL_CLK_RISE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_CLK_FALL_EDGE`"]
    #[inline(always)]
    pub fn is_ctl_clk_fall_edge(&self) -> bool {
        *self == CKRE_A::CTL_CLK_FALL_EDGE
    }
    #[doc = "Checks if the value of the field is `CTL_CLK_RISE_EDGE`"]
    #[inline(always)]
    pub fn is_ctl_clk_rise_edge(&self) -> bool {
        *self == CKRE_A::CTL_CLK_RISE_EDGE
    }
}
#[doc = "Write proxy for field `CKRE`"]
pub struct CKRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock falling edge"]
    #[inline(always)]
    pub fn ctl_clk_fall_edge(self) -> &'a mut W {
        self.variant(CKRE_A::CTL_CLK_FALL_EDGE)
    }
    #[doc = "Clock rising edge"]
    #[inline(always)]
    pub fn ctl_clk_rise_edge(self) -> &'a mut W {
        self.variant(CKRE_A::CTL_CLK_RISE_EDGE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Frame Sync Required\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSR_A {
    #[doc = "0: No frame sync required"]
    CTL_FS_NOT_REQ = 0,
    #[doc = "1: Frame sync required"]
    CTL_FS_REQ = 1,
}
impl From<FSR_A> for bool {
    #[inline(always)]
    fn from(variant: FSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSR`"]
pub type FSR_R = crate::R<bool, FSR_A>;
impl FSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSR_A {
        match self.bits {
            false => FSR_A::CTL_FS_NOT_REQ,
            true => FSR_A::CTL_FS_REQ,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_FS_NOT_REQ`"]
    #[inline(always)]
    pub fn is_ctl_fs_not_req(&self) -> bool {
        *self == FSR_A::CTL_FS_NOT_REQ
    }
    #[doc = "Checks if the value of the field is `CTL_FS_REQ`"]
    #[inline(always)]
    pub fn is_ctl_fs_req(&self) -> bool {
        *self == FSR_A::CTL_FS_REQ
    }
}
#[doc = "Write proxy for field `FSR`"]
pub struct FSR_W<'a> {
    w: &'a mut W,
}
impl<'a> FSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No frame sync required"]
    #[inline(always)]
    pub fn ctl_fs_not_req(self) -> &'a mut W {
        self.variant(FSR_A::CTL_FS_NOT_REQ)
    }
    #[doc = "Frame sync required"]
    #[inline(always)]
    pub fn ctl_fs_req(self) -> &'a mut W {
        self.variant(FSR_A::CTL_FS_REQ)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Internal Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFS_A {
    #[doc = "0: External frame sync"]
    CTL_EXTERNAL_FS = 0,
    #[doc = "1: Internal frame sync"]
    CTL_INTERNAL_FS = 1,
}
impl From<IFS_A> for bool {
    #[inline(always)]
    fn from(variant: IFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFS`"]
pub type IFS_R = crate::R<bool, IFS_A>;
impl IFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFS_A {
        match self.bits {
            false => IFS_A::CTL_EXTERNAL_FS,
            true => IFS_A::CTL_INTERNAL_FS,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_EXTERNAL_FS`"]
    #[inline(always)]
    pub fn is_ctl_external_fs(&self) -> bool {
        *self == IFS_A::CTL_EXTERNAL_FS
    }
    #[doc = "Checks if the value of the field is `CTL_INTERNAL_FS`"]
    #[inline(always)]
    pub fn is_ctl_internal_fs(&self) -> bool {
        *self == IFS_A::CTL_INTERNAL_FS
    }
}
#[doc = "Write proxy for field `IFS`"]
pub struct IFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External frame sync"]
    #[inline(always)]
    pub fn ctl_external_fs(self) -> &'a mut W {
        self.variant(IFS_A::CTL_EXTERNAL_FS)
    }
    #[doc = "Internal frame sync"]
    #[inline(always)]
    pub fn ctl_internal_fs(self) -> &'a mut W {
        self.variant(IFS_A::CTL_INTERNAL_FS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Data-Independent Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFS_A {
    #[doc = "0: Data-dependent frame sync"]
    CTL_DATA_DEP_FS = 0,
    #[doc = "1: Data-independent frame sync"]
    CTL_DATA_INDP_FS = 1,
}
impl From<DIFS_A> for bool {
    #[inline(always)]
    fn from(variant: DIFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIFS`"]
pub type DIFS_R = crate::R<bool, DIFS_A>;
impl DIFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFS_A {
        match self.bits {
            false => DIFS_A::CTL_DATA_DEP_FS,
            true => DIFS_A::CTL_DATA_INDP_FS,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_DATA_DEP_FS`"]
    #[inline(always)]
    pub fn is_ctl_data_dep_fs(&self) -> bool {
        *self == DIFS_A::CTL_DATA_DEP_FS
    }
    #[doc = "Checks if the value of the field is `CTL_DATA_INDP_FS`"]
    #[inline(always)]
    pub fn is_ctl_data_indp_fs(&self) -> bool {
        *self == DIFS_A::CTL_DATA_INDP_FS
    }
}
#[doc = "Write proxy for field `DIFS`"]
pub struct DIFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data-dependent frame sync"]
    #[inline(always)]
    pub fn ctl_data_dep_fs(self) -> &'a mut W {
        self.variant(DIFS_A::CTL_DATA_DEP_FS)
    }
    #[doc = "Data-independent frame sync"]
    #[inline(always)]
    pub fn ctl_data_indp_fs(self) -> &'a mut W {
        self.variant(DIFS_A::CTL_DATA_INDP_FS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Active-Low Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFS_A {
    #[doc = "0: Active high frame sync"]
    CTL_FS_LO = 0,
    #[doc = "1: Active low frame sync"]
    CTL_FS_HI = 1,
}
impl From<LFS_A> for bool {
    #[inline(always)]
    fn from(variant: LFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFS`"]
pub type LFS_R = crate::R<bool, LFS_A>;
impl LFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFS_A {
        match self.bits {
            false => LFS_A::CTL_FS_LO,
            true => LFS_A::CTL_FS_HI,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_FS_LO`"]
    #[inline(always)]
    pub fn is_ctl_fs_lo(&self) -> bool {
        *self == LFS_A::CTL_FS_LO
    }
    #[doc = "Checks if the value of the field is `CTL_FS_HI`"]
    #[inline(always)]
    pub fn is_ctl_fs_hi(&self) -> bool {
        *self == LFS_A::CTL_FS_HI
    }
}
#[doc = "Write proxy for field `LFS`"]
pub struct LFS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active high frame sync"]
    #[inline(always)]
    pub fn ctl_fs_lo(self) -> &'a mut W {
        self.variant(LFS_A::CTL_FS_LO)
    }
    #[doc = "Active low frame sync"]
    #[inline(always)]
    pub fn ctl_fs_hi(self) -> &'a mut W {
        self.variant(LFS_A::CTL_FS_HI)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Late Frame Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LAFS_A {
    #[doc = "0: Early frame sync"]
    CTL_EARLY_FS = 0,
    #[doc = "1: Late frame sync"]
    CTL_LATE_FS = 1,
}
impl From<LAFS_A> for bool {
    #[inline(always)]
    fn from(variant: LAFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LAFS`"]
pub type LAFS_R = crate::R<bool, LAFS_A>;
impl LAFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LAFS_A {
        match self.bits {
            false => LAFS_A::CTL_EARLY_FS,
            true => LAFS_A::CTL_LATE_FS,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_EARLY_FS`"]
    #[inline(always)]
    pub fn is_ctl_early_fs(&self) -> bool {
        *self == LAFS_A::CTL_EARLY_FS
    }
    #[doc = "Checks if the value of the field is `CTL_LATE_FS`"]
    #[inline(always)]
    pub fn is_ctl_late_fs(&self) -> bool {
        *self == LAFS_A::CTL_LATE_FS
    }
}
#[doc = "Write proxy for field `LAFS`"]
pub struct LAFS_W<'a> {
    w: &'a mut W,
}
impl<'a> LAFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LAFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Early frame sync"]
    #[inline(always)]
    pub fn ctl_early_fs(self) -> &'a mut W {
        self.variant(LAFS_A::CTL_EARLY_FS)
    }
    #[doc = "Late frame sync"]
    #[inline(always)]
    pub fn ctl_late_fs(self) -> &'a mut W {
        self.variant(LAFS_A::CTL_LATE_FS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Packing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACK_A {
    #[doc = "0: Disable"]
    CTL_PACK_DIS = 0,
    #[doc = "1: 8-bit packing enable"]
    CTL_PACK_8BIT = 1,
    #[doc = "2: 16-bit packing enable"]
    CTL_PACK_16BIT = 2,
    #[doc = "3: Reserved"]
    CTL_PACK_RSV = 3,
}
impl From<PACK_A> for u8 {
    #[inline(always)]
    fn from(variant: PACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACK`"]
pub type PACK_R = crate::R<u8, PACK_A>;
impl PACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PACK_A {
        match self.bits {
            0 => PACK_A::CTL_PACK_DIS,
            1 => PACK_A::CTL_PACK_8BIT,
            2 => PACK_A::CTL_PACK_16BIT,
            3 => PACK_A::CTL_PACK_RSV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTL_PACK_DIS`"]
    #[inline(always)]
    pub fn is_ctl_pack_dis(&self) -> bool {
        *self == PACK_A::CTL_PACK_DIS
    }
    #[doc = "Checks if the value of the field is `CTL_PACK_8BIT`"]
    #[inline(always)]
    pub fn is_ctl_pack_8bit(&self) -> bool {
        *self == PACK_A::CTL_PACK_8BIT
    }
    #[doc = "Checks if the value of the field is `CTL_PACK_16BIT`"]
    #[inline(always)]
    pub fn is_ctl_pack_16bit(&self) -> bool {
        *self == PACK_A::CTL_PACK_16BIT
    }
    #[doc = "Checks if the value of the field is `CTL_PACK_RSV`"]
    #[inline(always)]
    pub fn is_ctl_pack_rsv(&self) -> bool {
        *self == PACK_A::CTL_PACK_RSV
    }
}
#[doc = "Write proxy for field `PACK`"]
pub struct PACK_W<'a> {
    w: &'a mut W,
}
impl<'a> PACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ctl_pack_dis(self) -> &'a mut W {
        self.variant(PACK_A::CTL_PACK_DIS)
    }
    #[doc = "8-bit packing enable"]
    #[inline(always)]
    pub fn ctl_pack_8bit(self) -> &'a mut W {
        self.variant(PACK_A::CTL_PACK_8BIT)
    }
    #[doc = "16-bit packing enable"]
    #[inline(always)]
    pub fn ctl_pack_16bit(self) -> &'a mut W {
        self.variant(PACK_A::CTL_PACK_16BIT)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn ctl_pack_rsv(self) -> &'a mut W {
        self.variant(PACK_A::CTL_PACK_RSV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `FSERRMODE`"]
pub type FSERRMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSERRMODE`"]
pub struct FSERRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSERRMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Gated Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCLKEN_A {
    #[doc = "0: Disable"]
    CTL_GCLK_DIS = 0,
    #[doc = "1: Enable"]
    CTL_GCLK_EN = 1,
}
impl From<GCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GCLKEN`"]
pub type GCLKEN_R = crate::R<bool, GCLKEN_A>;
impl GCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCLKEN_A {
        match self.bits {
            false => GCLKEN_A::CTL_GCLK_DIS,
            true => GCLKEN_A::CTL_GCLK_EN,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_GCLK_DIS`"]
    #[inline(always)]
    pub fn is_ctl_gclk_dis(&self) -> bool {
        *self == GCLKEN_A::CTL_GCLK_DIS
    }
    #[doc = "Checks if the value of the field is `CTL_GCLK_EN`"]
    #[inline(always)]
    pub fn is_ctl_gclk_en(&self) -> bool {
        *self == GCLKEN_A::CTL_GCLK_EN
    }
}
#[doc = "Write proxy for field `GCLKEN`"]
pub struct GCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ctl_gclk_dis(self) -> &'a mut W {
        self.variant(GCLKEN_A::CTL_GCLK_DIS)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ctl_gclk_en(self) -> &'a mut W {
        self.variant(GCLKEN_A::CTL_GCLK_EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Serial Port Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTRAN_A {
    #[doc = "0: Receive"]
    CTL_RX = 0,
    #[doc = "1: Transmit"]
    CTL_TX = 1,
}
impl From<SPTRAN_A> for bool {
    #[inline(always)]
    fn from(variant: SPTRAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPTRAN`"]
pub type SPTRAN_R = crate::R<bool, SPTRAN_A>;
impl SPTRAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTRAN_A {
        match self.bits {
            false => SPTRAN_A::CTL_RX,
            true => SPTRAN_A::CTL_TX,
        }
    }
    #[doc = "Checks if the value of the field is `CTL_RX`"]
    #[inline(always)]
    pub fn is_ctl_rx(&self) -> bool {
        *self == SPTRAN_A::CTL_RX
    }
    #[doc = "Checks if the value of the field is `CTL_TX`"]
    #[inline(always)]
    pub fn is_ctl_tx(&self) -> bool {
        *self == SPTRAN_A::CTL_TX
    }
}
#[doc = "Write proxy for field `SPTRAN`"]
pub struct SPTRAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPTRAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPTRAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn ctl_rx(self) -> &'a mut W {
        self.variant(SPTRAN_A::CTL_RX)
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn ctl_tx(self) -> &'a mut W {
        self.variant(SPTRAN_A::CTL_TX)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Serial Port Enable"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Multiplexer Select"]
    #[inline(always)]
    pub fn fsmuxsel(&self) -> FSMUXSEL_R {
        FSMUXSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Multiplexer Select"]
    #[inline(always)]
    pub fn ckmuxsel(&self) -> CKMUXSEL_R {
        CKMUXSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Least-Significant Bit First"]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - Serial Word Length"]
    #[inline(always)]
    pub fn slen(&self) -> SLEN_R {
        SLEN_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Internal Clock"]
    #[inline(always)]
    pub fn iclk(&self) -> ICLK_R {
        ICLK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Clock Rising Edge"]
    #[inline(always)]
    pub fn ckre(&self) -> CKRE_R {
        CKRE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Frame Sync Required"]
    #[inline(always)]
    pub fn fsr(&self) -> FSR_R {
        FSR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Internal Frame Sync"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Data-Independent Frame Sync"]
    #[inline(always)]
    pub fn difs(&self) -> DIFS_R {
        DIFS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Active-Low Frame Sync"]
    #[inline(always)]
    pub fn lfs(&self) -> LFS_R {
        LFS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Late Frame Sync"]
    #[inline(always)]
    pub fn lafs(&self) -> LAFS_R {
        LAFS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Packing Enable"]
    #[inline(always)]
    pub fn pack(&self) -> PACK_R {
        PACK_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Frame Sync Error Operation"]
    #[inline(always)]
    pub fn fserrmode(&self) -> FSERRMODE_R {
        FSERRMODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Gated Clock Enable"]
    #[inline(always)]
    pub fn gclken(&self) -> GCLKEN_R {
        GCLKEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Serial Port Transfer Direction"]
    #[inline(always)]
    pub fn sptran(&self) -> SPTRAN_R {
        SPTRAN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Port Enable"]
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W {
        SPEN_W { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Multiplexer Select"]
    #[inline(always)]
    pub fn fsmuxsel(&mut self) -> FSMUXSEL_W {
        FSMUXSEL_W { w: self }
    }
    #[doc = "Bit 2 - Clock Multiplexer Select"]
    #[inline(always)]
    pub fn ckmuxsel(&mut self) -> CKMUXSEL_W {
        CKMUXSEL_W { w: self }
    }
    #[doc = "Bit 3 - Least-Significant Bit First"]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W {
        LSBF_W { w: self }
    }
    #[doc = "Bits 4:8 - Serial Word Length"]
    #[inline(always)]
    pub fn slen(&mut self) -> SLEN_W {
        SLEN_W { w: self }
    }
    #[doc = "Bit 10 - Internal Clock"]
    #[inline(always)]
    pub fn iclk(&mut self) -> ICLK_W {
        ICLK_W { w: self }
    }
    #[doc = "Bit 11 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bit 12 - Clock Rising Edge"]
    #[inline(always)]
    pub fn ckre(&mut self) -> CKRE_W {
        CKRE_W { w: self }
    }
    #[doc = "Bit 13 - Frame Sync Required"]
    #[inline(always)]
    pub fn fsr(&mut self) -> FSR_W {
        FSR_W { w: self }
    }
    #[doc = "Bit 14 - Internal Frame Sync"]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W {
        IFS_W { w: self }
    }
    #[doc = "Bit 15 - Data-Independent Frame Sync"]
    #[inline(always)]
    pub fn difs(&mut self) -> DIFS_W {
        DIFS_W { w: self }
    }
    #[doc = "Bit 16 - Active-Low Frame Sync"]
    #[inline(always)]
    pub fn lfs(&mut self) -> LFS_W {
        LFS_W { w: self }
    }
    #[doc = "Bit 17 - Late Frame Sync"]
    #[inline(always)]
    pub fn lafs(&mut self) -> LAFS_W {
        LAFS_W { w: self }
    }
    #[doc = "Bits 18:19 - Packing Enable"]
    #[inline(always)]
    pub fn pack(&mut self) -> PACK_W {
        PACK_W { w: self }
    }
    #[doc = "Bit 20 - Frame Sync Error Operation"]
    #[inline(always)]
    pub fn fserrmode(&mut self) -> FSERRMODE_W {
        FSERRMODE_W { w: self }
    }
    #[doc = "Bit 21 - Gated Clock Enable"]
    #[inline(always)]
    pub fn gclken(&mut self) -> GCLKEN_W {
        GCLKEN_W { w: self }
    }
    #[doc = "Bit 25 - Serial Port Transfer Direction"]
    #[inline(always)]
    pub fn sptran(&mut self) -> SPTRAN_W {
        SPTRAN_W { w: self }
    }
    #[doc = "Bit 26 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
