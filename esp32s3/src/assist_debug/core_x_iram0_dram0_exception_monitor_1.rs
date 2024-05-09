#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Register `CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1` writer"]
pub type W = crate::W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` reader - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1` writer - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
pub type CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
    #[inline(always)]
    pub fn core_x_iram0_dram0_limit_cycle_1(&self) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_x_iram0_dram0_limit_cycle_1",
                &self.core_x_iram0_dram0_limit_cycle_1().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - non busy cycle,for example: when cycle=100 and cycle=10,it means that in 100 cycle, if busy access success time less than 10, it will trigger interrutpt"]
    #[inline(always)]
    #[must_use]
    pub fn core_x_iram0_dram0_limit_cycle_1(
        &mut self,
    ) -> CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC> {
        CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1_W::new(self, 0)
    }
}
#[doc = "bus busy configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_x_iram0_dram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_x_iram0_dram0_exception_monitor_1::W`](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 to value 0x000f_ffff"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
