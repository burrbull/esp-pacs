#[doc = "Register `TXLINK_DSCR_BF0` reader"]
pub type R = crate::R<TXLINK_DSCR_BF0_SPEC>;
#[doc = "Field `TXLINK_DSCR_BF0` reader - "]
pub type TXLINK_DSCR_BF0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn txlink_dscr_bf0(&self) -> TXLINK_DSCR_BF0_R {
        TXLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXLINK_DSCR_BF0")
            .field(
                "txlink_dscr_bf0",
                &format_args!("{}", self.txlink_dscr_bf0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXLINK_DSCR_BF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlink_dscr_bf0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for TXLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlink_dscr_bf0::R`](R) reader structure"]
impl crate::Readable for TXLINK_DSCR_BF0_SPEC {}
#[doc = "`reset()` method sets TXLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for TXLINK_DSCR_BF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
