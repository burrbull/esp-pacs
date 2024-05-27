///Register `DBIAS_TIMER` reader
pub type R = crate::R<DBIAS_TIMER_SPEC>;
///Register `DBIAS_TIMER` writer
pub type W = crate::W<DBIAS_TIMER_SPEC>;
///Field `TIMER_TARGET` reader - needs field desc
pub type TIMER_TARGET_R = crate::FieldReader<u16>;
///Field `TIMER_TARGET` writer - needs field desc
pub type TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TIMER_EN` reader - needs field desc
pub type TIMER_EN_R = crate::BitReader;
///Field `TIMER_EN` writer - needs field desc
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 15:30 - needs field desc
    #[inline(always)]
    pub fn timer_target(&self) -> TIMER_TARGET_R {
        TIMER_TARGET_R::new(((self.bits >> 15) & 0xffff) as u16)
    }
    ///Bit 31 - needs field desc
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_TIMER")
            .field("timer_target", &self.timer_target())
            .field("timer_en", &self.timer_en())
            .finish()
    }
}
impl W {
    ///Bits 15:30 - needs field desc
    #[inline(always)]
    #[must_use]
    pub fn timer_target(&mut self) -> TIMER_TARGET_W<DBIAS_TIMER_SPEC> {
        TIMER_TARGET_W::new(self, 15)
    }
    ///Bit 31 - needs field desc
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<DBIAS_TIMER_SPEC> {
        TIMER_EN_W::new(self, 31)
    }
}
/**needs desc

You can [`read`](crate::generic::Reg::read) this register and get [`dbias_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBIAS_TIMER_SPEC;
impl crate::RegisterSpec for DBIAS_TIMER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbias_timer::R`](R) reader structure
impl crate::Readable for DBIAS_TIMER_SPEC {}
///`write(|w| ..)` method takes [`dbias_timer::W`](W) writer structure
impl crate::Writable for DBIAS_TIMER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBIAS_TIMER to value 0x7fff_8000
impl crate::Resettable for DBIAS_TIMER_SPEC {
    const RESET_VALUE: u32 = 0x7fff_8000;
}
