#[doc = "Register `PKT_LEN0` reader"]
pub type R = crate::R<PKT_LEN0_SPEC>;
#[doc = "Field `HOSTSLC0_LEN0` reader - "]
pub type HOSTSLC0_LEN0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn hostslc0_len0(&self) -> HOSTSLC0_LEN0_R {
        HOSTSLC0_LEN0_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKT_LEN0")
            .field(
                "hostslc0_len0",
                &format_args!("{}", self.hostslc0_len0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PKT_LEN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkt_len0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKT_LEN0_SPEC;
impl crate::RegisterSpec for PKT_LEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkt_len0::R`](R) reader structure"]
impl crate::Readable for PKT_LEN0_SPEC {}
#[doc = "`reset()` method sets PKT_LEN0 to value 0"]
impl crate::Resettable for PKT_LEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
