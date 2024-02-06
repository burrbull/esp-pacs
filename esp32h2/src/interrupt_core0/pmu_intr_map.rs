#[doc = "Register `PMU_INTR_MAP` reader"]
pub type R = crate::R<PMU_INTR_MAP_SPEC>;
#[doc = "Register `PMU_INTR_MAP` writer"]
pub type W = crate::W<PMU_INTR_MAP_SPEC>;
#[doc = "Field `PMU_INTR_MAP` reader - CORE0_PMU_INTR mapping register"]
pub type PMU_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PMU_INTR_MAP` writer - CORE0_PMU_INTR mapping register"]
pub type PMU_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_PMU_INTR mapping register"]
    #[inline(always)]
    pub fn pmu_intr_map(&self) -> PMU_INTR_MAP_R {
        PMU_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMU_INTR_MAP")
            .field(
                "pmu_intr_map",
                &format_args!("{}", self.pmu_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMU_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_PMU_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_intr_map(&mut self) -> PMU_INTR_MAP_W<PMU_INTR_MAP_SPEC> {
        PMU_INTR_MAP_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmu_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmu_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMU_INTR_MAP_SPEC;
impl crate::RegisterSpec for PMU_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_intr_map::R`](R) reader structure"]
impl crate::Readable for PMU_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmu_intr_map::W`](W) writer structure"]
impl crate::Writable for PMU_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMU_INTR_MAP to value 0"]
impl crate::Resettable for PMU_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
