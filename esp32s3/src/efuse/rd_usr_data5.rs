///Register `RD_USR_DATA5` reader
pub type R = crate::R<RD_USR_DATA5_SPEC>;
///Field `USR_DATA5` reader - Stores the fifth 32 bits of BLOCK3 (user).
pub type USR_DATA5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the fifth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data5(&self) -> USR_DATA5_R {
        USR_DATA5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_USR_DATA5")
            .field("usr_data5", &self.usr_data5())
            .finish()
    }
}
/**Register 5 of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_USR_DATA5_SPEC;
impl crate::RegisterSpec for RD_USR_DATA5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_usr_data5::R`](R) reader structure
impl crate::Readable for RD_USR_DATA5_SPEC {}
///`reset()` method sets RD_USR_DATA5 to value 0
impl crate::Resettable for RD_USR_DATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
