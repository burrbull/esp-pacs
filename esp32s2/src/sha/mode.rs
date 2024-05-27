#[doc = "Register `MODE` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `MODE` reader - Defines the SHA algorithm."]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Defines the SHA algorithm."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Defines the SHA algorithm."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE").field("mode", &self.mode()).finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the SHA algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<MODE_SPEC> {
        MODE_W::new(self, 0)
    }
}
#[doc = "Defines the algorithm of SHA accelerator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
