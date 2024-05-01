///Register `CORE_1_RCD_SP` reader
pub type R = crate::R<CORE_1_RCD_SP_SPEC>;
///Field `CORE_1_RCD_SP` reader - Core1_stack pointer
pub type CORE_1_RCD_SP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Core1_stack pointer
    #[inline(always)]
    pub fn core_1_rcd_sp(&self) -> CORE_1_RCD_SP_R {
        CORE_1_RCD_SP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_SP")
            .field("core_1_rcd_sp", &self.core_1_rcd_sp())
            .finish()
    }
}
/**Core1 pdebug status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_sp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_RCD_SP_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_SP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_rcd_sp::R`](R) reader structure
impl crate::Readable for CORE_1_RCD_SP_SPEC {}
///`reset()` method sets CORE_1_RCD_SP to value 0
impl crate::Resettable for CORE_1_RCD_SP_SPEC {
    const RESET_VALUE: u32 = 0;
}
