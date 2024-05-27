///Register `DSCR_BF1` reader
pub type R = crate::R<DSCR_BF1_SPEC>;
///Field `OUTLINK_DSCR_BF1` reader - The address of the second-to-last outlink descriptor's next address y-2.
pub type OUTLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The address of the second-to-last outlink descriptor's next address y-2.
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OUTLINK_DSCR_BF1_R {
        OUTLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCR_BF1")
            .field("outlink_dscr_bf1", &self.outlink_dscr_bf1())
            .finish()
    }
}
/**TX CHx second-to-last dscr addr register

You can [`read`](crate::generic::Reg::read) this register and get [`dscr_bf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DSCR_BF1_SPEC;
impl crate::RegisterSpec for DSCR_BF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dscr_bf1::R`](R) reader structure
impl crate::Readable for DSCR_BF1_SPEC {}
///`reset()` method sets DSCR_BF1 to value 0
impl crate::Resettable for DSCR_BF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
