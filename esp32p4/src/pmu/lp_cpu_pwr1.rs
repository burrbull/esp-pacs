///Register `LP_CPU_PWR1` writer
pub type W = crate::W<LP_CPU_PWR1_SPEC>;
///Field `LP_CPU_SLEEP_REQ` writer - need_des
pub type LP_CPU_SLEEP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CPU_PWR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_sleep_req(&mut self) -> LP_CPU_SLEEP_REQ_W<LP_CPU_PWR1_SPEC> {
        LP_CPU_SLEEP_REQ_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CPU_PWR1_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR1_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lp_cpu_pwr1::W`](W) writer structure
impl crate::Writable for LP_CPU_PWR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_CPU_PWR1 to value 0
impl crate::Resettable for LP_CPU_PWR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
