///Register `SYNC_L1_CACHE_PRELOAD_INT_CLR` reader
pub type R = crate::R<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC>;
///Register `SYNC_L1_CACHE_PRELOAD_INT_CLR` writer
pub type W = crate::W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC>;
///Field `L1_ICACHE0_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L1-ICache0 preload-operation is done.
pub type L1_ICACHE0_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L1-ICache1 preload-operation is done.
pub type L1_ICACHE1_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE2_PLD_DONE_INT_CLR` reader - Reserved
pub type L1_ICACHE2_PLD_DONE_INT_CLR_R = crate::BitReader;
///Field `L1_ICACHE3_PLD_DONE_INT_CLR` reader - Reserved
pub type L1_ICACHE3_PLD_DONE_INT_CLR_R = crate::BitReader;
///Field `L1_DCACHE_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L1-DCache preload-operation is done.
pub type L1_DCACHE_PLD_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when Cache sync-operation is done.
pub type SYNC_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE0_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L1-ICache0 preload-operation error.
pub type L1_ICACHE0_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L1-ICache1 preload-operation error.
pub type L1_ICACHE1_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE2_PLD_ERR_INT_CLR` reader - Reserved
pub type L1_ICACHE2_PLD_ERR_INT_CLR_R = crate::BitReader;
///Field `L1_ICACHE3_PLD_ERR_INT_CLR` reader - Reserved
pub type L1_ICACHE3_PLD_ERR_INT_CLR_R = crate::BitReader;
///Field `L1_DCACHE_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L1-DCache preload-operation error.
pub type L1_DCACHE_PLD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNC_ERR_INT_CLR` writer - The bit is used to clear interrupt of Cache sync-operation error.
pub type SYNC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn l1_icache2_pld_done_int_clr(&self) -> L1_ICACHE2_PLD_DONE_INT_CLR_R {
        L1_ICACHE2_PLD_DONE_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    pub fn l1_icache3_pld_done_int_clr(&self) -> L1_ICACHE3_PLD_DONE_INT_CLR_R {
        L1_ICACHE3_PLD_DONE_INT_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    pub fn l1_icache2_pld_err_int_clr(&self) -> L1_ICACHE2_PLD_ERR_INT_CLR_R {
        L1_ICACHE2_PLD_ERR_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Reserved
    #[inline(always)]
    pub fn l1_icache3_pld_err_int_clr(&self) -> L1_ICACHE3_PLD_ERR_INT_CLR_R {
        L1_ICACHE3_PLD_ERR_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_L1_CACHE_PRELOAD_INT_CLR")
            .field(
                "l1_icache2_pld_done_int_clr",
                &self.l1_icache2_pld_done_int_clr(),
            )
            .field(
                "l1_icache3_pld_done_int_clr",
                &self.l1_icache3_pld_done_int_clr(),
            )
            .field(
                "l1_icache2_pld_err_int_clr",
                &self.l1_icache2_pld_err_int_clr(),
            )
            .field(
                "l1_icache3_pld_err_int_clr",
                &self.l1_icache3_pld_err_int_clr(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to clear interrupt that occurs only when L1-ICache0 preload-operation is done.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_pld_done_int_clr(
        &mut self,
    ) -> L1_ICACHE0_PLD_DONE_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        L1_ICACHE0_PLD_DONE_INT_CLR_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to clear interrupt that occurs only when L1-ICache1 preload-operation is done.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_pld_done_int_clr(
        &mut self,
    ) -> L1_ICACHE1_PLD_DONE_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        L1_ICACHE1_PLD_DONE_INT_CLR_W::new(self, 1)
    }
    ///Bit 4 - The bit is used to clear interrupt that occurs only when L1-DCache preload-operation is done.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_pld_done_int_clr(
        &mut self,
    ) -> L1_DCACHE_PLD_DONE_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        L1_DCACHE_PLD_DONE_INT_CLR_W::new(self, 4)
    }
    ///Bit 6 - The bit is used to clear interrupt that occurs only when Cache sync-operation is done.
    #[inline(always)]
    #[must_use]
    pub fn sync_done_int_clr(&mut self) -> SYNC_DONE_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        SYNC_DONE_INT_CLR_W::new(self, 6)
    }
    ///Bit 7 - The bit is used to clear interrupt of L1-ICache0 preload-operation error.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_pld_err_int_clr(
        &mut self,
    ) -> L1_ICACHE0_PLD_ERR_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        L1_ICACHE0_PLD_ERR_INT_CLR_W::new(self, 7)
    }
    ///Bit 8 - The bit is used to clear interrupt of L1-ICache1 preload-operation error.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_pld_err_int_clr(
        &mut self,
    ) -> L1_ICACHE1_PLD_ERR_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        L1_ICACHE1_PLD_ERR_INT_CLR_W::new(self, 8)
    }
    ///Bit 11 - The bit is used to clear interrupt of L1-DCache preload-operation error.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_pld_err_int_clr(
        &mut self,
    ) -> L1_DCACHE_PLD_ERR_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        L1_DCACHE_PLD_ERR_INT_CLR_W::new(self, 11)
    }
    ///Bit 13 - The bit is used to clear interrupt of Cache sync-operation error.
    #[inline(always)]
    #[must_use]
    pub fn sync_err_int_clr(&mut self) -> SYNC_ERR_INT_CLR_W<SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC> {
        SYNC_ERR_INT_CLR_W::new(self, 13)
    }
}
/**Sync Preload operation Interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_int_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_l1_cache_preload_int_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC;
impl crate::RegisterSpec for SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sync_l1_cache_preload_int_clr::R`](R) reader structure
impl crate::Readable for SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC {}
///`write(|w| ..)` method takes [`sync_l1_cache_preload_int_clr::W`](W) writer structure
impl crate::Writable for SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYNC_L1_CACHE_PRELOAD_INT_CLR to value 0
impl crate::Resettable for SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
