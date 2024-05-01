///Register `DBUS1_ACS_MISS_CNT` reader
pub type R = crate::R<DBUS1_ACS_MISS_CNT_SPEC>;
///Field `DBUS1_ACS_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by dbus1 access.
pub type DBUS1_ACS_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The bits are used to count the number of the cache miss caused by dbus1 access.
    #[inline(always)]
    pub fn dbus1_acs_miss_cnt(&self) -> DBUS1_ACS_MISS_CNT_R {
        DBUS1_ACS_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS1_ACS_MISS_CNT")
            .field("dbus1_acs_miss_cnt", &self.dbus1_acs_miss_cnt())
            .finish()
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`dbus1_acs_miss_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBUS1_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for DBUS1_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbus1_acs_miss_cnt::R`](R) reader structure
impl crate::Readable for DBUS1_ACS_MISS_CNT_SPEC {}
///`reset()` method sets DBUS1_ACS_MISS_CNT to value 0
impl crate::Resettable for DBUS1_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
