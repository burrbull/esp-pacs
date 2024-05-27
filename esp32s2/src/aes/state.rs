///Register `STATE` reader
pub type R = crate::R<STATE_SPEC>;
///Field `STATE` reader - Stores the working status of the AES Accelerator. For details, see Table 3 for Typical AES working mode and Table 9 for DMA AES working mode. For typical AES; 0 = idle; 1 = busy. For DMA-AES; 0 = idle; 1 = busy; 2 = calculation_done.
pub type STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Stores the working status of the AES Accelerator. For details, see Table 3 for Typical AES working mode and Table 9 for DMA AES working mode. For typical AES; 0 = idle; 1 = busy. For DMA-AES; 0 = idle; 1 = busy; 2 = calculation_done.
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("state", &self.state())
            .finish()
    }
}
/**Operation status register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`state::R`](R) reader structure
impl crate::Readable for STATE_SPEC {}
///`reset()` method sets STATE to value 0
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
