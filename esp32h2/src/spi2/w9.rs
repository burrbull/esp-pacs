#[doc = "Register `W9` reader"]
pub struct R(crate::R<W9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W9` writer"]
pub struct W(crate::W<W9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W9_SPEC>;
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
impl From<crate::W<W9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF9` reader - data buffer"]
pub type BUF9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF9` writer - data buffer"]
pub type BUF9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, W9_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf9(&self) -> BUF9_R {
        BUF9_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf9(&mut self) -> BUF9_W<0> {
        BUF9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w9](index.html) module"]
pub struct W9_SPEC;
impl crate::RegisterSpec for W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w9::R](R) reader structure"]
impl crate::Readable for W9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w9::W](W) writer structure"]
impl crate::Writable for W9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W9 to value 0"]
impl crate::Resettable for W9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}