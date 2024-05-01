///Register `ETM_EVENT_CH%s_CFG` reader
pub type R = crate::R<ETM_EVENT_CH_CFG_SPEC>;
///Register `ETM_EVENT_CH%s_CFG` writer
pub type W = crate::W<ETM_EVENT_CH_CFG_SPEC>;
///Field `EVENT_SEL` reader - Etm event channel select gpio.
pub type EVENT_SEL_R = crate::FieldReader;
///Field `EVENT_SEL` writer - Etm event channel select gpio.
pub type EVENT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `EVENT_EN` reader - Etm event send enable bit.
pub type EVENT_EN_R = crate::BitReader;
///Field `EVENT_EN` writer - Etm event send enable bit.
pub type EVENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Etm event channel select gpio.
    #[inline(always)]
    pub fn event_sel(&self) -> EVENT_SEL_R {
        EVENT_SEL_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 7 - Etm event send enable bit.
    #[inline(always)]
    pub fn event_en(&self) -> EVENT_EN_R {
        EVENT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_EVENT_CH_CFG")
            .field("event_sel", &self.event_sel())
            .field("event_en", &self.event_en())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Etm event channel select gpio.
    #[inline(always)]
    #[must_use]
    pub fn event_sel(&mut self) -> EVENT_SEL_W<ETM_EVENT_CH_CFG_SPEC> {
        EVENT_SEL_W::new(self, 0)
    }
    ///Bit 7 - Etm event send enable bit.
    #[inline(always)]
    #[must_use]
    pub fn event_en(&mut self) -> EVENT_EN_W<ETM_EVENT_CH_CFG_SPEC> {
        EVENT_EN_W::new(self, 7)
    }
}
/**Etm Config register of Channel%s

You can [`read`](crate::generic::Reg::read) this register and get [`etm_event_ch_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ETM_EVENT_CH_CFG_SPEC;
impl crate::RegisterSpec for ETM_EVENT_CH_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`etm_event_ch_cfg::R`](R) reader structure
impl crate::Readable for ETM_EVENT_CH_CFG_SPEC {}
///`write(|w| ..)` method takes [`etm_event_ch_cfg::W`](W) writer structure
impl crate::Writable for ETM_EVENT_CH_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETM_EVENT_CH%s_CFG to value 0
impl crate::Resettable for ETM_EVENT_CH_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
