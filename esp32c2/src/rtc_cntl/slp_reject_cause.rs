///Register `SLP_REJECT_CAUSE` reader
pub type R = crate::R<SLP_REJECT_CAUSE_SPEC>;
///Register `SLP_REJECT_CAUSE` writer
pub type W = crate::W<SLP_REJECT_CAUSE_SPEC>;
///Field `REJECT_CAUSE` reader - sleep reject cause
pub type REJECT_CAUSE_R = crate::FieldReader<u32>;
///Field `REJECT_CAUSE` writer - sleep reject cause
pub type REJECT_CAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 0:17 - sleep reject cause
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_REJECT_CAUSE")
            .field("reject_cause", &self.reject_cause())
            .finish()
    }
}
impl W {
    ///Bits 0:17 - sleep reject cause
    #[inline(always)]
    #[must_use]
    pub fn reject_cause(&mut self) -> REJECT_CAUSE_W<SLP_REJECT_CAUSE_SPEC> {
        REJECT_CAUSE_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_cause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_cause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLP_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slp_reject_cause::R`](R) reader structure
impl crate::Readable for SLP_REJECT_CAUSE_SPEC {}
///`write(|w| ..)` method takes [`slp_reject_cause::W`](W) writer structure
impl crate::Writable for SLP_REJECT_CAUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_REJECT_CAUSE to value 0
impl crate::Resettable for SLP_REJECT_CAUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
