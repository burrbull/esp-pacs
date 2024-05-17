///Register `RESET_STATE` reader
pub type R = crate::R<RESET_STATE_SPEC>;
///Register `RESET_STATE` writer
pub type W = crate::W<RESET_STATE_SPEC>;
///Field `RESET_CAUSE_PROCPU` reader - reset cause of PRO CPU
pub type RESET_CAUSE_PROCPU_R = crate::FieldReader;
///Field `RESET_CAUSE_APPCPU` reader - reset cause of APP CPU
pub type RESET_CAUSE_APPCPU_R = crate::FieldReader;
///Field `APPCPU_STAT_VECTOR_SEL` reader - APP CPU state vector sel
pub type APPCPU_STAT_VECTOR_SEL_R = crate::BitReader;
///Field `APPCPU_STAT_VECTOR_SEL` writer - APP CPU state vector sel
pub type APPCPU_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROCPU_STAT_VECTOR_SEL` reader - PRO CPU state vector sel
pub type PROCPU_STAT_VECTOR_SEL_R = crate::BitReader;
///Field `PROCPU_STAT_VECTOR_SEL` writer - PRO CPU state vector sel
pub type PROCPU_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_FLAG_PROCPU` reader - PRO CPU reset_flag
pub type RESET_FLAG_PROCPU_R = crate::BitReader;
///Field `RESET_FLAG_APPCPU` reader - APP CPU reset flag
pub type RESET_FLAG_APPCPU_R = crate::BitReader;
///Field `RESET_FLAG_PROCPU_CLR` writer - clear PRO CPU reset_flag
pub type RESET_FLAG_PROCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_FLAG_APPCPU_CLR` writer - clear APP CPU reset flag
pub type RESET_FLAG_APPCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APPCPU_OCD_HALT_ON_RESET` reader - APPCPU OcdHaltOnReset
pub type APPCPU_OCD_HALT_ON_RESET_R = crate::BitReader;
///Field `APPCPU_OCD_HALT_ON_RESET` writer - APPCPU OcdHaltOnReset
pub type APPCPU_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROCPU_OCD_HALT_ON_RESET` reader - PROCPU OcdHaltOnReset
pub type PROCPU_OCD_HALT_ON_RESET_R = crate::BitReader;
///Field `PROCPU_OCD_HALT_ON_RESET` writer - PROCPU OcdHaltOnReset
pub type PROCPU_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_FLAG_JTAG_PROCPU` reader - jtag reset flag
pub type RESET_FLAG_JTAG_PROCPU_R = crate::BitReader;
///Field `RESET_FLAG_JTAG_APPCPU` reader - jtag reset flag
pub type RESET_FLAG_JTAG_APPCPU_R = crate::BitReader;
///Field `RESET_FLAG_JTAG_PROCPU_CLR` writer - clear jtag reset flag
pub type RESET_FLAG_JTAG_PROCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET_FLAG_JTAG_APPCPU_CLR` writer - clear jtag reset flag
pub type RESET_FLAG_JTAG_APPCPU_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_DRESET_MASK` reader - bypass cpu1 dreset
pub type APP_DRESET_MASK_R = crate::BitReader;
///Field `APP_DRESET_MASK` writer - bypass cpu1 dreset
pub type APP_DRESET_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_DRESET_MASK` reader - bypass cpu0 dreset
pub type PRO_DRESET_MASK_R = crate::BitReader;
///Field `PRO_DRESET_MASK` writer - bypass cpu0 dreset
pub type PRO_DRESET_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - reset cause of PRO CPU
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:11 - reset cause of APP CPU
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bit 12 - APP CPU state vector sel
    #[inline(always)]
    pub fn appcpu_stat_vector_sel(&self) -> APPCPU_STAT_VECTOR_SEL_R {
        APPCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PRO CPU state vector sel
    #[inline(always)]
    pub fn procpu_stat_vector_sel(&self) -> PROCPU_STAT_VECTOR_SEL_R {
        PROCPU_STAT_VECTOR_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PRO CPU reset_flag
    #[inline(always)]
    pub fn reset_flag_procpu(&self) -> RESET_FLAG_PROCPU_R {
        RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - APP CPU reset flag
    #[inline(always)]
    pub fn reset_flag_appcpu(&self) -> RESET_FLAG_APPCPU_R {
        RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - APPCPU OcdHaltOnReset
    #[inline(always)]
    pub fn appcpu_ocd_halt_on_reset(&self) -> APPCPU_OCD_HALT_ON_RESET_R {
        APPCPU_OCD_HALT_ON_RESET_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PROCPU OcdHaltOnReset
    #[inline(always)]
    pub fn procpu_ocd_halt_on_reset(&self) -> PROCPU_OCD_HALT_ON_RESET_R {
        PROCPU_OCD_HALT_ON_RESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - jtag reset flag
    #[inline(always)]
    pub fn reset_flag_jtag_procpu(&self) -> RESET_FLAG_JTAG_PROCPU_R {
        RESET_FLAG_JTAG_PROCPU_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - jtag reset flag
    #[inline(always)]
    pub fn reset_flag_jtag_appcpu(&self) -> RESET_FLAG_JTAG_APPCPU_R {
        RESET_FLAG_JTAG_APPCPU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - bypass cpu1 dreset
    #[inline(always)]
    pub fn app_dreset_mask(&self) -> APP_DRESET_MASK_R {
        APP_DRESET_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - bypass cpu0 dreset
    #[inline(always)]
    pub fn pro_dreset_mask(&self) -> PRO_DRESET_MASK_R {
        PRO_DRESET_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_STATE")
            .field("reset_cause_procpu", &self.reset_cause_procpu())
            .field("reset_cause_appcpu", &self.reset_cause_appcpu())
            .field("appcpu_stat_vector_sel", &self.appcpu_stat_vector_sel())
            .field("procpu_stat_vector_sel", &self.procpu_stat_vector_sel())
            .field("reset_flag_procpu", &self.reset_flag_procpu())
            .field("reset_flag_appcpu", &self.reset_flag_appcpu())
            .field("appcpu_ocd_halt_on_reset", &self.appcpu_ocd_halt_on_reset())
            .field("procpu_ocd_halt_on_reset", &self.procpu_ocd_halt_on_reset())
            .field("reset_flag_jtag_procpu", &self.reset_flag_jtag_procpu())
            .field("reset_flag_jtag_appcpu", &self.reset_flag_jtag_appcpu())
            .field("app_dreset_mask", &self.app_dreset_mask())
            .field("pro_dreset_mask", &self.pro_dreset_mask())
            .finish()
    }
}
impl W {
    ///Bit 12 - APP CPU state vector sel
    #[inline(always)]
    #[must_use]
    pub fn appcpu_stat_vector_sel(
        &mut self,
    ) -> APPCPU_STAT_VECTOR_SEL_W<RESET_STATE_SPEC> {
        APPCPU_STAT_VECTOR_SEL_W::new(self, 12)
    }
    ///Bit 13 - PRO CPU state vector sel
    #[inline(always)]
    #[must_use]
    pub fn procpu_stat_vector_sel(
        &mut self,
    ) -> PROCPU_STAT_VECTOR_SEL_W<RESET_STATE_SPEC> {
        PROCPU_STAT_VECTOR_SEL_W::new(self, 13)
    }
    ///Bit 16 - clear PRO CPU reset_flag
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_procpu_clr(
        &mut self,
    ) -> RESET_FLAG_PROCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_PROCPU_CLR_W::new(self, 16)
    }
    ///Bit 17 - clear APP CPU reset flag
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_appcpu_clr(
        &mut self,
    ) -> RESET_FLAG_APPCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_APPCPU_CLR_W::new(self, 17)
    }
    ///Bit 18 - APPCPU OcdHaltOnReset
    #[inline(always)]
    #[must_use]
    pub fn appcpu_ocd_halt_on_reset(
        &mut self,
    ) -> APPCPU_OCD_HALT_ON_RESET_W<RESET_STATE_SPEC> {
        APPCPU_OCD_HALT_ON_RESET_W::new(self, 18)
    }
    ///Bit 19 - PROCPU OcdHaltOnReset
    #[inline(always)]
    #[must_use]
    pub fn procpu_ocd_halt_on_reset(
        &mut self,
    ) -> PROCPU_OCD_HALT_ON_RESET_W<RESET_STATE_SPEC> {
        PROCPU_OCD_HALT_ON_RESET_W::new(self, 19)
    }
    ///Bit 22 - clear jtag reset flag
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_jtag_procpu_clr(
        &mut self,
    ) -> RESET_FLAG_JTAG_PROCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_JTAG_PROCPU_CLR_W::new(self, 22)
    }
    ///Bit 23 - clear jtag reset flag
    #[inline(always)]
    #[must_use]
    pub fn reset_flag_jtag_appcpu_clr(
        &mut self,
    ) -> RESET_FLAG_JTAG_APPCPU_CLR_W<RESET_STATE_SPEC> {
        RESET_FLAG_JTAG_APPCPU_CLR_W::new(self, 23)
    }
    ///Bit 24 - bypass cpu1 dreset
    #[inline(always)]
    #[must_use]
    pub fn app_dreset_mask(&mut self) -> APP_DRESET_MASK_W<RESET_STATE_SPEC> {
        APP_DRESET_MASK_W::new(self, 24)
    }
    ///Bit 25 - bypass cpu0 dreset
    #[inline(always)]
    #[must_use]
    pub fn pro_dreset_mask(&mut self) -> PRO_DRESET_MASK_W<RESET_STATE_SPEC> {
        PRO_DRESET_MASK_W::new(self, 25)
    }
}
/**get reset state

You can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`reset_state::R`](R) reader structure
impl crate::Readable for RESET_STATE_SPEC {}
///`write(|w| ..)` method takes [`reset_state::W`](W) writer structure
impl crate::Writable for RESET_STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESET_STATE to value 0x3000
impl crate::Resettable for RESET_STATE_SPEC {
    const RESET_VALUE: u32 = 0x3000;
}
