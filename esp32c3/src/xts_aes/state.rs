///Register `STATE` reader
pub type R = crate::R<STATE_SPEC>;
///Field `STATE` reader - Those bits shows XTS-AES status. 0=IDLE, 1=WORK, 2=RELEASE, 3=USE. IDLE means that XTS-AES is idle. WORK means that XTS-AES is busy with calculation. RELEASE means the encrypted result is generated but not visible to mspi. USE means that the encrypted result is visible to mspi.
pub type STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Those bits shows XTS-AES status. 0=IDLE, 1=WORK, 2=RELEASE, 3=USE. IDLE means that XTS-AES is idle. WORK means that XTS-AES is busy with calculation. RELEASE means the encrypted result is generated but not visible to mspi. USE means that the encrypted result is visible to mspi.
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE").field("state", &self.state()).finish()
    }
}
/**XTS-AES status register

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
