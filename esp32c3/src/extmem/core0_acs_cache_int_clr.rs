///Register `CORE0_ACS_CACHE_INT_CLR` writer
pub type W = crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>;
///Field `CORE0_IBUS_ACS_MSK_IC` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access.
pub type CORE0_IBUS_ACS_MSK_IC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CORE0_IBUS_WR_IC` writer - The bit is used to clear interrupt by ibus trying to write icache
pub type CORE0_IBUS_WR_IC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CORE0_IBUS_REJECT` writer - The bit is used to clear interrupt by authentication fail.
pub type CORE0_IBUS_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CORE0_DBUS_ACS_MSK_IC` writer - The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access.
pub type CORE0_DBUS_ACS_MSK_IC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CORE0_DBUS_REJECT` writer - The bit is used to clear interrupt by authentication fail.
pub type CORE0_DBUS_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CORE0_DBUS_WR_IC` writer - The bit is used to clear interrupt by dbus trying to write icache
pub type CORE0_DBUS_WR_IC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE0_ACS_CACHE_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access.
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_acs_msk_ic(
        &mut self,
    ) -> CORE0_IBUS_ACS_MSK_IC_W<CORE0_ACS_CACHE_INT_CLR_SPEC> {
        CORE0_IBUS_ACS_MSK_IC_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to clear interrupt by ibus trying to write icache
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_wr_ic(
        &mut self,
    ) -> CORE0_IBUS_WR_IC_W<CORE0_ACS_CACHE_INT_CLR_SPEC> {
        CORE0_IBUS_WR_IC_W::new(self, 1)
    }
    ///Bit 2 - The bit is used to clear interrupt by authentication fail.
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_reject(
        &mut self,
    ) -> CORE0_IBUS_REJECT_W<CORE0_ACS_CACHE_INT_CLR_SPEC> {
        CORE0_IBUS_REJECT_W::new(self, 2)
    }
    ///Bit 3 - The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access.
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_acs_msk_ic(
        &mut self,
    ) -> CORE0_DBUS_ACS_MSK_IC_W<CORE0_ACS_CACHE_INT_CLR_SPEC> {
        CORE0_DBUS_ACS_MSK_IC_W::new(self, 3)
    }
    ///Bit 4 - The bit is used to clear interrupt by authentication fail.
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_reject(
        &mut self,
    ) -> CORE0_DBUS_REJECT_W<CORE0_ACS_CACHE_INT_CLR_SPEC> {
        CORE0_DBUS_REJECT_W::new(self, 4)
    }
    ///Bit 5 - The bit is used to clear interrupt by dbus trying to write icache
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_wr_ic(
        &mut self,
    ) -> CORE0_DBUS_WR_IC_W<CORE0_ACS_CACHE_INT_CLR_SPEC> {
        CORE0_DBUS_WR_IC_W::new(self, 5)
    }
}
/**This description will be updated in the near future.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE0_ACS_CACHE_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`core0_acs_cache_int_clr::W`](W) writer structure
impl crate::Writable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
///`reset()` method sets CORE0_ACS_CACHE_INT_CLR to value 0
impl crate::Resettable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
