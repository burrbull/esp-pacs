#[doc = "Register `DBUS0_ACS_CNT` reader"]
pub type R = crate::R<DBUS0_ACS_CNT_SPEC>;
#[doc = "Field `DBUS0_ACS_CNT` reader - The bits are used to count the number of dbus0 access dcache."]
pub type DBUS0_ACS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of dbus0 access dcache."]
    #[inline(always)]
    pub fn dbus0_acs_cnt(&self) -> DBUS0_ACS_CNT_R {
        DBUS0_ACS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS0_ACS_CNT")
            .field("dbus0_acs_cnt", &self.dbus0_acs_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus0_acs_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS0_ACS_CNT_SPEC;
impl crate::RegisterSpec for DBUS0_ACS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus0_acs_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS0_ACS_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS0_ACS_CNT to value 0"]
impl crate::Resettable for DBUS0_ACS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
