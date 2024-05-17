///Register `LOWPULSE` reader
pub type R = crate::R<LOWPULSE_SPEC>;
///Field `MIN_CNT` reader - This register stores the value of the minimum duration time of the low level pulse. It is used in baud rate detection.
pub type MIN_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19 - This register stores the value of the minimum duration time of the low level pulse. It is used in baud rate detection.
    #[inline(always)]
    pub fn min_cnt(&self) -> MIN_CNT_R {
        MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOWPULSE").field("min_cnt", &self.min_cnt()).finish()
    }
}
/**Autobaud minimum low pulse duration register

You can [`read`](crate::generic::Reg::read) this register and get [`lowpulse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOWPULSE_SPEC;
impl crate::RegisterSpec for LOWPULSE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lowpulse::R`](R) reader structure
impl crate::Readable for LOWPULSE_SPEC {}
///`reset()` method sets LOWPULSE to value 0x000f_ffff
impl crate::Resettable for LOWPULSE_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
