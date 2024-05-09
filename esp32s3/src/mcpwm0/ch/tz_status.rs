#[doc = "Register `TZ_STATUS` reader"]
pub type R = crate::R<TZ_STATUS_SPEC>;
#[doc = "Field `CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
pub type CBC_ON_R = crate::BitReader;
#[doc = "Field `OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going"]
pub type OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn cbc_on(&self) -> CBC_ON_R {
        CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going"]
    #[inline(always)]
    pub fn ost_on(&self) -> OST_ON_R {
        OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ_STATUS")
            .field("cbc_on", &self.cbc_on().bit())
            .field("ost_on", &self.ost_on().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Status of fault events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tz_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZ_STATUS_SPEC;
impl crate::RegisterSpec for TZ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz_status::R`](R) reader structure"]
impl crate::Readable for TZ_STATUS_SPEC {}
#[doc = "`reset()` method sets TZ_STATUS to value 0"]
impl crate::Resettable for TZ_STATUS_SPEC {}
