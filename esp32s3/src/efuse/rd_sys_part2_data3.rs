///Register `RD_SYS_PART2_DATA3` reader
pub type R = crate::R<RD_SYS_PART2_DATA3_SPEC>;
///Field `SYS_DATA_PART2_3` reader - Stores the 3rd 32 bits of the 2nd part of system data.
pub type SYS_DATA_PART2_3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the 3rd 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_3(&self) -> SYS_DATA_PART2_3_R {
        SYS_DATA_PART2_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART2_DATA3")
            .field("sys_data_part2_3", &self.sys_data_part2_3())
            .finish()
    }
}
/**Register 3 of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_SYS_PART2_DATA3_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_sys_part2_data3::R`](R) reader structure
impl crate::Readable for RD_SYS_PART2_DATA3_SPEC {}
///`reset()` method sets RD_SYS_PART2_DATA3 to value 0
impl crate::Resettable for RD_SYS_PART2_DATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
