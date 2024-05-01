///Register `SHA384_BUSY` reader
pub type R = crate::R<SHA384_BUSY_SPEC>;
///Field `SHA384_BUSY` reader - SHA-384 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle.
pub type SHA384_BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - SHA-384 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle.
    #[inline(always)]
    pub fn sha384_busy(&self) -> SHA384_BUSY_R {
        SHA384_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA384_BUSY")
            .field("sha384_busy", &self.sha384_busy())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sha384_busy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHA384_BUSY_SPEC;
impl crate::RegisterSpec for SHA384_BUSY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sha384_busy::R`](R) reader structure
impl crate::Readable for SHA384_BUSY_SPEC {}
///`reset()` method sets SHA384_BUSY to value 0
impl crate::Resettable for SHA384_BUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
