///Register `BLK1_W6` reader
pub type R = crate::R<BLK1_W6_SPEC>;
///Field `BLOCK1_W6` reader - Otp block1 word6 data.
pub type BLOCK1_W6_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block1 word6 data.
    #[inline(always)]
    pub fn block1_w6(&self) -> BLOCK1_W6_R {
        BLOCK1_W6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_W6")
            .field("block1_w6", &self.block1_w6())
            .finish()
    }
}
/**Otp debuger block1 data register6.

You can [`read`](crate::generic::Reg::read) this register and get [`blk1_w6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK1_W6_SPEC;
impl crate::RegisterSpec for BLK1_W6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk1_w6::R`](R) reader structure
impl crate::Readable for BLK1_W6_SPEC {}
///`reset()` method sets BLK1_W6 to value 0
impl crate::Resettable for BLK1_W6_SPEC {
    const RESET_VALUE: u32 = 0;
}
