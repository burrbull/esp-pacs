///Register `DMA_TSTATUS` reader
pub type R = crate::R<DMA_TSTATUS_SPEC>;
///Field `DMA_IN_STATUS` reader - spi dma write data to memory status.
pub type DMA_IN_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - spi dma write data to memory status.
    #[inline(always)]
    pub fn dma_in_status(&self) -> DMA_IN_STATUS_R {
        DMA_IN_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TSTATUS")
            .field("dma_in_status", &self.dma_in_status())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma_tstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_TSTATUS_SPEC;
impl crate::RegisterSpec for DMA_TSTATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_tstatus::R`](R) reader structure
impl crate::Readable for DMA_TSTATUS_SPEC {}
///`reset()` method sets DMA_TSTATUS to value 0
impl crate::Resettable for DMA_TSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
