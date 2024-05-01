///Register `DMA_IN_ERR_EOF_DES_ADDR` reader
pub type R = crate::R<DMA_IN_ERR_EOF_DES_ADDR_SPEC>;
///Field `IN_ERR_EOF_DES_ADDR` reader - This register stores the address of the receive descriptor when there are some errors in this descriptor.
pub type IN_ERR_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register stores the address of the receive descriptor when there are some errors in this descriptor.
    #[inline(always)]
    pub fn in_err_eof_des_addr(&self) -> IN_ERR_EOF_DES_ADDR_R {
        IN_ERR_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_ERR_EOF_DES_ADDR")
            .field("in_err_eof_des_addr", &self.in_err_eof_des_addr())
            .finish()
    }
}
/**Inlink descriptor address when errors occur

You can [`read`](crate::generic::Reg::read) this register and get [`dma_in_err_eof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_IN_ERR_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for DMA_IN_ERR_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_in_err_eof_des_addr::R`](R) reader structure
impl crate::Readable for DMA_IN_ERR_EOF_DES_ADDR_SPEC {}
///`reset()` method sets DMA_IN_ERR_EOF_DES_ADDR to value 0
impl crate::Resettable for DMA_IN_ERR_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
