///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `LP_CPU_EXC` reader - need_des
pub type LP_CPU_EXC_R = crate::BitReader;
///Field `LP_CPU_EXC` writer - need_des
pub type LP_CPU_EXC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_IDLE` reader - need_des
pub type SDIO_IDLE_R = crate::BitReader;
///Field `SDIO_IDLE` writer - need_des
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW` reader - need_des
pub type SW_R = crate::BitReader;
///Field `SW` writer - need_des
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOC_SLEEP_REJECT` reader - need_des
pub type SOC_SLEEP_REJECT_R = crate::BitReader;
///Field `SOC_SLEEP_REJECT` writer - need_des
pub type SOC_SLEEP_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOC_WAKEUP` reader - need_des
pub type SOC_WAKEUP_R = crate::BitReader;
///Field `SOC_WAKEUP` writer - need_des
pub type SOC_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn lp_cpu_exc(&self) -> LP_CPU_EXC_R {
        LP_CPU_EXC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn soc_sleep_reject(&self) -> SOC_SLEEP_REJECT_R {
        SOC_SLEEP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("lp_cpu_exc", &self.lp_cpu_exc())
            .field("sdio_idle", &self.sdio_idle())
            .field("sw", &self.sw())
            .field("soc_sleep_reject", &self.soc_sleep_reject())
            .field("soc_wakeup", &self.soc_wakeup())
            .finish()
    }
}
impl W {
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_exc(&mut self) -> LP_CPU_EXC_W<INT_ENA_SPEC> {
        LP_CPU_EXC_W::new(self, 27)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_ENA_SPEC> {
        SDIO_IDLE_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<INT_ENA_SPEC> {
        SW_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn soc_sleep_reject(&mut self) -> SOC_SLEEP_REJECT_W<INT_ENA_SPEC> {
        SOC_SLEEP_REJECT_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup(&mut self) -> SOC_WAKEUP_W<INT_ENA_SPEC> {
        SOC_WAKEUP_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
