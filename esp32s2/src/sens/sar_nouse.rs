#[doc = "Register `SAR_NOUSE` reader"]
pub type R = crate::R<SAR_NOUSE_SPEC>;
#[doc = "Register `SAR_NOUSE` writer"]
pub type W = crate::W<SAR_NOUSE_SPEC>;
#[doc = "Field `SAR_NOUSE` reader - sar nouse"]
pub type SAR_NOUSE_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_NOUSE` writer - sar nouse"]
pub type SAR_NOUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sar nouse"]
    #[inline(always)]
    pub fn sar_nouse(&self) -> SAR_NOUSE_R {
        SAR_NOUSE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_NOUSE")
            .field("sar_nouse", &format_args!("{}", self.sar_nouse().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_NOUSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - sar nouse"]
    #[inline(always)]
    #[must_use]
    pub fn sar_nouse(&mut self) -> SAR_NOUSE_W<SAR_NOUSE_SPEC> {
        SAR_NOUSE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sar nouse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_nouse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_nouse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_NOUSE_SPEC;
impl crate::RegisterSpec for SAR_NOUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_nouse::R`](R) reader structure"]
impl crate::Readable for SAR_NOUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_nouse::W`](W) writer structure"]
impl crate::Writable for SAR_NOUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_NOUSE to value 0"]
impl crate::Resettable for SAR_NOUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
