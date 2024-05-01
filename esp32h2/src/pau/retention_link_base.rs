///Register `RETENTION_LINK_BASE` reader
pub type R = crate::R<RETENTION_LINK_BASE_SPEC>;
///Register `RETENTION_LINK_BASE` writer
pub type W = crate::W<RETENTION_LINK_BASE_SPEC>;
///Field `LINK_BASE_ADDR` reader - retention dma link base
pub type LINK_BASE_ADDR_R = crate::FieldReader<u32>;
///Field `LINK_BASE_ADDR` writer - retention dma link base
pub type LINK_BASE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 0:26 - retention dma link base
    #[inline(always)]
    pub fn link_base_addr(&self) -> LINK_BASE_ADDR_R {
        LINK_BASE_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_LINK_BASE")
            .field("link_base_addr", &self.link_base_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:26 - retention dma link base
    #[inline(always)]
    #[must_use]
    pub fn link_base_addr(&mut self) -> LINK_BASE_ADDR_W<RETENTION_LINK_BASE_SPEC> {
        LINK_BASE_ADDR_W::new(self, 0)
    }
}
/**retention dma link base

You can [`read`](crate::generic::Reg::read) this register and get [`retention_link_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_link_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RETENTION_LINK_BASE_SPEC;
impl crate::RegisterSpec for RETENTION_LINK_BASE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`retention_link_base::R`](R) reader structure
impl crate::Readable for RETENTION_LINK_BASE_SPEC {}
///`write(|w| ..)` method takes [`retention_link_base::W`](W) writer structure
impl crate::Writable for RETENTION_LINK_BASE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RETENTION_LINK_BASE to value 0
impl crate::Resettable for RETENTION_LINK_BASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
