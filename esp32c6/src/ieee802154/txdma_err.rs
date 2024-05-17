///Register `TXDMA_ERR` reader
pub type R = crate::R<TXDMA_ERR_SPEC>;
///Register `TXDMA_ERR` writer
pub type W = crate::W<TXDMA_ERR_SPEC>;
///Field `TXDMA_ERR` reader -
pub type TXDMA_ERR_R = crate::FieldReader;
///Field `TXDMA_ERR` writer -
pub type TXDMA_ERR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn txdma_err(&self) -> TXDMA_ERR_R {
        TXDMA_ERR_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDMA_ERR").field("txdma_err", &self.txdma_err()).finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    #[must_use]
    pub fn txdma_err(&mut self) -> TXDMA_ERR_W<TXDMA_ERR_SPEC> {
        TXDMA_ERR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`txdma_err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdma_err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXDMA_ERR_SPEC;
impl crate::RegisterSpec for TXDMA_ERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`txdma_err::R`](R) reader structure
impl crate::Readable for TXDMA_ERR_SPEC {}
///`write(|w| ..)` method takes [`txdma_err::W`](W) writer structure
impl crate::Writable for TXDMA_ERR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXDMA_ERR to value 0
impl crate::Resettable for TXDMA_ERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
