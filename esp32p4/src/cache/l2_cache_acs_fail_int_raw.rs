///Register `L2_CACHE_ACS_FAIL_INT_RAW` reader
pub type R = crate::R<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
///Register `L2_CACHE_ACS_FAIL_INT_RAW` writer
pub type W = crate::W<L2_CACHE_ACS_FAIL_INT_RAW_SPEC>;
///Field `L2_CACHE_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L2-Cache.
pub type L2_CACHE_FAIL_INT_RAW_R = crate::BitReader;
///Field `L2_CACHE_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L2-Cache.
pub type L2_CACHE_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - The raw bit of the interrupt of access fail that occurs in L2-Cache.
    #[inline(always)]
    pub fn l2_cache_fail_int_raw(&self) -> L2_CACHE_FAIL_INT_RAW_R {
        L2_CACHE_FAIL_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_FAIL_INT_RAW")
            .field("l2_cache_fail_int_raw", &self.l2_cache_fail_int_raw())
            .finish()
    }
}
impl W {
    ///Bit 5 - The raw bit of the interrupt of access fail that occurs in L2-Cache.
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_fail_int_raw(
        &mut self,
    ) -> L2_CACHE_FAIL_INT_RAW_W<L2_CACHE_ACS_FAIL_INT_RAW_SPEC> {
        L2_CACHE_FAIL_INT_RAW_W::new(self, 5)
    }
}
/**Cache Access Fail Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_ACS_FAIL_INT_RAW_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_acs_fail_int_raw::R`](R) reader structure
impl crate::Readable for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_acs_fail_int_raw::W`](W) writer structure
impl crate::Writable for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_ACS_FAIL_INT_RAW to value 0
impl crate::Resettable for L2_CACHE_ACS_FAIL_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
