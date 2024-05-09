#[doc = "Register `LOG_MIN` reader"]
pub type R = crate::R<LOG_MIN_SPEC>;
#[doc = "Register `LOG_MIN` writer"]
pub type W = crate::W<LOG_MIN_SPEC>;
#[doc = "Field `LOG_MIN` reader - check region min addr"]
pub type LOG_MIN_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_MIN` writer - check region min addr"]
pub type LOG_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - check region min addr"]
    #[inline(always)]
    pub fn log_min(&self) -> LOG_MIN_R {
        LOG_MIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MIN")
            .field("log_min", &self.log_min().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - check region min addr"]
    #[inline(always)]
    #[must_use]
    pub fn log_min(&mut self) -> LOG_MIN_W<LOG_MIN_SPEC> {
        LOG_MIN_W::new(self, 0)
    }
}
#[doc = "log check region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_min::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_min::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_MIN_SPEC;
impl crate::RegisterSpec for LOG_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_min::R`](R) reader structure"]
impl crate::Readable for LOG_MIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_min::W`](W) writer structure"]
impl crate::Writable for LOG_MIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOG_MIN to value 0"]
impl crate::Resettable for LOG_MIN_SPEC {}
