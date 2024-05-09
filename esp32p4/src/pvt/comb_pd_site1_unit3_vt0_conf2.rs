#[doc = "Register `COMB_PD_SITE1_UNIT3_VT0_CONF2` reader"]
pub type R = crate::R<COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC>;
#[doc = "Register `COMB_PD_SITE1_UNIT3_VT0_CONF2` writer"]
pub type W = crate::W<COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC>;
#[doc = "Field `MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3` reader - needs field desc"]
pub type MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_R = crate::FieldReader;
#[doc = "Field `MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3` writer - needs field desc"]
pub type MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_OVF_VT0_PD_SITE1_UNIT3` reader - needs field desc"]
pub type DELAY_OVF_VT0_PD_SITE1_UNIT3_R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3` reader - needs field desc"]
pub type TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit3(&self) -> MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_R {
        MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - needs field desc"]
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site1_unit3(&self) -> DELAY_OVF_VT0_PD_SITE1_UNIT3_R {
        DELAY_OVF_VT0_PD_SITE1_UNIT3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site1_unit3(&self) -> TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_R {
        TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PD_SITE1_UNIT3_VT0_CONF2")
            .field(
                "monitor_edg_mod_vt0_pd_site1_unit3",
                &self.monitor_edg_mod_vt0_pd_site1_unit3().bits(),
            )
            .field(
                "delay_ovf_vt0_pd_site1_unit3",
                &self.delay_ovf_vt0_pd_site1_unit3().bit(),
            )
            .field(
                "timing_err_cnt_o_vt0_pd_site1_unit3",
                &self.timing_err_cnt_o_vt0_pd_site1_unit3().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn monitor_edg_mod_vt0_pd_site1_unit3(
        &mut self,
    ) -> MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_W<COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC> {
        MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt0_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt0_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC;
impl crate::RegisterSpec for COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site1_unit3_vt0_conf2::R`](R) reader structure"]
impl crate::Readable for COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site1_unit3_vt0_conf2::W`](W) writer structure"]
impl crate::Writable for COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMB_PD_SITE1_UNIT3_VT0_CONF2 to value 0"]
impl crate::Resettable for COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC {}
