///Register `INT_ENA_RTC_W1TS` writer
pub type W = crate::W<INT_ENA_RTC_W1TS_SPEC>;
///Field `SLP_WAKEUP` writer - enable sleep wakeup interrupt
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `SLP_REJECT` writer - enable sleep reject interrupt
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `WDT` writer - enable RTC WDT interrupt
pub type WDT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `BROWN_OUT` writer - enable brown out interrupt
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `MAIN_TIMER` writer - enable RTC main timer interrupt
pub type MAIN_TIMER_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `SWD` writer - enable super watch dog interrupt
pub type SWD_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `XTAL32K_DEAD` writer - enable xtal32k_dead interrupt
pub type XTAL32K_DEAD_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `GLITCH_DET` writer - enbale gitch det interrupt
pub type GLITCH_DET_W<'a, REG> = crate::BitWriter1S<'a, REG>;
///Field `BBPLL_CAL` writer - enbale bbpll cal interrupt
pub type BBPLL_CAL_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - enable sleep wakeup interrupt
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<INT_ENA_RTC_W1TS_SPEC> {
        SLP_WAKEUP_W::new(self, 0)
    }
    ///Bit 1 - enable sleep reject interrupt
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<INT_ENA_RTC_W1TS_SPEC> {
        SLP_REJECT_W::new(self, 1)
    }
    ///Bit 3 - enable RTC WDT interrupt
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_RTC_W1TS_SPEC> {
        WDT_W::new(self, 3)
    }
    ///Bit 9 - enable brown out interrupt
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_RTC_W1TS_SPEC> {
        BROWN_OUT_W::new(self, 9)
    }
    ///Bit 10 - enable RTC main timer interrupt
    #[inline(always)]
    #[must_use]
    pub fn main_timer(&mut self) -> MAIN_TIMER_W<INT_ENA_RTC_W1TS_SPEC> {
        MAIN_TIMER_W::new(self, 10)
    }
    ///Bit 15 - enable super watch dog interrupt
    #[inline(always)]
    #[must_use]
    pub fn swd(&mut self) -> SWD_W<INT_ENA_RTC_W1TS_SPEC> {
        SWD_W::new(self, 15)
    }
    ///Bit 16 - enable xtal32k_dead interrupt
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead(&mut self) -> XTAL32K_DEAD_W<INT_ENA_RTC_W1TS_SPEC> {
        XTAL32K_DEAD_W::new(self, 16)
    }
    ///Bit 19 - enbale gitch det interrupt
    #[inline(always)]
    #[must_use]
    pub fn glitch_det(&mut self) -> GLITCH_DET_W<INT_ENA_RTC_W1TS_SPEC> {
        GLITCH_DET_W::new(self, 19)
    }
    ///Bit 20 - enbale bbpll cal interrupt
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal(&mut self) -> BBPLL_CAL_W<INT_ENA_RTC_W1TS_SPEC> {
        BBPLL_CAL_W::new(self, 20)
    }
}
/**rtc configure register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_RTC_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_W1TS_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_ena_rtc_w1ts::W`](W) writer structure
impl crate::Writable for INT_ENA_RTC_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0019_860b;
}
///`reset()` method sets INT_ENA_RTC_W1TS to value 0
impl crate::Resettable for INT_ENA_RTC_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
