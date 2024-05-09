#[doc = "Register `LACTHI` reader"]
pub type R = crate::R<LACTHI_SPEC>;
#[doc = "Field `HI` reader - Reserved."]
pub type HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTHI")
            .field("hi", &self.hi().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LACT high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTHI_SPEC;
impl crate::RegisterSpec for LACTHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lacthi::R`](R) reader structure"]
impl crate::Readable for LACTHI_SPEC {}
#[doc = "`reset()` method sets LACTHI to value 0"]
impl crate::Resettable for LACTHI_SPEC {}
