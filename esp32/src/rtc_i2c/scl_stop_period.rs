#[doc = "Register `SCL_STOP_PERIOD` reader"]
pub type R = crate::R<SCL_STOP_PERIOD_SPEC>;
#[doc = "Register `SCL_STOP_PERIOD` writer"]
pub type W = crate::W<SCL_STOP_PERIOD_SPEC>;
#[doc = "Field `SCL_STOP_PERIOD` reader - Number of FAST_CLK cycles to wait before generating stop condition"]
pub type SCL_STOP_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `SCL_STOP_PERIOD` writer - Number of FAST_CLK cycles to wait before generating stop condition"]
pub type SCL_STOP_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles to wait before generating stop condition"]
    #[inline(always)]
    pub fn scl_stop_period(&self) -> SCL_STOP_PERIOD_R {
        SCL_STOP_PERIOD_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STOP_PERIOD")
            .field("scl_stop_period", &self.scl_stop_period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Number of FAST_CLK cycles to wait before generating stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn scl_stop_period(&mut self) -> SCL_STOP_PERIOD_W<SCL_STOP_PERIOD_SPEC> {
        SCL_STOP_PERIOD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STOP_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_STOP_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stop_period::R`](R) reader structure"]
impl crate::Readable for SCL_STOP_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stop_period::W`](W) writer structure"]
impl crate::Writable for SCL_STOP_PERIOD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_STOP_PERIOD to value 0"]
impl crate::Resettable for SCL_STOP_PERIOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
