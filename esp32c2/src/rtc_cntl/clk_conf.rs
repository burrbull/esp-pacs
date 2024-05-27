///Register `CLK_CONF` reader
pub type R = crate::R<CLK_CONF_SPEC>;
///Register `CLK_CONF` writer
pub type W = crate::W<CLK_CONF_SPEC>;
///Field `EFUSE_CLK_FORCE_GATING` reader - Need add desc
pub type EFUSE_CLK_FORCE_GATING_R = crate::BitReader;
///Field `EFUSE_CLK_FORCE_GATING` writer - Need add desc
pub type EFUSE_CLK_FORCE_GATING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EFUSE_CLK_FORCE_NOGATING` reader - Need add desc
pub type EFUSE_CLK_FORCE_NOGATING_R = crate::BitReader;
///Field `EFUSE_CLK_FORCE_NOGATING` writer - Need add desc
pub type EFUSE_CLK_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK8M_DIV_SEL_VLD` reader - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel
pub type CK8M_DIV_SEL_VLD_R = crate::BitReader;
///Field `CK8M_DIV_SEL_VLD` writer - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel
pub type CK8M_DIV_SEL_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK8M_DIV` reader - CK8M_D256_OUT divider. 00: div128
pub type CK8M_DIV_R = crate::FieldReader;
///Field `CK8M_DIV` writer - CK8M_D256_OUT divider. 00: div128
pub type CK8M_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ENB_CK8M` reader - disable CK8M and CK8M_D256_OUT
pub type ENB_CK8M_R = crate::BitReader;
///Field `ENB_CK8M` writer - disable CK8M and CK8M_D256_OUT
pub type ENB_CK8M_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENB_CK8M_DIV` reader - 1: CK8M_D256_OUT is actually CK8M
pub type ENB_CK8M_DIV_R = crate::BitReader;
///Field `ENB_CK8M_DIV` writer - 1: CK8M_D256_OUT is actually CK8M
pub type ENB_CK8M_DIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIG_XTAL32K_EN` reader - enable CK_XTAL_32K for digital core (no relationship with RTC core)
pub type DIG_XTAL32K_EN_R = crate::BitReader;
///Field `DIG_XTAL32K_EN` writer - enable CK_XTAL_32K for digital core (no relationship with RTC core)
pub type DIG_XTAL32K_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIG_CLK8M_D256_EN` reader - enable CK8M_D256_OUT for digital core (no relationship with RTC core)
pub type DIG_CLK8M_D256_EN_R = crate::BitReader;
///Field `DIG_CLK8M_D256_EN` writer - enable CK8M_D256_OUT for digital core (no relationship with RTC core)
pub type DIG_CLK8M_D256_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIG_CLK8M_EN` reader - enable CK8M for digital core (no relationship with RTC core)
pub type DIG_CLK8M_EN_R = crate::BitReader;
///Field `DIG_CLK8M_EN` writer - enable CK8M for digital core (no relationship with RTC core)
pub type DIG_CLK8M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK8M_DIV_SEL` reader - divider = reg_ck8m_div_sel + 1
pub type CK8M_DIV_SEL_R = crate::FieldReader;
///Field `CK8M_DIV_SEL` writer - divider = reg_ck8m_div_sel + 1
pub type CK8M_DIV_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `XTAL_FORCE_NOGATING` reader - XTAL force no gating during sleep
pub type XTAL_FORCE_NOGATING_R = crate::BitReader;
///Field `XTAL_FORCE_NOGATING` writer - XTAL force no gating during sleep
pub type XTAL_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK8M_FORCE_NOGATING` reader - CK8M force no gating during sleep
pub type CK8M_FORCE_NOGATING_R = crate::BitReader;
///Field `CK8M_FORCE_NOGATING` writer - CK8M force no gating during sleep
pub type CK8M_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK8M_DFREQ` reader - CK8M_DFREQ
pub type CK8M_DFREQ_R = crate::FieldReader;
///Field `CK8M_DFREQ` writer - CK8M_DFREQ
pub type CK8M_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CK8M_FORCE_PD` reader - CK8M force power down
pub type CK8M_FORCE_PD_R = crate::BitReader;
///Field `CK8M_FORCE_PD` writer - CK8M force power down
pub type CK8M_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK8M_FORCE_PU` reader - CK8M force power up
pub type CK8M_FORCE_PU_R = crate::BitReader;
///Field `CK8M_FORCE_PU` writer - CK8M force power up
pub type CK8M_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTAL_GLOBAL_FORCE_GATING` reader - Need add desc
pub type XTAL_GLOBAL_FORCE_GATING_R = crate::BitReader;
///Field `XTAL_GLOBAL_FORCE_GATING` writer - Need add desc
pub type XTAL_GLOBAL_FORCE_GATING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTAL_GLOBAL_FORCE_NOGATING` reader - Need add desc
pub type XTAL_GLOBAL_FORCE_NOGATING_R = crate::BitReader;
///Field `XTAL_GLOBAL_FORCE_NOGATING` writer - Need add desc
pub type XTAL_GLOBAL_FORCE_NOGATING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAST_CLK_RTC_SEL` reader - fast_clk_rtc sel. 0: XTAL div 4
pub type FAST_CLK_RTC_SEL_R = crate::BitReader;
///Field `FAST_CLK_RTC_SEL` writer - fast_clk_rtc sel. 0: XTAL div 4
pub type FAST_CLK_RTC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANA_CLK_RTC_SEL` reader - Need add desc
pub type ANA_CLK_RTC_SEL_R = crate::FieldReader;
///Field `ANA_CLK_RTC_SEL` writer - Need add desc
pub type ANA_CLK_RTC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 1 - Need add desc
    #[inline(always)]
    pub fn efuse_clk_force_gating(&self) -> EFUSE_CLK_FORCE_GATING_R {
        EFUSE_CLK_FORCE_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Need add desc
    #[inline(always)]
    pub fn efuse_clk_force_nogating(&self) -> EFUSE_CLK_FORCE_NOGATING_R {
        EFUSE_CLK_FORCE_NOGATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel
    #[inline(always)]
    pub fn ck8m_div_sel_vld(&self) -> CK8M_DIV_SEL_VLD_R {
        CK8M_DIV_SEL_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - CK8M_D256_OUT divider. 00: div128
    #[inline(always)]
    pub fn ck8m_div(&self) -> CK8M_DIV_R {
        CK8M_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - disable CK8M and CK8M_D256_OUT
    #[inline(always)]
    pub fn enb_ck8m(&self) -> ENB_CK8M_R {
        ENB_CK8M_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1: CK8M_D256_OUT is actually CK8M
    #[inline(always)]
    pub fn enb_ck8m_div(&self) -> ENB_CK8M_DIV_R {
        ENB_CK8M_DIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)
    #[inline(always)]
    pub fn dig_xtal32k_en(&self) -> DIG_XTAL32K_EN_R {
        DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)
    #[inline(always)]
    pub fn dig_clk8m_d256_en(&self) -> DIG_CLK8M_D256_EN_R {
        DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - enable CK8M for digital core (no relationship with RTC core)
    #[inline(always)]
    pub fn dig_clk8m_en(&self) -> DIG_CLK8M_EN_R {
        DIG_CLK8M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 12:14 - divider = reg_ck8m_div_sel + 1
    #[inline(always)]
    pub fn ck8m_div_sel(&self) -> CK8M_DIV_SEL_R {
        CK8M_DIV_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - XTAL force no gating during sleep
    #[inline(always)]
    pub fn xtal_force_nogating(&self) -> XTAL_FORCE_NOGATING_R {
        XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CK8M force no gating during sleep
    #[inline(always)]
    pub fn ck8m_force_nogating(&self) -> CK8M_FORCE_NOGATING_R {
        CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:24 - CK8M_DFREQ
    #[inline(always)]
    pub fn ck8m_dfreq(&self) -> CK8M_DFREQ_R {
        CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    ///Bit 25 - CK8M force power down
    #[inline(always)]
    pub fn ck8m_force_pd(&self) -> CK8M_FORCE_PD_R {
        CK8M_FORCE_PD_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CK8M force power up
    #[inline(always)]
    pub fn ck8m_force_pu(&self) -> CK8M_FORCE_PU_R {
        CK8M_FORCE_PU_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Need add desc
    #[inline(always)]
    pub fn xtal_global_force_gating(&self) -> XTAL_GLOBAL_FORCE_GATING_R {
        XTAL_GLOBAL_FORCE_GATING_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Need add desc
    #[inline(always)]
    pub fn xtal_global_force_nogating(&self) -> XTAL_GLOBAL_FORCE_NOGATING_R {
        XTAL_GLOBAL_FORCE_NOGATING_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - fast_clk_rtc sel. 0: XTAL div 4
    #[inline(always)]
    pub fn fast_clk_rtc_sel(&self) -> FAST_CLK_RTC_SEL_R {
        FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Need add desc
    #[inline(always)]
    pub fn ana_clk_rtc_sel(&self) -> ANA_CLK_RTC_SEL_R {
        ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("efuse_clk_force_gating", &self.efuse_clk_force_gating())
            .field("efuse_clk_force_nogating", &self.efuse_clk_force_nogating())
            .field("ck8m_div_sel_vld", &self.ck8m_div_sel_vld())
            .field("ck8m_div", &self.ck8m_div())
            .field("enb_ck8m", &self.enb_ck8m())
            .field("enb_ck8m_div", &self.enb_ck8m_div())
            .field("dig_xtal32k_en", &self.dig_xtal32k_en())
            .field("dig_clk8m_d256_en", &self.dig_clk8m_d256_en())
            .field("dig_clk8m_en", &self.dig_clk8m_en())
            .field("ck8m_div_sel", &self.ck8m_div_sel())
            .field("xtal_force_nogating", &self.xtal_force_nogating())
            .field("ck8m_force_nogating", &self.ck8m_force_nogating())
            .field("ck8m_dfreq", &self.ck8m_dfreq())
            .field("ck8m_force_pd", &self.ck8m_force_pd())
            .field("ck8m_force_pu", &self.ck8m_force_pu())
            .field("xtal_global_force_gating", &self.xtal_global_force_gating())
            .field(
                "xtal_global_force_nogating",
                &self.xtal_global_force_nogating(),
            )
            .field("fast_clk_rtc_sel", &self.fast_clk_rtc_sel())
            .field("ana_clk_rtc_sel", &self.ana_clk_rtc_sel())
            .finish()
    }
}
impl W {
    ///Bit 1 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn efuse_clk_force_gating(&mut self) -> EFUSE_CLK_FORCE_GATING_W<CLK_CONF_SPEC> {
        EFUSE_CLK_FORCE_GATING_W::new(self, 1)
    }
    ///Bit 2 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn efuse_clk_force_nogating(&mut self) -> EFUSE_CLK_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        EFUSE_CLK_FORCE_NOGATING_W::new(self, 2)
    }
    ///Bit 3 - used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div_sel_vld(&mut self) -> CK8M_DIV_SEL_VLD_W<CLK_CONF_SPEC> {
        CK8M_DIV_SEL_VLD_W::new(self, 3)
    }
    ///Bits 4:5 - CK8M_D256_OUT divider. 00: div128
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div(&mut self) -> CK8M_DIV_W<CLK_CONF_SPEC> {
        CK8M_DIV_W::new(self, 4)
    }
    ///Bit 6 - disable CK8M and CK8M_D256_OUT
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m(&mut self) -> ENB_CK8M_W<CLK_CONF_SPEC> {
        ENB_CK8M_W::new(self, 6)
    }
    ///Bit 7 - 1: CK8M_D256_OUT is actually CK8M
    #[inline(always)]
    #[must_use]
    pub fn enb_ck8m_div(&mut self) -> ENB_CK8M_DIV_W<CLK_CONF_SPEC> {
        ENB_CK8M_DIV_W::new(self, 7)
    }
    ///Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)
    #[inline(always)]
    #[must_use]
    pub fn dig_xtal32k_en(&mut self) -> DIG_XTAL32K_EN_W<CLK_CONF_SPEC> {
        DIG_XTAL32K_EN_W::new(self, 8)
    }
    ///Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_d256_en(&mut self) -> DIG_CLK8M_D256_EN_W<CLK_CONF_SPEC> {
        DIG_CLK8M_D256_EN_W::new(self, 9)
    }
    ///Bit 10 - enable CK8M for digital core (no relationship with RTC core)
    #[inline(always)]
    #[must_use]
    pub fn dig_clk8m_en(&mut self) -> DIG_CLK8M_EN_W<CLK_CONF_SPEC> {
        DIG_CLK8M_EN_W::new(self, 10)
    }
    ///Bits 12:14 - divider = reg_ck8m_div_sel + 1
    #[inline(always)]
    #[must_use]
    pub fn ck8m_div_sel(&mut self) -> CK8M_DIV_SEL_W<CLK_CONF_SPEC> {
        CK8M_DIV_SEL_W::new(self, 12)
    }
    ///Bit 15 - XTAL force no gating during sleep
    #[inline(always)]
    #[must_use]
    pub fn xtal_force_nogating(&mut self) -> XTAL_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        XTAL_FORCE_NOGATING_W::new(self, 15)
    }
    ///Bit 16 - CK8M force no gating during sleep
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_nogating(&mut self) -> CK8M_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        CK8M_FORCE_NOGATING_W::new(self, 16)
    }
    ///Bits 17:24 - CK8M_DFREQ
    #[inline(always)]
    #[must_use]
    pub fn ck8m_dfreq(&mut self) -> CK8M_DFREQ_W<CLK_CONF_SPEC> {
        CK8M_DFREQ_W::new(self, 17)
    }
    ///Bit 25 - CK8M force power down
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pd(&mut self) -> CK8M_FORCE_PD_W<CLK_CONF_SPEC> {
        CK8M_FORCE_PD_W::new(self, 25)
    }
    ///Bit 26 - CK8M force power up
    #[inline(always)]
    #[must_use]
    pub fn ck8m_force_pu(&mut self) -> CK8M_FORCE_PU_W<CLK_CONF_SPEC> {
        CK8M_FORCE_PU_W::new(self, 26)
    }
    ///Bit 27 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn xtal_global_force_gating(&mut self) -> XTAL_GLOBAL_FORCE_GATING_W<CLK_CONF_SPEC> {
        XTAL_GLOBAL_FORCE_GATING_W::new(self, 27)
    }
    ///Bit 28 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn xtal_global_force_nogating(&mut self) -> XTAL_GLOBAL_FORCE_NOGATING_W<CLK_CONF_SPEC> {
        XTAL_GLOBAL_FORCE_NOGATING_W::new(self, 28)
    }
    ///Bit 29 - fast_clk_rtc sel. 0: XTAL div 4
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_rtc_sel(&mut self) -> FAST_CLK_RTC_SEL_W<CLK_CONF_SPEC> {
        FAST_CLK_RTC_SEL_W::new(self, 29)
    }
    ///Bits 30:31 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_rtc_sel(&mut self) -> ANA_CLK_RTC_SEL_W<CLK_CONF_SPEC> {
        ANA_CLK_RTC_SEL_W::new(self, 30)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_conf::R`](R) reader structure
impl crate::Readable for CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_CONF to value 0x1158_3218
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1158_3218;
}
