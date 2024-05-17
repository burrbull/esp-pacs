///Register `DMA_APBPERI_SPI2_PMS_CONSTRAIN_0` reader
pub type R = crate::R<DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>;
///Register `DMA_APBPERI_SPI2_PMS_CONSTRAIN_0` writer
pub type W = crate::W<DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC>;
///Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK` reader - dma_apbperi_spi2_pms_constrain_lock
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK_R = crate::BitReader;
///Field `DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK` writer - dma_apbperi_spi2_pms_constrain_lock
pub type DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - dma_apbperi_spi2_pms_constrain_lock
    #[inline(always)]
    pub fn dma_apbperi_spi2_pms_constrain_lock(
        &self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK_R {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_SPI2_PMS_CONSTRAIN_0")
            .field(
                "dma_apbperi_spi2_pms_constrain_lock",
                &self.dma_apbperi_spi2_pms_constrain_lock(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - dma_apbperi_spi2_pms_constrain_lock
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_spi2_pms_constrain_lock(
        &mut self,
    ) -> DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK_W<DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC> {
        DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK_W::new(self, 0)
    }
}
/**SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_REG

You can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_spi2_pms_constrain_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_spi2_pms_constrain_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_apbperi_spi2_pms_constrain_0::R`](R) reader structure
impl crate::Readable for DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC {}
///`write(|w| ..)` method takes [`dma_apbperi_spi2_pms_constrain_0::W`](W) writer structure
impl crate::Writable for DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 to value 0
impl crate::Resettable for DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
