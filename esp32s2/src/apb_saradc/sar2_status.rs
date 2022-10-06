#[doc = "Register `SAR2_STATUS` reader"]
pub struct R(crate::R<SAR2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR2_STATUS` reader - digital adc2 status"]
pub type SAR2_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - digital adc2 status"]
    #[inline(always)]
    pub fn sar2_status(&self) -> SAR2_STATUS_R {
        SAR2_STATUS_R::new(self.bits)
    }
}
#[doc = "digital adc2 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar2_status](index.html) module"]
pub struct SAR2_STATUS_SPEC;
impl crate::RegisterSpec for SAR2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar2_status::R](R) reader structure"]
impl crate::Readable for SAR2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR2_STATUS to value 0"]
impl crate::Resettable for SAR2_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}