///Register `TIMER3` reader
pub type R = crate::R<TIMER3_SPEC>;
///Register `TIMER3` writer
pub type W = crate::W<TIMER3_SPEC>;
///Field `WIFI_WAIT_TIMER` reader - wifi power domain wakeup time
pub type WIFI_WAIT_TIMER_R = crate::FieldReader<u16>;
///Field `WIFI_WAIT_TIMER` writer - wifi power domain wakeup time
pub type WIFI_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `WIFI_POWERUP_TIMER` reader - wifi power domain power on time
pub type WIFI_POWERUP_TIMER_R = crate::FieldReader;
///Field `WIFI_POWERUP_TIMER` writer - wifi power domain power on time
pub type WIFI_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `BT_WAIT_TIMER` reader - bt power domain wakeup time
pub type BT_WAIT_TIMER_R = crate::FieldReader<u16>;
///Field `BT_WAIT_TIMER` writer - bt power domain wakeup time
pub type BT_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `BT_POWERUP_TIMER` reader - bt power domain power on time
pub type BT_POWERUP_TIMER_R = crate::FieldReader;
///Field `BT_POWERUP_TIMER` writer - bt power domain power on time
pub type BT_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - wifi power domain wakeup time
    #[inline(always)]
    pub fn wifi_wait_timer(&self) -> WIFI_WAIT_TIMER_R {
        WIFI_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - wifi power domain power on time
    #[inline(always)]
    pub fn wifi_powerup_timer(&self) -> WIFI_POWERUP_TIMER_R {
        WIFI_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:24 - bt power domain wakeup time
    #[inline(always)]
    pub fn bt_wait_timer(&self) -> BT_WAIT_TIMER_R {
        BT_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - bt power domain power on time
    #[inline(always)]
    pub fn bt_powerup_timer(&self) -> BT_POWERUP_TIMER_R {
        BT_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER3")
            .field("wifi_wait_timer", &self.wifi_wait_timer())
            .field("wifi_powerup_timer", &self.wifi_powerup_timer())
            .field("bt_wait_timer", &self.bt_wait_timer())
            .field("bt_powerup_timer", &self.bt_powerup_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - wifi power domain wakeup time
    #[inline(always)]
    #[must_use]
    pub fn wifi_wait_timer(&mut self) -> WIFI_WAIT_TIMER_W<TIMER3_SPEC> {
        WIFI_WAIT_TIMER_W::new(self, 0)
    }
    ///Bits 9:15 - wifi power domain power on time
    #[inline(always)]
    #[must_use]
    pub fn wifi_powerup_timer(&mut self) -> WIFI_POWERUP_TIMER_W<TIMER3_SPEC> {
        WIFI_POWERUP_TIMER_W::new(self, 9)
    }
    ///Bits 16:24 - bt power domain wakeup time
    #[inline(always)]
    #[must_use]
    pub fn bt_wait_timer(&mut self) -> BT_WAIT_TIMER_W<TIMER3_SPEC> {
        BT_WAIT_TIMER_W::new(self, 16)
    }
    ///Bits 25:31 - bt power domain power on time
    #[inline(always)]
    #[must_use]
    pub fn bt_powerup_timer(&mut self) -> BT_POWERUP_TIMER_W<TIMER3_SPEC> {
        BT_POWERUP_TIMER_W::new(self, 25)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMER3_SPEC;
impl crate::RegisterSpec for TIMER3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timer3::R`](R) reader structure
impl crate::Readable for TIMER3_SPEC {}
///`write(|w| ..)` method takes [`timer3::W`](W) writer structure
impl crate::Writable for TIMER3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMER3 to value 0x0a08_0a08
impl crate::Resettable for TIMER3_SPEC {
    const RESET_VALUE: u32 = 0x0a08_0a08;
}
