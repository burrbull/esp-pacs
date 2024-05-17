///Register `L2_CACHE_AUTOLOAD_SCT2_SIZE` reader
pub type R = crate::R<L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
///Register `L2_CACHE_AUTOLOAD_SCT2_SIZE` writer
pub type W = crate::W<L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
///Field `L2_CACHE_AUTOLOAD_SCT2_SIZE` reader - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA.
pub type L2_CACHE_AUTOLOAD_SCT2_SIZE_R = crate::FieldReader<u32>;
///Field `L2_CACHE_AUTOLOAD_SCT2_SIZE` writer - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA.
pub type L2_CACHE_AUTOLOAD_SCT2_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA.
    #[inline(always)]
    pub fn l2_cache_autoload_sct2_size(&self) -> L2_CACHE_AUTOLOAD_SCT2_SIZE_R {
        L2_CACHE_AUTOLOAD_SCT2_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_AUTOLOAD_SCT2_SIZE")
            .field("l2_cache_autoload_sct2_size", &self.l2_cache_autoload_sct2_size())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA.
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_autoload_sct2_size(
        &mut self,
    ) -> L2_CACHE_AUTOLOAD_SCT2_SIZE_W<L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC> {
        L2_CACHE_AUTOLOAD_SCT2_SIZE_W::new(self, 0)
    }
}
/**L2 Cache autoload section 2 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct2_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct2_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_autoload_sct2_size::R`](R) reader structure
impl crate::Readable for L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_autoload_sct2_size::W`](W) writer structure
impl crate::Writable for L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_AUTOLOAD_SCT2_SIZE to value 0
impl crate::Resettable for L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
