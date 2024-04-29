#[doc = "Register `SLC0_RXPKTU_E_DSCR` reader"]
pub type R = crate::R<SLC0_RXPKTU_E_DSCR_SPEC>;
#[doc = "Field `RX_PKT_END_DSCR_ADDR` reader - "]
pub type RX_PKT_END_DSCR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_pkt_end_dscr_addr(&self) -> RX_PKT_END_DSCR_ADDR_R {
        RX_PKT_END_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_RXPKTU_E_DSCR")
            .field(
                "rx_pkt_end_dscr_addr",
                &format_args!("{}", self.rx_pkt_end_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_RXPKTU_E_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rxpktu_e_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_RXPKTU_E_DSCR_SPEC;
impl crate::RegisterSpec for SLC0_RXPKTU_E_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_rxpktu_e_dscr::R`](R) reader structure"]
impl crate::Readable for SLC0_RXPKTU_E_DSCR_SPEC {}
#[doc = "`reset()` method sets SLC0_RXPKTU_E_DSCR to value 0"]
impl crate::Resettable for SLC0_RXPKTU_E_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
