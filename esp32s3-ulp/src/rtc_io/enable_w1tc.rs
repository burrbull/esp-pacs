#[doc = "Register `ENABLE_W1TC` writer"]
pub type W = crate::W<ENABLE_W1TC_SPEC>;
#[doc = "Field `GPIO_ENABLE_W1TC` writer - RTC GPIO 0 ~ 21 enable write 1 to clear"]
pub type GPIO_ENABLE_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - RTC GPIO 0 ~ 21 enable write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_enable_w1tc(&mut self) -> GPIO_ENABLE_W1TC_W<ENABLE_W1TC_SPEC> {
        GPIO_ENABLE_W1TC_W::new(self, 10)
    }
}
#[doc = "one clear RTC GPIO output enable\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable_w1tc::W`](W) writer structure"]
impl crate::Writable for ENABLE_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE_W1TC to value 0"]
impl crate::Resettable for ENABLE_W1TC_SPEC {}
