///Register `CMDARG` reader
pub type R = crate::R<CMDARG_SPEC>;
///Register `CMDARG` writer
pub type W = crate::W<CMDARG_SPEC>;
///Field `CMDARG` reader - Value indicates command argument to be passed to the card.
pub type CMDARG_R = crate::FieldReader<u32>;
///Field `CMDARG` writer - Value indicates command argument to be passed to the card.
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Value indicates command argument to be passed to the card.
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDARG").field("cmdarg", &self.cmdarg()).finish()
    }
}
impl W {
    ///Bits 0:31 - Value indicates command argument to be passed to the card.
    #[inline(always)]
    #[must_use]
    pub fn cmdarg(&mut self) -> CMDARG_W<CMDARG_SPEC> {
        CMDARG_W::new(self, 0)
    }
}
/**Command argument data register

You can [`read`](crate::generic::Reg::read) this register and get [`cmdarg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdarg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMDARG_SPEC;
impl crate::RegisterSpec for CMDARG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cmdarg::R`](R) reader structure
impl crate::Readable for CMDARG_SPEC {}
///`write(|w| ..)` method takes [`cmdarg::W`](W) writer structure
impl crate::Writable for CMDARG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMDARG to value 0
impl crate::Resettable for CMDARG_SPEC {
    const RESET_VALUE: u32 = 0;
}
