///Register `APP_INTR_STATUS_2` reader
pub type R = crate::R<APP_INTR_STATUS_2_SPEC>;
///Field `APP_INTR_STATUS_2` reader -
pub type APP_INTR_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn app_intr_status_2(&self) -> APP_INTR_STATUS_2_R {
        APP_INTR_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_INTR_STATUS_2")
            .field("app_intr_status_2", &self.app_intr_status_2())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_intr_status_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_INTR_STATUS_2_SPEC;
impl crate::RegisterSpec for APP_INTR_STATUS_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_intr_status_2::R`](R) reader structure
impl crate::Readable for APP_INTR_STATUS_2_SPEC {}
///`reset()` method sets APP_INTR_STATUS_2 to value 0
impl crate::Resettable for APP_INTR_STATUS_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
