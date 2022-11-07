#[doc = "Register `DOEPDMA4` reader"]
pub struct R(crate::R<DOEPDMA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMA4` writer"]
pub struct W(crate::W<DOEPDMA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA4_SPEC>;
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
impl From<crate::W<DOEPDMA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR4` reader - "]
pub type DMAADDR4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR4` writer - "]
pub type DMAADDR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPDMA4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr4(&self) -> DMAADDR4_R {
        DMAADDR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr4(&mut self) -> DMAADDR4_W<0> {
        DMAADDR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdma4](index.html) module"]
pub struct DOEPDMA4_SPEC;
impl crate::RegisterSpec for DOEPDMA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdma4::R](R) reader structure"]
impl crate::Readable for DOEPDMA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdma4::W](W) writer structure"]
impl crate::Writable for DOEPDMA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPDMA4 to value 0"]
impl crate::Resettable for DOEPDMA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}