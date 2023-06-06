#[doc = "Register `SLC0HOST_LEN_WD` reader"]
pub struct R(crate::R<SLC0HOST_LEN_WD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC0HOST_LEN_WD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC0HOST_LEN_WD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC0HOST_LEN_WD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC0HOST_LEN_WD` writer"]
pub struct W(crate::W<SLC0HOST_LEN_WD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC0HOST_LEN_WD_SPEC>;
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
impl From<crate::W<SLC0HOST_LEN_WD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC0HOST_LEN_WD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0HOST_LEN_WD` reader - *******Description***********"]
pub type SLC0HOST_LEN_WD_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0HOST_LEN_WD` writer - *******Description***********"]
pub type SLC0HOST_LEN_WD_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLC0HOST_LEN_WD_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc0host_len_wd(&self) -> SLC0HOST_LEN_WD_R {
        SLC0HOST_LEN_WD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0HOST_LEN_WD")
            .field(
                "slc0host_len_wd",
                &format_args!("{}", self.slc0host_len_wd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0HOST_LEN_WD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_len_wd(&mut self) -> SLC0HOST_LEN_WD_W<0> {
        SLC0HOST_LEN_WD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc0host_len_wd](index.html) module"]
pub struct SLC0HOST_LEN_WD_SPEC;
impl crate::RegisterSpec for SLC0HOST_LEN_WD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc0host_len_wd::R](R) reader structure"]
impl crate::Readable for SLC0HOST_LEN_WD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc0host_len_wd::W](W) writer structure"]
impl crate::Writable for SLC0HOST_LEN_WD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC0HOST_LEN_WD to value 0"]
impl crate::Resettable for SLC0HOST_LEN_WD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
