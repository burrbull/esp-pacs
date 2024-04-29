#[doc = "Register `PKT_LEN1` reader"]
pub type R = crate::R<PKT_LEN1_SPEC>;
#[doc = "Field `HOSTSLC0_LEN1` reader - "]
pub type HOSTSLC0_LEN1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn hostslc0_len1(&self) -> HOSTSLC0_LEN1_R {
        HOSTSLC0_LEN1_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKT_LEN1")
            .field(
                "hostslc0_len1",
                &format_args!("{}", self.hostslc0_len1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PKT_LEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkt_len1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKT_LEN1_SPEC;
impl crate::RegisterSpec for PKT_LEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkt_len1::R`](R) reader structure"]
impl crate::Readable for PKT_LEN1_SPEC {}
#[doc = "`reset()` method sets PKT_LEN1 to value 0"]
impl crate::Resettable for PKT_LEN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
