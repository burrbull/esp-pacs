#[doc = "Register `SLC0_SUB_START_DES` reader"]
pub type R = crate::R<SLC0_SUB_START_DES_SPEC>;
#[doc = "Field `SUB_PAC_START_DSCR_ADDR` reader - "]
pub type SUB_PAC_START_DSCR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sub_pac_start_dscr_addr(&self) -> SUB_PAC_START_DSCR_ADDR_R {
        SUB_PAC_START_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_SUB_START_DES")
            .field(
                "sub_pac_start_dscr_addr",
                &format_args!("{}", self.sub_pac_start_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_SUB_START_DES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_sub_start_des::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_SUB_START_DES_SPEC;
impl crate::RegisterSpec for SLC0_SUB_START_DES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_sub_start_des::R`](R) reader structure"]
impl crate::Readable for SLC0_SUB_START_DES_SPEC {}
#[doc = "`reset()` method sets SLC0_SUB_START_DES to value 0"]
impl crate::Resettable for SLC0_SUB_START_DES_SPEC {
    const RESET_VALUE: u32 = 0;
}
