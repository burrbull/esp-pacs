#[doc = "Register `PKT_LEN` reader"]
pub type R = crate::R<PKT_LEN_SPEC>;
#[doc = "Field `HOSTSLC0_LEN` reader - "]
pub type HOSTSLC0_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `HOSTSLC0_LEN_CHECK` reader - "]
pub type HOSTSLC0_LEN_CHECK_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn hostslc0_len(&self) -> HOSTSLC0_LEN_R {
        HOSTSLC0_LEN_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn hostslc0_len_check(&self) -> HOSTSLC0_LEN_CHECK_R {
        HOSTSLC0_LEN_CHECK_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKT_LEN")
            .field(
                "hostslc0_len",
                &format_args!("{}", self.hostslc0_len().bits()),
            )
            .field(
                "hostslc0_len_check",
                &format_args!("{}", self.hostslc0_len_check().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PKT_LEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkt_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKT_LEN_SPEC;
impl crate::RegisterSpec for PKT_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkt_len::R`](R) reader structure"]
impl crate::Readable for PKT_LEN_SPEC {}
#[doc = "`reset()` method sets PKT_LEN to value 0"]
impl crate::Resettable for PKT_LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
