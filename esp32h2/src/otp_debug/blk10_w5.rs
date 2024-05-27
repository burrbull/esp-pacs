///Register `BLK10_W5` reader
pub type R = crate::R<BLK10_W5_SPEC>;
///Field `BLOCK10_W5` reader - Otp block10 word5 data.
pub type BLOCK10_W5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block10 word5 data.
    #[inline(always)]
    pub fn block10_w5(&self) -> BLOCK10_W5_R {
        BLOCK10_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK10_W5")
            .field("block10_w5", &self.block10_w5())
            .finish()
    }
}
/**Otp debuger block10 data register5.

You can [`read`](crate::generic::Reg::read) this register and get [`blk10_w5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK10_W5_SPEC;
impl crate::RegisterSpec for BLK10_W5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk10_w5::R`](R) reader structure
impl crate::Readable for BLK10_W5_SPEC {}
///`reset()` method sets BLK10_W5 to value 0
impl crate::Resettable for BLK10_W5_SPEC {
    const RESET_VALUE: u32 = 0;
}
