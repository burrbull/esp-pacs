#[doc = "Register `SCL_HIGH_PERIOD` reader"]
pub type R = crate::R<SCL_HIGH_PERIOD_SPEC>;
#[doc = "Register `SCL_HIGH_PERIOD` writer"]
pub type W = crate::W<SCL_HIGH_PERIOD_SPEC>;
#[doc = "Field `SCL_HIGH_PERIOD` reader - This register is used to configure the clock num during SCL is low level."]
pub type SCL_HIGH_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `SCL_HIGH_PERIOD` writer - This register is used to configure the clock num during SCL is low level."]
pub type SCL_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - This register is used to configure the clock num during SCL is low level."]
    #[inline(always)]
    pub fn scl_high_period(&self) -> SCL_HIGH_PERIOD_R {
        SCL_HIGH_PERIOD_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_HIGH_PERIOD")
            .field("scl_high_period", &self.scl_high_period().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_HIGH_PERIOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - This register is used to configure the clock num during SCL is low level."]
    #[inline(always)]
    #[must_use]
    pub fn scl_high_period(&mut self) -> SCL_HIGH_PERIOD_W<SCL_HIGH_PERIOD_SPEC> {
        SCL_HIGH_PERIOD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_high_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_HIGH_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_HIGH_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_high_period::R`](R) reader structure"]
impl crate::Readable for SCL_HIGH_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_high_period::W`](W) writer structure"]
impl crate::Writable for SCL_HIGH_PERIOD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_HIGH_PERIOD to value 0"]
impl crate::Resettable for SCL_HIGH_PERIOD_SPEC {}
