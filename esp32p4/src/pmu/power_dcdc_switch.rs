///Register `POWER_DCDC_SWITCH` reader
pub type R = crate::R<POWER_DCDC_SWITCH_SPEC>;
///Register `POWER_DCDC_SWITCH` writer
pub type W = crate::W<POWER_DCDC_SWITCH_SPEC>;
///Field `FORCE_DCDC_SWITCH_PU` reader - need_des
pub type FORCE_DCDC_SWITCH_PU_R = crate::BitReader;
///Field `FORCE_DCDC_SWITCH_PU` writer - need_des
pub type FORCE_DCDC_SWITCH_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_DCDC_SWITCH_PD` reader - need_des
pub type FORCE_DCDC_SWITCH_PD_R = crate::BitReader;
///Field `FORCE_DCDC_SWITCH_PD` writer - need_des
pub type FORCE_DCDC_SWITCH_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn force_dcdc_switch_pu(&self) -> FORCE_DCDC_SWITCH_PU_R {
        FORCE_DCDC_SWITCH_PU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn force_dcdc_switch_pd(&self) -> FORCE_DCDC_SWITCH_PD_R {
        FORCE_DCDC_SWITCH_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_DCDC_SWITCH")
            .field("force_dcdc_switch_pu", &self.force_dcdc_switch_pu())
            .field("force_dcdc_switch_pd", &self.force_dcdc_switch_pd())
            .finish()
    }
}
impl W {
    ///Bit 0 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_dcdc_switch_pu(&mut self) -> FORCE_DCDC_SWITCH_PU_W<POWER_DCDC_SWITCH_SPEC> {
        FORCE_DCDC_SWITCH_PU_W::new(self, 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_dcdc_switch_pd(&mut self) -> FORCE_DCDC_SWITCH_PD_W<POWER_DCDC_SWITCH_SPEC> {
        FORCE_DCDC_SWITCH_PD_W::new(self, 1)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_dcdc_switch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_dcdc_switch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POWER_DCDC_SWITCH_SPEC;
impl crate::RegisterSpec for POWER_DCDC_SWITCH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`power_dcdc_switch::R`](R) reader structure
impl crate::Readable for POWER_DCDC_SWITCH_SPEC {}
///`write(|w| ..)` method takes [`power_dcdc_switch::W`](W) writer structure
impl crate::Writable for POWER_DCDC_SWITCH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER_DCDC_SWITCH to value 0x01
impl crate::Resettable for POWER_DCDC_SWITCH_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
