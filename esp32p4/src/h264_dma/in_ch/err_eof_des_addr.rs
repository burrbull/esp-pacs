#[doc = "Register `ERR_EOF_DES_ADDR` reader"]
pub type R = crate::R<ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Field `IN_ERR_EOF_DES_ADDR` reader - This register stores the address of the inlink descriptor when there are some errors in current receiving data."]
pub type IN_ERR_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn in_err_eof_des_addr(&self) -> IN_ERR_EOF_DES_ADDR_R {
        IN_ERR_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_EOF_DES_ADDR")
            .field("in_err_eof_des_addr", &self.in_err_eof_des_addr())
            .finish()
    }
}
#[doc = "RX CHx err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_eof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for ERR_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for ERR_EOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets ERR_EOF_DES_ADDR to value 0"]
impl crate::Resettable for ERR_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
