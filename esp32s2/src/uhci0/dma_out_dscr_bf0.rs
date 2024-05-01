///Register `DMA_OUT_DSCR_BF0` reader
pub type R = crate::R<DMA_OUT_DSCR_BF0_SPEC>;
///Field `OUTLINK_DSCR_BF0` reader - This register stores the third word of the current transmit descriptor.
pub type OUTLINK_DSCR_BF0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register stores the third word of the current transmit descriptor.
    #[inline(always)]
    pub fn outlink_dscr_bf0(&self) -> OUTLINK_DSCR_BF0_R {
        OUTLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_DSCR_BF0")
            .field("outlink_dscr_bf0", &self.outlink_dscr_bf0())
            .finish()
    }
}
/**The third word of current transmit descriptor

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_dscr_bf0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_OUT_DSCR_BF0_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_BF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_out_dscr_bf0::R`](R) reader structure
impl crate::Readable for DMA_OUT_DSCR_BF0_SPEC {}
///`reset()` method sets DMA_OUT_DSCR_BF0 to value 0
impl crate::Resettable for DMA_OUT_DSCR_BF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
