#[doc = "Register `HP_APM_M0_INTR_MAP` reader"]
pub type R = crate::R<HP_APM_M0_INTR_MAP_SPEC>;
#[doc = "Register `HP_APM_M0_INTR_MAP` writer"]
pub type W = crate::W<HP_APM_M0_INTR_MAP_SPEC>;
#[doc = "Field `HP_APM_M0_INTR_MAP` reader - CORE0_HP_APM_M0_INTR mapping register"]
pub type HP_APM_M0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `HP_APM_M0_INTR_MAP` writer - CORE0_HP_APM_M0_INTR mapping register"]
pub type HP_APM_M0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_HP_APM_M0_INTR mapping register"]
    #[inline(always)]
    pub fn hp_apm_m0_intr_map(&self) -> HP_APM_M0_INTR_MAP_R {
        HP_APM_M0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_APM_M0_INTR_MAP")
            .field("hp_apm_m0_intr_map", &self.hp_apm_m0_intr_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_APM_M0_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_HP_APM_M0_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn hp_apm_m0_intr_map(&mut self) -> HP_APM_M0_INTR_MAP_W<HP_APM_M0_INTR_MAP_SPEC> {
        HP_APM_M0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_apm_m0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_apm_m0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_APM_M0_INTR_MAP_SPEC;
impl crate::RegisterSpec for HP_APM_M0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_apm_m0_intr_map::R`](R) reader structure"]
impl crate::Readable for HP_APM_M0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_apm_m0_intr_map::W`](W) writer structure"]
impl crate::Writable for HP_APM_M0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_APM_M0_INTR_MAP to value 0"]
impl crate::Resettable for HP_APM_M0_INTR_MAP_SPEC {}
