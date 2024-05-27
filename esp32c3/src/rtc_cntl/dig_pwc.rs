///Register `DIG_PWC` reader
pub type R = crate::R<DIG_PWC_SPEC>;
///Register `DIG_PWC` writer
pub type W = crate::W<DIG_PWC_SPEC>;
///Field `VDD_SPI_PWR_DRV` reader - vdd_spi drv's software value
pub type VDD_SPI_PWR_DRV_R = crate::FieldReader;
///Field `VDD_SPI_PWR_DRV` writer - vdd_spi drv's software value
pub type VDD_SPI_PWR_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VDD_SPI_PWR_FORCE` reader - vdd_spi drv use software value
pub type VDD_SPI_PWR_FORCE_R = crate::BitReader;
///Field `VDD_SPI_PWR_FORCE` writer - vdd_spi drv use software value
pub type VDD_SPI_PWR_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSLP_MEM_FORCE_PD` reader - memories in digital core force PD in sleep
pub type LSLP_MEM_FORCE_PD_R = crate::BitReader;
///Field `LSLP_MEM_FORCE_PD` writer - memories in digital core force PD in sleep
pub type LSLP_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSLP_MEM_FORCE_PU` reader - memories in digital core force PU in sleep
pub type LSLP_MEM_FORCE_PU_R = crate::BitReader;
///Field `LSLP_MEM_FORCE_PU` writer - memories in digital core force PU in sleep
pub type LSLP_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_FORCE_PD` reader - bt force power down
pub type BT_FORCE_PD_R = crate::BitReader;
///Field `BT_FORCE_PD` writer - bt force power down
pub type BT_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_FORCE_PU` reader - bt force power up
pub type BT_FORCE_PU_R = crate::BitReader;
///Field `BT_FORCE_PU` writer - bt force power up
pub type BT_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_PERI_FORCE_PD` reader - digital peri force power down
pub type DG_PERI_FORCE_PD_R = crate::BitReader;
///Field `DG_PERI_FORCE_PD` writer - digital peri force power down
pub type DG_PERI_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_PERI_FORCE_PU` reader - digital peri force power up
pub type DG_PERI_FORCE_PU_R = crate::BitReader;
///Field `DG_PERI_FORCE_PU` writer - digital peri force power up
pub type DG_PERI_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FASTMEM_FORCE_LPD` reader - fastmemory retention mode in sleep
pub type FASTMEM_FORCE_LPD_R = crate::BitReader;
///Field `FASTMEM_FORCE_LPD` writer - fastmemory retention mode in sleep
pub type FASTMEM_FORCE_LPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FASTMEM_FORCE_LPU` reader - fastmemory donlt entry retention mode in sleep
pub type FASTMEM_FORCE_LPU_R = crate::BitReader;
///Field `FASTMEM_FORCE_LPU` writer - fastmemory donlt entry retention mode in sleep
pub type FASTMEM_FORCE_LPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIFI_FORCE_PD` reader - wifi force power down
pub type WIFI_FORCE_PD_R = crate::BitReader;
///Field `WIFI_FORCE_PD` writer - wifi force power down
pub type WIFI_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIFI_FORCE_PU` reader - wifi force power up
pub type WIFI_FORCE_PU_R = crate::BitReader;
///Field `WIFI_FORCE_PU` writer - wifi force power up
pub type WIFI_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_WRAP_FORCE_PD` reader - digital core force power down
pub type DG_WRAP_FORCE_PD_R = crate::BitReader;
///Field `DG_WRAP_FORCE_PD` writer - digital core force power down
pub type DG_WRAP_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_WRAP_FORCE_PU` reader - digital core force power up
pub type DG_WRAP_FORCE_PU_R = crate::BitReader;
///Field `DG_WRAP_FORCE_PU` writer - digital core force power up
pub type DG_WRAP_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPU_TOP_FORCE_PD` reader - cpu core force power down
pub type CPU_TOP_FORCE_PD_R = crate::BitReader;
///Field `CPU_TOP_FORCE_PD` writer - cpu core force power down
pub type CPU_TOP_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPU_TOP_FORCE_PU` reader - cpu force power up
pub type CPU_TOP_FORCE_PU_R = crate::BitReader;
///Field `CPU_TOP_FORCE_PU` writer - cpu force power up
pub type CPU_TOP_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BT_PD_EN` reader - enable power down bt in sleep
pub type BT_PD_EN_R = crate::BitReader;
///Field `BT_PD_EN` writer - enable power down bt in sleep
pub type BT_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_PERI_PD_EN` reader - enable power down digital peri in sleep
pub type DG_PERI_PD_EN_R = crate::BitReader;
///Field `DG_PERI_PD_EN` writer - enable power down digital peri in sleep
pub type DG_PERI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPU_TOP_PD_EN` reader - enable power down cpu in sleep
pub type CPU_TOP_PD_EN_R = crate::BitReader;
///Field `CPU_TOP_PD_EN` writer - enable power down cpu in sleep
pub type CPU_TOP_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WIFI_PD_EN` reader - enable power down wifi in sleep
pub type WIFI_PD_EN_R = crate::BitReader;
///Field `WIFI_PD_EN` writer - enable power down wifi in sleep
pub type WIFI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DG_WRAP_PD_EN` reader - enable power down digital wrap in sleep
pub type DG_WRAP_PD_EN_R = crate::BitReader;
///Field `DG_WRAP_PD_EN` writer - enable power down digital wrap in sleep
pub type DG_WRAP_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - vdd_spi drv's software value
    #[inline(always)]
    pub fn vdd_spi_pwr_drv(&self) -> VDD_SPI_PWR_DRV_R {
        VDD_SPI_PWR_DRV_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - vdd_spi drv use software value
    #[inline(always)]
    pub fn vdd_spi_pwr_force(&self) -> VDD_SPI_PWR_FORCE_R {
        VDD_SPI_PWR_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - memories in digital core force PD in sleep
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - memories in digital core force PU in sleep
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 11 - bt force power down
    #[inline(always)]
    pub fn bt_force_pd(&self) -> BT_FORCE_PD_R {
        BT_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - bt force power up
    #[inline(always)]
    pub fn bt_force_pu(&self) -> BT_FORCE_PU_R {
        BT_FORCE_PU_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - digital peri force power down
    #[inline(always)]
    pub fn dg_peri_force_pd(&self) -> DG_PERI_FORCE_PD_R {
        DG_PERI_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - digital peri force power up
    #[inline(always)]
    pub fn dg_peri_force_pu(&self) -> DG_PERI_FORCE_PU_R {
        DG_PERI_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - fastmemory retention mode in sleep
    #[inline(always)]
    pub fn fastmem_force_lpd(&self) -> FASTMEM_FORCE_LPD_R {
        FASTMEM_FORCE_LPD_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - fastmemory donlt entry retention mode in sleep
    #[inline(always)]
    pub fn fastmem_force_lpu(&self) -> FASTMEM_FORCE_LPU_R {
        FASTMEM_FORCE_LPU_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - wifi force power down
    #[inline(always)]
    pub fn wifi_force_pd(&self) -> WIFI_FORCE_PD_R {
        WIFI_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - wifi force power up
    #[inline(always)]
    pub fn wifi_force_pu(&self) -> WIFI_FORCE_PU_R {
        WIFI_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 21 - cpu core force power down
    #[inline(always)]
    pub fn cpu_top_force_pd(&self) -> CPU_TOP_FORCE_PD_R {
        CPU_TOP_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - cpu force power up
    #[inline(always)]
    pub fn cpu_top_force_pu(&self) -> CPU_TOP_FORCE_PU_R {
        CPU_TOP_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - enable power down bt in sleep
    #[inline(always)]
    pub fn bt_pd_en(&self) -> BT_PD_EN_R {
        BT_PD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - enable power down digital peri in sleep
    #[inline(always)]
    pub fn dg_peri_pd_en(&self) -> DG_PERI_PD_EN_R {
        DG_PERI_PD_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - enable power down cpu in sleep
    #[inline(always)]
    pub fn cpu_top_pd_en(&self) -> CPU_TOP_PD_EN_R {
        CPU_TOP_PD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable power down wifi in sleep
    #[inline(always)]
    pub fn wifi_pd_en(&self) -> WIFI_PD_EN_R {
        WIFI_PD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - enable power down digital wrap in sleep
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
            .field("lslp_mem_force_pd", &self.lslp_mem_force_pd())
            .field("lslp_mem_force_pu", &self.lslp_mem_force_pu())
            .field("bt_force_pd", &self.bt_force_pd())
            .field("bt_force_pu", &self.bt_force_pu())
            .field("dg_peri_force_pd", &self.dg_peri_force_pd())
            .field("dg_peri_force_pu", &self.dg_peri_force_pu())
            .field("fastmem_force_lpd", &self.fastmem_force_lpd())
            .field("fastmem_force_lpu", &self.fastmem_force_lpu())
            .field("wifi_force_pd", &self.wifi_force_pd())
            .field("wifi_force_pu", &self.wifi_force_pu())
            .field("dg_wrap_force_pd", &self.dg_wrap_force_pd())
            .field("dg_wrap_force_pu", &self.dg_wrap_force_pu())
            .field("cpu_top_force_pd", &self.cpu_top_force_pd())
            .field("cpu_top_force_pu", &self.cpu_top_force_pu())
            .field("bt_pd_en", &self.bt_pd_en())
            .field("dg_peri_pd_en", &self.dg_peri_pd_en())
            .field("cpu_top_pd_en", &self.cpu_top_pd_en())
            .field("wifi_pd_en", &self.wifi_pd_en())
            .field("dg_wrap_pd_en", &self.dg_wrap_pd_en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - vdd_spi drv's software value
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_drv(&mut self) -> VDD_SPI_PWR_DRV_W<DIG_PWC_SPEC> {
        VDD_SPI_PWR_DRV_W::new(self, 0)
    }
    ///Bit 2 - vdd_spi drv use software value
    #[inline(always)]
    #[must_use]
    pub fn vdd_spi_pwr_force(&mut self) -> VDD_SPI_PWR_FORCE_W<DIG_PWC_SPEC> {
        VDD_SPI_PWR_FORCE_W::new(self, 2)
    }
    ///Bit 3 - memories in digital core force PD in sleep
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W<DIG_PWC_SPEC> {
        LSLP_MEM_FORCE_PD_W::new(self, 3)
    }
    ///Bit 4 - memories in digital core force PU in sleep
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W<DIG_PWC_SPEC> {
        LSLP_MEM_FORCE_PU_W::new(self, 4)
    }
    ///Bit 11 - bt force power down
    #[inline(always)]
    #[must_use]
    pub fn bt_force_pd(&mut self) -> BT_FORCE_PD_W<DIG_PWC_SPEC> {
        BT_FORCE_PD_W::new(self, 11)
    }
    ///Bit 12 - bt force power up
    #[inline(always)]
    #[must_use]
    pub fn bt_force_pu(&mut self) -> BT_FORCE_PU_W<DIG_PWC_SPEC> {
        BT_FORCE_PU_W::new(self, 12)
    }
    ///Bit 13 - digital peri force power down
    #[inline(always)]
    #[must_use]
    pub fn dg_peri_force_pd(&mut self) -> DG_PERI_FORCE_PD_W<DIG_PWC_SPEC> {
        DG_PERI_FORCE_PD_W::new(self, 13)
    }
    ///Bit 14 - digital peri force power up
    #[inline(always)]
    #[must_use]
    pub fn dg_peri_force_pu(&mut self) -> DG_PERI_FORCE_PU_W<DIG_PWC_SPEC> {
        DG_PERI_FORCE_PU_W::new(self, 14)
    }
    ///Bit 15 - fastmemory retention mode in sleep
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpd(&mut self) -> FASTMEM_FORCE_LPD_W<DIG_PWC_SPEC> {
        FASTMEM_FORCE_LPD_W::new(self, 15)
    }
    ///Bit 16 - fastmemory donlt entry retention mode in sleep
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpu(&mut self) -> FASTMEM_FORCE_LPU_W<DIG_PWC_SPEC> {
        FASTMEM_FORCE_LPU_W::new(self, 16)
    }
    ///Bit 17 - wifi force power down
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_pd(&mut self) -> WIFI_FORCE_PD_W<DIG_PWC_SPEC> {
        WIFI_FORCE_PD_W::new(self, 17)
    }
    ///Bit 18 - wifi force power up
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_pu(&mut self) -> WIFI_FORCE_PU_W<DIG_PWC_SPEC> {
        WIFI_FORCE_PU_W::new(self, 18)
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
    ///Bit 21 - cpu core force power down
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_force_pd(&mut self) -> CPU_TOP_FORCE_PD_W<DIG_PWC_SPEC> {
        CPU_TOP_FORCE_PD_W::new(self, 21)
    }
    ///Bit 22 - cpu force power up
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_force_pu(&mut self) -> CPU_TOP_FORCE_PU_W<DIG_PWC_SPEC> {
        CPU_TOP_FORCE_PU_W::new(self, 22)
    }
    ///Bit 27 - enable power down bt in sleep
    #[inline(always)]
    #[must_use]
    pub fn bt_pd_en(&mut self) -> BT_PD_EN_W<DIG_PWC_SPEC> {
        BT_PD_EN_W::new(self, 27)
    }
    ///Bit 28 - enable power down digital peri in sleep
    #[inline(always)]
    #[must_use]
    pub fn dg_peri_pd_en(&mut self) -> DG_PERI_PD_EN_W<DIG_PWC_SPEC> {
        DG_PERI_PD_EN_W::new(self, 28)
    }
    ///Bit 29 - enable power down cpu in sleep
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_pd_en(&mut self) -> CPU_TOP_PD_EN_W<DIG_PWC_SPEC> {
        CPU_TOP_PD_EN_W::new(self, 29)
    }
    ///Bit 30 - enable power down wifi in sleep
    #[inline(always)]
    #[must_use]
    pub fn wifi_pd_en(&mut self) -> WIFI_PD_EN_W<DIG_PWC_SPEC> {
        WIFI_PD_EN_W::new(self, 30)
    }
    ///Bit 31 - enable power down digital wrap in sleep
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W<DIG_PWC_SPEC> {
        DG_WRAP_PD_EN_W::new(self, 31)
    }
}
/**rtc configure register

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
///`reset()` method sets DIG_PWC to value 0x0055_5010
impl crate::Resettable for DIG_PWC_SPEC {
    const RESET_VALUE: u32 = 0x0055_5010;
}
