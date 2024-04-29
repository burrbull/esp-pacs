#[doc = "Register `TX_SUC_EOF` reader"]
pub type R = crate::R<TX_SUC_EOF_SPEC>;
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
        f.debug_struct("TX_SUC_EOF")
            .field("des_addr", &format_args!("{}", self.des_addr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_SUC_EOF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_suc_eof::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_SUC_EOF_SPEC;
impl crate::RegisterSpec for TX_SUC_EOF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_suc_eof::R`](R) reader structure"]
impl crate::Readable for TX_SUC_EOF_SPEC {}
#[doc = "`reset()` method sets TX_SUC_EOF to value 0"]
impl crate::Resettable for TX_SUC_EOF_SPEC {
    const RESET_VALUE: u32 = 0;
}
