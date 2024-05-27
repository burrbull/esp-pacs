///Register `OPERATOR_TIMERSEL` reader
pub type R = crate::R<OPERATOR_TIMERSEL_SPEC>;
///Register `OPERATOR_TIMERSEL` writer
pub type W = crate::W<OPERATOR_TIMERSEL_SPEC>;
///Field `OPERATOR0_TIMERSEL` reader - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2
pub type OPERATOR0_TIMERSEL_R = crate::FieldReader;
///Field `OPERATOR0_TIMERSEL` writer - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2
pub type OPERATOR0_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OPERATOR1_TIMERSEL` reader - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2
pub type OPERATOR1_TIMERSEL_R = crate::FieldReader;
///Field `OPERATOR1_TIMERSEL` writer - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2
pub type OPERATOR1_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OPERATOR2_TIMERSEL` reader - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2
pub type OPERATOR2_TIMERSEL_R = crate::FieldReader;
///Field `OPERATOR2_TIMERSEL` writer - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2
pub type OPERATOR2_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2
    #[inline(always)]
    pub fn operator0_timersel(&self) -> OPERATOR0_TIMERSEL_R {
        OPERATOR0_TIMERSEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2
    #[inline(always)]
    pub fn operator1_timersel(&self) -> OPERATOR1_TIMERSEL_R {
        OPERATOR1_TIMERSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2
    #[inline(always)]
    pub fn operator2_timersel(&self) -> OPERATOR2_TIMERSEL_R {
        OPERATOR2_TIMERSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPERATOR_TIMERSEL")
            .field("operator0_timersel", &self.operator0_timersel())
            .field("operator1_timersel", &self.operator1_timersel())
            .field("operator2_timersel", &self.operator2_timersel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Select which PWM timer's is the timing reference for PWM operator0, 0: timer0, 1: timer1, 2: timer2
    #[inline(always)]
    #[must_use]
    pub fn operator0_timersel(&mut self) -> OPERATOR0_TIMERSEL_W<OPERATOR_TIMERSEL_SPEC> {
        OPERATOR0_TIMERSEL_W::new(self, 0)
    }
    ///Bits 2:3 - Select which PWM timer's is the timing reference for PWM operator1, 0: timer0, 1: timer1, 2: timer2
    #[inline(always)]
    #[must_use]
    pub fn operator1_timersel(&mut self) -> OPERATOR1_TIMERSEL_W<OPERATOR_TIMERSEL_SPEC> {
        OPERATOR1_TIMERSEL_W::new(self, 2)
    }
    ///Bits 4:5 - Select which PWM timer's is the timing reference for PWM operator2, 0: timer0, 1: timer1, 2: timer2
    #[inline(always)]
    #[must_use]
    pub fn operator2_timersel(&mut self) -> OPERATOR2_TIMERSEL_W<OPERATOR_TIMERSEL_SPEC> {
        OPERATOR2_TIMERSEL_W::new(self, 4)
    }
}
/**Select specific timer for PWM operators.

You can [`read`](crate::generic::Reg::read) this register and get [`operator_timersel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operator_timersel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OPERATOR_TIMERSEL_SPEC;
impl crate::RegisterSpec for OPERATOR_TIMERSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`operator_timersel::R`](R) reader structure
impl crate::Readable for OPERATOR_TIMERSEL_SPEC {}
///`write(|w| ..)` method takes [`operator_timersel::W`](W) writer structure
impl crate::Writable for OPERATOR_TIMERSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPERATOR_TIMERSEL to value 0
impl crate::Resettable for OPERATOR_TIMERSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
