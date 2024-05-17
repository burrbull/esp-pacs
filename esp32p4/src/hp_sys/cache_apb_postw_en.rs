///Register `CACHE_APB_POSTW_EN` reader
pub type R = crate::R<CACHE_APB_POSTW_EN_SPEC>;
///Register `CACHE_APB_POSTW_EN` writer
pub type W = crate::W<CACHE_APB_POSTW_EN_SPEC>;
///Field `REG_CACHE_APB_POSTW_EN` reader - cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
pub type REG_CACHE_APB_POSTW_EN_R = crate::BitReader;
///Field `REG_CACHE_APB_POSTW_EN` writer - cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
pub type REG_CACHE_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub fn reg_cache_apb_postw_en(&self) -> REG_CACHE_APB_POSTW_EN_R {
        REG_CACHE_APB_POSTW_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_APB_POSTW_EN")
            .field("reg_cache_apb_postw_en", &self.reg_cache_apb_postw_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    #[must_use]
    pub fn reg_cache_apb_postw_en(
        &mut self,
    ) -> REG_CACHE_APB_POSTW_EN_W<CACHE_APB_POSTW_EN_SPEC> {
        REG_CACHE_APB_POSTW_EN_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`cache_apb_postw_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_apb_postw_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_APB_POSTW_EN_SPEC;
impl crate::RegisterSpec for CACHE_APB_POSTW_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_apb_postw_en::R`](R) reader structure
impl crate::Readable for CACHE_APB_POSTW_EN_SPEC {}
///`write(|w| ..)` method takes [`cache_apb_postw_en::W`](W) writer structure
impl crate::Writable for CACHE_APB_POSTW_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_APB_POSTW_EN to value 0
impl crate::Resettable for CACHE_APB_POSTW_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
