#[doc = "Register `BLK1_W2` reader"]
pub struct R(crate::R<BLK1_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK1_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK1_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK1_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK1_W2` reader - Otp block1 word2 data."]
pub type BLOCK1_W2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block1 word2 data."]
    #[inline(always)]
    pub fn block1_w2(&self) -> BLOCK1_W2_R {
        BLOCK1_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_W2")
            .field("block1_w2", &format_args!("{}", self.block1_w2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK1_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block1 data register2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk1_w2](index.html) module"]
pub struct BLK1_W2_SPEC;
impl crate::RegisterSpec for BLK1_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk1_w2::R](R) reader structure"]
impl crate::Readable for BLK1_W2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK1_W2 to value 0"]
impl crate::Resettable for BLK1_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
