///Register `L1_CACHE_SYNC_RST_CTRL` reader
pub type R = crate::R<L1_CACHE_SYNC_RST_CTRL_SPEC>;
///Register `L1_CACHE_SYNC_RST_CTRL` writer
pub type W = crate::W<L1_CACHE_SYNC_RST_CTRL_SPEC>;
///Field `L1_ICACHE0_SYNC_RST` reader - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
pub type L1_ICACHE0_SYNC_RST_R = crate::BitReader;
///Field `L1_ICACHE1_SYNC_RST` reader - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
pub type L1_ICACHE1_SYNC_RST_R = crate::BitReader;
///Field `L1_ICACHE2_SYNC_RST` reader - Reserved
pub type L1_ICACHE2_SYNC_RST_R = crate::BitReader;
///Field `L1_ICACHE3_SYNC_RST` reader - Reserved
pub type L1_ICACHE3_SYNC_RST_R = crate::BitReader;
///Field `L1_CACHE_SYNC_RST` reader - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
pub type L1_CACHE_SYNC_RST_R = crate::BitReader;
///Field `L1_CACHE_SYNC_RST` writer - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
pub type L1_CACHE_SYNC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
    #[inline(always)]
    pub fn l1_icache0_sync_rst(&self) -> L1_ICACHE0_SYNC_RST_R {
        L1_ICACHE0_SYNC_RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
    #[inline(always)]
    pub fn l1_icache1_sync_rst(&self) -> L1_ICACHE1_SYNC_RST_R {
        L1_ICACHE1_SYNC_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn l1_icache2_sync_rst(&self) -> L1_ICACHE2_SYNC_RST_R {
        L1_ICACHE2_SYNC_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    pub fn l1_icache3_sync_rst(&self) -> L1_ICACHE3_SYNC_RST_R {
        L1_ICACHE3_SYNC_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
    #[inline(always)]
    pub fn l1_cache_sync_rst(&self) -> L1_CACHE_SYNC_RST_R {
        L1_CACHE_SYNC_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_SYNC_RST_CTRL")
            .field("l1_icache0_sync_rst", &self.l1_icache0_sync_rst())
            .field("l1_icache1_sync_rst", &self.l1_icache1_sync_rst())
            .field("l1_icache2_sync_rst", &self.l1_icache2_sync_rst())
            .field("l1_icache3_sync_rst", &self.l1_icache3_sync_rst())
            .field("l1_cache_sync_rst", &self.l1_cache_sync_rst())
            .finish()
    }
}
impl W {
    ///Bit 4 - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs.
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_sync_rst(&mut self) -> L1_CACHE_SYNC_RST_W<L1_CACHE_SYNC_RST_CTRL_SPEC> {
        L1_CACHE_SYNC_RST_W::new(self, 4)
    }
}
/**Cache Sync Reset control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_sync_rst_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_sync_rst_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_SYNC_RST_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_SYNC_RST_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_sync_rst_ctrl::R`](R) reader structure
impl crate::Readable for L1_CACHE_SYNC_RST_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_sync_rst_ctrl::W`](W) writer structure
impl crate::Writable for L1_CACHE_SYNC_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_SYNC_RST_CTRL to value 0
impl crate::Resettable for L1_CACHE_SYNC_RST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
