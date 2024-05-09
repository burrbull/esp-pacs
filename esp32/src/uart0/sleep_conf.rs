#[doc = "Register `SLEEP_CONF` reader"]
pub type R = crate::R<SLEEP_CONF_SPEC>;
#[doc = "Register `SLEEP_CONF` writer"]
pub type W = crate::W<SLEEP_CONF_SPEC>;
#[doc = "Field `ACTIVE_THRESHOLD` reader - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
pub type ACTIVE_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE_THRESHOLD` writer - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
pub type ACTIVE_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ACTIVE_THRESHOLD_R {
        ACTIVE_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF")
            .field("active_threshold", &self.active_threshold().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLEEP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - When the input rxd edge changes more than this register value. the uart is active from light sleeping mode."]
    #[inline(always)]
    #[must_use]
    pub fn active_threshold(&mut self) -> ACTIVE_THRESHOLD_W<SLEEP_CONF_SPEC> {
        ACTIVE_THRESHOLD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF_SPEC;
impl crate::RegisterSpec for SLEEP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF to value 0xf0"]
impl crate::Resettable for SLEEP_CONF_SPEC {
    const RESET_VALUE: u32 = 0xf0;
}
