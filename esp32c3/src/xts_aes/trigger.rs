///Register `TRIGGER` writer
pub type W = crate::W<TRIGGER_SPEC>;
///Field `TRIGGER` writer - Set this bit to start manual encryption calculation
pub type TRIGGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to start manual encryption calculation
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<TRIGGER_SPEC> {
        TRIGGER_W::new(self, 0)
    }
}
/**XTS-AES trigger register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`trigger::W`](W) writer structure
impl crate::Writable for TRIGGER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TRIGGER to value 0
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: u32 = 0;
}
