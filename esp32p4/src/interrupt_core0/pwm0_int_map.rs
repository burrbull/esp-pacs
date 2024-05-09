#[doc = "Register `PWM0_INT_MAP` reader"]
pub type R = crate::R<PWM0_INT_MAP_SPEC>;
#[doc = "Register `PWM0_INT_MAP` writer"]
pub type W = crate::W<PWM0_INT_MAP_SPEC>;
#[doc = "Field `CORE0_PWM0_INT_MAP` reader - NA"]
pub type CORE0_PWM0_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE0_PWM0_INT_MAP` writer - NA"]
pub type CORE0_PWM0_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core0_pwm0_int_map(&self) -> CORE0_PWM0_INT_MAP_R {
        CORE0_PWM0_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0_INT_MAP")
            .field("core0_pwm0_int_map", &self.core0_pwm0_int_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWM0_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn core0_pwm0_int_map(&mut self) -> CORE0_PWM0_INT_MAP_W<PWM0_INT_MAP_SPEC> {
        CORE0_PWM0_INT_MAP_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm0_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm0_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM0_INT_MAP_SPEC;
impl crate::RegisterSpec for PWM0_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm0_int_map::R`](R) reader structure"]
impl crate::Readable for PWM0_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm0_int_map::W`](W) writer structure"]
impl crate::Writable for PWM0_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM0_INT_MAP to value 0"]
impl crate::Resettable for PWM0_INT_MAP_SPEC {}
