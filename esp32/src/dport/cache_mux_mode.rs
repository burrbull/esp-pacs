///Register `CACHE_MUX_MODE` reader
pub type R = crate::R<CACHE_MUX_MODE_SPEC>;
///Register `CACHE_MUX_MODE` writer
pub type W = crate::W<CACHE_MUX_MODE_SPEC>;
///Field `CACHE_MUX_MODE` reader -
pub type CACHE_MUX_MODE_R = crate::FieldReader;
///Field `CACHE_MUX_MODE` writer -
pub type CACHE_MUX_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn cache_mux_mode(&self) -> CACHE_MUX_MODE_R {
        CACHE_MUX_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MUX_MODE")
            .field("cache_mux_mode", &self.cache_mux_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn cache_mux_mode(&mut self) -> CACHE_MUX_MODE_W<CACHE_MUX_MODE_SPEC> {
        CACHE_MUX_MODE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`cache_mux_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mux_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_MUX_MODE_SPEC;
impl crate::RegisterSpec for CACHE_MUX_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_mux_mode::R`](R) reader structure
impl crate::Readable for CACHE_MUX_MODE_SPEC {}
///`write(|w| ..)` method takes [`cache_mux_mode::W`](W) writer structure
impl crate::Writable for CACHE_MUX_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_MUX_MODE to value 0
impl crate::Resettable for CACHE_MUX_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
