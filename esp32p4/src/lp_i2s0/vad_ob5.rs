///Register `VAD_OB5` reader
pub type R = crate::R<VAD_OB5_SPEC>;
///Field `OFFSET_OB` reader - Reg offset observe signal
pub type OFFSET_OB_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Reg offset observe signal
    #[inline(always)]
    pub fn offset_ob(&self) -> OFFSET_OB_R {
        OFFSET_OB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB5")
            .field("offset_ob", &self.offset_ob())
            .finish()
    }
}
/**I2S VAD Observe register

You can [`read`](crate::generic::Reg::read) this register and get [`vad_ob5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VAD_OB5_SPEC;
impl crate::RegisterSpec for VAD_OB5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vad_ob5::R`](R) reader structure
impl crate::Readable for VAD_OB5_SPEC {}
///`reset()` method sets VAD_OB5 to value 0
impl crate::Resettable for VAD_OB5_SPEC {
    const RESET_VALUE: u32 = 0;
}
