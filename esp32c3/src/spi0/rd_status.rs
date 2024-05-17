///Register `RD_STATUS` reader
pub type R = crate::R<RD_STATUS_SPEC>;
///Register `RD_STATUS` writer
pub type W = crate::W<RD_STATUS_SPEC>;
///Field `WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
pub type WB_MODE_R = crate::FieldReader;
///Field `WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
pub type WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_STATUS").field("wb_mode", &self.wb_mode()).finish()
    }
}
impl W {
    ///Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit.
    #[inline(always)]
    #[must_use]
    pub fn wb_mode(&mut self) -> WB_MODE_W<RD_STATUS_SPEC> {
        WB_MODE_W::new(self, 16)
    }
}
/**SPI0 read control register.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_status::R`](R) reader structure
impl crate::Readable for RD_STATUS_SPEC {}
///`write(|w| ..)` method takes [`rd_status::W`](W) writer structure
impl crate::Writable for RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RD_STATUS to value 0
impl crate::Resettable for RD_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
