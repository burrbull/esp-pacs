#[doc = "Register `SET_INVALIDATE_DS` writer"]
pub type W = crate::W<SET_INVALIDATE_DS_SPEC>;
#[doc = "Field `SET_INVALIDATE_DS` writer - Clear result from hmac downstream DS."]
pub type SET_INVALIDATE_DS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_INVALIDATE_DS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear result from hmac downstream DS."]
    #[inline(always)]
    #[must_use]
    pub fn set_invalidate_ds(&mut self) -> SET_INVALIDATE_DS_W<SET_INVALIDATE_DS_SPEC> {
        SET_INVALIDATE_DS_W::new(self, 0)
    }
}
#[doc = "Invalidate register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_invalidate_ds::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_INVALIDATE_DS_SPEC;
impl crate::RegisterSpec for SET_INVALIDATE_DS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_invalidate_ds::W`](W) writer structure"]
impl crate::Writable for SET_INVALIDATE_DS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_INVALIDATE_DS to value 0"]
impl crate::Resettable for SET_INVALIDATE_DS_SPEC {}
