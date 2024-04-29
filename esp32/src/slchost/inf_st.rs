#[doc = "Register `INF_ST` reader"]
pub type R = crate::R<INF_ST_SPEC>;
#[doc = "Field `SDIO20_MODE` reader - "]
pub type SDIO20_MODE_R = crate::FieldReader;
#[doc = "Field `SDIO_NEG_SAMP` reader - "]
pub type SDIO_NEG_SAMP_R = crate::FieldReader;
#[doc = "Field `SDIO_QUICK_IN` reader - "]
pub type SDIO_QUICK_IN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sdio20_mode(&self) -> SDIO20_MODE_R {
        SDIO20_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn sdio_neg_samp(&self) -> SDIO_NEG_SAMP_R {
        SDIO_NEG_SAMP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn sdio_quick_in(&self) -> SDIO_QUICK_IN_R {
        SDIO_QUICK_IN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF_ST")
            .field(
                "sdio20_mode",
                &format_args!("{}", self.sdio20_mode().bits()),
            )
            .field(
                "sdio_neg_samp",
                &format_args!("{}", self.sdio_neg_samp().bits()),
            )
            .field(
                "sdio_quick_in",
                &format_args!("{}", self.sdio_quick_in().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INF_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inf_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INF_ST_SPEC;
impl crate::RegisterSpec for INF_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inf_st::R`](R) reader structure"]
impl crate::Readable for INF_ST_SPEC {}
#[doc = "`reset()` method sets INF_ST to value 0"]
impl crate::Resettable for INF_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
