///Register `SAR1_STATUS` reader
pub type R = crate::R<SAR1_STATUS_SPEC>;
///Field `SAR1_STATUS` reader - Need add description
pub type SAR1_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Need add description
    #[inline(always)]
    pub fn sar1_status(&self) -> SAR1_STATUS_R {
        SAR1_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_STATUS").field("sar1_status", &self.sar1_status()).finish()
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`sar1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR1_STATUS_SPEC;
impl crate::RegisterSpec for SAR1_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar1_status::R`](R) reader structure
impl crate::Readable for SAR1_STATUS_SPEC {}
///`reset()` method sets SAR1_STATUS to value 0
impl crate::Resettable for SAR1_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
