///Register `IN_ERR_EOF_DES_ADDR` reader
pub type R = crate::R<IN_ERR_EOF_DES_ADDR_SPEC>;
///Field `DMA_IN_ERR_EOF_DES_ADDR` reader - The inlink descriptor address when spi dma produce receiving error.
pub type DMA_IN_ERR_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The inlink descriptor address when spi dma produce receiving error.
    #[inline(always)]
    pub fn dma_in_err_eof_des_addr(&self) -> DMA_IN_ERR_EOF_DES_ADDR_R {
        DMA_IN_ERR_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ERR_EOF_DES_ADDR")
            .field("dma_in_err_eof_des_addr", &self.dma_in_err_eof_des_addr())
            .finish()
    }
}
/**The latest SPI DMA RX descriptor address receiving error

You can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IN_ERR_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for IN_ERR_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`in_err_eof_des_addr::R`](R) reader structure
impl crate::Readable for IN_ERR_EOF_DES_ADDR_SPEC {}
///`reset()` method sets IN_ERR_EOF_DES_ADDR to value 0
impl crate::Resettable for IN_ERR_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
