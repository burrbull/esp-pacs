///Register `DEV_CHAR_TABLE4_LOC4` reader
pub type R = crate::R<DEV_CHAR_TABLE4_LOC4_SPEC>;
///Field `DCT_DEV4_LOC4` reader - NA
pub type DCT_DEV4_LOC4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn dct_dev4_loc4(&self) -> DCT_DEV4_LOC4_R {
        DCT_DEV4_LOC4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_CHAR_TABLE4_LOC4")
            .field("dct_dev4_loc4", &self.dct_dev4_loc4())
            .finish()
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`dev_char_table4_loc4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEV_CHAR_TABLE4_LOC4_SPEC;
impl crate::RegisterSpec for DEV_CHAR_TABLE4_LOC4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dev_char_table4_loc4::R`](R) reader structure
impl crate::Readable for DEV_CHAR_TABLE4_LOC4_SPEC {}
///`reset()` method sets DEV_CHAR_TABLE4_LOC4 to value 0
impl crate::Resettable for DEV_CHAR_TABLE4_LOC4_SPEC {
    const RESET_VALUE: u32 = 0;
}
