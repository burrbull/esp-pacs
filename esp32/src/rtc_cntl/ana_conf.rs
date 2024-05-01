///Register `ANA_CONF` reader
pub type R = crate::R<ANA_CONF_SPEC>;
///Register `ANA_CONF` writer
pub type W = crate::W<ANA_CONF_SPEC>;
///Field `PLLA_FORCE_PD` reader - PLLA force power down
pub type PLLA_FORCE_PD_R = crate::BitReader;
///Field `PLLA_FORCE_PD` writer - PLLA force power down
pub type PLLA_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLA_FORCE_PU` reader - PLLA force power up
pub type PLLA_FORCE_PU_R = crate::BitReader;
///Field `PLLA_FORCE_PU` writer - PLLA force power up
pub type PLLA_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BBPLL_CAL_SLP_START` reader - start BBPLL calibration during sleep
pub type BBPLL_CAL_SLP_START_R = crate::BitReader;
///Field `BBPLL_CAL_SLP_START` writer - start BBPLL calibration during sleep
pub type BBPLL_CAL_SLP_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVTMON_PU` reader - 1: PVTMON power up otherwise power down
pub type PVTMON_PU_R = crate::BitReader;
///Field `PVTMON_PU` writer - 1: PVTMON power up otherwise power down
pub type PVTMON_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXRF_I2C_PU` reader - 1: TXRF_I2C power up otherwise power down
pub type TXRF_I2C_PU_R = crate::BitReader;
///Field `TXRF_I2C_PU` writer - 1: TXRF_I2C power up otherwise power down
pub type TXRF_I2C_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFRX_PBUS_PU` reader - 1: RFRX_PBUS power up otherwise power down
pub type RFRX_PBUS_PU_R = crate::BitReader;
///Field `RFRX_PBUS_PU` writer - 1: RFRX_PBUS power up otherwise power down
pub type RFRX_PBUS_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKGEN_I2C_PU` reader - 1: CKGEN_I2C power up otherwise power down
pub type CKGEN_I2C_PU_R = crate::BitReader;
///Field `CKGEN_I2C_PU` writer - 1: CKGEN_I2C power up otherwise power down
pub type CKGEN_I2C_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL_I2C_PU` reader - 1: PLL_I2C power up otherwise power down
pub type PLL_I2C_PU_R = crate::BitReader;
///Field `PLL_I2C_PU` writer - 1: PLL_I2C power up otherwise power down
pub type PLL_I2C_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 23 - PLLA force power down
    #[inline(always)]
    pub fn plla_force_pd(&self) -> PLLA_FORCE_PD_R {
        PLLA_FORCE_PD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - PLLA force power up
    #[inline(always)]
    pub fn plla_force_pu(&self) -> PLLA_FORCE_PU_R {
        PLLA_FORCE_PU_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - start BBPLL calibration during sleep
    #[inline(always)]
    pub fn bbpll_cal_slp_start(&self) -> BBPLL_CAL_SLP_START_R {
        BBPLL_CAL_SLP_START_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - 1: PVTMON power up otherwise power down
    #[inline(always)]
    pub fn pvtmon_pu(&self) -> PVTMON_PU_R {
        PVTMON_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 1: TXRF_I2C power up otherwise power down
    #[inline(always)]
    pub fn txrf_i2c_pu(&self) -> TXRF_I2C_PU_R {
        TXRF_I2C_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - 1: RFRX_PBUS power up otherwise power down
    #[inline(always)]
    pub fn rfrx_pbus_pu(&self) -> RFRX_PBUS_PU_R {
        RFRX_PBUS_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - 1: CKGEN_I2C power up otherwise power down
    #[inline(always)]
    pub fn ckgen_i2c_pu(&self) -> CKGEN_I2C_PU_R {
        CKGEN_I2C_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - 1: PLL_I2C power up otherwise power down
    #[inline(always)]
    pub fn pll_i2c_pu(&self) -> PLL_I2C_PU_R {
        PLL_I2C_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_CONF")
            .field("plla_force_pd", &self.plla_force_pd())
            .field("plla_force_pu", &self.plla_force_pu())
            .field("bbpll_cal_slp_start", &self.bbpll_cal_slp_start())
            .field("pvtmon_pu", &self.pvtmon_pu())
            .field("txrf_i2c_pu", &self.txrf_i2c_pu())
            .field("rfrx_pbus_pu", &self.rfrx_pbus_pu())
            .field("ckgen_i2c_pu", &self.ckgen_i2c_pu())
            .field("pll_i2c_pu", &self.pll_i2c_pu())
            .finish()
    }
}
impl W {
    ///Bit 23 - PLLA force power down
    #[inline(always)]
    #[must_use]
    pub fn plla_force_pd(&mut self) -> PLLA_FORCE_PD_W<ANA_CONF_SPEC> {
        PLLA_FORCE_PD_W::new(self, 23)
    }
    ///Bit 24 - PLLA force power up
    #[inline(always)]
    #[must_use]
    pub fn plla_force_pu(&mut self) -> PLLA_FORCE_PU_W<ANA_CONF_SPEC> {
        PLLA_FORCE_PU_W::new(self, 24)
    }
    ///Bit 25 - start BBPLL calibration during sleep
    #[inline(always)]
    #[must_use]
    pub fn bbpll_cal_slp_start(&mut self) -> BBPLL_CAL_SLP_START_W<ANA_CONF_SPEC> {
        BBPLL_CAL_SLP_START_W::new(self, 25)
    }
    ///Bit 26 - 1: PVTMON power up otherwise power down
    #[inline(always)]
    #[must_use]
    pub fn pvtmon_pu(&mut self) -> PVTMON_PU_W<ANA_CONF_SPEC> {
        PVTMON_PU_W::new(self, 26)
    }
    ///Bit 27 - 1: TXRF_I2C power up otherwise power down
    #[inline(always)]
    #[must_use]
    pub fn txrf_i2c_pu(&mut self) -> TXRF_I2C_PU_W<ANA_CONF_SPEC> {
        TXRF_I2C_PU_W::new(self, 27)
    }
    ///Bit 28 - 1: RFRX_PBUS power up otherwise power down
    #[inline(always)]
    #[must_use]
    pub fn rfrx_pbus_pu(&mut self) -> RFRX_PBUS_PU_W<ANA_CONF_SPEC> {
        RFRX_PBUS_PU_W::new(self, 28)
    }
    ///Bit 30 - 1: CKGEN_I2C power up otherwise power down
    #[inline(always)]
    #[must_use]
    pub fn ckgen_i2c_pu(&mut self) -> CKGEN_I2C_PU_W<ANA_CONF_SPEC> {
        CKGEN_I2C_PU_W::new(self, 30)
    }
    ///Bit 31 - 1: PLL_I2C power up otherwise power down
    #[inline(always)]
    #[must_use]
    pub fn pll_i2c_pu(&mut self) -> PLL_I2C_PU_W<ANA_CONF_SPEC> {
        PLL_I2C_PU_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ANA_CONF_SPEC;
impl crate::RegisterSpec for ANA_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ana_conf::R`](R) reader structure
impl crate::Readable for ANA_CONF_SPEC {}
///`write(|w| ..)` method takes [`ana_conf::W`](W) writer structure
impl crate::Writable for ANA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ANA_CONF to value 0x0080_0000
impl crate::Resettable for ANA_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
