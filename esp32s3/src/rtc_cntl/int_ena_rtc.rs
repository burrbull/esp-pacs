#[doc = "Register `INT_ENA_RTC` reader"]
pub struct R(crate::R<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_RTC` writer"]
pub struct W(crate::W<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SLP_WAKEUP_INT_ENA` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `SLP_REJECT_INT_ENA` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SLP_REJECT_INT_ENA` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA` reader - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_IDLE_INT_ENA` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_WDT_INT_ENA` reader - enable RTC WDT interrupt"]
pub type RTC_WDT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_WDT_INT_ENA` writer - enable RTC WDT interrupt"]
pub type RTC_WDT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_ENA` reader - enable touch scan done interrupt"]
pub type RTC_TOUCH_SCAN_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_SCAN_DONE_INT_ENA` writer - enable touch scan done interrupt"]
pub type RTC_TOUCH_SCAN_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_ULP_CP_INT_ENA` reader - enable ULP-coprocessor interrupt"]
pub type RTC_ULP_CP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_ULP_CP_INT_ENA` writer - enable ULP-coprocessor interrupt"]
pub type RTC_ULP_CP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_DONE_INT_ENA` reader - enable touch done interrupt"]
pub type RTC_TOUCH_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_DONE_INT_ENA` writer - enable touch done interrupt"]
pub type RTC_TOUCH_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_ENA` reader - enable touch active interrupt"]
pub type RTC_TOUCH_ACTIVE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_ACTIVE_INT_ENA` writer - enable touch active interrupt"]
pub type RTC_TOUCH_ACTIVE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_ENA` reader - enable touch inactive interrupt"]
pub type RTC_TOUCH_INACTIVE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_INACTIVE_INT_ENA` writer - enable touch inactive interrupt"]
pub type RTC_TOUCH_INACTIVE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA` reader - enable brown out interrupt"]
pub type RTC_BROWN_OUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_BROWN_OUT_INT_ENA` writer - enable brown out interrupt"]
pub type RTC_BROWN_OUT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA` reader - enable RTC main timer interrupt"]
pub type RTC_MAIN_TIMER_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_TIMER_INT_ENA` writer - enable RTC main timer interrupt"]
pub type RTC_MAIN_TIMER_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SARADC1_INT_ENA` reader - enable saradc1 interrupt"]
pub type RTC_SARADC1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SARADC1_INT_ENA` writer - enable saradc1 interrupt"]
pub type RTC_SARADC1_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TSENS_INT_ENA` reader - enable tsens interrupt"]
pub type RTC_TSENS_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TSENS_INT_ENA` writer - enable tsens interrupt"]
pub type RTC_TSENS_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_COCPU_INT_ENA` reader - enable riscV cocpu interrupt"]
pub type RTC_COCPU_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_INT_ENA` writer - enable riscV cocpu interrupt"]
pub type RTC_COCPU_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SARADC2_INT_ENA` reader - enable saradc2 interrupt"]
pub type RTC_SARADC2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SARADC2_INT_ENA` writer - enable saradc2 interrupt"]
pub type RTC_SARADC2_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_SWD_INT_ENA` reader - enable super watch dog interrupt"]
pub type RTC_SWD_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SWD_INT_ENA` writer - enable super watch dog interrupt"]
pub type RTC_SWD_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ENA` reader - enable xtal32k_dead interrupt"]
pub type RTC_XTAL32K_DEAD_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ENA` writer - enable xtal32k_dead interrupt"]
pub type RTC_XTAL32K_DEAD_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_COCPU_TRAP_INT_ENA` reader - enable cocpu trap interrupt"]
pub type RTC_COCPU_TRAP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_TRAP_INT_ENA` writer - enable cocpu trap interrupt"]
pub type RTC_COCPU_TRAP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_ENA` reader - enable touch timeout interrupt"]
pub type RTC_TOUCH_TIMEOUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_TIMEOUT_INT_ENA` writer - enable touch timeout interrupt"]
pub type RTC_TOUCH_TIMEOUT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_GLITCH_DET_INT_ENA` reader - enbale gitch det interrupt"]
pub type RTC_GLITCH_DET_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_GLITCH_DET_INT_ENA` writer - enbale gitch det interrupt"]
pub type RTC_GLITCH_DET_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA` reader - touch approach mode loop interrupt"]
pub type RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA` writer - touch approach mode loop interrupt"]
pub type RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_RTC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&self) -> SLP_WAKEUP_INT_ENA_R {
        SLP_WAKEUP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena(&self) -> SLP_REJECT_INT_ENA_R {
        SLP_REJECT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&self) -> SDIO_IDLE_INT_ENA_R {
        SDIO_IDLE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena(&self) -> RTC_WDT_INT_ENA_R {
        RTC_WDT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_ena(&self) -> RTC_TOUCH_SCAN_DONE_INT_ENA_R {
        RTC_TOUCH_SCAN_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_ena(&self) -> RTC_ULP_CP_INT_ENA_R {
        RTC_ULP_CP_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_done_int_ena(&self) -> RTC_TOUCH_DONE_INT_ENA_R {
        RTC_TOUCH_DONE_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    pub fn rtc_touch_active_int_ena(&self) -> RTC_TOUCH_ACTIVE_INT_ENA_R {
        RTC_TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_ena(&self) -> RTC_TOUCH_INACTIVE_INT_ENA_R {
        RTC_TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena(&self) -> RTC_BROWN_OUT_INT_ENA_R {
        RTC_BROWN_OUT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena(&self) -> RTC_MAIN_TIMER_INT_ENA_R {
        RTC_MAIN_TIMER_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    pub fn rtc_saradc1_int_ena(&self) -> RTC_SARADC1_INT_ENA_R {
        RTC_SARADC1_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    pub fn rtc_tsens_int_ena(&self) -> RTC_TSENS_INT_ENA_R {
        RTC_TSENS_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    pub fn rtc_cocpu_int_ena(&self) -> RTC_COCPU_INT_ENA_R {
        RTC_COCPU_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    pub fn rtc_saradc2_int_ena(&self) -> RTC_SARADC2_INT_ENA_R {
        RTC_SARADC2_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena(&self) -> RTC_SWD_INT_ENA_R {
        RTC_SWD_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_ena(&self) -> RTC_XTAL32K_DEAD_INT_ENA_R {
        RTC_XTAL32K_DEAD_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_ena(&self) -> RTC_COCPU_TRAP_INT_ENA_R {
        RTC_COCPU_TRAP_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_ena(&self) -> RTC_TOUCH_TIMEOUT_INT_ENA_R {
        RTC_TOUCH_TIMEOUT_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_ena(&self) -> RTC_GLITCH_DET_INT_ENA_R {
        RTC_GLITCH_DET_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_ena(&self) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_R {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&mut self) -> SLP_WAKEUP_INT_ENA_W<0> {
        SLP_WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena(&mut self) -> SLP_REJECT_INT_ENA_W<1> {
        SLP_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&mut self) -> SDIO_IDLE_INT_ENA_W<2> {
        SDIO_IDLE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn rtc_wdt_int_ena(&mut self) -> RTC_WDT_INT_ENA_W<3> {
        RTC_WDT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - enable touch scan done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_scan_done_int_ena(&mut self) -> RTC_TOUCH_SCAN_DONE_INT_ENA_W<4> {
        RTC_TOUCH_SCAN_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn rtc_ulp_cp_int_ena(&mut self) -> RTC_ULP_CP_INT_ENA_W<5> {
        RTC_ULP_CP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - enable touch done interrupt"]
    #[inline(always)]
    pub fn rtc_touch_done_int_ena(&mut self) -> RTC_TOUCH_DONE_INT_ENA_W<6> {
        RTC_TOUCH_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - enable touch active interrupt"]
    #[inline(always)]
    pub fn rtc_touch_active_int_ena(&mut self) -> RTC_TOUCH_ACTIVE_INT_ENA_W<7> {
        RTC_TOUCH_ACTIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - enable touch inactive interrupt"]
    #[inline(always)]
    pub fn rtc_touch_inactive_int_ena(&mut self) -> RTC_TOUCH_INACTIVE_INT_ENA_W<8> {
        RTC_TOUCH_INACTIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - enable brown out interrupt"]
    #[inline(always)]
    pub fn rtc_brown_out_int_ena(&mut self) -> RTC_BROWN_OUT_INT_ENA_W<9> {
        RTC_BROWN_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn rtc_main_timer_int_ena(&mut self) -> RTC_MAIN_TIMER_INT_ENA_W<10> {
        RTC_MAIN_TIMER_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - enable saradc1 interrupt"]
    #[inline(always)]
    pub fn rtc_saradc1_int_ena(&mut self) -> RTC_SARADC1_INT_ENA_W<11> {
        RTC_SARADC1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - enable tsens interrupt"]
    #[inline(always)]
    pub fn rtc_tsens_int_ena(&mut self) -> RTC_TSENS_INT_ENA_W<12> {
        RTC_TSENS_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - enable riscV cocpu interrupt"]
    #[inline(always)]
    pub fn rtc_cocpu_int_ena(&mut self) -> RTC_COCPU_INT_ENA_W<13> {
        RTC_COCPU_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - enable saradc2 interrupt"]
    #[inline(always)]
    pub fn rtc_saradc2_int_ena(&mut self) -> RTC_SARADC2_INT_ENA_W<14> {
        RTC_SARADC2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - enable super watch dog interrupt"]
    #[inline(always)]
    pub fn rtc_swd_int_ena(&mut self) -> RTC_SWD_INT_ENA_W<15> {
        RTC_SWD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - enable xtal32k_dead interrupt"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_ena(&mut self) -> RTC_XTAL32K_DEAD_INT_ENA_W<16> {
        RTC_XTAL32K_DEAD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - enable cocpu trap interrupt"]
    #[inline(always)]
    pub fn rtc_cocpu_trap_int_ena(&mut self) -> RTC_COCPU_TRAP_INT_ENA_W<17> {
        RTC_COCPU_TRAP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - enable touch timeout interrupt"]
    #[inline(always)]
    pub fn rtc_touch_timeout_int_ena(&mut self) -> RTC_TOUCH_TIMEOUT_INT_ENA_W<18> {
        RTC_TOUCH_TIMEOUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - enbale gitch det interrupt"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_ena(&mut self) -> RTC_GLITCH_DET_INT_ENA_W<19> {
        RTC_GLITCH_DET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - touch approach mode loop interrupt"]
    #[inline(always)]
    pub fn rtc_touch_approach_loop_done_int_ena(
        &mut self,
    ) -> RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W<20> {
        RTC_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc](index.html) module"]
pub struct INT_ENA_RTC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_rtc::R](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_RTC to value 0"]
impl crate::Resettable for INT_ENA_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}