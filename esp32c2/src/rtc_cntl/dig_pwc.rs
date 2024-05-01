///Register `DIG_PWC` reader
pub type R = crate::R<DIG_PWC_SPEC>;
///Register `DIG_PWC` writer
pub type W = crate::W<DIG_PWC_SPEC>;
///Field `VDD_SPI_PWR_DRV` reader - Need add desc
pub type VDD_SPI_PWR_DRV_R = crate::FieldReader;
///Field `VDD_SPI_PWR_DRV` writer - Need add desc
pub type VDD_SPI_PWR_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VDD_SPI_PWR_FORCE` reader - Need add desc
pub type VDD_SPI_PWR_FORCE_R = crate::BitReader;
///Field `VDD_SPI_PWR_FORCE` writer - Need add desc
pub type VDD_SPI_PWR_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDD_SPI_PD_EN` reader - Need add desc
pub type VDD_SPI_PD_EN_R = crate::BitReader;
///Field `VDD_SPI_PD_EN` writer - Need add desc
pub type VDD_SPI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSLP_MEM_FORCE_PD` reader - memories in digital core force PD in sleep
pub type LSLP_MEM_FORCE_PD_R = crate::BitReader;
///Field `LSLP_MEM_FORCE_PD` writer - memories in digital core force PD in sleep
pub type LSLP_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSLP_MEM_FORCE_PU` reader - memories in digital core force no PD in sleep
pub type LSLP_MEM_FORCE_PU_R = crate::BitReader;
///Field `LSLP_MEM_FORCE_PU` writer - memories in digital core force no PD in sleep
pub type LSLP_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_WRAP_FORCE_PD` reader - digital core force power down
pub type DG_WRAP_FORCE_PD_R = crate::BitReader;
///Field `DG_WRAP_FORCE_PD` writer - digital core force power down
pub type DG_WRAP_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_WRAP_FORCE_PU` reader - digital core force power up
pub type DG_WRAP_FORCE_PU_R = crate::BitReader;
///Field `DG_WRAP_FORCE_PU` writer - digital core force power up
pub type DG_WRAP_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_WRAP_PD_EN` reader - Need add desc
pub type DG_WRAP_PD_EN_R = crate::BitReader;
///Field `DG_WRAP_PD_EN` writer - Need add desc
pub type DG_WRAP_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Need add desc
    #[inline(always)]
    pub fn vdd_spi_pwr_drv(&self) -> VDD_SPI_PWR_DRV_R {
        VDD_SPI_PWR_DRV_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Need add desc
    #[inline(always)]
    pub fn vdd_spi_pwr_force(&self) -> VDD_SPI_PWR_FORCE_R {
        VDD_SPI_PWR_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Need add desc
    #[inline(always)]
    pub fn vdd_spi_pd_en(&self) -> VDD_SPI_PD_EN_R {
        VDD_SPI_PD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - memories in digital core force PD in sleep
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - memories in digital core force no PD in sleep
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 19 - digital core force power down
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - digital core force power up
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 31 - Need add desc
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PWC")
            .field("vdd_spi_pwr_drv", &self.vdd_spi_pwr_drv())
            .field("vdd_spi_pwr_force", &self.vdd_spi_pwr_force())
            .field("vdd_spi_pd_en", &self.vdd_spi_pd_en())
            .field("lslp_mem_force_pd", &self.lslp_mem_force_pd())
            .field("lslp_mem_force_pu", &self.lslp_mem_force_pu())
            .field("dg_wrap_force_pd", &self.dg_wrap_force_pd())
            .field("dg_wrap_force_pu", &self.dg_wrap_force_pu())
            .field("dg_wrap_pd_en", &self.dg_wrap_pd_en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_drv(&mut self) -> VDD_SPI_PWR_DRV_W<DIG_PWC_SPEC> {
        VDD_SPI_PWR_DRV_W::new(self, 0)
    }
    ///Bit 2 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_force(&mut self) -> VDD_SPI_PWR_FORCE_W<DIG_PWC_SPEC> {
        VDD_SPI_PWR_FORCE_W::new(self, 2)
    }
    ///Bit 3 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pd_en(&mut self) -> VDD_SPI_PD_EN_W<DIG_PWC_SPEC> {
        VDD_SPI_PD_EN_W::new(self, 3)
    }
    ///Bit 4 - memories in digital core force PD in sleep
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W<DIG_PWC_SPEC> {
        LSLP_MEM_FORCE_PD_W::new(self, 4)
    }
    ///Bit 5 - memories in digital core force no PD in sleep
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W<DIG_PWC_SPEC> {
        LSLP_MEM_FORCE_PU_W::new(self, 5)
    }
    ///Bit 19 - digital core force power down
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W<DIG_PWC_SPEC> {
        DG_WRAP_FORCE_PD_W::new(self, 19)
    }
    ///Bit 20 - digital core force power up
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W<DIG_PWC_SPEC> {
        DG_WRAP_FORCE_PU_W::new(self, 20)
    }
    ///Bit 31 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W<DIG_PWC_SPEC> {
        DG_WRAP_PD_EN_W::new(self, 31)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dig_pwc::R`](R) reader structure
impl crate::Readable for DIG_PWC_SPEC {}
///`write(|w| ..)` method takes [`dig_pwc::W`](W) writer structure
impl crate::Writable for DIG_PWC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIG_PWC to value 0x0010_0020
impl crate::Resettable for DIG_PWC_SPEC {
    const RESET_VALUE: u32 = 0x0010_0020;
}
