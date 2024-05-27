///Register `SDIO_SELECT` reader
pub type R = crate::R<SDIO_SELECT_SPEC>;
///Register `SDIO_SELECT` writer
pub type W = crate::W<SDIO_SELECT_SPEC>;
///Field `SDIO_SEL` reader - Reserved
pub type SDIO_SEL_R = crate::FieldReader;
///Field `SDIO_SEL` writer - Reserved
pub type SDIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    pub fn sdio_sel(&self) -> SDIO_SEL_R {
        SDIO_SEL_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SELECT")
            .field("sdio_sel", &self.sdio_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn sdio_sel(&mut self) -> SDIO_SEL_W<SDIO_SELECT_SPEC> {
        SDIO_SEL_W::new(self, 0)
    }
}
/**GPIO SDIO selection register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_SELECT_SPEC;
impl crate::RegisterSpec for SDIO_SELECT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_select::R`](R) reader structure
impl crate::Readable for SDIO_SELECT_SPEC {}
///`write(|w| ..)` method takes [`sdio_select::W`](W) writer structure
impl crate::Writable for SDIO_SELECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDIO_SELECT to value 0
impl crate::Resettable for SDIO_SELECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
