///Register `RD_KEY0_DATA6` reader
pub type R = crate::R<RD_KEY0_DATA6_SPEC>;
///Field `KEY0_DATA6` reader - Stores the sixth 32 bits of KEY0.
pub type KEY0_DATA6_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the sixth 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data6(&self) -> KEY0_DATA6_R {
        KEY0_DATA6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY0_DATA6").field("key0_data6", &self.key0_data6()).finish()
    }
}
/**Register 6 of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_KEY0_DATA6_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_key0_data6::R`](R) reader structure
impl crate::Readable for RD_KEY0_DATA6_SPEC {}
///`reset()` method sets RD_KEY0_DATA6 to value 0
impl crate::Resettable for RD_KEY0_DATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
