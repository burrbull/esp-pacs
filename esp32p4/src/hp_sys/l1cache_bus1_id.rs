///Register `L1CACHE_BUS1_ID` reader
pub type R = crate::R<L1CACHE_BUS1_ID_SPEC>;
///Register `L1CACHE_BUS1_ID` writer
pub type W = crate::W<L1CACHE_BUS1_ID_SPEC>;
///Field `REG_L1_CACHE_BUS1_ID` reader - NA
pub type REG_L1_CACHE_BUS1_ID_R = crate::FieldReader;
///Field `REG_L1_CACHE_BUS1_ID` writer - NA
pub type REG_L1_CACHE_BUS1_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NA
    #[inline(always)]
    pub fn reg_l1_cache_bus1_id(&self) -> REG_L1_CACHE_BUS1_ID_R {
        REG_L1_CACHE_BUS1_ID_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CACHE_BUS1_ID")
            .field("reg_l1_cache_bus1_id", &self.reg_l1_cache_bus1_id())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_cache_bus1_id(
        &mut self,
    ) -> REG_L1_CACHE_BUS1_ID_W<L1CACHE_BUS1_ID_SPEC> {
        REG_L1_CACHE_BUS1_ID_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`l1cache_bus1_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1cache_bus1_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1CACHE_BUS1_ID_SPEC;
impl crate::RegisterSpec for L1CACHE_BUS1_ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1cache_bus1_id::R`](R) reader structure
impl crate::Readable for L1CACHE_BUS1_ID_SPEC {}
///`write(|w| ..)` method takes [`l1cache_bus1_id::W`](W) writer structure
impl crate::Writable for L1CACHE_BUS1_ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1CACHE_BUS1_ID to value 0
impl crate::Resettable for L1CACHE_BUS1_ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
