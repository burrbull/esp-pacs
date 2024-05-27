///Register `RD_WR_DIS` reader
pub type R = crate::R<RD_WR_DIS_SPEC>;
///Field `WR_DIS` reader - Disable programming of individual eFuses.
pub type WR_DIS_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Disable programming of individual eFuses.
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_WR_DIS")
            .field("wr_dis", &self.wr_dis())
            .finish()
    }
}
/**BLOCK0 data register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_WR_DIS_SPEC;
impl crate::RegisterSpec for RD_WR_DIS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_wr_dis::R`](R) reader structure
impl crate::Readable for RD_WR_DIS_SPEC {}
///`reset()` method sets RD_WR_DIS to value 0
impl crate::Resettable for RD_WR_DIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
