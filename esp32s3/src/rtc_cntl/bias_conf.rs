///Register `BIAS_CONF` reader
pub type R = crate::R<BIAS_CONF_SPEC>;
///Register `BIAS_CONF` writer
pub type W = crate::W<BIAS_CONF_SPEC>;
///Field `BIAS_BUF_IDLE` reader - No public
pub type BIAS_BUF_IDLE_R = crate::BitReader;
///Field `BIAS_BUF_IDLE` writer - No public
pub type BIAS_BUF_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_BUF_WAKE` reader - No public
pub type BIAS_BUF_WAKE_R = crate::BitReader;
///Field `BIAS_BUF_WAKE` writer - No public
pub type BIAS_BUF_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_BUF_DEEP_SLP` reader - No public
pub type BIAS_BUF_DEEP_SLP_R = crate::BitReader;
///Field `BIAS_BUF_DEEP_SLP` writer - No public
pub type BIAS_BUF_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_BUF_MONITOR` reader - No public
pub type BIAS_BUF_MONITOR_R = crate::BitReader;
///Field `BIAS_BUF_MONITOR` writer - No public
pub type BIAS_BUF_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_CUR_DEEP_SLP` reader - xpd cur when rtc in sleep_state
pub type PD_CUR_DEEP_SLP_R = crate::BitReader;
///Field `PD_CUR_DEEP_SLP` writer - xpd cur when rtc in sleep_state
pub type PD_CUR_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_CUR_MONITOR` reader - xpd cur when rtc in monitor state
pub type PD_CUR_MONITOR_R = crate::BitReader;
///Field `PD_CUR_MONITOR` writer - xpd cur when rtc in monitor state
pub type PD_CUR_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_SLEEP_DEEP_SLP` reader - bias_sleep when rtc in sleep_state
pub type BIAS_SLEEP_DEEP_SLP_R = crate::BitReader;
///Field `BIAS_SLEEP_DEEP_SLP` writer - bias_sleep when rtc in sleep_state
pub type BIAS_SLEEP_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_SLEEP_MONITOR` reader - bias_sleep when rtc in monitor state
pub type BIAS_SLEEP_MONITOR_R = crate::BitReader;
///Field `BIAS_SLEEP_MONITOR` writer - bias_sleep when rtc in monitor state
pub type BIAS_SLEEP_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_ATTEN_DEEP_SLP` reader - DBG_ATTEN when rtc in sleep state
pub type DBG_ATTEN_DEEP_SLP_R = crate::FieldReader;
///Field `DBG_ATTEN_DEEP_SLP` writer - DBG_ATTEN when rtc in sleep state
pub type DBG_ATTEN_DEEP_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DBG_ATTEN_MONITOR` reader - DBG_ATTEN when rtc in monitor state
pub type DBG_ATTEN_MONITOR_R = crate::FieldReader;
///Field `DBG_ATTEN_MONITOR` writer - DBG_ATTEN when rtc in monitor state
pub type DBG_ATTEN_MONITOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DBG_ATTEN_WAKEUP` reader - No public
pub type DBG_ATTEN_WAKEUP_R = crate::FieldReader;
///Field `DBG_ATTEN_WAKEUP` writer - No public
pub type DBG_ATTEN_WAKEUP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 10 - No public
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - No public
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - No public
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - No public
    #[inline(always)]
    pub fn bias_buf_monitor(&self) -> BIAS_BUF_MONITOR_R {
        BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - xpd cur when rtc in sleep_state
    #[inline(always)]
    pub fn pd_cur_deep_slp(&self) -> PD_CUR_DEEP_SLP_R {
        PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - xpd cur when rtc in monitor state
    #[inline(always)]
    pub fn pd_cur_monitor(&self) -> PD_CUR_MONITOR_R {
        PD_CUR_MONITOR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - bias_sleep when rtc in sleep_state
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&self) -> BIAS_SLEEP_DEEP_SLP_R {
        BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - bias_sleep when rtc in monitor state
    #[inline(always)]
    pub fn bias_sleep_monitor(&self) -> BIAS_SLEEP_MONITOR_R {
        BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:21 - DBG_ATTEN when rtc in sleep state
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&self) -> DBG_ATTEN_DEEP_SLP_R {
        DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:25 - DBG_ATTEN when rtc in monitor state
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bits 26:29 - No public
    #[inline(always)]
    pub fn dbg_atten_wakeup(&self) -> DBG_ATTEN_WAKEUP_R {
        DBG_ATTEN_WAKEUP_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIAS_CONF")
            .field("bias_buf_idle", &self.bias_buf_idle())
            .field("bias_buf_wake", &self.bias_buf_wake())
            .field("bias_buf_deep_slp", &self.bias_buf_deep_slp())
            .field("bias_buf_monitor", &self.bias_buf_monitor())
            .field("pd_cur_deep_slp", &self.pd_cur_deep_slp())
            .field("pd_cur_monitor", &self.pd_cur_monitor())
            .field("bias_sleep_deep_slp", &self.bias_sleep_deep_slp())
            .field("bias_sleep_monitor", &self.bias_sleep_monitor())
            .field("dbg_atten_deep_slp", &self.dbg_atten_deep_slp())
            .field("dbg_atten_monitor", &self.dbg_atten_monitor())
            .field("dbg_atten_wakeup", &self.dbg_atten_wakeup())
            .finish()
    }
}
impl W {
    ///Bit 10 - No public
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W<BIAS_CONF_SPEC> {
        BIAS_BUF_IDLE_W::new(self, 10)
    }
    ///Bit 11 - No public
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W<BIAS_CONF_SPEC> {
        BIAS_BUF_WAKE_W::new(self, 11)
    }
    ///Bit 12 - No public
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W<BIAS_CONF_SPEC> {
        BIAS_BUF_DEEP_SLP_W::new(self, 12)
    }
    ///Bit 13 - No public
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W<BIAS_CONF_SPEC> {
        BIAS_BUF_MONITOR_W::new(self, 13)
    }
    ///Bit 14 - xpd cur when rtc in sleep_state
    #[inline(always)]
    #[must_use]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W<BIAS_CONF_SPEC> {
        PD_CUR_DEEP_SLP_W::new(self, 14)
    }
    ///Bit 15 - xpd cur when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W<BIAS_CONF_SPEC> {
        PD_CUR_MONITOR_W::new(self, 15)
    }
    ///Bit 16 - bias_sleep when rtc in sleep_state
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W<BIAS_CONF_SPEC> {
        BIAS_SLEEP_DEEP_SLP_W::new(self, 16)
    }
    ///Bit 17 - bias_sleep when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W<BIAS_CONF_SPEC> {
        BIAS_SLEEP_MONITOR_W::new(self, 17)
    }
    ///Bits 18:21 - DBG_ATTEN when rtc in sleep state
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_DEEP_SLP_W::new(self, 18)
    }
    ///Bits 22:25 - DBG_ATTEN when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_MONITOR_W::new(self, 22)
    }
    ///Bits 26:29 - No public
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_wakeup(&mut self) -> DBG_ATTEN_WAKEUP_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_WAKEUP_W::new(self, 26)
    }
}
/**No public

You can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bias_conf::R`](R) reader structure
impl crate::Readable for BIAS_CONF_SPEC {}
///`write(|w| ..)` method takes [`bias_conf::W`](W) writer structure
impl crate::Writable for BIAS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BIAS_CONF to value 0x0001_0800
impl crate::Resettable for BIAS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_0800;
}
