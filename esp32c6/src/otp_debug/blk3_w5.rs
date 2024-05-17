///Register `BLK3_W5` reader
pub type R = crate::R<BLK3_W5_SPEC>;
///Field `BLOCK3_W5` reader - Otp block3 word5 data.
pub type BLOCK3_W5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block3 word5 data.
    #[inline(always)]
    pub fn block3_w5(&self) -> BLOCK3_W5_R {
        BLOCK3_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_W5").field("block3_w5", &self.block3_w5()).finish()
    }
}
/**Otp debuger block3 data register5.

You can [`read`](crate::generic::Reg::read) this register and get [`blk3_w5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK3_W5_SPEC;
impl crate::RegisterSpec for BLK3_W5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk3_w5::R`](R) reader structure
impl crate::Readable for BLK3_W5_SPEC {}
///`reset()` method sets BLK3_W5 to value 0
impl crate::Resettable for BLK3_W5_SPEC {
    const RESET_VALUE: u32 = 0;
}
