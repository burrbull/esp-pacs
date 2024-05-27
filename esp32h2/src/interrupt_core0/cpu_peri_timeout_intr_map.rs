///Register `CPU_PERI_TIMEOUT_INTR_MAP` reader
pub type R = crate::R<CPU_PERI_TIMEOUT_INTR_MAP_SPEC>;
///Register `CPU_PERI_TIMEOUT_INTR_MAP` writer
pub type W = crate::W<CPU_PERI_TIMEOUT_INTR_MAP_SPEC>;
///Field `CPU_PERI_TIMEOUT_INTR_MAP` reader - CORE0_CPU_PERI_TIMEOUT_INTR mapping register
pub type CPU_PERI_TIMEOUT_INTR_MAP_R = crate::FieldReader;
///Field `CPU_PERI_TIMEOUT_INTR_MAP` writer - CORE0_CPU_PERI_TIMEOUT_INTR mapping register
pub type CPU_PERI_TIMEOUT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - CORE0_CPU_PERI_TIMEOUT_INTR mapping register
    #[inline(always)]
    pub fn cpu_peri_timeout_intr_map(&self) -> CPU_PERI_TIMEOUT_INTR_MAP_R {
        CPU_PERI_TIMEOUT_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_TIMEOUT_INTR_MAP")
            .field(
                "cpu_peri_timeout_intr_map",
                &self.cpu_peri_timeout_intr_map(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:4 - CORE0_CPU_PERI_TIMEOUT_INTR mapping register
    #[inline(always)]
    #[must_use]
    pub fn cpu_peri_timeout_intr_map(
        &mut self,
    ) -> CPU_PERI_TIMEOUT_INTR_MAP_W<CPU_PERI_TIMEOUT_INTR_MAP_SPEC> {
        CPU_PERI_TIMEOUT_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_timeout_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_timeout_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_PERI_TIMEOUT_INTR_MAP_SPEC;
impl crate::RegisterSpec for CPU_PERI_TIMEOUT_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_peri_timeout_intr_map::R`](R) reader structure
impl crate::Readable for CPU_PERI_TIMEOUT_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`cpu_peri_timeout_intr_map::W`](W) writer structure
impl crate::Writable for CPU_PERI_TIMEOUT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_PERI_TIMEOUT_INTR_MAP to value 0
impl crate::Resettable for CPU_PERI_TIMEOUT_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
