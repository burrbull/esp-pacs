#[doc = "Register `RXLINK_DSCR` reader"]
pub type R = crate::R<RXLINK_DSCR_SPEC>;
#[doc = "Field `RXLINK_DSCR` reader - "]
pub type RXLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxlink_dscr(&self) -> RXLINK_DSCR_R {
        RXLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXLINK_DSCR")
            .field(
                "rxlink_dscr",
                &format_args!("{}", self.rxlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RXLINK_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlink_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLINK_DSCR_SPEC;
impl crate::RegisterSpec for RXLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlink_dscr::R`](R) reader structure"]
impl crate::Readable for RXLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets RXLINK_DSCR to value 0"]
impl crate::Resettable for RXLINK_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
