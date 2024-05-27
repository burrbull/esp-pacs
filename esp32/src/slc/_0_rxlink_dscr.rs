///Register `_0_RXLINK_DSCR` reader
pub type R = crate::R<_0_RXLINK_DSCR_SPEC>;
///Field `SLC0_RXLINK_DSCR` reader -
pub type SLC0_RXLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn slc0_rxlink_dscr(&self) -> SLC0_RXLINK_DSCR_R {
        SLC0_RXLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_RXLINK_DSCR")
            .field("slc0_rxlink_dscr", &self.slc0_rxlink_dscr())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_0_rxlink_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _0_RXLINK_DSCR_SPEC;
impl crate::RegisterSpec for _0_RXLINK_DSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_0_rxlink_dscr::R`](R) reader structure
impl crate::Readable for _0_RXLINK_DSCR_SPEC {}
///`reset()` method sets _0_RXLINK_DSCR to value 0
impl crate::Resettable for _0_RXLINK_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
