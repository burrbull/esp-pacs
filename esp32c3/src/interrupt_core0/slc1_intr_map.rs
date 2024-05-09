#[doc = "Register `SLC1_INTR_MAP` reader"]
pub type R = crate::R<SLC1_INTR_MAP_SPEC>;
#[doc = "Register `SLC1_INTR_MAP` writer"]
pub type W = crate::W<SLC1_INTR_MAP_SPEC>;
#[doc = "Field `SLC1_INTR_MAP` reader - reg_core0_slc1_intr_map"]
pub type SLC1_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `SLC1_INTR_MAP` writer - reg_core0_slc1_intr_map"]
pub type SLC1_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_slc1_intr_map"]
    #[inline(always)]
    pub fn slc1_intr_map(&self) -> SLC1_INTR_MAP_R {
        SLC1_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC1_INTR_MAP")
            .field("slc1_intr_map", &self.slc1_intr_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC1_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_slc1_intr_map"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_intr_map(&mut self) -> SLC1_INTR_MAP_W<SLC1_INTR_MAP_SPEC> {
        SLC1_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "slc1 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC1_INTR_MAP_SPEC;
impl crate::RegisterSpec for SLC1_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc1_intr_map::R`](R) reader structure"]
impl crate::Readable for SLC1_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc1_intr_map::W`](W) writer structure"]
impl crate::Writable for SLC1_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLC1_INTR_MAP to value 0"]
impl crate::Resettable for SLC1_INTR_MAP_SPEC {}
