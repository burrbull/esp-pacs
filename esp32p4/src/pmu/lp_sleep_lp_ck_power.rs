///Register `LP_SLEEP_LP_CK_POWER` reader
pub type R = crate::R<LP_SLEEP_LP_CK_POWER_SPEC>;
///Register `LP_SLEEP_LP_CK_POWER` writer
pub type W = crate::W<LP_SLEEP_LP_CK_POWER_SPEC>;
///Field `LP_SLEEP_XPD_LPPLL` reader - need_des
pub type LP_SLEEP_XPD_LPPLL_R = crate::BitReader;
///Field `LP_SLEEP_XPD_LPPLL` writer - need_des
pub type LP_SLEEP_XPD_LPPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SLEEP_XPD_XTAL32K` reader - need_des
pub type LP_SLEEP_XPD_XTAL32K_R = crate::BitReader;
///Field `LP_SLEEP_XPD_XTAL32K` writer - need_des
pub type LP_SLEEP_XPD_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SLEEP_XPD_RC32K` reader - need_des
pub type LP_SLEEP_XPD_RC32K_R = crate::BitReader;
///Field `LP_SLEEP_XPD_RC32K` writer - need_des
pub type LP_SLEEP_XPD_RC32K_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SLEEP_XPD_FOSC_CLK` reader - need_des
pub type LP_SLEEP_XPD_FOSC_CLK_R = crate::BitReader;
///Field `LP_SLEEP_XPD_FOSC_CLK` writer - need_des
pub type LP_SLEEP_XPD_FOSC_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_SLEEP_PD_OSC_CLK` reader - need_des
pub type LP_SLEEP_PD_OSC_CLK_R = crate::BitReader;
///Field `LP_SLEEP_PD_OSC_CLK` writer - need_des
pub type LP_SLEEP_PD_OSC_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn lp_sleep_xpd_lppll(&self) -> LP_SLEEP_XPD_LPPLL_R {
        LP_SLEEP_XPD_LPPLL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn lp_sleep_xpd_xtal32k(&self) -> LP_SLEEP_XPD_XTAL32K_R {
        LP_SLEEP_XPD_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn lp_sleep_xpd_rc32k(&self) -> LP_SLEEP_XPD_RC32K_R {
        LP_SLEEP_XPD_RC32K_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn lp_sleep_xpd_fosc_clk(&self) -> LP_SLEEP_XPD_FOSC_CLK_R {
        LP_SLEEP_XPD_FOSC_CLK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_sleep_pd_osc_clk(&self) -> LP_SLEEP_PD_OSC_CLK_R {
        LP_SLEEP_PD_OSC_CLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_SLEEP_LP_CK_POWER")
            .field("lp_sleep_xpd_lppll", &self.lp_sleep_xpd_lppll())
            .field("lp_sleep_xpd_xtal32k", &self.lp_sleep_xpd_xtal32k())
            .field("lp_sleep_xpd_rc32k", &self.lp_sleep_xpd_rc32k())
            .field("lp_sleep_xpd_fosc_clk", &self.lp_sleep_xpd_fosc_clk())
            .field("lp_sleep_pd_osc_clk", &self.lp_sleep_pd_osc_clk())
            .finish()
    }
}
impl W {
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_xpd_lppll(&mut self) -> LP_SLEEP_XPD_LPPLL_W<LP_SLEEP_LP_CK_POWER_SPEC> {
        LP_SLEEP_XPD_LPPLL_W::new(self, 27)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_xpd_xtal32k(&mut self) -> LP_SLEEP_XPD_XTAL32K_W<LP_SLEEP_LP_CK_POWER_SPEC> {
        LP_SLEEP_XPD_XTAL32K_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_xpd_rc32k(&mut self) -> LP_SLEEP_XPD_RC32K_W<LP_SLEEP_LP_CK_POWER_SPEC> {
        LP_SLEEP_XPD_RC32K_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_xpd_fosc_clk(&mut self) -> LP_SLEEP_XPD_FOSC_CLK_W<LP_SLEEP_LP_CK_POWER_SPEC> {
        LP_SLEEP_XPD_FOSC_CLK_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_pd_osc_clk(&mut self) -> LP_SLEEP_PD_OSC_CLK_W<LP_SLEEP_LP_CK_POWER_SPEC> {
        LP_SLEEP_PD_OSC_CLK_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_ck_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_ck_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_SLEEP_LP_CK_POWER_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_CK_POWER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_sleep_lp_ck_power::R`](R) reader structure
impl crate::Readable for LP_SLEEP_LP_CK_POWER_SPEC {}
///`write(|w| ..)` method takes [`lp_sleep_lp_ck_power::W`](W) writer structure
impl crate::Writable for LP_SLEEP_LP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_SLEEP_LP_CK_POWER to value 0x4000_0000
impl crate::Resettable for LP_SLEEP_LP_CK_POWER_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
