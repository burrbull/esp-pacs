///Register `STATE0` reader
pub type R = crate::R<STATE0_SPEC>;
///Register `STATE0` writer
pub type W = crate::W<STATE0_SPEC>;
///Field `SW_CPU_INT` writer - rtc software interrupt to main cpu
pub type SW_CPU_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_REJECT_CAUSE_CLR` writer - clear rtc sleep reject cause
pub type SLP_REJECT_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2RTC_BRIDGE_SEL` reader - 1: APB to RTC using bridge, 0: APB to RTC using sync
pub type APB2RTC_BRIDGE_SEL_R = crate::BitReader;
///Field `APB2RTC_BRIDGE_SEL` writer - 1: APB to RTC using bridge, 0: APB to RTC using sync
pub type APB2RTC_BRIDGE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_ACTIVE_IND` reader - SDIO active indication
pub type SDIO_ACTIVE_IND_R = crate::BitReader;
///Field `SLP_WAKEUP` reader - leep wakeup bit
pub type SLP_WAKEUP_R = crate::BitReader;
///Field `SLP_WAKEUP` writer - leep wakeup bit
pub type SLP_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_REJECT` reader - leep reject bit
pub type SLP_REJECT_R = crate::BitReader;
///Field `SLP_REJECT` writer - leep reject bit
pub type SLP_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_EN` reader - sleep enable bit
pub type SLEEP_EN_R = crate::BitReader;
///Field `SLEEP_EN` writer - sleep enable bit
pub type SLEEP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 22 - 1: APB to RTC using bridge, 0: APB to RTC using sync
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 28 - SDIO active indication
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - leep wakeup bit
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - leep reject bit
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - sleep enable bit
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE0")
            .field("apb2rtc_bridge_sel", &self.apb2rtc_bridge_sel())
            .field("sdio_active_ind", &self.sdio_active_ind())
            .field("slp_wakeup", &self.slp_wakeup())
            .field("slp_reject", &self.slp_reject())
            .field("sleep_en", &self.sleep_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - rtc software interrupt to main cpu
    #[inline(always)]
    #[must_use]
    pub fn sw_cpu_int(&mut self) -> SW_CPU_INT_W<STATE0_SPEC> {
        SW_CPU_INT_W::new(self, 0)
    }
    ///Bit 1 - clear rtc sleep reject cause
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_cause_clr(&mut self) -> SLP_REJECT_CAUSE_CLR_W<STATE0_SPEC> {
        SLP_REJECT_CAUSE_CLR_W::new(self, 1)
    }
    ///Bit 22 - 1: APB to RTC using bridge, 0: APB to RTC using sync
    #[inline(always)]
    #[must_use]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W<STATE0_SPEC> {
        APB2RTC_BRIDGE_SEL_W::new(self, 22)
    }
    ///Bit 29 - leep wakeup bit
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<STATE0_SPEC> {
        SLP_WAKEUP_W::new(self, 29)
    }
    ///Bit 30 - leep reject bit
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<STATE0_SPEC> {
        SLP_REJECT_W::new(self, 30)
    }
    ///Bit 31 - sleep enable bit
    #[inline(always)]
    #[must_use]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<STATE0_SPEC> {
        SLEEP_EN_W::new(self, 31)
    }
}
/**configure chip sleep

You can [`read`](crate::generic::Reg::read) this register and get [`state0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`state0::R`](R) reader structure
impl crate::Readable for STATE0_SPEC {}
///`write(|w| ..)` method takes [`state0::W`](W) writer structure
impl crate::Writable for STATE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATE0 to value 0
impl crate::Resettable for STATE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
