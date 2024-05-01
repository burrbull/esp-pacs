///Register `APP_DCACHE_DBUG6` reader
pub type R = crate::R<APP_DCACHE_DBUG6_SPEC>;
///Field `APP_IRAM0ADDR_IA` reader -
pub type APP_IRAM0ADDR_IA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn app_iram0addr_ia(&self) -> APP_IRAM0ADDR_IA_R {
        APP_IRAM0ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG6")
            .field("app_iram0addr_ia", &self.app_iram0addr_ia())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_dcache_dbug6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_DCACHE_DBUG6_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_dcache_dbug6::R`](R) reader structure
impl crate::Readable for APP_DCACHE_DBUG6_SPEC {}
///`reset()` method sets APP_DCACHE_DBUG6 to value 0
impl crate::Resettable for APP_DCACHE_DBUG6_SPEC {
    const RESET_VALUE: u32 = 0;
}
