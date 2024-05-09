#[doc = "Register `DMMU_TABLE5` reader"]
pub type R = crate::R<DMMU_TABLE5_SPEC>;
#[doc = "Register `DMMU_TABLE5` writer"]
pub type W = crate::W<DMMU_TABLE5_SPEC>;
#[doc = "Field `DMMU_TABLE5` reader - "]
pub type DMMU_TABLE5_R = crate::FieldReader;
#[doc = "Field `DMMU_TABLE5` writer - "]
pub type DMMU_TABLE5_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table5(&self) -> DMMU_TABLE5_R {
        DMMU_TABLE5_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_TABLE5")
            .field("dmmu_table5", &self.dmmu_table5().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_TABLE5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_table5(&mut self) -> DMMU_TABLE5_W<DMMU_TABLE5_SPEC> {
        DMMU_TABLE5_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_table5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_table5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_TABLE5_SPEC;
impl crate::RegisterSpec for DMMU_TABLE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_table5::R`](R) reader structure"]
impl crate::Readable for DMMU_TABLE5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_table5::W`](W) writer structure"]
impl crate::Writable for DMMU_TABLE5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMMU_TABLE5 to value 0x05"]
impl crate::Resettable for DMMU_TABLE5_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
