///Register `BLK8_W11` reader
pub type R = crate::R<BLK8_W11_SPEC>;
///Field `BLOCK8_W11` reader - Otp block8 word11 data.
pub type BLOCK8_W11_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block8 word11 data.
    #[inline(always)]
    pub fn block8_w11(&self) -> BLOCK8_W11_R {
        BLOCK8_W11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK8_W11").field("block8_w11", &self.block8_w11()).finish()
    }
}
/**Otp debuger block8 data register11.

You can [`read`](crate::generic::Reg::read) this register and get [`blk8_w11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK8_W11_SPEC;
impl crate::RegisterSpec for BLK8_W11_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk8_w11::R`](R) reader structure
impl crate::Readable for BLK8_W11_SPEC {}
///`reset()` method sets BLK8_W11 to value 0
impl crate::Resettable for BLK8_W11_SPEC {
    const RESET_VALUE: u32 = 0;
}
