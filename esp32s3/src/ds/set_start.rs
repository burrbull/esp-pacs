#[doc = "Register `SET_START` writer"]
pub type W = crate::W<SET_START_SPEC>;
#[doc = "Field `SET_START` writer - Write 1 to this register to active the DS peripheral"]
pub type SET_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this register to active the DS peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn set_start(&mut self) -> SET_START_W<SET_START_SPEC> {
        SET_START_W::new(self, 0)
    }
}
#[doc = "Activates the DS peripheral\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_START_SPEC;
impl crate::RegisterSpec for SET_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start::W`](W) writer structure"]
impl crate::Writable for SET_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START to value 0"]
impl crate::Resettable for SET_START_SPEC {}
