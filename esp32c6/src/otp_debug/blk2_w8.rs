#[doc = "Register `BLK2_W8` reader"]
pub type R = crate::R<BLK2_W8_SPEC>;
#[doc = "Field `BLOCK2_W8` reader - Otp block2 word8 data."]
pub type BLOCK2_W8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block2 word8 data."]
    #[inline(always)]
    pub fn block2_w8(&self) -> BLOCK2_W8_R {
        BLOCK2_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_W8")
            .field("block2_w8", &format_args!("{}", self.block2_w8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block2 data register8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_W8_SPEC;
impl crate::RegisterSpec for BLK2_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_w8::R`](R) reader structure"]
impl crate::Readable for BLK2_W8_SPEC {}
#[doc = "`reset()` method sets BLK2_W8 to value 0"]
impl crate::Resettable for BLK2_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
