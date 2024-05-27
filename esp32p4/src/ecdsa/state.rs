///Register `STATE` reader
pub type R = crate::R<STATE_SPEC>;
///Field `BUSY` reader - The status bits of ECDSA Accelerator. ECDSA is at 0: IDLE, 1: LOAD, 2: GET, 3: BUSY state.
pub type BUSY_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - The status bits of ECDSA Accelerator. ECDSA is at 0: IDLE, 1: LOAD, 2: GET, 3: BUSY state.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE").field("busy", &self.busy()).finish()
    }
}
/**ECDSA status register

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
