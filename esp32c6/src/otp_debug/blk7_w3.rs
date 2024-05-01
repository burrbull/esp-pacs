///Register `BLK7_W3` reader
pub type R = crate::R<BLK7_W3_SPEC>;
///Field `BLOCK7_W3` reader - Otp block7 word3 data.
pub type BLOCK7_W3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block7 word3 data.
    #[inline(always)]
    pub fn block7_w3(&self) -> BLOCK7_W3_R {
        BLOCK7_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W3")
            .field("block7_w3", &self.block7_w3())
            .finish()
    }
}
/**Otp debuger block7 data register3.

You can [`read`](crate::generic::Reg::read) this register and get [`blk7_w3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK7_W3_SPEC;
impl crate::RegisterSpec for BLK7_W3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk7_w3::R`](R) reader structure
impl crate::Readable for BLK7_W3_SPEC {}
///`reset()` method sets BLK7_W3 to value 0
impl crate::Resettable for BLK7_W3_SPEC {
    const RESET_VALUE: u32 = 0;
}
