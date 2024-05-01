///Register `CPU_INTR_FROM_CPU_0_MAP` reader
pub type R = crate::R<CPU_INTR_FROM_CPU_0_MAP_SPEC>;
///Register `CPU_INTR_FROM_CPU_0_MAP` writer
pub type W = crate::W<CPU_INTR_FROM_CPU_0_MAP_SPEC>;
///Field `CPU_INTR_FROM_CPU_0_MAP` reader - reg_core0_cpu_intr_from_cpu_0_map
pub type CPU_INTR_FROM_CPU_0_MAP_R = crate::FieldReader;
///Field `CPU_INTR_FROM_CPU_0_MAP` writer - reg_core0_cpu_intr_from_cpu_0_map
pub type CPU_INTR_FROM_CPU_0_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - reg_core0_cpu_intr_from_cpu_0_map
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0_map(&self) -> CPU_INTR_FROM_CPU_0_MAP_R {
        CPU_INTR_FROM_CPU_0_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_0_MAP")
            .field("cpu_intr_from_cpu_0_map", &self.cpu_intr_from_cpu_0_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - reg_core0_cpu_intr_from_cpu_0_map
    #[inline(always)]
    #[must_use]
    pub fn cpu_intr_from_cpu_0_map(
        &mut self,
    ) -> CPU_INTR_FROM_CPU_0_MAP_W<CPU_INTR_FROM_CPU_0_MAP_SPEC> {
        CPU_INTR_FROM_CPU_0_MAP_W::new(self, 0)
    }
}
/**cpu from cpu 0 intr map register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_INTR_FROM_CPU_0_MAP_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_intr_from_cpu_0_map::R`](R) reader structure
impl crate::Readable for CPU_INTR_FROM_CPU_0_MAP_SPEC {}
///`write(|w| ..)` method takes [`cpu_intr_from_cpu_0_map::W`](W) writer structure
impl crate::Writable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_INTR_FROM_CPU_0_MAP to value 0
impl crate::Resettable for CPU_INTR_FROM_CPU_0_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
