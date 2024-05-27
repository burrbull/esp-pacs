///Register `AE_WINPIXNUM` reader
pub type R = crate::R<AE_WINPIXNUM_SPEC>;
///Register `AE_WINPIXNUM` writer
pub type W = crate::W<AE_WINPIXNUM_SPEC>;
///Field `AE_SUBWIN_PIXNUM` reader - this field configures the pixel number of each sub win
pub type AE_SUBWIN_PIXNUM_R = crate::FieldReader<u32>;
///Field `AE_SUBWIN_PIXNUM` writer - this field configures the pixel number of each sub win
pub type AE_SUBWIN_PIXNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    ///Bits 0:16 - this field configures the pixel number of each sub win
    #[inline(always)]
    pub fn ae_subwin_pixnum(&self) -> AE_SUBWIN_PIXNUM_R {
        AE_SUBWIN_PIXNUM_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_WINPIXNUM")
            .field("ae_subwin_pixnum", &self.ae_subwin_pixnum())
            .finish()
    }
}
impl W {
    ///Bits 0:16 - this field configures the pixel number of each sub win
    #[inline(always)]
    #[must_use]
    pub fn ae_subwin_pixnum(&mut self) -> AE_SUBWIN_PIXNUM_W<AE_WINPIXNUM_SPEC> {
        AE_SUBWIN_PIXNUM_W::new(self, 0)
    }
}
/**ae sub-window pix num register

You can [`read`](crate::generic::Reg::read) this register and get [`ae_winpixnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_winpixnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AE_WINPIXNUM_SPEC;
impl crate::RegisterSpec for AE_WINPIXNUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ae_winpixnum::R`](R) reader structure
impl crate::Readable for AE_WINPIXNUM_SPEC {}
///`write(|w| ..)` method takes [`ae_winpixnum::W`](W) writer structure
impl crate::Writable for AE_WINPIXNUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AE_WINPIXNUM to value 0x0001_4400
impl crate::Resettable for AE_WINPIXNUM_SPEC {
    const RESET_VALUE: u32 = 0x0001_4400;
}
