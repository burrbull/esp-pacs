#[doc = "Register `LOADHI` reader"]
pub type R = crate::R<LOADHI_SPEC>;
#[doc = "Register `LOADHI` writer"]
pub type W = crate::W<LOADHI_SPEC>;
#[doc = "Field `LOAD_HI` reader - higher 32 bits of the value that will load into timer 0 time-base counter"]
pub type LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_HI` writer - higher 32 bits of the value that will load into timer 0 time-base counter"]
pub type LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 0 time-base counter"]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOADHI")
            .field("load_hi", &self.load_hi().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 0 time-base counter"]
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<LOADHI_SPEC> {
        LOAD_HI_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loadhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loadhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOADHI_SPEC;
impl crate::RegisterSpec for LOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loadhi::R`](R) reader structure"]
impl crate::Readable for LOADHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`loadhi::W`](W) writer structure"]
impl crate::Writable for LOADHI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOADHI to value 0"]
impl crate::Resettable for LOADHI_SPEC {}
