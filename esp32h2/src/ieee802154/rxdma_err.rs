///Register `RXDMA_ERR` reader
pub type R = crate::R<RXDMA_ERR_SPEC>;
///Register `RXDMA_ERR` writer
pub type W = crate::W<RXDMA_ERR_SPEC>;
///Field `RXDMA_ERR` reader -
pub type RXDMA_ERR_R = crate::FieldReader;
///Field `RXDMA_ERR` writer -
pub type RXDMA_ERR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn rxdma_err(&self) -> RXDMA_ERR_R {
        RXDMA_ERR_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDMA_ERR")
            .field("rxdma_err", &self.rxdma_err())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    #[must_use]
    pub fn rxdma_err(&mut self) -> RXDMA_ERR_W<RXDMA_ERR_SPEC> {
        RXDMA_ERR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rxdma_err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RXDMA_ERR_SPEC;
impl crate::RegisterSpec for RXDMA_ERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rxdma_err::R`](R) reader structure
impl crate::Readable for RXDMA_ERR_SPEC {}
///`write(|w| ..)` method takes [`rxdma_err::W`](W) writer structure
impl crate::Writable for RXDMA_ERR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RXDMA_ERR to value 0
impl crate::Resettable for RXDMA_ERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
