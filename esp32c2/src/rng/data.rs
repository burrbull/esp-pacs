///Register `DATA` reader
pub type R = crate::R<DATA_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Random number data

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DATA_SPEC {}
///`reset()` method sets DATA to value 0
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
