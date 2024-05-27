///Register `BACKUP_DMA_CFG2` reader
pub type R = crate::R<BACKUP_DMA_CFG2_SPEC>;
///Register `BACKUP_DMA_CFG2` writer
pub type W = crate::W<BACKUP_DMA_CFG2_SPEC>;
///Field `LINK_ADDR_AON` reader - need_des
pub type LINK_ADDR_AON_R = crate::FieldReader<u32>;
///Field `LINK_ADDR_AON` writer - need_des
pub type LINK_ADDR_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn link_addr_aon(&self) -> LINK_ADDR_AON_R {
        LINK_ADDR_AON_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_DMA_CFG2")
            .field("link_addr_aon", &self.link_addr_aon())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn link_addr_aon(&mut self) -> LINK_ADDR_AON_W<BACKUP_DMA_CFG2_SPEC> {
        LINK_ADDR_AON_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`backup_dma_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_dma_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BACKUP_DMA_CFG2_SPEC;
impl crate::RegisterSpec for BACKUP_DMA_CFG2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`backup_dma_cfg2::R`](R) reader structure
impl crate::Readable for BACKUP_DMA_CFG2_SPEC {}
///`write(|w| ..)` method takes [`backup_dma_cfg2::W`](W) writer structure
impl crate::Writable for BACKUP_DMA_CFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BACKUP_DMA_CFG2 to value 0
impl crate::Resettable for BACKUP_DMA_CFG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
