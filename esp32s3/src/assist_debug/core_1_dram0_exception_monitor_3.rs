#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_3` reader"]
pub type R = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_ADDR_1` reader - The second dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_WR_1` reader - The second dram0's wr status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_WR_1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:21 - The second dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_addr_1(&self) -> CORE_1_DRAM0_RECORDING_ADDR_1_R {
        CORE_1_DRAM0_RECORDING_ADDR_1_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - The second dram0's wr status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_wr_1(&self) -> CORE_1_DRAM0_RECORDING_WR_1_R {
        CORE_1_DRAM0_RECORDING_WR_1_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_3")
            .field(
                "core_1_dram0_recording_addr_1",
                &self.core_1_dram0_recording_addr_1(),
            )
            .field(
                "core_1_dram0_recording_wr_1",
                &self.core_1_dram0_recording_wr_1(),
            )
            .finish()
    }
}
#[doc = "Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_3::R`](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_3 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
