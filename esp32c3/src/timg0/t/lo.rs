///Register `LO` reader
pub type R = crate::R<LO_SPEC>;
///Field `LO` reader - t0_lo
pub type LO_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - t0_lo
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LO").field("lo", &self.lo()).finish()
    }
}
/**TIMG_T0LO_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lo::R`](R) reader structure
impl crate::Readable for LO_SPEC {}
///`reset()` method sets LO to value 0
impl crate::Resettable for LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
