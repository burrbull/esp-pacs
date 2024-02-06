#[doc = "Register `ENABLE_W1TS` writer"]
pub type W = crate::W<ENABLE_W1TS_SPEC>;
#[doc = "Field `REG_RTCIO_REG_GPIO_ENABLE_W1TS` writer - GPIO0 ~ 21 output enable set register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_ENABLE_REG will be set to 1. Recommended operation: use this register to set RTCIO_RTC_GPIO_ENABLE_REG."]
pub type REG_RTCIO_REG_GPIO_ENABLE_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable set register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_ENABLE_REG will be set to 1. Recommended operation: use this register to set RTCIO_RTC_GPIO_ENABLE_REG."]
    #[inline(always)]
    #[must_use]
    pub fn reg_rtcio_reg_gpio_enable_w1ts(
        &mut self,
    ) -> REG_RTCIO_REG_GPIO_ENABLE_W1TS_W<ENABLE_W1TS_SPEC> {
        REG_RTCIO_REG_GPIO_ENABLE_W1TS_W::new(self, 10)
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
#[doc = "RTC GPIO output enable bit set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable_w1ts::W`](W) writer structure"]
impl crate::Writable for ENABLE_W1TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE_W1TS to value 0"]
impl crate::Resettable for ENABLE_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
