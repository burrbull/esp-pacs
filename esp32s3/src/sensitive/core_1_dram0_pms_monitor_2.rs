#[doc = "Register `CORE_1_DRAM0_PMS_MONITOR_2` reader"]
pub type R = crate::R<CORE_1_DRAM0_PMS_MONITOR_2_SPEC>;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR` reader - recorded core1 dram0 permission monitor interrupt status."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK` reader - recorded core1 dram0 lock status, 1 means s32c1i access."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - recorded core1 dram0 world status, 0x1 means world0, 0x2 means world1."]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R = crate::FieldReader;
#[doc = "Field `CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - recorded core1 dram0 address\\[25:4\\] status when core1 dram0 violated permission,the real address is 0x3c000000+addr*16"]
pub type CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - recorded core1 dram0 permission monitor interrupt status."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_intr(&self) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - recorded core1 dram0 lock status, 1 means s32c1i access."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_lock(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - recorded core1 dram0 world status, 0x1 means world0, 0x2 means world1."]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_world(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:25 - recorded core1 dram0 address\\[25:4\\] status when core1 dram0 violated permission,the real address is 0x3c000000+addr*16"]
    #[inline(always)]
    pub fn core_1_dram0_pms_monitor_violate_status_addr(
        &self,
    ) -> CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        CORE_1_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new((self.bits >> 4) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_PMS_MONITOR_2")
            .field(
                "core_1_dram0_pms_monitor_violate_intr",
                &self.core_1_dram0_pms_monitor_violate_intr().bit(),
            )
            .field(
                "core_1_dram0_pms_monitor_violate_status_lock",
                &self.core_1_dram0_pms_monitor_violate_status_lock().bit(),
            )
            .field(
                "core_1_dram0_pms_monitor_violate_status_world",
                &self.core_1_dram0_pms_monitor_violate_status_world().bits(),
            )
            .field(
                "core_1_dram0_pms_monitor_violate_status_addr",
                &self.core_1_dram0_pms_monitor_violate_status_addr().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_DRAM0_PMS_MONITOR_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "core1 dram0 permission monitor configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_pms_monitor_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_DRAM0_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_pms_monitor_2::R`](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_PMS_MONITOR_2_SPEC {}
#[doc = "`reset()` method sets CORE_1_DRAM0_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_PMS_MONITOR_2_SPEC {}
