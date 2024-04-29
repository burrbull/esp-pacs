#[doc = "Register `SLC%s_TX_ERREOF_DES_ADDR` reader"]
pub type R = crate::R<SLC_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = "Field `TX_ERR_EOF_DES_ADDR` reader - "]
pub type TX_ERR_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_err_eof_des_addr(&self) -> TX_ERR_EOF_DES_ADDR_R {
        TX_ERR_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TX_ERREOF_DES_ADDR")
            .field(
                "tx_err_eof_des_addr",
                &format_args!("{}", self.tx_err_eof_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TX_ERREOF_DES_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_erreof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TX_ERREOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for SLC_TX_ERREOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_tx_erreof_des_addr::R`](R) reader structure"]
impl crate::Readable for SLC_TX_ERREOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets SLC%s_TX_ERREOF_DES_ADDR to value 0"]
impl crate::Resettable for SLC_TX_ERREOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
