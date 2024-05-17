///Register `CACHE_REQUEST` reader
pub type R = crate::R<CACHE_REQUEST_SPEC>;
///Register `CACHE_REQUEST` writer
pub type W = crate::W<CACHE_REQUEST_SPEC>;
///Field `BYPASS` reader - The bit is used to disable request recording which could cause performance issue
pub type BYPASS_R = crate::BitReader;
///Field `BYPASS` writer - The bit is used to disable request recording which could cause performance issue
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to disable request recording which could cause performance issue
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_REQUEST").field("bypass", &self.bypass()).finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to disable request recording which could cause performance issue
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<CACHE_REQUEST_SPEC> {
        BYPASS_W::new(self, 0)
    }
}
/**This description will be updated in the near future.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_request::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_request::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_REQUEST_SPEC;
impl crate::RegisterSpec for CACHE_REQUEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_request::R`](R) reader structure
impl crate::Readable for CACHE_REQUEST_SPEC {}
///`write(|w| ..)` method takes [`cache_request::W`](W) writer structure
impl crate::Writable for CACHE_REQUEST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_REQUEST to value 0
impl crate::Resettable for CACHE_REQUEST_SPEC {
    const RESET_VALUE: u32 = 0;
}
