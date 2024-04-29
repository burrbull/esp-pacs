#[doc = "Register `SLC0_LENGTH` reader"]
pub type R = crate::R<SLC0_LENGTH_SPEC>;
#[doc = "Field `LEN` reader - "]
pub type LEN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_LENGTH")
            .field("len", &format_args!("{}", self.len().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_LENGTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_length::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_LENGTH_SPEC;
impl crate::RegisterSpec for SLC0_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_length::R`](R) reader structure"]
impl crate::Readable for SLC0_LENGTH_SPEC {}
#[doc = "`reset()` method sets SLC0_LENGTH to value 0"]
impl crate::Resettable for SLC0_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
