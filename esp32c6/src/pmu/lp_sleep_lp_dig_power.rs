#[doc = "Register `LP_SLEEP_LP_DIG_POWER` reader"]
pub struct R(crate::R<LP_SLEEP_LP_DIG_POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_SLEEP_LP_DIG_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_SLEEP_LP_DIG_POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_SLEEP_LP_DIG_POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_SLEEP_LP_DIG_POWER` writer"]
pub struct W(crate::W<LP_SLEEP_LP_DIG_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_SLEEP_LP_DIG_POWER_SPEC>;
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
impl From<crate::W<LP_SLEEP_LP_DIG_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_SLEEP_LP_DIG_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_SLEEP_LP_MEM_DSLP` reader - need_des"]
pub type LP_SLEEP_LP_MEM_DSLP_R = crate::BitReader<bool>;
#[doc = "Field `LP_SLEEP_LP_MEM_DSLP` writer - need_des"]
pub type LP_SLEEP_LP_MEM_DSLP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_SLEEP_LP_DIG_POWER_SPEC, bool, O>;
#[doc = "Field `LP_SLEEP_PD_LP_PERI_PD_EN` reader - need_des"]
pub type LP_SLEEP_PD_LP_PERI_PD_EN_R = crate::BitReader<bool>;
#[doc = "Field `LP_SLEEP_PD_LP_PERI_PD_EN` writer - need_des"]
pub type LP_SLEEP_PD_LP_PERI_PD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_SLEEP_LP_DIG_POWER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_mem_dslp(&self) -> LP_SLEEP_LP_MEM_DSLP_R {
        LP_SLEEP_LP_MEM_DSLP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_pd_lp_peri_pd_en(&self) -> LP_SLEEP_PD_LP_PERI_PD_EN_R {
        LP_SLEEP_PD_LP_PERI_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_mem_dslp(&mut self) -> LP_SLEEP_LP_MEM_DSLP_W<30> {
        LP_SLEEP_LP_MEM_DSLP_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_pd_lp_peri_pd_en(&mut self) -> LP_SLEEP_PD_LP_PERI_PD_EN_W<31> {
        LP_SLEEP_PD_LP_PERI_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_sleep_lp_dig_power](index.html) module"]
pub struct LP_SLEEP_LP_DIG_POWER_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_DIG_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_sleep_lp_dig_power::R](R) reader structure"]
impl crate::Readable for LP_SLEEP_LP_DIG_POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_sleep_lp_dig_power::W](W) writer structure"]
impl crate::Writable for LP_SLEEP_LP_DIG_POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_DIG_POWER to value 0"]
impl crate::Resettable for LP_SLEEP_LP_DIG_POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}