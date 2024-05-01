///Register `TIMER4` reader
pub type R = crate::R<TIMER4_SPEC>;
///Register `TIMER4` writer
pub type W = crate::W<TIMER4_SPEC>;
///Field `CPU_TOP_WAIT_TIMER` reader - cpu top power domain wakeup time
pub type CPU_TOP_WAIT_TIMER_R = crate::FieldReader<u16>;
///Field `CPU_TOP_WAIT_TIMER` writer - cpu top power domain wakeup time
pub type CPU_TOP_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CPU_TOP_POWERUP_TIMER` reader - cpu top power domain power on time
pub type CPU_TOP_POWERUP_TIMER_R = crate::FieldReader;
///Field `CPU_TOP_POWERUP_TIMER` writer - cpu top power domain power on time
pub type CPU_TOP_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DG_WRAP_WAIT_TIMER` reader - digital wrap power domain wakeup time
pub type DG_WRAP_WAIT_TIMER_R = crate::FieldReader<u16>;
///Field `DG_WRAP_WAIT_TIMER` writer - digital wrap power domain wakeup time
pub type DG_WRAP_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DG_WRAP_POWERUP_TIMER` reader - digital wrap power domain power on time
pub type DG_WRAP_POWERUP_TIMER_R = crate::FieldReader;
///Field `DG_WRAP_POWERUP_TIMER` writer - digital wrap power domain power on time
pub type DG_WRAP_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - cpu top power domain wakeup time
    #[inline(always)]
    pub fn cpu_top_wait_timer(&self) -> CPU_TOP_WAIT_TIMER_R {
        CPU_TOP_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - cpu top power domain power on time
    #[inline(always)]
    pub fn cpu_top_powerup_timer(&self) -> CPU_TOP_POWERUP_TIMER_R {
        CPU_TOP_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:24 - digital wrap power domain wakeup time
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - digital wrap power domain power on time
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER4")
            .field("cpu_top_wait_timer", &self.cpu_top_wait_timer())
            .field("cpu_top_powerup_timer", &self.cpu_top_powerup_timer())
            .field("dg_wrap_wait_timer", &self.dg_wrap_wait_timer())
            .field("dg_wrap_powerup_timer", &self.dg_wrap_powerup_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - cpu top power domain wakeup time
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_wait_timer(&mut self) -> CPU_TOP_WAIT_TIMER_W<TIMER4_SPEC> {
        CPU_TOP_WAIT_TIMER_W::new(self, 0)
    }
    ///Bits 9:15 - cpu top power domain power on time
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_powerup_timer(&mut self) -> CPU_TOP_POWERUP_TIMER_W<TIMER4_SPEC> {
        CPU_TOP_POWERUP_TIMER_W::new(self, 9)
    }
    ///Bits 16:24 - digital wrap power domain wakeup time
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W<TIMER4_SPEC> {
        DG_WRAP_WAIT_TIMER_W::new(self, 16)
    }
    ///Bits 25:31 - digital wrap power domain power on time
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W<TIMER4_SPEC> {
        DG_WRAP_POWERUP_TIMER_W::new(self, 25)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMER4_SPEC;
impl crate::RegisterSpec for TIMER4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timer4::R`](R) reader structure
impl crate::Readable for TIMER4_SPEC {}
///`write(|w| ..)` method takes [`timer4::W`](W) writer structure
impl crate::Writable for TIMER4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMER4 to value 0x1020_0a08
impl crate::Resettable for TIMER4_SPEC {
    const RESET_VALUE: u32 = 0x1020_0a08;
}
