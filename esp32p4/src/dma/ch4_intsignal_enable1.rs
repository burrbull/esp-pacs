#[doc = "Register `CH4_INTSIGNAL_ENABLE1` reader"]
pub type R = crate::R<CH4_INTSIGNAL_ENABLE1_SPEC>;
#[doc = "Field `CH4_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL` reader - NA"]
pub type CH4_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `CH4_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL` reader - NA"]
pub type CH4_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `CH4_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL` reader - NA"]
pub type CH4_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL_R = crate::BitReader;
#[doc = "Field `CH4_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL` reader - NA"]
pub type CH4_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch4_enable_ecc_prot_chmem_correrr_intsignal(
        &self,
    ) -> CH4_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL_R {
        CH4_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch4_enable_ecc_prot_chmem_uncorrerr_intsignal(
        &self,
    ) -> CH4_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL_R {
        CH4_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch4_enable_ecc_prot_uidmem_correrr_intsignal(
        &self,
    ) -> CH4_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL_R {
        CH4_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch4_enable_ecc_prot_uidmem_uncorrerr_intsignal(
        &self,
    ) -> CH4_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL_R {
        CH4_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_INTSIGNAL_ENABLE1")
            .field(
                "ch4_enable_ecc_prot_chmem_correrr_intsignal",
                &format_args!(
                    "{}",
                    self.ch4_enable_ecc_prot_chmem_correrr_intsignal().bit()
                ),
            )
            .field(
                "ch4_enable_ecc_prot_chmem_uncorrerr_intsignal",
                &format_args!(
                    "{}",
                    self.ch4_enable_ecc_prot_chmem_uncorrerr_intsignal().bit()
                ),
            )
            .field(
                "ch4_enable_ecc_prot_uidmem_correrr_intsignal",
                &format_args!(
                    "{}",
                    self.ch4_enable_ecc_prot_uidmem_correrr_intsignal().bit()
                ),
            )
            .field(
                "ch4_enable_ecc_prot_uidmem_uncorrerr_intsignal",
                &format_args!(
                    "{}",
                    self.ch4_enable_ecc_prot_uidmem_uncorrerr_intsignal().bit()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_INTSIGNAL_ENABLE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intsignal_enable1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_INTSIGNAL_ENABLE1_SPEC;
impl crate::RegisterSpec for CH4_INTSIGNAL_ENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_intsignal_enable1::R`](R) reader structure"]
impl crate::Readable for CH4_INTSIGNAL_ENABLE1_SPEC {}
#[doc = "`reset()` method sets CH4_INTSIGNAL_ENABLE1 to value 0x0f"]
impl crate::Resettable for CH4_INTSIGNAL_ENABLE1_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
