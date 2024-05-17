///Register `EXT_LDO_P1_0P1A` reader
pub type R = crate::R<EXT_LDO_P1_0P1A_SPEC>;
///Register `EXT_LDO_P1_0P1A` writer
pub type W = crate::W<EXT_LDO_P1_0P1A_SPEC>;
///Field `_0P1A_FORCE_TIEH_SEL_1` reader - need_des
pub type _0P1A_FORCE_TIEH_SEL_1_R = crate::BitReader;
///Field `_0P1A_FORCE_TIEH_SEL_1` writer - need_des
pub type _0P1A_FORCE_TIEH_SEL_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `_0P1A_XPD_1` reader - need_des
pub type _0P1A_XPD_1_R = crate::BitReader;
///Field `_0P1A_XPD_1` writer - need_des
pub type _0P1A_XPD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `_0P1A_TIEH_SEL_1` reader - need_des
pub type _0P1A_TIEH_SEL_1_R = crate::FieldReader;
///Field `_0P1A_TIEH_SEL_1` writer - need_des
pub type _0P1A_TIEH_SEL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `_0P1A_TIEH_POS_EN_1` reader - need_des
pub type _0P1A_TIEH_POS_EN_1_R = crate::BitReader;
///Field `_0P1A_TIEH_POS_EN_1` writer - need_des
pub type _0P1A_TIEH_POS_EN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `_0P1A_TIEH_NEG_EN_1` reader - need_des
pub type _0P1A_TIEH_NEG_EN_1_R = crate::BitReader;
///Field `_0P1A_TIEH_NEG_EN_1` writer - need_des
pub type _0P1A_TIEH_NEG_EN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `_0P1A_TIEH_1` reader - need_des
pub type _0P1A_TIEH_1_R = crate::BitReader;
///Field `_0P1A_TIEH_1` writer - need_des
pub type _0P1A_TIEH_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `_0P1A_TARGET1_1` reader - need_des
pub type _0P1A_TARGET1_1_R = crate::FieldReader;
///Field `_0P1A_TARGET1_1` writer - need_des
pub type _0P1A_TARGET1_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `_0P1A_TARGET0_1` reader - need_des
pub type _0P1A_TARGET0_1_R = crate::FieldReader;
///Field `_0P1A_TARGET0_1` writer - need_des
pub type _0P1A_TARGET0_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `_0P1A_LDO_CNT_PRESCALER_SEL_1` reader - need_des
pub type _0P1A_LDO_CNT_PRESCALER_SEL_1_R = crate::BitReader;
///Field `_0P1A_LDO_CNT_PRESCALER_SEL_1` writer - need_des
pub type _0P1A_LDO_CNT_PRESCALER_SEL_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - need_des
    #[inline(always)]
    pub fn _0p1a_force_tieh_sel_1(&self) -> _0P1A_FORCE_TIEH_SEL_1_R {
        _0P1A_FORCE_TIEH_SEL_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - need_des
    #[inline(always)]
    pub fn _0p1a_xpd_1(&self) -> _0P1A_XPD_1_R {
        _0P1A_XPD_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - need_des
    #[inline(always)]
    pub fn _0p1a_tieh_sel_1(&self) -> _0P1A_TIEH_SEL_1_R {
        _0P1A_TIEH_SEL_1_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12 - need_des
    #[inline(always)]
    pub fn _0p1a_tieh_pos_en_1(&self) -> _0P1A_TIEH_POS_EN_1_R {
        _0P1A_TIEH_POS_EN_1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - need_des
    #[inline(always)]
    pub fn _0p1a_tieh_neg_en_1(&self) -> _0P1A_TIEH_NEG_EN_1_R {
        _0P1A_TIEH_NEG_EN_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - need_des
    #[inline(always)]
    pub fn _0p1a_tieh_1(&self) -> _0P1A_TIEH_1_R {
        _0P1A_TIEH_1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:22 - need_des
    #[inline(always)]
    pub fn _0p1a_target1_1(&self) -> _0P1A_TARGET1_1_R {
        _0P1A_TARGET1_1_R::new(((self.bits >> 15) & 0xff) as u8)
    }
    ///Bits 23:30 - need_des
    #[inline(always)]
    pub fn _0p1a_target0_1(&self) -> _0P1A_TARGET0_1_R {
        _0P1A_TARGET0_1_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn _0p1a_ldo_cnt_prescaler_sel_1(&self) -> _0P1A_LDO_CNT_PRESCALER_SEL_1_R {
        _0P1A_LDO_CNT_PRESCALER_SEL_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_P1_0P1A")
            .field("_0p1a_force_tieh_sel_1", &self._0p1a_force_tieh_sel_1())
            .field("_0p1a_xpd_1", &self._0p1a_xpd_1())
            .field("_0p1a_tieh_sel_1", &self._0p1a_tieh_sel_1())
            .field("_0p1a_tieh_pos_en_1", &self._0p1a_tieh_pos_en_1())
            .field("_0p1a_tieh_neg_en_1", &self._0p1a_tieh_neg_en_1())
            .field("_0p1a_tieh_1", &self._0p1a_tieh_1())
            .field("_0p1a_target1_1", &self._0p1a_target1_1())
            .field("_0p1a_target0_1", &self._0p1a_target0_1())
            .field(
                "_0p1a_ldo_cnt_prescaler_sel_1",
                &self._0p1a_ldo_cnt_prescaler_sel_1(),
            )
            .finish()
    }
}
impl W {
    ///Bit 7 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_force_tieh_sel_1(
        &mut self,
    ) -> _0P1A_FORCE_TIEH_SEL_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_FORCE_TIEH_SEL_1_W::new(self, 7)
    }
    ///Bit 8 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_xpd_1(&mut self) -> _0P1A_XPD_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_XPD_1_W::new(self, 8)
    }
    ///Bits 9:11 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_sel_1(&mut self) -> _0P1A_TIEH_SEL_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_TIEH_SEL_1_W::new(self, 9)
    }
    ///Bit 12 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_pos_en_1(
        &mut self,
    ) -> _0P1A_TIEH_POS_EN_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_TIEH_POS_EN_1_W::new(self, 12)
    }
    ///Bit 13 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_neg_en_1(
        &mut self,
    ) -> _0P1A_TIEH_NEG_EN_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_TIEH_NEG_EN_1_W::new(self, 13)
    }
    ///Bit 14 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_tieh_1(&mut self) -> _0P1A_TIEH_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_TIEH_1_W::new(self, 14)
    }
    ///Bits 15:22 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_target1_1(&mut self) -> _0P1A_TARGET1_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_TARGET1_1_W::new(self, 15)
    }
    ///Bits 23:30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_target0_1(&mut self) -> _0P1A_TARGET0_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_TARGET0_1_W::new(self, 23)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_ldo_cnt_prescaler_sel_1(
        &mut self,
    ) -> _0P1A_LDO_CNT_PRESCALER_SEL_1_W<EXT_LDO_P1_0P1A_SPEC> {
        _0P1A_LDO_CNT_PRESCALER_SEL_1_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_LDO_P1_0P1A_SPEC;
impl crate::RegisterSpec for EXT_LDO_P1_0P1A_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_ldo_p1_0p1a::R`](R) reader structure
impl crate::Readable for EXT_LDO_P1_0P1A_SPEC {}
///`write(|w| ..)` method takes [`ext_ldo_p1_0p1a::W`](W) writer structure
impl crate::Writable for EXT_LDO_P1_0P1A_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXT_LDO_P1_0P1A to value 0x4020_0000
impl crate::Resettable for EXT_LDO_P1_0P1A_SPEC {
    const RESET_VALUE: u32 = 0x4020_0000;
}
