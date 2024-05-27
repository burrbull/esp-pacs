#[doc = "Register `CORE_1_IRAM0_PMS_MONITOR_0` reader"]
pub type R = crate::R<CORE_1_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "Register `CORE_1_IRAM0_PMS_MONITOR_0` writer"]
pub type W = crate::W<CORE_1_IRAM0_PMS_MONITOR_0_SPEC>;
#[doc = "Field `CORE_1_IRAM0_PMS_MONITOR_LOCK` reader - Set 1 to lock core1 iram0 permission monitor register"]
pub type CORE_1_IRAM0_PMS_MONITOR_LOCK_R = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_PMS_MONITOR_LOCK` writer - Set 1 to lock core1 iram0 permission monitor register"]
pub type CORE_1_IRAM0_PMS_MONITOR_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock core1 iram0 permission monitor register"]
    #[inline(always)]
    pub fn core_1_iram0_pms_monitor_lock(&self) -> CORE_1_IRAM0_PMS_MONITOR_LOCK_R {
        CORE_1_IRAM0_PMS_MONITOR_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_IRAM0_PMS_MONITOR_0")
            .field(
                "core_1_iram0_pms_monitor_lock",
                &self.core_1_iram0_pms_monitor_lock(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock core1 iram0 permission monitor register"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_iram0_pms_monitor_lock(
        &mut self,
    ) -> CORE_1_IRAM0_PMS_MONITOR_LOCK_W<CORE_1_IRAM0_PMS_MONITOR_0_SPEC> {
        CORE_1_IRAM0_PMS_MONITOR_LOCK_W::new(self, 0)
    }
}
#[doc = "core1 iram0 permission monitor configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_pms_monitor_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_iram0_pms_monitor_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_IRAM0_PMS_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_1_IRAM0_PMS_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_iram0_pms_monitor_0::R`](R) reader structure"]
impl crate::Readable for CORE_1_IRAM0_PMS_MONITOR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_iram0_pms_monitor_0::W`](W) writer structure"]
impl crate::Writable for CORE_1_IRAM0_PMS_MONITOR_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_IRAM0_PMS_MONITOR_0 to value 0"]
impl crate::Resettable for CORE_1_IRAM0_PMS_MONITOR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
