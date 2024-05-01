///Register `OCCUPY_1` reader
pub type R = crate::R<OCCUPY_1_SPEC>;
///Register `OCCUPY_1` writer
pub type W = crate::W<OCCUPY_1_SPEC>;
///Field `OCCUPY_CACHE` reader - Configure whether SRAM Block 0-3 is used as cache memory.
pub type OCCUPY_CACHE_R = crate::FieldReader;
///Field `OCCUPY_CACHE` writer - Configure whether SRAM Block 0-3 is used as cache memory.
pub type OCCUPY_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Configure whether SRAM Block 0-3 is used as cache memory.
    #[inline(always)]
    pub fn occupy_cache(&self) -> OCCUPY_CACHE_R {
        OCCUPY_CACHE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_1")
            .field("occupy_cache", &self.occupy_cache())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Configure whether SRAM Block 0-3 is used as cache memory.
    #[inline(always)]
    #[must_use]
    pub fn occupy_cache(&mut self) -> OCCUPY_CACHE_W<OCCUPY_1_SPEC> {
        OCCUPY_CACHE_W::new(self, 0)
    }
}
/**Occupy permission control register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`occupy_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OCCUPY_1_SPEC;
impl crate::RegisterSpec for OCCUPY_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`occupy_1::R`](R) reader structure
impl crate::Readable for OCCUPY_1_SPEC {}
///`write(|w| ..)` method takes [`occupy_1::W`](W) writer structure
impl crate::Writable for OCCUPY_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OCCUPY_1 to value 0
impl crate::Resettable for OCCUPY_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
