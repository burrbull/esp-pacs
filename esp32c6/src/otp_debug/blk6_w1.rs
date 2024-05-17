///Register `BLK6_W1` reader
pub type R = crate::R<BLK6_W1_SPEC>;
///Field `BLOCK6_W1` reader - Otp block6 word1 data.
pub type BLOCK6_W1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block6 word1 data.
    #[inline(always)]
    pub fn block6_w1(&self) -> BLOCK6_W1_R {
        BLOCK6_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK6_W1").field("block6_w1", &self.block6_w1()).finish()
    }
}
/**Otp debuger block6 data register1.

You can [`read`](crate::generic::Reg::read) this register and get [`blk6_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK6_W1_SPEC;
impl crate::RegisterSpec for BLK6_W1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk6_w1::R`](R) reader structure
impl crate::Readable for BLK6_W1_SPEC {}
///`reset()` method sets BLK6_W1 to value 0
impl crate::Resettable for BLK6_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
