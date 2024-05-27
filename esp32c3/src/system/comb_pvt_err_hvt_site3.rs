#[doc = "Register `COMB_PVT_ERR_HVT_SITE3` reader"]
pub type R = crate::R<COMB_PVT_ERR_HVT_SITE3_SPEC>;
#[doc = "Field `COMB_TIMING_ERR_CNT_HVT_SITE3` reader - reg_comb_timing_err_cnt_hvt_site3"]
pub type COMB_TIMING_ERR_CNT_HVT_SITE3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - reg_comb_timing_err_cnt_hvt_site3"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_hvt_site3(&self) -> COMB_TIMING_ERR_CNT_HVT_SITE3_R {
        COMB_TIMING_ERR_CNT_HVT_SITE3_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_ERR_HVT_SITE3")
            .field(
                "comb_timing_err_cnt_hvt_site3",
                &self.comb_timing_err_cnt_hvt_site3(),
            )
            .finish()
    }
}
#[doc = "mem pvt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_hvt_site3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PVT_ERR_HVT_SITE3_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_HVT_SITE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pvt_err_hvt_site3::R`](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_HVT_SITE3_SPEC {}
#[doc = "`reset()` method sets COMB_PVT_ERR_HVT_SITE3 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_HVT_SITE3_SPEC {
    const RESET_VALUE: u32 = 0;
}
