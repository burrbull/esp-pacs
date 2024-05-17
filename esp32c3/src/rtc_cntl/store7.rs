///Register `STORE7` reader
pub type R = crate::R<STORE7_SPEC>;
///Register `STORE7` writer
pub type W = crate::W<STORE7_SPEC>;
///Field `SCRATCH7` reader - reserved register
pub type SCRATCH7_R = crate::FieldReader<u32>;
///Field `SCRATCH7` writer - reserved register
pub type SCRATCH7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reserved register
    #[inline(always)]
    pub fn scratch7(&self) -> SCRATCH7_R {
        SCRATCH7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE7").field("scratch7", &self.scratch7()).finish()
    }
}
impl W {
    ///Bits 0:31 - reserved register
    #[inline(always)]
    #[must_use]
    pub fn scratch7(&mut self) -> SCRATCH7_W<STORE7_SPEC> {
        SCRATCH7_W::new(self, 0)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STORE7_SPEC;
impl crate::RegisterSpec for STORE7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`store7::R`](R) reader structure
impl crate::Readable for STORE7_SPEC {}
///`write(|w| ..)` method takes [`store7::W`](W) writer structure
impl crate::Writable for STORE7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STORE7 to value 0
impl crate::Resettable for STORE7_SPEC {
    const RESET_VALUE: u32 = 0;
}
