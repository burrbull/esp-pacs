///Register `RD_KEY3_DATA3` reader
pub type R = crate::R<RD_KEY3_DATA3_SPEC>;
///Field `KEY3_DATA3` reader - Stores the third 32 bits of KEY3.
pub type KEY3_DATA3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the third 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data3(&self) -> KEY3_DATA3_R {
        KEY3_DATA3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY3_DATA3").field("key3_data3", &self.key3_data3()).finish()
    }
}
/**Register $n of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_KEY3_DATA3_SPEC;
impl crate::RegisterSpec for RD_KEY3_DATA3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_key3_data3::R`](R) reader structure
impl crate::Readable for RD_KEY3_DATA3_SPEC {}
///`reset()` method sets RD_KEY3_DATA3 to value 0
impl crate::Resettable for RD_KEY3_DATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
