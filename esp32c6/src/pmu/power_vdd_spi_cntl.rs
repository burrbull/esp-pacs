///Register `POWER_VDD_SPI_CNTL` reader
pub type R = crate::R<POWER_VDD_SPI_CNTL_SPEC>;
///Register `POWER_VDD_SPI_CNTL` writer
pub type W = crate::W<POWER_VDD_SPI_CNTL_SPEC>;
///Field `VDD_SPI_PWR_WAIT` reader - need_des
pub type VDD_SPI_PWR_WAIT_R = crate::FieldReader<u16>;
///Field `VDD_SPI_PWR_WAIT` writer - need_des
pub type VDD_SPI_PWR_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `VDD_SPI_PWR_SW` reader - need_des
pub type VDD_SPI_PWR_SW_R = crate::FieldReader;
///Field `VDD_SPI_PWR_SW` writer - need_des
pub type VDD_SPI_PWR_SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VDD_SPI_PWR_SEL_SW` reader - need_des
pub type VDD_SPI_PWR_SEL_SW_R = crate::BitReader;
///Field `VDD_SPI_PWR_SEL_SW` writer - need_des
pub type VDD_SPI_PWR_SEL_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 18:28 - need_des
    #[inline(always)]
    pub fn vdd_spi_pwr_wait(&self) -> VDD_SPI_PWR_WAIT_R {
        VDD_SPI_PWR_WAIT_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    ///Bits 29:30 - need_des
    #[inline(always)]
    pub fn vdd_spi_pwr_sw(&self) -> VDD_SPI_PWR_SW_R {
        VDD_SPI_PWR_SW_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn vdd_spi_pwr_sel_sw(&self) -> VDD_SPI_PWR_SEL_SW_R {
        VDD_SPI_PWR_SEL_SW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_VDD_SPI_CNTL")
            .field("vdd_spi_pwr_wait", &self.vdd_spi_pwr_wait())
            .field("vdd_spi_pwr_sw", &self.vdd_spi_pwr_sw())
            .field("vdd_spi_pwr_sel_sw", &self.vdd_spi_pwr_sel_sw())
            .finish()
    }
}
impl W {
    ///Bits 18:28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_wait(&mut self) -> VDD_SPI_PWR_WAIT_W<POWER_VDD_SPI_CNTL_SPEC> {
        VDD_SPI_PWR_WAIT_W::new(self, 18)
    }
    ///Bits 29:30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_sw(&mut self) -> VDD_SPI_PWR_SW_W<POWER_VDD_SPI_CNTL_SPEC> {
        VDD_SPI_PWR_SW_W::new(self, 29)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_sel_sw(
        &mut self,
    ) -> VDD_SPI_PWR_SEL_SW_W<POWER_VDD_SPI_CNTL_SPEC> {
        VDD_SPI_PWR_SEL_SW_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_vdd_spi_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_vdd_spi_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POWER_VDD_SPI_CNTL_SPEC;
impl crate::RegisterSpec for POWER_VDD_SPI_CNTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`power_vdd_spi_cntl::R`](R) reader structure
impl crate::Readable for POWER_VDD_SPI_CNTL_SPEC {}
///`write(|w| ..)` method takes [`power_vdd_spi_cntl::W`](W) writer structure
impl crate::Writable for POWER_VDD_SPI_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER_VDD_SPI_CNTL to value 0x63fc_0000
impl crate::Resettable for POWER_VDD_SPI_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x63fc_0000;
}
