///Register `APP_DCACHE_DBUG4` reader
pub type R = crate::R<APP_DCACHE_DBUG4_SPEC>;
///Field `APP_DRAM1ADDR0_IA` reader -
pub type APP_DRAM1ADDR0_IA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn app_dram1addr0_ia(&self) -> APP_DRAM1ADDR0_IA_R {
        APP_DRAM1ADDR0_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG4")
            .field("app_dram1addr0_ia", &self.app_dram1addr0_ia())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_DCACHE_DBUG4_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_dcache_dbug4::R`](R) reader structure
impl crate::Readable for APP_DCACHE_DBUG4_SPEC {}
///`reset()` method sets APP_DCACHE_DBUG4 to value 0
impl crate::Resettable for APP_DCACHE_DBUG4_SPEC {
    const RESET_VALUE: u32 = 0;
}
