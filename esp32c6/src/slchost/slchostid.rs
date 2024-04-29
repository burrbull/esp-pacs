#[doc = "Register `SLCHOSTID` reader"]
pub type R = crate::R<SLCHOSTID_SPEC>;
#[doc = "Register `SLCHOSTID` writer"]
pub type W = crate::W<SLCHOSTID_SPEC>;
#[doc = "Field `ID` reader - *******Description***********"]
pub type ID_R = crate::FieldReader<u32>;
#[doc = "Field `ID` writer - *******Description***********"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLCHOSTID")
            .field("id", &format_args!("{}", self.id().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLCHOSTID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<SLCHOSTID_SPEC> {
        ID_W::new(self, 0)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slchostid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slchostid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLCHOSTID_SPEC;
impl crate::RegisterSpec for SLCHOSTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slchostid::R`](R) reader structure"]
impl crate::Readable for SLCHOSTID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slchostid::W`](W) writer structure"]
impl crate::Writable for SLCHOSTID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLCHOSTID to value 0x0600"]
impl crate::Resettable for SLCHOSTID_SPEC {
    const RESET_VALUE: u32 = 0x0600;
}
