///Register `DMA_TX_I_2` reader
pub type R = crate::R<DMA_TX_I_2_SPEC>;
///Register `DMA_TX_I_2` writer
pub type W = crate::W<DMA_TX_I_2_SPEC>;
///Field `DMA_TX_I_ILG_CLR` reader - The clear signal for TX Copy DMA access interrupt.
pub type DMA_TX_I_ILG_CLR_R = crate::BitReader;
///Field `DMA_TX_I_ILG_CLR` writer - The clear signal for TX Copy DMA access interrupt.
pub type DMA_TX_I_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_TX_I_ILG_EN` reader - The enable signal for TX Copy DMA access interrupt.
pub type DMA_TX_I_ILG_EN_R = crate::BitReader;
///Field `DMA_TX_I_ILG_EN` writer - The enable signal for TX Copy DMA access interrupt.
pub type DMA_TX_I_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_TX_I_ILG_INTR` reader - TX Copy DMA access interrupt signal.
pub type DMA_TX_I_ILG_INTR_R = crate::BitReader;
impl R {
    ///Bit 0 - The clear signal for TX Copy DMA access interrupt.
    #[inline(always)]
    pub fn dma_tx_i_ilg_clr(&self) -> DMA_TX_I_ILG_CLR_R {
        DMA_TX_I_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The enable signal for TX Copy DMA access interrupt.
    #[inline(always)]
    pub fn dma_tx_i_ilg_en(&self) -> DMA_TX_I_ILG_EN_R {
        DMA_TX_I_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX Copy DMA access interrupt signal.
    #[inline(always)]
    pub fn dma_tx_i_ilg_intr(&self) -> DMA_TX_I_ILG_INTR_R {
        DMA_TX_I_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TX_I_2")
            .field("dma_tx_i_ilg_clr", &self.dma_tx_i_ilg_clr())
            .field("dma_tx_i_ilg_en", &self.dma_tx_i_ilg_en())
            .field("dma_tx_i_ilg_intr", &self.dma_tx_i_ilg_intr())
            .finish()
    }
}
impl W {
    ///Bit 0 - The clear signal for TX Copy DMA access interrupt.
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_i_ilg_clr(&mut self) -> DMA_TX_I_ILG_CLR_W<DMA_TX_I_2_SPEC> {
        DMA_TX_I_ILG_CLR_W::new(self, 0)
    }
    ///Bit 1 - The enable signal for TX Copy DMA access interrupt.
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_i_ilg_en(&mut self) -> DMA_TX_I_ILG_EN_W<DMA_TX_I_2_SPEC> {
        DMA_TX_I_ILG_EN_W::new(self, 1)
    }
}
/**TX Copy DMA permission control register 2.

You can [`read`](crate::generic::Reg::read) this register and get [`dma_tx_i_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tx_i_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_TX_I_2_SPEC;
impl crate::RegisterSpec for DMA_TX_I_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_tx_i_2::R`](R) reader structure
impl crate::Readable for DMA_TX_I_2_SPEC {}
///`write(|w| ..)` method takes [`dma_tx_i_2::W`](W) writer structure
impl crate::Writable for DMA_TX_I_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_TX_I_2 to value 0
impl crate::Resettable for DMA_TX_I_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
