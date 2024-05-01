///Register `L2_CACHE_ACS_CNT_INT_ENA` reader
pub type R = crate::R<L2_CACHE_ACS_CNT_INT_ENA_SPEC>;
///Register `L2_CACHE_ACS_CNT_INT_ENA` writer
pub type W = crate::W<L2_CACHE_ACS_CNT_INT_ENA_SPEC>;
///Field `L2_IBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
pub type L2_IBUS0_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_IBUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
pub type L2_IBUS0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_IBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
pub type L2_IBUS1_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_IBUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
pub type L2_IBUS1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_IBUS2_OVF_INT_ENA` reader - Reserved
pub type L2_IBUS2_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_IBUS3_OVF_INT_ENA` reader - Reserved
pub type L2_IBUS3_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_DBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
pub type L2_DBUS0_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_DBUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
pub type L2_DBUS0_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_DBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
pub type L2_DBUS1_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_DBUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
pub type L2_DBUS1_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_DBUS2_OVF_INT_ENA` reader - Reserved
pub type L2_DBUS2_OVF_INT_ENA_R = crate::BitReader;
///Field `L2_DBUS3_OVF_INT_ENA` reader - Reserved
pub type L2_DBUS3_OVF_INT_ENA_R = crate::BitReader;
impl R {
    ///Bit 8 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_ena(&self) -> L2_IBUS0_OVF_INT_ENA_R {
        L2_IBUS0_OVF_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_ena(&self) -> L2_IBUS1_OVF_INT_ENA_R {
        L2_IBUS1_OVF_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Reserved
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_ena(&self) -> L2_IBUS2_OVF_INT_ENA_R {
        L2_IBUS2_OVF_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reserved
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_ena(&self) -> L2_IBUS3_OVF_INT_ENA_R {
        L2_IBUS3_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_ena(&self) -> L2_DBUS0_OVF_INT_ENA_R {
        L2_DBUS0_OVF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_ena(&self) -> L2_DBUS1_OVF_INT_ENA_R {
        L2_DBUS1_OVF_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Reserved
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_ena(&self) -> L2_DBUS2_OVF_INT_ENA_R {
        L2_DBUS2_OVF_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Reserved
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_ena(&self) -> L2_DBUS3_OVF_INT_ENA_R {
        L2_DBUS3_OVF_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_CNT_INT_ENA")
            .field("l2_ibus0_ovf_int_ena", &self.l2_ibus0_ovf_int_ena())
            .field("l2_ibus1_ovf_int_ena", &self.l2_ibus1_ovf_int_ena())
            .field("l2_ibus2_ovf_int_ena", &self.l2_ibus2_ovf_int_ena())
            .field("l2_ibus3_ovf_int_ena", &self.l2_ibus3_ovf_int_ena())
            .field("l2_dbus0_ovf_int_ena", &self.l2_dbus0_ovf_int_ena())
            .field("l2_dbus1_ovf_int_ena", &self.l2_dbus1_ovf_int_ena())
            .field("l2_dbus2_ovf_int_ena", &self.l2_dbus2_ovf_int_ena())
            .field("l2_dbus3_ovf_int_ena", &self.l2_dbus3_ovf_int_ena())
            .finish()
    }
}
impl W {
    ///Bit 8 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus0_ovf_int_ena(
        &mut self,
    ) -> L2_IBUS0_OVF_INT_ENA_W<L2_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L2_IBUS0_OVF_INT_ENA_W::new(self, 8)
    }
    ///Bit 9 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus1_ovf_int_ena(
        &mut self,
    ) -> L2_IBUS1_OVF_INT_ENA_W<L2_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L2_IBUS1_OVF_INT_ENA_W::new(self, 9)
    }
    ///Bit 12 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-Cache.
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus0_ovf_int_ena(
        &mut self,
    ) -> L2_DBUS0_OVF_INT_ENA_W<L2_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L2_DBUS0_OVF_INT_ENA_W::new(self, 12)
    }
    ///Bit 13 - The bit is used to enable interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-Cache.
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus1_ovf_int_ena(
        &mut self,
    ) -> L2_DBUS1_OVF_INT_ENA_W<L2_CACHE_ACS_CNT_INT_ENA_SPEC> {
        L2_DBUS1_OVF_INT_ENA_W::new(self, 13)
    }
}
/**Cache Access Counter Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_ACS_CNT_INT_ENA_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_CNT_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_acs_cnt_int_ena::R`](R) reader structure
impl crate::Readable for L2_CACHE_ACS_CNT_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_acs_cnt_int_ena::W`](W) writer structure
impl crate::Writable for L2_CACHE_ACS_CNT_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_ACS_CNT_INT_ENA to value 0
impl crate::Resettable for L2_CACHE_ACS_CNT_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
