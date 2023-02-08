#[doc = "Register `HP_APM_M0_INTR_MAP` reader"]
pub struct R(crate::R<HP_APM_M0_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_APM_M0_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_APM_M0_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_APM_M0_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_APM_M0_INTR_MAP` writer"]
pub struct W(crate::W<HP_APM_M0_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_APM_M0_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HP_APM_M0_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_APM_M0_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_APM_M0_INTR_MAP` reader - Need add description"]
pub type HP_APM_M0_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_APM_M0_INTR_MAP` writer - Need add description"]
pub type HP_APM_M0_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HP_APM_M0_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn hp_apm_m0_intr_map(&self) -> HP_APM_M0_INTR_MAP_R {
        HP_APM_M0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn hp_apm_m0_intr_map(&mut self) -> HP_APM_M0_INTR_MAP_W<0> {
        HP_APM_M0_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_apm_m0_intr_map](index.html) module"]
pub struct HP_APM_M0_INTR_MAP_SPEC;
impl crate::RegisterSpec for HP_APM_M0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_apm_m0_intr_map::R](R) reader structure"]
impl crate::Readable for HP_APM_M0_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_apm_m0_intr_map::W](W) writer structure"]
impl crate::Writable for HP_APM_M0_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_APM_M0_INTR_MAP to value 0"]
impl crate::Resettable for HP_APM_M0_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}