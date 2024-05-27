///Register `CLK_CONF_FORCE_ON` reader
pub type R = crate::R<CLK_CONF_FORCE_ON_SPEC>;
///Register `CLK_CONF_FORCE_ON` writer
pub type W = crate::W<CLK_CONF_FORCE_ON_SPEC>;
///Field `CLK_ETM_FO` reader -
pub type CLK_ETM_FO_R = crate::BitReader;
///Field `CLK_ETM_FO` writer -
pub type CLK_ETM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_ZB_APB_FO` reader -
pub type CLK_ZB_APB_FO_R = crate::BitReader;
///Field `CLK_ZB_APB_FO` writer -
pub type CLK_ZB_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_ZB_MAC_FO` reader -
pub type CLK_ZB_MAC_FO_R = crate::BitReader;
///Field `CLK_ZB_MAC_FO` writer -
pub type CLK_ZB_MAC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_MODEM_SEC_ECB_FO` reader -
pub type CLK_MODEM_SEC_ECB_FO_R = crate::BitReader;
///Field `CLK_MODEM_SEC_ECB_FO` writer -
pub type CLK_MODEM_SEC_ECB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_MODEM_SEC_CCM_FO` reader -
pub type CLK_MODEM_SEC_CCM_FO_R = crate::BitReader;
///Field `CLK_MODEM_SEC_CCM_FO` writer -
pub type CLK_MODEM_SEC_CCM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_MODEM_SEC_BAH_FO` reader -
pub type CLK_MODEM_SEC_BAH_FO_R = crate::BitReader;
///Field `CLK_MODEM_SEC_BAH_FO` writer -
pub type CLK_MODEM_SEC_BAH_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_MODEM_SEC_APB_FO` reader -
pub type CLK_MODEM_SEC_APB_FO_R = crate::BitReader;
///Field `CLK_MODEM_SEC_APB_FO` writer -
pub type CLK_MODEM_SEC_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_MODEM_SEC_FO` reader -
pub type CLK_MODEM_SEC_FO_R = crate::BitReader;
///Field `CLK_MODEM_SEC_FO` writer -
pub type CLK_MODEM_SEC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_BLE_TIMER_FO` reader -
pub type CLK_BLE_TIMER_FO_R = crate::BitReader;
///Field `CLK_BLE_TIMER_FO` writer -
pub type CLK_BLE_TIMER_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DATA_DUMP_FO` reader -
pub type CLK_DATA_DUMP_FO_R = crate::BitReader;
///Field `CLK_DATA_DUMP_FO` writer -
pub type CLK_DATA_DUMP_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 22
    #[inline(always)]
    pub fn clk_etm_fo(&self) -> CLK_ETM_FO_R {
        CLK_ETM_FO_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23
    #[inline(always)]
    pub fn clk_zb_apb_fo(&self) -> CLK_ZB_APB_FO_R {
        CLK_ZB_APB_FO_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn clk_zb_mac_fo(&self) -> CLK_ZB_MAC_FO_R {
        CLK_ZB_MAC_FO_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25
    #[inline(always)]
    pub fn clk_modem_sec_ecb_fo(&self) -> CLK_MODEM_SEC_ECB_FO_R {
        CLK_MODEM_SEC_ECB_FO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26
    #[inline(always)]
    pub fn clk_modem_sec_ccm_fo(&self) -> CLK_MODEM_SEC_CCM_FO_R {
        CLK_MODEM_SEC_CCM_FO_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27
    #[inline(always)]
    pub fn clk_modem_sec_bah_fo(&self) -> CLK_MODEM_SEC_BAH_FO_R {
        CLK_MODEM_SEC_BAH_FO_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28
    #[inline(always)]
    pub fn clk_modem_sec_apb_fo(&self) -> CLK_MODEM_SEC_APB_FO_R {
        CLK_MODEM_SEC_APB_FO_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29
    #[inline(always)]
    pub fn clk_modem_sec_fo(&self) -> CLK_MODEM_SEC_FO_R {
        CLK_MODEM_SEC_FO_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30
    #[inline(always)]
    pub fn clk_ble_timer_fo(&self) -> CLK_BLE_TIMER_FO_R {
        CLK_BLE_TIMER_FO_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn clk_data_dump_fo(&self) -> CLK_DATA_DUMP_FO_R {
        CLK_DATA_DUMP_FO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_FORCE_ON")
            .field("clk_etm_fo", &self.clk_etm_fo())
            .field("clk_zb_apb_fo", &self.clk_zb_apb_fo())
            .field("clk_zb_mac_fo", &self.clk_zb_mac_fo())
            .field("clk_modem_sec_ecb_fo", &self.clk_modem_sec_ecb_fo())
            .field("clk_modem_sec_ccm_fo", &self.clk_modem_sec_ccm_fo())
            .field("clk_modem_sec_bah_fo", &self.clk_modem_sec_bah_fo())
            .field("clk_modem_sec_apb_fo", &self.clk_modem_sec_apb_fo())
            .field("clk_modem_sec_fo", &self.clk_modem_sec_fo())
            .field("clk_ble_timer_fo", &self.clk_ble_timer_fo())
            .field("clk_data_dump_fo", &self.clk_data_dump_fo())
            .finish()
    }
}
impl W {
    ///Bit 22
    #[inline(always)]
    #[must_use]
    pub fn clk_etm_fo(&mut self) -> CLK_ETM_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_ETM_FO_W::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    #[must_use]
    pub fn clk_zb_apb_fo(&mut self) -> CLK_ZB_APB_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_ZB_APB_FO_W::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    #[must_use]
    pub fn clk_zb_mac_fo(&mut self) -> CLK_ZB_MAC_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_ZB_MAC_FO_W::new(self, 24)
    }
    ///Bit 25
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_ecb_fo(&mut self) -> CLK_MODEM_SEC_ECB_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_ECB_FO_W::new(self, 25)
    }
    ///Bit 26
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_ccm_fo(&mut self) -> CLK_MODEM_SEC_CCM_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_CCM_FO_W::new(self, 26)
    }
    ///Bit 27
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_bah_fo(&mut self) -> CLK_MODEM_SEC_BAH_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_BAH_FO_W::new(self, 27)
    }
    ///Bit 28
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_apb_fo(&mut self) -> CLK_MODEM_SEC_APB_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_APB_FO_W::new(self, 28)
    }
    ///Bit 29
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_sec_fo(&mut self) -> CLK_MODEM_SEC_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_FO_W::new(self, 29)
    }
    ///Bit 30
    #[inline(always)]
    #[must_use]
    pub fn clk_ble_timer_fo(&mut self) -> CLK_BLE_TIMER_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_BLE_TIMER_FO_W::new(self, 30)
    }
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn clk_data_dump_fo(&mut self) -> CLK_DATA_DUMP_FO_W<CLK_CONF_FORCE_ON_SPEC> {
        CLK_DATA_DUMP_FO_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf_force_on::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf_force_on::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_CONF_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_CONF_FORCE_ON_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_conf_force_on::R`](R) reader structure
impl crate::Readable for CLK_CONF_FORCE_ON_SPEC {}
///`write(|w| ..)` method takes [`clk_conf_force_on::W`](W) writer structure
impl crate::Writable for CLK_CONF_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_CONF_FORCE_ON to value 0
impl crate::Resettable for CLK_CONF_FORCE_ON_SPEC {
    const RESET_VALUE: u32 = 0;
}
