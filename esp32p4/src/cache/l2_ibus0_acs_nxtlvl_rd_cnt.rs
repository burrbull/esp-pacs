///Register `L2_IBUS0_ACS_NXTLVL_RD_CNT` reader
pub type R = crate::R<L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
///Field `L2_IBUS0_NXTLVL_RD_CNT` reader - The register records the number of times that L2-Cache accesses external memory due to L1-ICache0 accessing L2-Cache due to bus0 accessing L1-ICache0.
pub type L2_IBUS0_NXTLVL_RD_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The register records the number of times that L2-Cache accesses external memory due to L1-ICache0 accessing L2-Cache due to bus0 accessing L1-ICache0.
    #[inline(always)]
    pub fn l2_ibus0_nxtlvl_rd_cnt(&self) -> L2_IBUS0_NXTLVL_RD_CNT_R {
        L2_IBUS0_NXTLVL_RD_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_IBUS0_ACS_NXTLVL_RD_CNT")
            .field("l2_ibus0_nxtlvl_rd_cnt", &self.l2_ibus0_nxtlvl_rd_cnt())
            .finish()
    }
}
/**L2-Cache bus0 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus0_acs_nxtlvl_rd_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC;
impl crate::RegisterSpec for L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_ibus0_acs_nxtlvl_rd_cnt::R`](R) reader structure
impl crate::Readable for L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC {}
///`reset()` method sets L2_IBUS0_ACS_NXTLVL_RD_CNT to value 0
impl crate::Resettable for L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
