#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `SLP_WAKEUP` reader - sleep wakeup interrupt raw"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` reader - sleep reject interrupt raw"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `WDT` reader - RTC WDT interrupt raw"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` reader - brown out interrupt raw"]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER` reader - RTC main timer interrupt raw"]
pub type MAIN_TIMER_R = crate::BitReader;
#[doc = "Field `SWD` reader - super watch dog interrupt raw"]
pub type SWD_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD` reader - xtal32k dead detection interrupt raw"]
pub type XTAL32K_DEAD_R = crate::BitReader;
#[doc = "Field `GLITCH_DET` reader - glitch_det_interrupt_raw"]
pub type GLITCH_DET_R = crate::BitReader;
#[doc = "Field `BBPLL_CAL` reader - bbpll cal end interrupt state"]
pub type BBPLL_CAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt raw"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt raw"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt raw"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt raw"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt raw"]
    #[inline(always)]
    pub fn main_timer(&self) -> MAIN_TIMER_R {
        MAIN_TIMER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt raw"]
    #[inline(always)]
    pub fn swd(&self) -> SWD_R {
        SWD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt raw"]
    #[inline(always)]
    pub fn xtal32k_dead(&self) -> XTAL32K_DEAD_R {
        XTAL32K_DEAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt_raw"]
    #[inline(always)]
    pub fn glitch_det(&self) -> GLITCH_DET_R {
        GLITCH_DET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - bbpll cal end interrupt state"]
    #[inline(always)]
    pub fn bbpll_cal(&self) -> BBPLL_CAL_R {
        BBPLL_CAL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("slp_wakeup", &self.slp_wakeup().bit())
            .field("slp_reject", &self.slp_reject().bit())
            .field("wdt", &self.wdt().bit())
            .field("brown_out", &self.brown_out().bit())
            .field("main_timer", &self.main_timer().bit())
            .field("swd", &self.swd().bit())
            .field("xtal32k_dead", &self.xtal32k_dead().bit())
            .field("glitch_det", &self.glitch_det().bit())
            .field("bbpll_cal", &self.bbpll_cal().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
