///Register `SLP_TIMER0` reader
pub type R = crate::R<SLP_TIMER0_SPEC>;
///Register `SLP_TIMER0` writer
pub type W = crate::W<SLP_TIMER0_SPEC>;
///Field `SLP_VAL_LO` reader - RTC sleep timer low 32 bits
pub type SLP_VAL_LO_R = crate::FieldReader<u32>;
///Field `SLP_VAL_LO` writer - RTC sleep timer low 32 bits
pub type SLP_VAL_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - RTC sleep timer low 32 bits
    #[inline(always)]
    pub fn slp_val_lo(&self) -> SLP_VAL_LO_R {
        SLP_VAL_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER0").field("slp_val_lo", &self.slp_val_lo()).finish()
    }
}
impl W {
    ///Bits 0:31 - RTC sleep timer low 32 bits
    #[inline(always)]
    #[must_use]
    pub fn slp_val_lo(&mut self) -> SLP_VAL_LO_W<SLP_TIMER0_SPEC> {
        SLP_VAL_LO_W::new(self, 0)
    }
}
/**configure min sleep time

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLP_TIMER0_SPEC;
impl crate::RegisterSpec for SLP_TIMER0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slp_timer0::R`](R) reader structure
impl crate::Readable for SLP_TIMER0_SPEC {}
///`write(|w| ..)` method takes [`slp_timer0::W`](W) writer structure
impl crate::Writable for SLP_TIMER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_TIMER0 to value 0
impl crate::Resettable for SLP_TIMER0_SPEC {
    const RESET_VALUE: u32 = 0;
}
