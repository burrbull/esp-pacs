#[doc = "Register `DB_FED_CFG` reader"]
pub type R = crate::R<DB_FED_CFG_SPEC>;
#[doc = "Register `DB_FED_CFG` writer"]
pub type W = crate::W<DB_FED_CFG_SPEC>;
#[doc = "Field `FED` reader - Shadow register for FED"]
pub type FED_R = crate::FieldReader<u16>;
#[doc = "Field `FED` writer - Shadow register for FED"]
pub type FED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    pub fn fed(&self) -> FED_R {
        FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_FED_CFG")
            .field("fed", &self.fed().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DB_FED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for FED"]
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FED_W<DB_FED_CFG_SPEC> {
        FED_W::new(self, 0)
    }
}
#[doc = "Shadow register for falling edge delay (FED).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db_fed_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db_fed_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB_FED_CFG_SPEC;
impl crate::RegisterSpec for DB_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db_fed_cfg::R`](R) reader structure"]
impl crate::Readable for DB_FED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db_fed_cfg::W`](W) writer structure"]
impl crate::Writable for DB_FED_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DB_FED_CFG to value 0"]
impl crate::Resettable for DB_FED_CFG_SPEC {}
