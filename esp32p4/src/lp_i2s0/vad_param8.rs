#[doc = "Register `VAD_PARAM8` reader"]
pub type R = crate::R<VAD_PARAM8_SPEC>;
#[doc = "Register `VAD_PARAM8` writer"]
pub type W = crate::W<VAD_PARAM8_SPEC>;
#[doc = "Field `PARAM_THRES_UPD_BDL` reader - Noise_std boundary low when updating threshold."]
pub type PARAM_THRES_UPD_BDL_R = crate::FieldReader;
#[doc = "Field `PARAM_THRES_UPD_BDL` writer - Noise_std boundary low when updating threshold."]
pub type PARAM_THRES_UPD_BDL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARAM_THRES_UPD_BDH` reader - Noise_std boundary high when updating threshold."]
pub type PARAM_THRES_UPD_BDH_R = crate::FieldReader;
#[doc = "Field `PARAM_THRES_UPD_BDH` writer - Noise_std boundary high when updating threshold."]
pub type PARAM_THRES_UPD_BDH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARAM_FEATURE_BURST` reader - VAD parameter"]
pub type PARAM_FEATURE_BURST_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_FEATURE_BURST` writer - VAD parameter"]
pub type PARAM_FEATURE_BURST_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Noise_std boundary low when updating threshold."]
    #[inline(always)]
    pub fn param_thres_upd_bdl(&self) -> PARAM_THRES_UPD_BDL_R {
        PARAM_THRES_UPD_BDL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Noise_std boundary high when updating threshold."]
    #[inline(always)]
    pub fn param_thres_upd_bdh(&self) -> PARAM_THRES_UPD_BDH_R {
        PARAM_THRES_UPD_BDH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    pub fn param_feature_burst(&self) -> PARAM_FEATURE_BURST_R {
        PARAM_FEATURE_BURST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM8")
            .field(
                "param_thres_upd_bdl",
                &format_args!("{}", self.param_thres_upd_bdl().bits()),
            )
            .field(
                "param_thres_upd_bdh",
                &format_args!("{}", self.param_thres_upd_bdh().bits()),
            )
            .field(
                "param_feature_burst",
                &format_args!("{}", self.param_feature_burst().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_PARAM8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Noise_std boundary low when updating threshold."]
    #[inline(always)]
    #[must_use]
    pub fn param_thres_upd_bdl(&mut self) -> PARAM_THRES_UPD_BDL_W<VAD_PARAM8_SPEC> {
        PARAM_THRES_UPD_BDL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Noise_std boundary high when updating threshold."]
    #[inline(always)]
    #[must_use]
    pub fn param_thres_upd_bdh(&mut self) -> PARAM_THRES_UPD_BDH_W<VAD_PARAM8_SPEC> {
        PARAM_THRES_UPD_BDH_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_feature_burst(&mut self) -> PARAM_FEATURE_BURST_W<VAD_PARAM8_SPEC> {
        PARAM_FEATURE_BURST_W::new(self, 16)
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
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_param8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_param8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM8_SPEC;
impl crate::RegisterSpec for VAD_PARAM8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param8::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param8::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAD_PARAM8 to value 0x2000_5040"]
impl crate::Resettable for VAD_PARAM8_SPEC {
    const RESET_VALUE: u32 = 0x2000_5040;
}
