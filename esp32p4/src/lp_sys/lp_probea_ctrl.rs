///Register `LP_PROBEA_CTRL` reader
pub type R = crate::R<LP_PROBEA_CTRL_SPEC>;
///Register `LP_PROBEA_CTRL` writer
pub type W = crate::W<LP_PROBEA_CTRL_SPEC>;
///Field `PROBE_A_MOD_SEL` reader - need_des
pub type PROBE_A_MOD_SEL_R = crate::FieldReader<u16>;
///Field `PROBE_A_MOD_SEL` writer - need_des
pub type PROBE_A_MOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PROBE_A_TOP_SEL` reader - need_des
pub type PROBE_A_TOP_SEL_R = crate::FieldReader;
///Field `PROBE_A_TOP_SEL` writer - need_des
pub type PROBE_A_TOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PROBE_L_SEL` reader - need_des
pub type PROBE_L_SEL_R = crate::FieldReader;
///Field `PROBE_L_SEL` writer - need_des
pub type PROBE_L_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PROBE_H_SEL` reader - need_des
pub type PROBE_H_SEL_R = crate::FieldReader;
///Field `PROBE_H_SEL` writer - need_des
pub type PROBE_H_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PROBE_GLOBAL_EN` reader - need_des
pub type PROBE_GLOBAL_EN_R = crate::BitReader;
///Field `PROBE_GLOBAL_EN` writer - need_des
pub type PROBE_GLOBAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn probe_a_mod_sel(&self) -> PROBE_A_MOD_SEL_R {
        PROBE_A_MOD_SEL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - need_des
    #[inline(always)]
    pub fn probe_a_top_sel(&self) -> PROBE_A_TOP_SEL_R {
        PROBE_A_TOP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:25 - need_des
    #[inline(always)]
    pub fn probe_l_sel(&self) -> PROBE_L_SEL_R {
        PROBE_L_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - need_des
    #[inline(always)]
    pub fn probe_h_sel(&self) -> PROBE_H_SEL_R {
        PROBE_H_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn probe_global_en(&self) -> PROBE_GLOBAL_EN_R {
        PROBE_GLOBAL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PROBEA_CTRL")
            .field("probe_a_mod_sel", &self.probe_a_mod_sel())
            .field("probe_a_top_sel", &self.probe_a_top_sel())
            .field("probe_l_sel", &self.probe_l_sel())
            .field("probe_h_sel", &self.probe_h_sel())
            .field("probe_global_en", &self.probe_global_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_a_mod_sel(&mut self) -> PROBE_A_MOD_SEL_W<LP_PROBEA_CTRL_SPEC> {
        PROBE_A_MOD_SEL_W::new(self, 0)
    }
    ///Bits 16:23 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_a_top_sel(&mut self) -> PROBE_A_TOP_SEL_W<LP_PROBEA_CTRL_SPEC> {
        PROBE_A_TOP_SEL_W::new(self, 16)
    }
    ///Bits 24:25 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_l_sel(&mut self) -> PROBE_L_SEL_W<LP_PROBEA_CTRL_SPEC> {
        PROBE_L_SEL_W::new(self, 24)
    }
    ///Bits 26:27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_h_sel(&mut self) -> PROBE_H_SEL_W<LP_PROBEA_CTRL_SPEC> {
        PROBE_H_SEL_W::new(self, 26)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_global_en(&mut self) -> PROBE_GLOBAL_EN_W<LP_PROBEA_CTRL_SPEC> {
        PROBE_GLOBAL_EN_W::new(self, 28)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_probea_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_probea_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_PROBEA_CTRL_SPEC;
impl crate::RegisterSpec for LP_PROBEA_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_probea_ctrl::R`](R) reader structure
impl crate::Readable for LP_PROBEA_CTRL_SPEC {}
///`write(|w| ..)` method takes [`lp_probea_ctrl::W`](W) writer structure
impl crate::Writable for LP_PROBEA_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_PROBEA_CTRL to value 0
impl crate::Resettable for LP_PROBEA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
