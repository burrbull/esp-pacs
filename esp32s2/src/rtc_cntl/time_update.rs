///Register `TIME_UPDATE` reader
pub type R = crate::R<TIME_UPDATE_SPEC>;
///Register `TIME_UPDATE` writer
pub type W = crate::W<TIME_UPDATE_SPEC>;
///Field `TIMER_SYS_STALL` reader - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIMER_SYS_STALL_R = crate::BitReader;
///Field `TIMER_SYS_STALL` writer - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIMER_SYS_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_XTL_OFF` reader - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIMER_XTL_OFF_R = crate::BitReader;
///Field `TIMER_XTL_OFF` writer - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIMER_XTL_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_SYS_RST` reader - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIMER_SYS_RST_R = crate::BitReader;
///Field `TIMER_SYS_RST` writer - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIMER_SYS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIME_UPDATE` writer - Selects the triggering condition for the RTC timer. See details in Table 1-2.
pub type TIME_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 27 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    pub fn timer_sys_stall(&self) -> TIMER_SYS_STALL_R {
        TIMER_SYS_STALL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    pub fn timer_xtl_off(&self) -> TIMER_XTL_OFF_R {
        TIMER_XTL_OFF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    pub fn timer_sys_rst(&self) -> TIMER_SYS_RST_R {
        TIMER_SYS_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_UPDATE")
            .field("timer_sys_stall", &self.timer_sys_stall())
            .field("timer_xtl_off", &self.timer_xtl_off())
            .field("timer_sys_rst", &self.timer_sys_rst())
            .finish()
    }
}
impl W {
    ///Bit 27 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    #[must_use]
    pub fn timer_sys_stall(&mut self) -> TIMER_SYS_STALL_W<TIME_UPDATE_SPEC> {
        TIMER_SYS_STALL_W::new(self, 27)
    }
    ///Bit 28 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    #[must_use]
    pub fn timer_xtl_off(&mut self) -> TIMER_XTL_OFF_W<TIME_UPDATE_SPEC> {
        TIMER_XTL_OFF_W::new(self, 28)
    }
    ///Bit 29 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    #[must_use]
    pub fn timer_sys_rst(&mut self) -> TIMER_SYS_RST_W<TIME_UPDATE_SPEC> {
        TIMER_SYS_RST_W::new(self, 29)
    }
    ///Bit 31 - Selects the triggering condition for the RTC timer. See details in Table 1-2.
    #[inline(always)]
    #[must_use]
    pub fn time_update(&mut self) -> TIME_UPDATE_W<TIME_UPDATE_SPEC> {
        TIME_UPDATE_W::new(self, 31)
    }
}
/**RTC timer update control register

You can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIME_UPDATE_SPEC;
impl crate::RegisterSpec for TIME_UPDATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`time_update::R`](R) reader structure
impl crate::Readable for TIME_UPDATE_SPEC {}
///`write(|w| ..)` method takes [`time_update::W`](W) writer structure
impl crate::Writable for TIME_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIME_UPDATE to value 0
impl crate::Resettable for TIME_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
