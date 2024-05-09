#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_0` reader"]
pub type R = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_WR_0` reader - reg_core_1_dram0_recording_wr_0"]
pub type CORE_1_DRAM0_RECORDING_WR_0_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_RECORDING_BYTEEN_0` reader - reg_core_1_dram0_recording_byteen_0"]
pub type CORE_1_DRAM0_RECORDING_BYTEEN_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - reg_core_1_dram0_recording_wr_0"]
    #[inline(always)]
    pub fn core_1_dram0_recording_wr_0(&self) -> CORE_1_DRAM0_RECORDING_WR_0_R {
        CORE_1_DRAM0_RECORDING_WR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - reg_core_1_dram0_recording_byteen_0"]
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_0(&self) -> CORE_1_DRAM0_RECORDING_BYTEEN_0_R {
        CORE_1_DRAM0_RECORDING_BYTEEN_0_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_1_dram0_recording_wr_0",
                &self.core_1_dram0_recording_wr_0().bit(),
            )
            .field(
                "core_1_dram0_recording_byteen_0",
                &self.core_1_dram0_recording_byteen_0().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "exception monitor status register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_0::R`](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_0 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
