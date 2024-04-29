#[doc = "Register `TXLINK_DSCR` reader"]
pub type R = crate::R<TXLINK_DSCR_SPEC>;
#[doc = "Field `TXLINK_DSCR` reader - "]
pub type TXLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn txlink_dscr(&self) -> TXLINK_DSCR_R {
        TXLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXLINK_DSCR")
            .field(
                "txlink_dscr",
                &format_args!("{}", self.txlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXLINK_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlink_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLINK_DSCR_SPEC;
impl crate::RegisterSpec for TXLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlink_dscr::R`](R) reader structure"]
impl crate::Readable for TXLINK_DSCR_SPEC {}
#[doc = "`reset()` method sets TXLINK_DSCR to value 0"]
impl crate::Resettable for TXLINK_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
