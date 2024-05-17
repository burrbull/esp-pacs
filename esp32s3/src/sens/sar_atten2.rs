///Register `SAR_ATTEN2` reader
pub type R = crate::R<SAR_ATTEN2_SPEC>;
///Register `SAR_ATTEN2` writer
pub type W = crate::W<SAR_ATTEN2_SPEC>;
///Field `SAR2_ATTEN` reader - 2-bit attenuation for each pad
pub type SAR2_ATTEN_R = crate::FieldReader<u32>;
///Field `SAR2_ATTEN` writer - 2-bit attenuation for each pad
pub type SAR2_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 2-bit attenuation for each pad
    #[inline(always)]
    pub fn sar2_atten(&self) -> SAR2_ATTEN_R {
        SAR2_ATTEN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_ATTEN2").field("sar2_atten", &self.sar2_atten()).finish()
    }
}
impl W {
    ///Bits 0:31 - 2-bit attenuation for each pad
    #[inline(always)]
    #[must_use]
    pub fn sar2_atten(&mut self) -> SAR2_ATTEN_W<SAR_ATTEN2_SPEC> {
        SAR2_ATTEN_W::new(self, 0)
    }
}
/**configure saradc2 controller

You can [`read`](crate::generic::Reg::read) this register and get [`sar_atten2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_atten2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_ATTEN2_SPEC;
impl crate::RegisterSpec for SAR_ATTEN2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_atten2::R`](R) reader structure
impl crate::Readable for SAR_ATTEN2_SPEC {}
///`write(|w| ..)` method takes [`sar_atten2::W`](W) writer structure
impl crate::Writable for SAR_ATTEN2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_ATTEN2 to value 0xffff_ffff
impl crate::Resettable for SAR_ATTEN2_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
