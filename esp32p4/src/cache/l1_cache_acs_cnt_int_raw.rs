///Register `L1_CACHE_ACS_CNT_INT_RAW` reader
pub type R = crate::R<L1_CACHE_ACS_CNT_INT_RAW_SPEC>;
///Register `L1_CACHE_ACS_CNT_INT_RAW` writer
pub type W = crate::W<L1_CACHE_ACS_CNT_INT_RAW_SPEC>;
///Field `L1_IBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0.
pub type L1_IBUS0_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_IBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0.
pub type L1_IBUS0_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_IBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1.
pub type L1_IBUS1_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_IBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1.
pub type L1_IBUS1_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache2 due to bus2 accesses L1-ICache2.
pub type L1_IBUS2_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_IBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache2 due to bus2 accesses L1-ICache2.
pub type L1_IBUS2_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache3 due to bus3 accesses L1-ICache3.
pub type L1_IBUS3_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_IBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache3 due to bus3 accesses L1-ICache3.
pub type L1_IBUS3_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache.
pub type L1_DBUS0_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_DBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache.
pub type L1_DBUS0_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache.
pub type L1_DBUS1_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_DBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache.
pub type L1_DBUS1_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus2 accesses L1-DCache.
pub type L1_DBUS2_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_DBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus2 accesses L1-DCache.
pub type L1_DBUS2_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus3 accesses L1-DCache.
pub type L1_DBUS3_OVF_INT_RAW_R = crate::BitReader;
///Field `L1_DBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus3 accesses L1-DCache.
pub type L1_DBUS3_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0.
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_raw(&self) -> L1_IBUS0_OVF_INT_RAW_R {
        L1_IBUS0_OVF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1.
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_raw(&self) -> L1_IBUS1_OVF_INT_RAW_R {
        L1_IBUS1_OVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache2 due to bus2 accesses L1-ICache2.
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_raw(&self) -> L1_IBUS2_OVF_INT_RAW_R {
        L1_IBUS2_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache3 due to bus3 accesses L1-ICache3.
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_raw(&self) -> L1_IBUS3_OVF_INT_RAW_R {
        L1_IBUS3_OVF_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache.
    #[inline(always)]
    pub fn l1_dbus0_ovf_int_raw(&self) -> L1_DBUS0_OVF_INT_RAW_R {
        L1_DBUS0_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache.
    #[inline(always)]
    pub fn l1_dbus1_ovf_int_raw(&self) -> L1_DBUS1_OVF_INT_RAW_R {
        L1_DBUS1_OVF_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus2 accesses L1-DCache.
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_raw(&self) -> L1_DBUS2_OVF_INT_RAW_R {
        L1_DBUS2_OVF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus3 accesses L1-DCache.
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_raw(&self) -> L1_DBUS3_OVF_INT_RAW_R {
        L1_DBUS3_OVF_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_ACS_CNT_INT_RAW")
            .field("l1_ibus0_ovf_int_raw", &self.l1_ibus0_ovf_int_raw())
            .field("l1_ibus1_ovf_int_raw", &self.l1_ibus1_ovf_int_raw())
            .field("l1_ibus2_ovf_int_raw", &self.l1_ibus2_ovf_int_raw())
            .field("l1_ibus3_ovf_int_raw", &self.l1_ibus3_ovf_int_raw())
            .field("l1_dbus0_ovf_int_raw", &self.l1_dbus0_ovf_int_raw())
            .field("l1_dbus1_ovf_int_raw", &self.l1_dbus1_ovf_int_raw())
            .field("l1_dbus2_ovf_int_raw", &self.l1_dbus2_ovf_int_raw())
            .field("l1_dbus3_ovf_int_raw", &self.l1_dbus3_ovf_int_raw())
            .finish()
    }
}
impl W {
    ///Bit 0 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0.
    #[inline(always)]
    #[must_use]
    pub fn l1_ibus0_ovf_int_raw(
        &mut self,
    ) -> L1_IBUS0_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_IBUS0_OVF_INT_RAW_W::new(self, 0)
    }
    ///Bit 1 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1.
    #[inline(always)]
    #[must_use]
    pub fn l1_ibus1_ovf_int_raw(
        &mut self,
    ) -> L1_IBUS1_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_IBUS1_OVF_INT_RAW_W::new(self, 1)
    }
    ///Bit 2 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache2 due to bus2 accesses L1-ICache2.
    #[inline(always)]
    #[must_use]
    pub fn l1_ibus2_ovf_int_raw(
        &mut self,
    ) -> L1_IBUS2_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_IBUS2_OVF_INT_RAW_W::new(self, 2)
    }
    ///Bit 3 - The raw bit of the interrupt of one of counters overflow that occurs in L1-ICache3 due to bus3 accesses L1-ICache3.
    #[inline(always)]
    #[must_use]
    pub fn l1_ibus3_ovf_int_raw(
        &mut self,
    ) -> L1_IBUS3_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_IBUS3_OVF_INT_RAW_W::new(self, 3)
    }
    ///Bit 4 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache.
    #[inline(always)]
    #[must_use]
    pub fn l1_dbus0_ovf_int_raw(
        &mut self,
    ) -> L1_DBUS0_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_DBUS0_OVF_INT_RAW_W::new(self, 4)
    }
    ///Bit 5 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache.
    #[inline(always)]
    #[must_use]
    pub fn l1_dbus1_ovf_int_raw(
        &mut self,
    ) -> L1_DBUS1_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_DBUS1_OVF_INT_RAW_W::new(self, 5)
    }
    ///Bit 6 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus2 accesses L1-DCache.
    #[inline(always)]
    #[must_use]
    pub fn l1_dbus2_ovf_int_raw(
        &mut self,
    ) -> L1_DBUS2_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_DBUS2_OVF_INT_RAW_W::new(self, 6)
    }
    ///Bit 7 - The raw bit of the interrupt of one of counters overflow that occurs in L1-DCache due to bus3 accesses L1-DCache.
    #[inline(always)]
    #[must_use]
    pub fn l1_dbus3_ovf_int_raw(
        &mut self,
    ) -> L1_DBUS3_OVF_INT_RAW_W<L1_CACHE_ACS_CNT_INT_RAW_SPEC> {
        L1_DBUS3_OVF_INT_RAW_W::new(self, 7)
    }
}
/**Cache Access Counter Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_ACS_CNT_INT_RAW_SPEC;
impl crate::RegisterSpec for L1_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_acs_cnt_int_raw::R`](R) reader structure
impl crate::Readable for L1_CACHE_ACS_CNT_INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_acs_cnt_int_raw::W`](W) writer structure
impl crate::Writable for L1_CACHE_ACS_CNT_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_ACS_CNT_INT_RAW to value 0
impl crate::Resettable for L1_CACHE_ACS_CNT_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
