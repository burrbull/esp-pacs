///Register `CORE_0_PIF_PMS_CONSTRAIN_12` reader
pub type R = crate::R<CORE_0_PIF_PMS_CONSTRAIN_12_SPEC>;
///Register `CORE_0_PIF_PMS_CONSTRAIN_12` writer
pub type W = crate::W<CORE_0_PIF_PMS_CONSTRAIN_12_SPEC>;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L` reader - RTCSlow_0 memory low region permission in world 0 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L_R = crate::FieldReader;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L` writer - RTCSlow_0 memory low region permission in world 0 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L_W<'a, REG> = crate::FieldWriter<
    'a,
    REG,
    3,
>;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H` reader - RTCSlow_0 memory high region permission in world 0 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H_R = crate::FieldReader;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H` writer - RTCSlow_0 memory high region permission in world 0 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H_W<'a, REG> = crate::FieldWriter<
    'a,
    REG,
    3,
>;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L` reader - RTCSlow_0 memory low region permission in world 1 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L_R = crate::FieldReader;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L` writer - RTCSlow_0 memory low region permission in world 1 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L_W<'a, REG> = crate::FieldWriter<
    'a,
    REG,
    3,
>;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H` reader - RTCSlow_0 memory high region permission in world 1 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H_R = crate::FieldReader;
///Field `CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H` writer - RTCSlow_0 memory high region permission in world 1 for core0.
pub type CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H_W<'a, REG> = crate::FieldWriter<
    'a,
    REG,
    3,
>;
impl R {
    ///Bits 0:2 - RTCSlow_0 memory low region permission in world 0 for core0.
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_0_l(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - RTCSlow_0 memory high region permission in world 0 for core0.
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_0_h(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - RTCSlow_0 memory low region permission in world 1 for core0.
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_1_l(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - RTCSlow_0 memory high region permission in world 1 for core0.
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_1_h(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H_R::new(((self.bits >> 9) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_12")
            .field(
                "core_0_pif_pms_constrain_rtcslow_0_world_0_l",
                &self.core_0_pif_pms_constrain_rtcslow_0_world_0_l(),
            )
            .field(
                "core_0_pif_pms_constrain_rtcslow_0_world_0_h",
                &self.core_0_pif_pms_constrain_rtcslow_0_world_0_h(),
            )
            .field(
                "core_0_pif_pms_constrain_rtcslow_0_world_1_l",
                &self.core_0_pif_pms_constrain_rtcslow_0_world_1_l(),
            )
            .field(
                "core_0_pif_pms_constrain_rtcslow_0_world_1_h",
                &self.core_0_pif_pms_constrain_rtcslow_0_world_1_h(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:2 - RTCSlow_0 memory low region permission in world 0 for core0.
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_0_l(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L_W<
        CORE_0_PIF_PMS_CONSTRAIN_12_SPEC,
    > {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_L_W::new(self, 0)
    }
    ///Bits 3:5 - RTCSlow_0 memory high region permission in world 0 for core0.
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_0_h(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H_W<
        CORE_0_PIF_PMS_CONSTRAIN_12_SPEC,
    > {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_0_H_W::new(self, 3)
    }
    ///Bits 6:8 - RTCSlow_0 memory low region permission in world 1 for core0.
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_1_l(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L_W<
        CORE_0_PIF_PMS_CONSTRAIN_12_SPEC,
    > {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_L_W::new(self, 6)
    }
    ///Bits 9:11 - RTCSlow_0 memory high region permission in world 1 for core0.
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_rtcslow_0_world_1_h(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H_W<
        CORE_0_PIF_PMS_CONSTRAIN_12_SPEC,
    > {
        CORE_0_PIF_PMS_CONSTRAIN_RTCSLOW_0_WORLD_1_H_W::new(self, 9)
    }
}
/**Core0 access peripherals permission configuration register 12.

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_pif_pms_constrain_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_pif_pms_constrain_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_PIF_PMS_CONSTRAIN_12_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_12_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_pif_pms_constrain_12::R`](R) reader structure
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_12_SPEC {}
///`write(|w| ..)` method takes [`core_0_pif_pms_constrain_12::W`](W) writer structure
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_12 to value 0x0fff
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_12_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
