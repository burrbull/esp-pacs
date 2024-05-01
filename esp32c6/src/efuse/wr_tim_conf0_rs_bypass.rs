///Register `WR_TIM_CONF0_RS_BYPASS` reader
pub type R = crate::R<WR_TIM_CONF0_RS_BYPASS_SPEC>;
///Register `WR_TIM_CONF0_RS_BYPASS` writer
pub type W = crate::W<WR_TIM_CONF0_RS_BYPASS_SPEC>;
///Field `BYPASS_RS_CORRECTION` reader - Set this bit to bypass reed solomon correction step.
pub type BYPASS_RS_CORRECTION_R = crate::BitReader;
///Field `BYPASS_RS_CORRECTION` writer - Set this bit to bypass reed solomon correction step.
pub type BYPASS_RS_CORRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPASS_RS_BLK_NUM` reader - Configures block number of programming twice operation.
pub type BYPASS_RS_BLK_NUM_R = crate::FieldReader<u16>;
///Field `BYPASS_RS_BLK_NUM` writer - Configures block number of programming twice operation.
pub type BYPASS_RS_BLK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `UPDATE` writer - Set this bit to update multi-bit register signals.
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPGM_INACTIVE` reader - Configures the inactive programming time.
pub type TPGM_INACTIVE_R = crate::FieldReader;
///Field `TPGM_INACTIVE` writer - Configures the inactive programming time.
pub type TPGM_INACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Set this bit to bypass reed solomon correction step.
    #[inline(always)]
    pub fn bypass_rs_correction(&self) -> BYPASS_RS_CORRECTION_R {
        BYPASS_RS_CORRECTION_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:11 - Configures block number of programming twice operation.
    #[inline(always)]
    pub fn bypass_rs_blk_num(&self) -> BYPASS_RS_BLK_NUM_R {
        BYPASS_RS_BLK_NUM_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    ///Bits 13:20 - Configures the inactive programming time.
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> TPGM_INACTIVE_R {
        TPGM_INACTIVE_R::new(((self.bits >> 13) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF0_RS_BYPASS")
            .field("bypass_rs_correction", &self.bypass_rs_correction())
            .field("bypass_rs_blk_num", &self.bypass_rs_blk_num())
            .field("tpgm_inactive", &self.tpgm_inactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to bypass reed solomon correction step.
    #[inline(always)]
    #[must_use]
    pub fn bypass_rs_correction(&mut self) -> BYPASS_RS_CORRECTION_W<WR_TIM_CONF0_RS_BYPASS_SPEC> {
        BYPASS_RS_CORRECTION_W::new(self, 0)
    }
    ///Bits 1:11 - Configures block number of programming twice operation.
    #[inline(always)]
    #[must_use]
    pub fn bypass_rs_blk_num(&mut self) -> BYPASS_RS_BLK_NUM_W<WR_TIM_CONF0_RS_BYPASS_SPEC> {
        BYPASS_RS_BLK_NUM_W::new(self, 1)
    }
    ///Bit 12 - Set this bit to update multi-bit register signals.
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<WR_TIM_CONF0_RS_BYPASS_SPEC> {
        UPDATE_W::new(self, 12)
    }
    ///Bits 13:20 - Configures the inactive programming time.
    #[inline(always)]
    #[must_use]
    pub fn tpgm_inactive(&mut self) -> TPGM_INACTIVE_W<WR_TIM_CONF0_RS_BYPASS_SPEC> {
        TPGM_INACTIVE_W::new(self, 13)
    }
}
/**Configurarion register0 of eFuse programming time parameters and rs bypass operation.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0_rs_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0_rs_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WR_TIM_CONF0_RS_BYPASS_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF0_RS_BYPASS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wr_tim_conf0_rs_bypass::R`](R) reader structure
impl crate::Readable for WR_TIM_CONF0_RS_BYPASS_SPEC {}
///`write(|w| ..)` method takes [`wr_tim_conf0_rs_bypass::W`](W) writer structure
impl crate::Writable for WR_TIM_CONF0_RS_BYPASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WR_TIM_CONF0_RS_BYPASS to value 0x2000
impl crate::Resettable for WR_TIM_CONF0_RS_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
