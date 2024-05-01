///Register `POWER_WAIT_TIMER1` reader
pub type R = crate::R<POWER_WAIT_TIMER1_SPEC>;
///Register `POWER_WAIT_TIMER1` writer
pub type W = crate::W<POWER_WAIT_TIMER1_SPEC>;
///Field `DG_LP_POWERDOWN_TIMER` reader - need_des
pub type DG_LP_POWERDOWN_TIMER_R = crate::FieldReader<u16>;
///Field `DG_LP_POWERDOWN_TIMER` writer - need_des
pub type DG_LP_POWERDOWN_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DG_LP_POWERUP_TIMER` reader - need_des
pub type DG_LP_POWERUP_TIMER_R = crate::FieldReader<u16>;
///Field `DG_LP_POWERUP_TIMER` writer - need_des
pub type DG_LP_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DG_LP_WAIT_TIMER` reader - need_des
pub type DG_LP_WAIT_TIMER_R = crate::FieldReader<u16>;
///Field `DG_LP_WAIT_TIMER` writer - need_des
pub type DG_LP_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 5:13 - need_des
    #[inline(always)]
    pub fn dg_lp_powerdown_timer(&self) -> DG_LP_POWERDOWN_TIMER_R {
        DG_LP_POWERDOWN_TIMER_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    ///Bits 14:22 - need_des
    #[inline(always)]
    pub fn dg_lp_powerup_timer(&self) -> DG_LP_POWERUP_TIMER_R {
        DG_LP_POWERUP_TIMER_R::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    ///Bits 23:31 - need_des
    #[inline(always)]
    pub fn dg_lp_wait_timer(&self) -> DG_LP_WAIT_TIMER_R {
        DG_LP_WAIT_TIMER_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_WAIT_TIMER1")
            .field("dg_lp_powerdown_timer", &self.dg_lp_powerdown_timer())
            .field("dg_lp_powerup_timer", &self.dg_lp_powerup_timer())
            .field("dg_lp_wait_timer", &self.dg_lp_wait_timer())
            .finish()
    }
}
impl W {
    ///Bits 5:13 - need_des
    #[inline(always)]
    #[must_use]
    pub fn dg_lp_powerdown_timer(&mut self) -> DG_LP_POWERDOWN_TIMER_W<POWER_WAIT_TIMER1_SPEC> {
        DG_LP_POWERDOWN_TIMER_W::new(self, 5)
    }
    ///Bits 14:22 - need_des
    #[inline(always)]
    #[must_use]
    pub fn dg_lp_powerup_timer(&mut self) -> DG_LP_POWERUP_TIMER_W<POWER_WAIT_TIMER1_SPEC> {
        DG_LP_POWERUP_TIMER_W::new(self, 14)
    }
    ///Bits 23:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn dg_lp_wait_timer(&mut self) -> DG_LP_WAIT_TIMER_W<POWER_WAIT_TIMER1_SPEC> {
        DG_LP_WAIT_TIMER_W::new(self, 23)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_wait_timer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_wait_timer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POWER_WAIT_TIMER1_SPEC;
impl crate::RegisterSpec for POWER_WAIT_TIMER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`power_wait_timer1::R`](R) reader structure
impl crate::Readable for POWER_WAIT_TIMER1_SPEC {}
///`write(|w| ..)` method takes [`power_wait_timer1::W`](W) writer structure
impl crate::Writable for POWER_WAIT_TIMER1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER_WAIT_TIMER1 to value 0x7fbf_dfe0
impl crate::Resettable for POWER_WAIT_TIMER1_SPEC {
    const RESET_VALUE: u32 = 0x7fbf_dfe0;
}
