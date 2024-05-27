///Register `_0_EOF_START_DES` reader
pub type R = crate::R<_0_EOF_START_DES_SPEC>;
///Field `SLC0_EOF_START_DES_ADDR` reader -
pub type SLC0_EOF_START_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn slc0_eof_start_des_addr(&self) -> SLC0_EOF_START_DES_ADDR_R {
        SLC0_EOF_START_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_EOF_START_DES")
            .field("slc0_eof_start_des_addr", &self.slc0_eof_start_des_addr())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_0_eof_start_des::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _0_EOF_START_DES_SPEC;
impl crate::RegisterSpec for _0_EOF_START_DES_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_0_eof_start_des::R`](R) reader structure
impl crate::Readable for _0_EOF_START_DES_SPEC {}
///`reset()` method sets _0_EOF_START_DES to value 0
impl crate::Resettable for _0_EOF_START_DES_SPEC {
    const RESET_VALUE: u32 = 0;
}
