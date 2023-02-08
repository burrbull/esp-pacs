#[doc = "Register `BLK1_W8` reader"]
pub struct R(crate::R<BLK1_W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK1_W8` reader - Otp block1 word8 data."]
pub type BLOCK1_W8_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block1 word8 data."]
    #[inline(always)]
    pub fn block1_w8(&self) -> BLOCK1_W8_R {
        BLOCK1_W8_R::new(self.bits)
    }
}
#[doc = "Otp debuger block1 data register8.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_w8](index.html) module"]
pub struct BLK1_W8_SPEC;
impl crate::RegisterSpec for BLK1_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_w8::R](R) reader structure"]
impl crate::Readable for BLK1_W8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK1_W8 to value 0"]
impl crate::Resettable for BLK1_W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}