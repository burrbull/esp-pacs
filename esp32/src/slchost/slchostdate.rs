#[doc = "Register `SLCHOSTDATE` reader"]
pub type R = crate::R<SLCHOSTDATE_SPEC>;
#[doc = "Register `SLCHOSTDATE` writer"]
pub type W = crate::W<SLCHOSTDATE_SPEC>;
#[doc = "Field `DATE` reader - "]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - "]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLCHOSTDATE")
            .field("date", &format_args!("{}", self.date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLCHOSTDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<SLCHOSTDATE_SPEC> {
        DATE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slchostdate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slchostdate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLCHOSTDATE_SPEC;
impl crate::RegisterSpec for SLCHOSTDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slchostdate::R`](R) reader structure"]
impl crate::Readable for SLCHOSTDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slchostdate::W`](W) writer structure"]
impl crate::Writable for SLCHOSTDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLCHOSTDATE to value 0x1602_2500"]
impl crate::Resettable for SLCHOSTDATE_SPEC {
    const RESET_VALUE: u32 = 0x1602_2500;
}
