///Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5` reader
pub type R = crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>;
///Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5` writer
pub type W = crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0` reader - core_x_dram0_dma_sram_line_1_category_0
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0_R = crate::FieldReader;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0` writer - core_x_dram0_dma_sram_line_1_category_0
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1` reader - core_x_dram0_dma_sram_line_1_category_1
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1_R = crate::FieldReader;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1` writer - core_x_dram0_dma_sram_line_1_category_1
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2` reader - core_x_dram0_dma_sram_line_1_category_2
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2_R = crate::FieldReader;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2` writer - core_x_dram0_dma_sram_line_1_category_2
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR` reader - core_x_dram0_dma_sram_line_1_splitaddr
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR_R = crate::FieldReader;
///Field `CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR` writer - core_x_dram0_dma_sram_line_1_splitaddr
pub type CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - core_x_dram0_dma_sram_line_1_category_0
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_1_category_0(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - core_x_dram0_dma_sram_line_1_category_1
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_1_category_1(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - core_x_dram0_dma_sram_line_1_category_2
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_1_category_2(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 14:21 - core_x_dram0_dma_sram_line_1_splitaddr
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_1_splitaddr(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5")
            .field(
                "core_x_dram0_dma_sram_line_1_category_0",
                &self.core_x_dram0_dma_sram_line_1_category_0(),
            )
            .field(
                "core_x_dram0_dma_sram_line_1_category_1",
                &self.core_x_dram0_dma_sram_line_1_category_1(),
            )
            .field(
                "core_x_dram0_dma_sram_line_1_category_2",
                &self.core_x_dram0_dma_sram_line_1_category_2(),
            )
            .field(
                "core_x_dram0_dma_sram_line_1_splitaddr",
                &self.core_x_dram0_dma_sram_line_1_splitaddr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:1 - core_x_dram0_dma_sram_line_1_category_0
    #[inline(always)]
    #[must_use]
    pub fn core_x_dram0_dma_sram_line_1_category_0(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0_W::new(self, 0)
    }
    ///Bits 2:3 - core_x_dram0_dma_sram_line_1_category_1
    #[inline(always)]
    #[must_use]
    pub fn core_x_dram0_dma_sram_line_1_category_1(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1_W::new(self, 2)
    }
    ///Bits 4:5 - core_x_dram0_dma_sram_line_1_category_2
    #[inline(always)]
    #[must_use]
    pub fn core_x_dram0_dma_sram_line_1_category_2(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2_W::new(self, 4)
    }
    ///Bits 14:21 - core_x_dram0_dma_sram_line_1_splitaddr
    #[inline(always)]
    #[must_use]
    pub fn core_x_dram0_dma_sram_line_1_splitaddr(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR_W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC>
    {
        CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR_W::new(self, 14)
    }
}
/**SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_REG

You can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_dma_split_line_constrain_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_dma_split_line_constrain_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_x_iram0_dram0_dma_split_line_constrain_5::R`](R) reader structure
impl crate::Readable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC {}
///`write(|w| ..)` method takes [`core_x_iram0_dram0_dma_split_line_constrain_5::W`](W) writer structure
impl crate::Writable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 to value 0
impl crate::Resettable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
