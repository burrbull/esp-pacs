///Register `SAR2DATA_STATUS` reader
pub type R = crate::R<SAR2DATA_STATUS_SPEC>;
///Field `SARADC2_DATA` reader - saradc2 data
pub type SARADC2_DATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:16 - saradc2 data
    #[inline(always)]
    pub fn saradc2_data(&self) -> SARADC2_DATA_R {
        SARADC2_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2DATA_STATUS")
            .field("saradc2_data", &self.saradc2_data())
            .finish()
    }
}
/**digital saradc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sar2data_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR2DATA_STATUS_SPEC;
impl crate::RegisterSpec for SAR2DATA_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar2data_status::R`](R) reader structure
impl crate::Readable for SAR2DATA_STATUS_SPEC {}
///`reset()` method sets SAR2DATA_STATUS to value 0
impl crate::Resettable for SAR2DATA_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
