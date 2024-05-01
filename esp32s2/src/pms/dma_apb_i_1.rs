///Register `DMA_APB_I_1` reader
pub type R = crate::R<DMA_APB_I_1_SPEC>;
///Register `DMA_APB_I_1` writer
pub type W = crate::W<DMA_APB_I_1_SPEC>;
///Field `DMA_APB_I_SRAM_0_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 0.
pub type DMA_APB_I_SRAM_0_R_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_0_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 0.
pub type DMA_APB_I_SRAM_0_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_0_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 0.
pub type DMA_APB_I_SRAM_0_W_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_0_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 0.
pub type DMA_APB_I_SRAM_0_W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_1_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 1.
pub type DMA_APB_I_SRAM_1_R_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_1_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 1.
pub type DMA_APB_I_SRAM_1_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_1_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 1.
pub type DMA_APB_I_SRAM_1_W_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_1_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 1.
pub type DMA_APB_I_SRAM_1_W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_2_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 2.
pub type DMA_APB_I_SRAM_2_R_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_2_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 2.
pub type DMA_APB_I_SRAM_2_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_2_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 2.
pub type DMA_APB_I_SRAM_2_W_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_2_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 2.
pub type DMA_APB_I_SRAM_2_W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_3_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 3.
pub type DMA_APB_I_SRAM_3_R_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_3_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 3.
pub type DMA_APB_I_SRAM_3_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_3_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 3.
pub type DMA_APB_I_SRAM_3_W_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_3_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 3.
pub type DMA_APB_I_SRAM_3_W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for internal DMA access.
pub type DMA_APB_I_SRAM_4_SPLTADDR_R = crate::FieldReader<u32>;
///Field `DMA_APB_I_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for internal DMA access.
pub type DMA_APB_I_SRAM_4_SPLTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
///Field `DMA_APB_I_SRAM_4_L_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region.
pub type DMA_APB_I_SRAM_4_L_R_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_4_L_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region.
pub type DMA_APB_I_SRAM_4_L_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_4_L_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region.
pub type DMA_APB_I_SRAM_4_L_W_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_4_L_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region.
pub type DMA_APB_I_SRAM_4_L_W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_4_H_R` reader - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region.
pub type DMA_APB_I_SRAM_4_H_R_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_4_H_R` writer - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region.
pub type DMA_APB_I_SRAM_4_H_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_APB_I_SRAM_4_H_W` reader - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region.
pub type DMA_APB_I_SRAM_4_H_W_R = crate::BitReader;
///Field `DMA_APB_I_SRAM_4_H_W` writer - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region.
pub type DMA_APB_I_SRAM_4_H_W_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Setting to 1 grants internal DMA permission to read SRAM Block 0.
    #[inline(always)]
    pub fn dma_apb_i_sram_0_r(&self) -> DMA_APB_I_SRAM_0_R_R {
        DMA_APB_I_SRAM_0_R_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Setting to 1 grants internal DMA permission to write SRAM Block 0.
    #[inline(always)]
    pub fn dma_apb_i_sram_0_w(&self) -> DMA_APB_I_SRAM_0_W_R {
        DMA_APB_I_SRAM_0_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Setting to 1 grants internal DMA permission to read SRAM Block 1.
    #[inline(always)]
    pub fn dma_apb_i_sram_1_r(&self) -> DMA_APB_I_SRAM_1_R_R {
        DMA_APB_I_SRAM_1_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Setting to 1 grants internal DMA permission to write SRAM Block 1.
    #[inline(always)]
    pub fn dma_apb_i_sram_1_w(&self) -> DMA_APB_I_SRAM_1_W_R {
        DMA_APB_I_SRAM_1_W_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Setting to 1 grants internal DMA permission to read SRAM Block 2.
    #[inline(always)]
    pub fn dma_apb_i_sram_2_r(&self) -> DMA_APB_I_SRAM_2_R_R {
        DMA_APB_I_SRAM_2_R_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Setting to 1 grants internal DMA permission to write SRAM Block 2.
    #[inline(always)]
    pub fn dma_apb_i_sram_2_w(&self) -> DMA_APB_I_SRAM_2_W_R {
        DMA_APB_I_SRAM_2_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Setting to 1 grants internal DMA permission to read SRAM Block 3.
    #[inline(always)]
    pub fn dma_apb_i_sram_3_r(&self) -> DMA_APB_I_SRAM_3_R_R {
        DMA_APB_I_SRAM_3_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Setting to 1 grants internal DMA permission to write SRAM Block 3.
    #[inline(always)]
    pub fn dma_apb_i_sram_3_w(&self) -> DMA_APB_I_SRAM_3_W_R {
        DMA_APB_I_SRAM_3_W_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:24 - Configure the split address of SRAM Block 4-21 for internal DMA access.
    #[inline(always)]
    pub fn dma_apb_i_sram_4_spltaddr(&self) -> DMA_APB_I_SRAM_4_SPLTADDR_R {
        DMA_APB_I_SRAM_4_SPLTADDR_R::new((self.bits >> 8) & 0x0001_ffff)
    }
    ///Bit 25 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region.
    #[inline(always)]
    pub fn dma_apb_i_sram_4_l_r(&self) -> DMA_APB_I_SRAM_4_L_R_R {
        DMA_APB_I_SRAM_4_L_R_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region.
    #[inline(always)]
    pub fn dma_apb_i_sram_4_l_w(&self) -> DMA_APB_I_SRAM_4_L_W_R {
        DMA_APB_I_SRAM_4_L_W_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region.
    #[inline(always)]
    pub fn dma_apb_i_sram_4_h_r(&self) -> DMA_APB_I_SRAM_4_H_R_R {
        DMA_APB_I_SRAM_4_H_R_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region.
    #[inline(always)]
    pub fn dma_apb_i_sram_4_h_w(&self) -> DMA_APB_I_SRAM_4_H_W_R {
        DMA_APB_I_SRAM_4_H_W_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APB_I_1")
            .field("dma_apb_i_sram_0_r", &self.dma_apb_i_sram_0_r())
            .field("dma_apb_i_sram_0_w", &self.dma_apb_i_sram_0_w())
            .field("dma_apb_i_sram_1_r", &self.dma_apb_i_sram_1_r())
            .field("dma_apb_i_sram_1_w", &self.dma_apb_i_sram_1_w())
            .field("dma_apb_i_sram_2_r", &self.dma_apb_i_sram_2_r())
            .field("dma_apb_i_sram_2_w", &self.dma_apb_i_sram_2_w())
            .field("dma_apb_i_sram_3_r", &self.dma_apb_i_sram_3_r())
            .field("dma_apb_i_sram_3_w", &self.dma_apb_i_sram_3_w())
            .field(
                "dma_apb_i_sram_4_spltaddr",
                &self.dma_apb_i_sram_4_spltaddr(),
            )
            .field("dma_apb_i_sram_4_l_r", &self.dma_apb_i_sram_4_l_r())
            .field("dma_apb_i_sram_4_l_w", &self.dma_apb_i_sram_4_l_w())
            .field("dma_apb_i_sram_4_h_r", &self.dma_apb_i_sram_4_h_r())
            .field("dma_apb_i_sram_4_h_w", &self.dma_apb_i_sram_4_h_w())
            .finish()
    }
}
impl W {
    ///Bit 0 - Setting to 1 grants internal DMA permission to read SRAM Block 0.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_0_r(&mut self) -> DMA_APB_I_SRAM_0_R_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_0_R_W::new(self, 0)
    }
    ///Bit 1 - Setting to 1 grants internal DMA permission to write SRAM Block 0.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_0_w(&mut self) -> DMA_APB_I_SRAM_0_W_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_0_W_W::new(self, 1)
    }
    ///Bit 2 - Setting to 1 grants internal DMA permission to read SRAM Block 1.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_1_r(&mut self) -> DMA_APB_I_SRAM_1_R_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_1_R_W::new(self, 2)
    }
    ///Bit 3 - Setting to 1 grants internal DMA permission to write SRAM Block 1.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_1_w(&mut self) -> DMA_APB_I_SRAM_1_W_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_1_W_W::new(self, 3)
    }
    ///Bit 4 - Setting to 1 grants internal DMA permission to read SRAM Block 2.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_2_r(&mut self) -> DMA_APB_I_SRAM_2_R_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_2_R_W::new(self, 4)
    }
    ///Bit 5 - Setting to 1 grants internal DMA permission to write SRAM Block 2.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_2_w(&mut self) -> DMA_APB_I_SRAM_2_W_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_2_W_W::new(self, 5)
    }
    ///Bit 6 - Setting to 1 grants internal DMA permission to read SRAM Block 3.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_3_r(&mut self) -> DMA_APB_I_SRAM_3_R_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_3_R_W::new(self, 6)
    }
    ///Bit 7 - Setting to 1 grants internal DMA permission to write SRAM Block 3.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_3_w(&mut self) -> DMA_APB_I_SRAM_3_W_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_3_W_W::new(self, 7)
    }
    ///Bits 8:24 - Configure the split address of SRAM Block 4-21 for internal DMA access.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_4_spltaddr(&mut self) -> DMA_APB_I_SRAM_4_SPLTADDR_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_4_SPLTADDR_W::new(self, 8)
    }
    ///Bit 25 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 low address region.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_4_l_r(&mut self) -> DMA_APB_I_SRAM_4_L_R_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_4_L_R_W::new(self, 25)
    }
    ///Bit 26 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 low address region.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_4_l_w(&mut self) -> DMA_APB_I_SRAM_4_L_W_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_4_L_W_W::new(self, 26)
    }
    ///Bit 27 - Setting to 1 grants internal DMA permission to read SRAM Block 4-21 high address region.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_4_h_r(&mut self) -> DMA_APB_I_SRAM_4_H_R_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_4_H_R_W::new(self, 27)
    }
    ///Bit 28 - Setting to 1 grants internal DMA permission to write SRAM Block 4-21 high address region.
    #[inline(always)]
    #[must_use]
    pub fn dma_apb_i_sram_4_h_w(&mut self) -> DMA_APB_I_SRAM_4_H_W_W<DMA_APB_I_1_SPEC> {
        DMA_APB_I_SRAM_4_H_W_W::new(self, 28)
    }
}
/**Internal DMA permission control register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`dma_apb_i_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apb_i_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_APB_I_1_SPEC;
impl crate::RegisterSpec for DMA_APB_I_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_apb_i_1::R`](R) reader structure
impl crate::Readable for DMA_APB_I_1_SPEC {}
///`write(|w| ..)` method takes [`dma_apb_i_1::W`](W) writer structure
impl crate::Writable for DMA_APB_I_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_APB_I_1 to value 0x1e00_00ff
impl crate::Resettable for DMA_APB_I_1_SPEC {
    const RESET_VALUE: u32 = 0x1e00_00ff;
}
