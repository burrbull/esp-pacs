///Register `SAR_TOUCH_STATUS15` reader
pub type R = crate::R<SAR_TOUCH_STATUS15_SPEC>;
///Field `TOUCH_SLP_DATA` reader - The data of touch sleep pad, depending on the setting of SENS_TOUCH_DATA_SEL.
pub type TOUCH_SLP_DATA_R = crate::FieldReader<u32>;
///Field `TOUCH_SLP_DEBOUNCE` reader - Touch sleep pad debouce value.
pub type TOUCH_SLP_DEBOUNCE_R = crate::FieldReader;
impl R {
    ///Bits 0:21 - The data of touch sleep pad, depending on the setting of SENS_TOUCH_DATA_SEL.
    #[inline(always)]
    pub fn touch_slp_data(&self) -> TOUCH_SLP_DATA_R {
        TOUCH_SLP_DATA_R::new(self.bits & 0x003f_ffff)
    }
    ///Bits 29:31 - Touch sleep pad debouce value.
    #[inline(always)]
    pub fn touch_slp_debounce(&self) -> TOUCH_SLP_DEBOUNCE_R {
        TOUCH_SLP_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS15")
            .field("touch_slp_data", &self.touch_slp_data())
            .field("touch_slp_debounce", &self.touch_slp_debounce())
            .finish()
    }
}
/**Touch sleep pad status

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_STATUS15_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS15_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_status15::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_STATUS15_SPEC {}
///`reset()` method sets SAR_TOUCH_STATUS15 to value 0
impl crate::Resettable for SAR_TOUCH_STATUS15_SPEC {
    const RESET_VALUE: u32 = 0;
}
