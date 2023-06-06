#[doc = "Register `RX_CH%sDATA` reader"]
pub struct R(crate::R<RX_CHDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CHDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CHDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CHDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHDATA` reader - Read and write data for channel 0 via APB FIFO."]
pub type CHDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read and write data for channel 0 via APB FIFO."]
    #[inline(always)]
    pub fn chdata(&self) -> CHDATA_R {
        CHDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHDATA")
            .field("chdata", &format_args!("{}", self.chdata().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CHDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The read and write data register for CHANNEL$n by apb fifo access.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_chdata](index.html) module"]
pub struct RX_CHDATA_SPEC;
impl crate::RegisterSpec for RX_CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_chdata::R](R) reader structure"]
impl crate::Readable for RX_CHDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_CH%sDATA to value 0"]
impl crate::Resettable for RX_CHDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
