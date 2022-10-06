#[doc = "Register `SCL_START_PERIOD` reader"]
pub struct R(crate::R<SCL_START_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_START_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_START_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_START_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_START_PERIOD` writer"]
pub struct W(crate::W<SCL_START_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_START_PERIOD_SPEC>;
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
impl From<crate::W<SCL_START_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_START_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_START_PERIOD` reader - time period for SCL to toggle after I2C start is triggered"]
pub type SCL_START_PERIOD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCL_START_PERIOD` writer - time period for SCL to toggle after I2C start is triggered"]
pub type SCL_START_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCL_START_PERIOD_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - time period for SCL to toggle after I2C start is triggered"]
    #[inline(always)]
    pub fn scl_start_period(&self) -> SCL_START_PERIOD_R {
        SCL_START_PERIOD_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - time period for SCL to toggle after I2C start is triggered"]
    #[inline(always)]
    pub fn scl_start_period(&mut self) -> SCL_START_PERIOD_W<0> {
        SCL_START_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure scl start period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_start_period](index.html) module"]
pub struct SCL_START_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_START_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_start_period::R](R) reader structure"]
impl crate::Readable for SCL_START_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_start_period::W](W) writer structure"]
impl crate::Writable for SCL_START_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_START_PERIOD to value 0x08"]
impl crate::Resettable for SCL_START_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}