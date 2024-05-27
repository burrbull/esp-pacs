///Register `MEAS_STATUS` reader
pub type R = crate::R<MEAS_STATUS_SPEC>;
///Field `SARADC_MEAS_STATUS` reader - N/A
pub type SARADC_MEAS_STATUS_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - N/A
    #[inline(always)]
    pub fn saradc_meas_status(&self) -> SARADC_MEAS_STATUS_R {
        SARADC_MEAS_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEAS_STATUS")
            .field("saradc_meas_status", &self.saradc_meas_status())
            .finish()
    }
}
/**N/A

You can [`read`](crate::generic::Reg::read) this register and get [`meas_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEAS_STATUS_SPEC;
impl crate::RegisterSpec for MEAS_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`meas_status::R`](R) reader structure
impl crate::Readable for MEAS_STATUS_SPEC {}
///`reset()` method sets MEAS_STATUS to value 0
impl crate::Resettable for MEAS_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
