#[doc = "Register `BLK3_W7` reader"]
pub type R = crate::R<BLK3_W7_SPEC>;
#[doc = "Field `BLOCK3_W7` reader - Otp block3 word7 data."]
pub type BLOCK3_W7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word7 data."]
    #[inline(always)]
    pub fn block3_w7(&self) -> BLOCK3_W7_R {
        BLOCK3_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_W7")
            .field("block3_w7", &format_args!("{}", self.block3_w7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block3 data register7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_w7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_W7_SPEC;
impl crate::RegisterSpec for BLK3_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_w7::R`](R) reader structure"]
impl crate::Readable for BLK3_W7_SPEC {}
#[doc = "`reset()` method sets BLK3_W7 to value 0"]
impl crate::Resettable for BLK3_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
