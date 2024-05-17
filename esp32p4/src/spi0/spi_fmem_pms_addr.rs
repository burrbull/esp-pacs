///Register `SPI_FMEM_PMS%s_ADDR` reader
pub type R = crate::R<SPI_FMEM_PMS_ADDR_SPEC>;
///Register `SPI_FMEM_PMS%s_ADDR` writer
pub type W = crate::W<SPI_FMEM_PMS_ADDR_SPEC>;
///Field `S` reader - SPI1 flash PMS section %s start address value
pub type S_R = crate::FieldReader<u32>;
///Field `S` writer - SPI1 flash PMS section %s start address value
pub type S_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 0:26 - SPI1 flash PMS section %s start address value
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FMEM_PMS_ADDR").field("s", &self.s()).finish()
    }
}
impl W {
    ///Bits 0:26 - SPI1 flash PMS section %s start address value
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<SPI_FMEM_PMS_ADDR_SPEC> {
        S_W::new(self, 0)
    }
}
/**SPI1 flash PMS section %s start address register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_fmem_pms_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_fmem_pms_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPI_FMEM_PMS_ADDR_SPEC;
impl crate::RegisterSpec for SPI_FMEM_PMS_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spi_fmem_pms_addr::R`](R) reader structure
impl crate::Readable for SPI_FMEM_PMS_ADDR_SPEC {}
///`write(|w| ..)` method takes [`spi_fmem_pms_addr::W`](W) writer structure
impl crate::Writable for SPI_FMEM_PMS_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_FMEM_PMS%s_ADDR to value 0
impl crate::Resettable for SPI_FMEM_PMS_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
