///Register `RD_MAC_SPI_SYS_2` reader
pub type R = crate::R<RD_MAC_SPI_SYS_2_SPEC>;
///Field `SPI_PAD_CONF_1` reader - Stores the first part of SPI_PAD_CONF.
pub type SPI_PAD_CONF_1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the first part of SPI_PAD_CONF.
    #[inline(always)]
    pub fn spi_pad_conf_1(&self) -> SPI_PAD_CONF_1_R {
        SPI_PAD_CONF_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SPI_SYS_2")
            .field("spi_pad_conf_1", &self.spi_pad_conf_1())
            .finish()
    }
}
/**BLOCK1 data register 2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_MAC_SPI_SYS_2_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_mac_spi_sys_2::R`](R) reader structure
impl crate::Readable for RD_MAC_SPI_SYS_2_SPEC {}
///`reset()` method sets RD_MAC_SPI_SYS_2 to value 0
impl crate::Resettable for RD_MAC_SPI_SYS_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
