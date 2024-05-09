#[doc = "Register `LOG_DATA_1` reader"]
pub type R = crate::R<LOG_DATA_1_SPEC>;
#[doc = "Register `LOG_DATA_1` writer"]
pub type W = crate::W<LOG_DATA_1_SPEC>;
#[doc = "Field `LOG_DATA_1` reader - check data1"]
pub type LOG_DATA_1_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_DATA_1` writer - check data1"]
pub type LOG_DATA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - check data1"]
    #[inline(always)]
    pub fn log_data_1(&self) -> LOG_DATA_1_R {
        LOG_DATA_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_1")
            .field("log_data_1", &self.log_data_1().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - check data1"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_1(&mut self) -> LOG_DATA_1_W<LOG_DATA_1_SPEC> {
        LOG_DATA_1_W::new(self, 0)
    }
}
#[doc = "log check data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_1_SPEC;
impl crate::RegisterSpec for LOG_DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data_1::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data_1::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOG_DATA_1 to value 0"]
impl crate::Resettable for LOG_DATA_1_SPEC {}
