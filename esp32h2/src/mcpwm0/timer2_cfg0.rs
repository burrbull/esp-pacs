#[doc = "Register `TIMER2_CFG0` reader"]
pub type R = crate::R<TIMER2_CFG0_SPEC>;
#[doc = "Register `TIMER2_CFG0` writer"]
pub type W = crate::W<TIMER2_CFG0_SPEC>;
#[doc = "Field `TIMER2_PRESCALE` reader - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
pub type TIMER2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TIMER2_PRESCALE` writer - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
pub type TIMER2_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER2_PERIOD` reader - period shadow register of PWM timer2"]
pub type TIMER2_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER2_PERIOD` writer - period shadow register of PWM timer2"]
pub type TIMER2_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER2_PERIOD_UPMETHOD` reader - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub type TIMER2_PERIOD_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER2_PERIOD_UPMETHOD` writer - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
pub type TIMER2_PERIOD_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
    #[inline(always)]
    pub fn timer2_prescale(&self) -> TIMER2_PRESCALE_R {
        TIMER2_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer2"]
    #[inline(always)]
    pub fn timer2_period(&self) -> TIMER2_PERIOD_R {
        TIMER2_PERIOD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer2_period_upmethod(&self) -> TIMER2_PERIOD_UPMETHOD_R {
        TIMER2_PERIOD_UPMETHOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2_CFG0")
            .field(
                "timer2_prescale",
                &format_args!("{}", self.timer2_prescale().bits()),
            )
            .field(
                "timer2_period",
                &format_args!("{}", self.timer2_period().bits()),
            )
            .field(
                "timer2_period_upmethod",
                &format_args!("{}", self.timer2_period_upmethod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - period of PT0_clk = Period of PWM_clk * (PWM_timer2_PRESCALE + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_prescale(&mut self) -> TIMER2_PRESCALE_W<TIMER2_CFG0_SPEC> {
        TIMER2_PRESCALE_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - period shadow register of PWM timer2"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_period(&mut self) -> TIMER2_PERIOD_W<TIMER2_CFG0_SPEC> {
        TIMER2_PERIOD_W::new(self, 8)
    }
    #[doc = "Bits 24:25 - Update method for active register of PWM timer2 period, 0: immediate, 1: TEZ, 2: sync, 3: TEZ | sync. TEZ here and below means timer equal zero event"]
    #[inline(always)]
    #[must_use]
    pub fn timer2_period_upmethod(&mut self) -> TIMER2_PERIOD_UPMETHOD_W<TIMER2_CFG0_SPEC> {
        TIMER2_PERIOD_UPMETHOD_W::new(self, 24)
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
#[doc = "PWM timer2 period and update method configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER2_CFG0_SPEC;
impl crate::RegisterSpec for TIMER2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_cfg0::R`](R) reader structure"]
impl crate::Readable for TIMER2_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer2_cfg0::W`](W) writer structure"]
impl crate::Writable for TIMER2_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2_CFG0 to value 0xff00"]
impl crate::Resettable for TIMER2_CFG0_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
