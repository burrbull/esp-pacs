///Register `CORE_1_RCD_PDEBUGENABLE` reader
pub type R = crate::R<CORE_1_RCD_PDEBUGENABLE_SPEC>;
///Register `CORE_1_RCD_PDEBUGENABLE` writer
pub type W = crate::W<CORE_1_RCD_PDEBUGENABLE_SPEC>;
///Field `CORE_1_RCD_PDEBUGENABLE` reader - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC
pub type CORE_1_RCD_PDEBUGENABLE_R = crate::BitReader;
///Field `CORE_1_RCD_PDEBUGENABLE` writer - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC
pub type CORE_1_RCD_PDEBUGENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC
    #[inline(always)]
    pub fn core_1_rcd_pdebugenable(&self) -> CORE_1_RCD_PDEBUGENABLE_R {
        CORE_1_RCD_PDEBUGENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGENABLE")
            .field("core_1_rcd_pdebugenable", &self.core_1_rcd_pdebugenable())
            .finish()
    }
}
impl W {
    ///Bit 0 - Core1 Pdebugenable,set 1 to open Core1 Pdebug interface, then can get Core1 PC
    #[inline(always)]
    #[must_use]
    pub fn core_1_rcd_pdebugenable(
        &mut self,
    ) -> CORE_1_RCD_PDEBUGENABLE_W<CORE_1_RCD_PDEBUGENABLE_SPEC> {
        CORE_1_RCD_PDEBUGENABLE_W::new(self, 0)
    }
}
/**Core1 pdebug configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_rcd_pdebugenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_RCD_PDEBUGENABLE_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_rcd_pdebugenable::R`](R) reader structure
impl crate::Readable for CORE_1_RCD_PDEBUGENABLE_SPEC {}
///`write(|w| ..)` method takes [`core_1_rcd_pdebugenable::W`](W) writer structure
impl crate::Writable for CORE_1_RCD_PDEBUGENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_RCD_PDEBUGENABLE to value 0
impl crate::Resettable for CORE_1_RCD_PDEBUGENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
