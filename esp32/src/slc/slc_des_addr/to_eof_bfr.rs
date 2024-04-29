#[doc = "Register `TO_EOF_BFR` reader"]
pub type R = crate::R<TO_EOF_BFR_SPEC>;
#[doc = "Field `DES_ADDR` reader - "]
pub type DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn des_addr(&self) -> DES_ADDR_R {
        DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO_EOF_BFR")
            .field("des_addr", &format_args!("{}", self.des_addr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TO_EOF_BFR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to_eof_bfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TO_EOF_BFR_SPEC;
impl crate::RegisterSpec for TO_EOF_BFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`to_eof_bfr::R`](R) reader structure"]
impl crate::Readable for TO_EOF_BFR_SPEC {}
#[doc = "`reset()` method sets TO_EOF_BFR to value 0"]
impl crate::Resettable for TO_EOF_BFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
