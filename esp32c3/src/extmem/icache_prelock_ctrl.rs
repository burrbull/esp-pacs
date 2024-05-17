///Register `ICACHE_PRELOCK_CTRL` reader
pub type R = crate::R<ICACHE_PRELOCK_CTRL_SPEC>;
///Register `ICACHE_PRELOCK_CTRL` writer
pub type W = crate::W<ICACHE_PRELOCK_CTRL_SPEC>;
///Field `ICACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function.
pub type ICACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
///Field `ICACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function.
pub type ICACHE_PRELOCK_SCT0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function.
pub type ICACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
///Field `ICACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function.
pub type ICACHE_PRELOCK_SCT1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to enable the first section of prelock function.
    #[inline(always)]
    pub fn icache_prelock_sct0_en(&self) -> ICACHE_PRELOCK_SCT0_EN_R {
        ICACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to enable the second section of prelock function.
    #[inline(always)]
    pub fn icache_prelock_sct1_en(&self) -> ICACHE_PRELOCK_SCT1_EN_R {
        ICACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_PRELOCK_CTRL")
            .field("icache_prelock_sct0_en", &self.icache_prelock_sct0_en())
            .field("icache_prelock_sct1_en", &self.icache_prelock_sct1_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to enable the first section of prelock function.
    #[inline(always)]
    #[must_use]
    pub fn icache_prelock_sct0_en(
        &mut self,
    ) -> ICACHE_PRELOCK_SCT0_EN_W<ICACHE_PRELOCK_CTRL_SPEC> {
        ICACHE_PRELOCK_SCT0_EN_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to enable the second section of prelock function.
    #[inline(always)]
    #[must_use]
    pub fn icache_prelock_sct1_en(
        &mut self,
    ) -> ICACHE_PRELOCK_SCT1_EN_W<ICACHE_PRELOCK_CTRL_SPEC> {
        ICACHE_PRELOCK_SCT1_EN_W::new(self, 1)
    }
}
/**This description will be updated in the near future.

You can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICACHE_PRELOCK_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_PRELOCK_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`icache_prelock_ctrl::R`](R) reader structure
impl crate::Readable for ICACHE_PRELOCK_CTRL_SPEC {}
///`write(|w| ..)` method takes [`icache_prelock_ctrl::W`](W) writer structure
impl crate::Writable for ICACHE_PRELOCK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICACHE_PRELOCK_CTRL to value 0
impl crate::Resettable for ICACHE_PRELOCK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
