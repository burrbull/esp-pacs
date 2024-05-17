///Register `BLK8_W4` reader
pub type R = crate::R<BLK8_W4_SPEC>;
///Field `BLOCK8_W4` reader - Otp block8 word4 data.
pub type BLOCK8_W4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block8 word4 data.
    #[inline(always)]
    pub fn block8_w4(&self) -> BLOCK8_W4_R {
        BLOCK8_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK8_W4").field("block8_w4", &self.block8_w4()).finish()
    }
}
/**Otp debuger block8 data register4.

You can [`read`](crate::generic::Reg::read) this register and get [`blk8_w4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK8_W4_SPEC;
impl crate::RegisterSpec for BLK8_W4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk8_w4::R`](R) reader structure
impl crate::Readable for BLK8_W4_SPEC {}
///`reset()` method sets BLK8_W4 to value 0
impl crate::Resettable for BLK8_W4_SPEC {
    const RESET_VALUE: u32 = 0;
}
