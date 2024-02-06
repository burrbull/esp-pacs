#[doc = "Register `RNG_DATA` reader"]
pub type R = crate::R<RNG_DATA_SPEC>;
#[doc = "Field `RND_DATA` reader - result of rng output"]
pub type RND_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - result of rng output"]
    #[inline(always)]
    pub fn rnd_data(&self) -> RND_DATA_R {
        RND_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_DATA")
            .field("rnd_data", &format_args!("{}", self.rnd_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RNG_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rng data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_DATA_SPEC;
impl crate::RegisterSpec for RNG_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_data::R`](R) reader structure"]
impl crate::Readable for RNG_DATA_SPEC {}
#[doc = "`reset()` method sets RNG_DATA to value 0"]
impl crate::Resettable for RNG_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
