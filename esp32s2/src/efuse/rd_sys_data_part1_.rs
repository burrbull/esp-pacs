///Register `RD_SYS_DATA_PART1_%s` reader
pub type R = crate::R<RD_SYS_DATA_PART1__SPEC>;
///Field `SYS_DATA_PART1` reader - Stores the %sth 32 bits of the first part of system data.
pub type SYS_DATA_PART1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the %sth 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1(&self) -> SYS_DATA_PART1_R {
        SYS_DATA_PART1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_DATA_PART1_")
            .field("sys_data_part1", &self.sys_data_part1())
            .finish()
    }
}
/**Register %s of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_data_part1_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_SYS_DATA_PART1__SPEC;
impl crate::RegisterSpec for RD_SYS_DATA_PART1__SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_sys_data_part1_::R`](R) reader structure
impl crate::Readable for RD_SYS_DATA_PART1__SPEC {}
///`reset()` method sets RD_SYS_DATA_PART1_%s to value 0
impl crate::Resettable for RD_SYS_DATA_PART1__SPEC {
    const RESET_VALUE: u32 = 0;
}
