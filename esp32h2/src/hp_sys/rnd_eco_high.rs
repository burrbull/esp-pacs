///Register `RND_ECO_HIGH` reader
pub type R = crate::R<RND_ECO_HIGH_SPEC>;
///Register `RND_ECO_HIGH` writer
pub type W = crate::W<RND_ECO_HIGH_SPEC>;
///Field `REDCY_HIGH` reader - Only reserved for ECO.
pub type REDCY_HIGH_R = crate::FieldReader<u32>;
///Field `REDCY_HIGH` writer - Only reserved for ECO.
pub type REDCY_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Only reserved for ECO.
    #[inline(always)]
    pub fn redcy_high(&self) -> REDCY_HIGH_R {
        REDCY_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO_HIGH").field("redcy_high", &self.redcy_high()).finish()
    }
}
impl W {
    ///Bits 0:31 - Only reserved for ECO.
    #[inline(always)]
    #[must_use]
    pub fn redcy_high(&mut self) -> REDCY_HIGH_W<RND_ECO_HIGH_SPEC> {
        REDCY_HIGH_W::new(self, 0)
    }
}
/**redcy eco high register.

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RND_ECO_HIGH_SPEC;
impl crate::RegisterSpec for RND_ECO_HIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rnd_eco_high::R`](R) reader structure
impl crate::Readable for RND_ECO_HIGH_SPEC {}
///`write(|w| ..)` method takes [`rnd_eco_high::W`](W) writer structure
impl crate::Writable for RND_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RND_ECO_HIGH to value 0xffff_ffff
impl crate::Resettable for RND_ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
