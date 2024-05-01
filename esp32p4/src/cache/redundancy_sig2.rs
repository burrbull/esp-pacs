///Register `REDUNDANCY_SIG2` reader
pub type R = crate::R<REDUNDANCY_SIG2_SPEC>;
///Register `REDUNDANCY_SIG2` writer
pub type W = crate::W<REDUNDANCY_SIG2_SPEC>;
///Field `REDCY_SIG2` reader - Those bits are prepared for ECO.
pub type REDCY_SIG2_R = crate::FieldReader<u32>;
///Field `REDCY_SIG2` writer - Those bits are prepared for ECO.
pub type REDCY_SIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Those bits are prepared for ECO.
    #[inline(always)]
    pub fn redcy_sig2(&self) -> REDCY_SIG2_R {
        REDCY_SIG2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANCY_SIG2")
            .field("redcy_sig2", &self.redcy_sig2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Those bits are prepared for ECO.
    #[inline(always)]
    #[must_use]
    pub fn redcy_sig2(&mut self) -> REDCY_SIG2_W<REDUNDANCY_SIG2_SPEC> {
        REDCY_SIG2_W::new(self, 0)
    }
}
/**Cache redundancy signal 2 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REDUNDANCY_SIG2_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`redundancy_sig2::R`](R) reader structure
impl crate::Readable for REDUNDANCY_SIG2_SPEC {}
///`write(|w| ..)` method takes [`redundancy_sig2::W`](W) writer structure
impl crate::Writable for REDUNDANCY_SIG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REDUNDANCY_SIG2 to value 0
impl crate::Resettable for REDUNDANCY_SIG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
