#[doc = "Register `VAD_PARAM1` reader"]
pub type R = crate::R<VAD_PARAM1_SPEC>;
#[doc = "Register `VAD_PARAM1` writer"]
pub type W = crate::W<VAD_PARAM1_SPEC>;
#[doc = "Field `PARAM_MIN_SPEECH_COUNT` reader - VAD parameter"]
pub type PARAM_MIN_SPEECH_COUNT_R = crate::FieldReader;
#[doc = "Field `PARAM_MIN_SPEECH_COUNT` writer - VAD parameter"]
pub type PARAM_MIN_SPEECH_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PARAM_MAX_SPEECH_COUNT` reader - VAD parameter"]
pub type PARAM_MAX_SPEECH_COUNT_R = crate::FieldReader;
#[doc = "Field `PARAM_MAX_SPEECH_COUNT` writer - VAD parameter"]
pub type PARAM_MAX_SPEECH_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PARAM_HANGOVER_SPEECH` reader - VAD parameter"]
pub type PARAM_HANGOVER_SPEECH_R = crate::FieldReader;
#[doc = "Field `PARAM_HANGOVER_SPEECH` writer - VAD parameter"]
pub type PARAM_HANGOVER_SPEECH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PARAM_HANGOVER_SILENT` reader - VAD parameter"]
pub type PARAM_HANGOVER_SILENT_R = crate::FieldReader;
#[doc = "Field `PARAM_HANGOVER_SILENT` writer - VAD parameter"]
pub type PARAM_HANGOVER_SILENT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARAM_MAX_OFFSET` reader - VAD parameter"]
pub type PARAM_MAX_OFFSET_R = crate::FieldReader;
#[doc = "Field `PARAM_MAX_OFFSET` writer - VAD parameter"]
pub type PARAM_MAX_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PARAM_SKIP_BAND_ENERGY` reader - Set 1 to skip band energy check."]
pub type PARAM_SKIP_BAND_ENERGY_R = crate::BitReader;
#[doc = "Field `PARAM_SKIP_BAND_ENERGY` writer - Set 1 to skip band energy check."]
pub type PARAM_SKIP_BAND_ENERGY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - VAD parameter"]
    #[inline(always)]
    pub fn param_min_speech_count(&self) -> PARAM_MIN_SPEECH_COUNT_R {
        PARAM_MIN_SPEECH_COUNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:10 - VAD parameter"]
    #[inline(always)]
    pub fn param_max_speech_count(&self) -> PARAM_MAX_SPEECH_COUNT_R {
        PARAM_MAX_SPEECH_COUNT_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:15 - VAD parameter"]
    #[inline(always)]
    pub fn param_hangover_speech(&self) -> PARAM_HANGOVER_SPEECH_R {
        PARAM_HANGOVER_SPEECH_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - VAD parameter"]
    #[inline(always)]
    pub fn param_hangover_silent(&self) -> PARAM_HANGOVER_SILENT_R {
        PARAM_HANGOVER_SILENT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - VAD parameter"]
    #[inline(always)]
    pub fn param_max_offset(&self) -> PARAM_MAX_OFFSET_R {
        PARAM_MAX_OFFSET_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Set 1 to skip band energy check."]
    #[inline(always)]
    pub fn param_skip_band_energy(&self) -> PARAM_SKIP_BAND_ENERGY_R {
        PARAM_SKIP_BAND_ENERGY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM1")
            .field(
                "param_min_speech_count",
                &format_args!("{}", self.param_min_speech_count().bits()),
            )
            .field(
                "param_max_speech_count",
                &format_args!("{}", self.param_max_speech_count().bits()),
            )
            .field(
                "param_hangover_speech",
                &format_args!("{}", self.param_hangover_speech().bits()),
            )
            .field(
                "param_hangover_silent",
                &format_args!("{}", self.param_hangover_silent().bits()),
            )
            .field(
                "param_max_offset",
                &format_args!("{}", self.param_max_offset().bits()),
            )
            .field(
                "param_skip_band_energy",
                &format_args!("{}", self.param_skip_band_energy().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_PARAM1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_min_speech_count(&mut self) -> PARAM_MIN_SPEECH_COUNT_W<VAD_PARAM1_SPEC> {
        PARAM_MIN_SPEECH_COUNT_W::new(self, 0)
    }
    #[doc = "Bits 4:10 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_max_speech_count(&mut self) -> PARAM_MAX_SPEECH_COUNT_W<VAD_PARAM1_SPEC> {
        PARAM_MAX_SPEECH_COUNT_W::new(self, 4)
    }
    #[doc = "Bits 11:15 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_hangover_speech(&mut self) -> PARAM_HANGOVER_SPEECH_W<VAD_PARAM1_SPEC> {
        PARAM_HANGOVER_SPEECH_W::new(self, 11)
    }
    #[doc = "Bits 16:23 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_hangover_silent(&mut self) -> PARAM_HANGOVER_SILENT_W<VAD_PARAM1_SPEC> {
        PARAM_HANGOVER_SILENT_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - VAD parameter"]
    #[inline(always)]
    #[must_use]
    pub fn param_max_offset(&mut self) -> PARAM_MAX_OFFSET_W<VAD_PARAM1_SPEC> {
        PARAM_MAX_OFFSET_W::new(self, 24)
    }
    #[doc = "Bit 31 - Set 1 to skip band energy check."]
    #[inline(always)]
    #[must_use]
    pub fn param_skip_band_energy(&mut self) -> PARAM_SKIP_BAND_ENERGY_W<VAD_PARAM1_SPEC> {
        PARAM_SKIP_BAND_ENERGY_W::new(self, 31)
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
#[doc = "I2S VAD Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_param1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_param1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_PARAM1_SPEC;
impl crate::RegisterSpec for VAD_PARAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_param1::R`](R) reader structure"]
impl crate::Readable for VAD_PARAM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vad_param1::W`](W) writer structure"]
impl crate::Writable for VAD_PARAM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VAD_PARAM1 to value 0x281e_1e43"]
impl crate::Resettable for VAD_PARAM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x281e_1e43;
}