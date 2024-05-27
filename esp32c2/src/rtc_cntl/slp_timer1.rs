///Register `SLP_TIMER1` reader
pub type R = crate::R<SLP_TIMER1_SPEC>;
///Register `SLP_TIMER1` writer
pub type W = crate::W<SLP_TIMER1_SPEC>;
///Field `SLP_VAL_HI` reader - RTC sleep timer high 16 bits
pub type SLP_VAL_HI_R = crate::FieldReader<u16>;
///Field `SLP_VAL_HI` writer - RTC sleep timer high 16 bits
pub type SLP_VAL_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MAIN_TIMER_ALARM_EN` reader - timer alarm enable bit
pub type MAIN_TIMER_ALARM_EN_R = crate::BitReader;
///Field `MAIN_TIMER_ALARM_EN` writer - timer alarm enable bit
pub type MAIN_TIMER_ALARM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - RTC sleep timer high 16 bits
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - timer alarm enable bit
    #[inline(always)]
    pub fn main_timer_alarm_en(&self) -> MAIN_TIMER_ALARM_EN_R {
        MAIN_TIMER_ALARM_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER1")
            .field("slp_val_hi", &self.slp_val_hi())
            .field("main_timer_alarm_en", &self.main_timer_alarm_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - RTC sleep timer high 16 bits
    #[inline(always)]
    #[must_use]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W<SLP_TIMER1_SPEC> {
        SLP_VAL_HI_W::new(self, 0)
    }
    ///Bit 16 - timer alarm enable bit
    #[inline(always)]
    #[must_use]
    pub fn main_timer_alarm_en(&mut self) -> MAIN_TIMER_ALARM_EN_W<SLP_TIMER1_SPEC> {
        MAIN_TIMER_ALARM_EN_W::new(self, 16)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLP_TIMER1_SPEC;
impl crate::RegisterSpec for SLP_TIMER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slp_timer1::R`](R) reader structure
impl crate::Readable for SLP_TIMER1_SPEC {}
///`write(|w| ..)` method takes [`slp_timer1::W`](W) writer structure
impl crate::Writable for SLP_TIMER1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_TIMER1 to value 0
impl crate::Resettable for SLP_TIMER1_SPEC {
    const RESET_VALUE: u32 = 0;
}
