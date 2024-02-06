#[doc = "Register `HCDMAB3` reader"]
pub type R = crate::R<HCDMAB3_SPEC>;
#[doc = "Field `H_HCDMAB3` reader - "]
pub type H_HCDMAB3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_hcdmab3(&self) -> H_HCDMAB3_R {
        H_HCDMAB3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB3")
            .field("h_hcdmab3", &format_args!("{}", self.h_hcdmab3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMAB3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB3_SPEC;
impl crate::RegisterSpec for HCDMAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab3::R`](R) reader structure"]
impl crate::Readable for HCDMAB3_SPEC {}
#[doc = "`reset()` method sets HCDMAB3 to value 0"]
impl crate::Resettable for HCDMAB3_SPEC {
    const RESET_VALUE: u32 = 0;
}
