///Register `DB_RED_CFG` reader
pub type R = crate::R<DB_RED_CFG_SPEC>;
///Register `DB_RED_CFG` writer
pub type W = crate::W<DB_RED_CFG_SPEC>;
///Field `RED` reader - Shadow register for RED
pub type RED_R = crate::FieldReader<u16>;
///Field `RED` writer - Shadow register for RED
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Shadow register for RED
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_RED_CFG").field("red", &self.red()).finish()
    }
}
impl W {
    ///Bits 0:15 - Shadow register for RED
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<DB_RED_CFG_SPEC> {
        RED_W::new(self, 0)
    }
}
/**Shadow register for rising edge delay (RED).

You can [`read`](crate::generic::Reg::read) this register and get [`db_red_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db_red_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DB_RED_CFG_SPEC;
impl crate::RegisterSpec for DB_RED_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`db_red_cfg::R`](R) reader structure
impl crate::Readable for DB_RED_CFG_SPEC {}
///`write(|w| ..)` method takes [`db_red_cfg::W`](W) writer structure
impl crate::Writable for DB_RED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_RED_CFG to value 0
impl crate::Resettable for DB_RED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
