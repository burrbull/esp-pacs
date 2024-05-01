///Register `NEGPULSE` reader
pub type R = crate::R<NEGPULSE_SPEC>;
///Field `NEGEDGE_MIN_CNT` reader - This register stores the minimal input clock count between two negative edges. It is used in baud rate detection.
pub type NEGEDGE_MIN_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19 - This register stores the minimal input clock count between two negative edges. It is used in baud rate detection.
    #[inline(always)]
    pub fn negedge_min_cnt(&self) -> NEGEDGE_MIN_CNT_R {
        NEGEDGE_MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NEGPULSE")
            .field("negedge_min_cnt", &self.negedge_min_cnt())
            .finish()
    }
}
/**Autobaud low pulse register

You can [`read`](crate::generic::Reg::read) this register and get [`negpulse::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NEGPULSE_SPEC;
impl crate::RegisterSpec for NEGPULSE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`negpulse::R`](R) reader structure
impl crate::Readable for NEGPULSE_SPEC {}
///`reset()` method sets NEGPULSE to value 0x000f_ffff
impl crate::Resettable for NEGPULSE_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
