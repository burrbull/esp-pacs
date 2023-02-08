#[doc = "Register `DT2_CFG` reader"]
pub struct R(crate::R<DT2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT2_CFG` writer"]
pub struct W(crate::W<DT2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DT2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB2_FED_UPMETHOD` reader - Update method for FED (falling edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
pub type DB2_FED_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB2_FED_UPMETHOD` writer - Update method for FED (falling edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
pub type DB2_FED_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT2_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DB2_RED_UPMETHOD` reader - Update method for RED (rising edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
pub type DB2_RED_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB2_RED_UPMETHOD` writer - Update method for RED (rising edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
pub type DB2_RED_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DT2_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DB2_DEB_MODE` reader - S8 in table, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB2_DEB_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DB2_DEB_MODE` writer - S8 in table, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
pub type DB2_DEB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_A_OUTSWAP` reader - S6 in table"]
pub type DB2_A_OUTSWAP_R = crate::BitReader<bool>;
#[doc = "Field `DB2_A_OUTSWAP` writer - S6 in table"]
pub type DB2_A_OUTSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_B_OUTSWAP` reader - S7 in table"]
pub type DB2_B_OUTSWAP_R = crate::BitReader<bool>;
#[doc = "Field `DB2_B_OUTSWAP` writer - S7 in table"]
pub type DB2_B_OUTSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_RED_INSEL` reader - S4 in table"]
pub type DB2_RED_INSEL_R = crate::BitReader<bool>;
#[doc = "Field `DB2_RED_INSEL` writer - S4 in table"]
pub type DB2_RED_INSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_FED_INSEL` reader - S5 in table"]
pub type DB2_FED_INSEL_R = crate::BitReader<bool>;
#[doc = "Field `DB2_FED_INSEL` writer - S5 in table"]
pub type DB2_FED_INSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_RED_OUTINVERT` reader - S2 in table"]
pub type DB2_RED_OUTINVERT_R = crate::BitReader<bool>;
#[doc = "Field `DB2_RED_OUTINVERT` writer - S2 in table"]
pub type DB2_RED_OUTINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_FED_OUTINVERT` reader - S3 in table"]
pub type DB2_FED_OUTINVERT_R = crate::BitReader<bool>;
#[doc = "Field `DB2_FED_OUTINVERT` writer - S3 in table"]
pub type DB2_FED_OUTINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_A_OUTBYPASS` reader - S1 in table"]
pub type DB2_A_OUTBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DB2_A_OUTBYPASS` writer - S1 in table"]
pub type DB2_A_OUTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_B_OUTBYPASS` reader - S0 in table"]
pub type DB2_B_OUTBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DB2_B_OUTBYPASS` writer - S0 in table"]
pub type DB2_B_OUTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
#[doc = "Field `DB2_CLK_SEL` reader - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
pub type DB2_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `DB2_CLK_SEL` writer - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
pub type DB2_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DT2_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
    #[inline(always)]
    pub fn db2_fed_upmethod(&self) -> DB2_FED_UPMETHOD_R {
        DB2_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
    #[inline(always)]
    pub fn db2_red_upmethod(&self) -> DB2_RED_UPMETHOD_R {
        DB2_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - S8 in table, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    pub fn db2_deb_mode(&self) -> DB2_DEB_MODE_R {
        DB2_DEB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S6 in table"]
    #[inline(always)]
    pub fn db2_a_outswap(&self) -> DB2_A_OUTSWAP_R {
        DB2_A_OUTSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S7 in table"]
    #[inline(always)]
    pub fn db2_b_outswap(&self) -> DB2_B_OUTSWAP_R {
        DB2_B_OUTSWAP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S4 in table"]
    #[inline(always)]
    pub fn db2_red_insel(&self) -> DB2_RED_INSEL_R {
        DB2_RED_INSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S5 in table"]
    #[inline(always)]
    pub fn db2_fed_insel(&self) -> DB2_FED_INSEL_R {
        DB2_FED_INSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - S2 in table"]
    #[inline(always)]
    pub fn db2_red_outinvert(&self) -> DB2_RED_OUTINVERT_R {
        DB2_RED_OUTINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - S3 in table"]
    #[inline(always)]
    pub fn db2_fed_outinvert(&self) -> DB2_FED_OUTINVERT_R {
        DB2_FED_OUTINVERT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - S1 in table"]
    #[inline(always)]
    pub fn db2_a_outbypass(&self) -> DB2_A_OUTBYPASS_R {
        DB2_A_OUTBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - S0 in table"]
    #[inline(always)]
    pub fn db2_b_outbypass(&self) -> DB2_B_OUTBYPASS_R {
        DB2_B_OUTBYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    pub fn db2_clk_sel(&self) -> DB2_CLK_SEL_R {
        DB2_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed_upmethod(&mut self) -> DB2_FED_UPMETHOD_W<0> {
        DB2_FED_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active register. 0: immediate,when bit0 is set to 1: tez, when bit1 is set to 1:tep, when bit2 is set to 1: sync, when bit3 is set to 1: disable the update"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red_upmethod(&mut self) -> DB2_RED_UPMETHOD_W<4> {
        DB2_RED_UPMETHOD_W::new(self)
    }
    #[doc = "Bit 8 - S8 in table, dual-edge B mode, 0: fed/red take effect on different path separately, 1: fed/red take effect on B path, A out is in bypass or dulpB mode"]
    #[inline(always)]
    #[must_use]
    pub fn db2_deb_mode(&mut self) -> DB2_DEB_MODE_W<8> {
        DB2_DEB_MODE_W::new(self)
    }
    #[doc = "Bit 9 - S6 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_a_outswap(&mut self) -> DB2_A_OUTSWAP_W<9> {
        DB2_A_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 10 - S7 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_b_outswap(&mut self) -> DB2_B_OUTSWAP_W<10> {
        DB2_B_OUTSWAP_W::new(self)
    }
    #[doc = "Bit 11 - S4 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red_insel(&mut self) -> DB2_RED_INSEL_W<11> {
        DB2_RED_INSEL_W::new(self)
    }
    #[doc = "Bit 12 - S5 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed_insel(&mut self) -> DB2_FED_INSEL_W<12> {
        DB2_FED_INSEL_W::new(self)
    }
    #[doc = "Bit 13 - S2 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red_outinvert(&mut self) -> DB2_RED_OUTINVERT_W<13> {
        DB2_RED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 14 - S3 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_fed_outinvert(&mut self) -> DB2_FED_OUTINVERT_W<14> {
        DB2_FED_OUTINVERT_W::new(self)
    }
    #[doc = "Bit 15 - S1 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_a_outbypass(&mut self) -> DB2_A_OUTBYPASS_W<15> {
        DB2_A_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 16 - S0 in table"]
    #[inline(always)]
    #[must_use]
    pub fn db2_b_outbypass(&mut self) -> DB2_B_OUTBYPASS_W<16> {
        DB2_B_OUTBYPASS_W::new(self)
    }
    #[doc = "Bit 17 - Dead time generator 2 clock selection. 0: PWM_clk, 1: PT_clk"]
    #[inline(always)]
    #[must_use]
    pub fn db2_clk_sel(&mut self) -> DB2_CLK_SEL_W<17> {
        DB2_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dead time type selection and configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt2_cfg](index.html) module"]
pub struct DT2_CFG_SPEC;
impl crate::RegisterSpec for DT2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt2_cfg::R](R) reader structure"]
impl crate::Readable for DT2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt2_cfg::W](W) writer structure"]
impl crate::Writable for DT2_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT2_CFG to value 0x0001_8000"]
impl crate::Resettable for DT2_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_8000;
}