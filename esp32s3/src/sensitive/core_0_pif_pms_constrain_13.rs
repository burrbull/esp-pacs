#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_13` reader"]
pub type R = crate::R<CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_13` writer"]
pub type W = crate::W<CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0` reader - RTCSlow_1 memory split address in world 0 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0` writer - RTCSlow_1 memory split address in world 0 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_W<'a, REG> =
    crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1` reader - RTCSlow_1 memory split address in world 1 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1` writer - RTCSlow_1 memory split address in world 1 for core0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_W<'a, REG> =
    crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - RTCSlow_1 memory split address in world 0 for core0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTCSlow_1 memory split address in world 1 for core0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_R::new(
            ((self.bits >> 11) & 0x07ff) as u16,
        )
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_13")
            .field(
                "core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_0",
                &self.core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_0(),
            )
            .field(
                "core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_1",
                &self.core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - RTCSlow_1 memory split address in world 0 for core0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_W<CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>
    {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_0_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - RTCSlow_1 memory split address in world 1 for core0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcslow_1_spltaddr_world_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_W<CORE_0_PIF_PMS_CONSTRAIN_13_SPEC>
    {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_1_SPLTADDR_WORLD_1_W::new(self, 11)
    }
}
#[doc = "Core0 access peripherals permission configuration register 13.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_PIF_PMS_CONSTRAIN_13_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_pif_pms_constrain_13::R`](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_pif_pms_constrain_13::W`](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_13 to value 0x003f_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_13_SPEC {
    const RESET_VALUE: u32 = 0x003f_ffff;
}
