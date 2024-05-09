#[doc = "Register `EXT_WAKEUP1_STATUS` reader"]
pub type R = crate::R<EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "Field `EXT_WAKEUP1_STATUS` reader - ext wakeup1 status"]
pub type EXT_WAKEUP1_STATUS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status(&self) -> EXT_WAKEUP1_STATUS_R {
        EXT_WAKEUP1_STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP1_STATUS")
            .field("ext_wakeup1_status", &self.ext_wakeup1_status().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP1_STATUS_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup1_status::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_STATUS_SPEC {}
#[doc = "`reset()` method sets EXT_WAKEUP1_STATUS to value 0"]
impl crate::Resettable for EXT_WAKEUP1_STATUS_SPEC {}
