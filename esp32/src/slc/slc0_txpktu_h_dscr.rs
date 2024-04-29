#[doc = "Register `SLC0_TXPKTU_H_DSCR` reader"]
pub type R = crate::R<SLC0_TXPKTU_H_DSCR_SPEC>;
#[doc = "Field `TX_PKT_START_DSCR_ADDR` reader - "]
pub type TX_PKT_START_DSCR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_pkt_start_dscr_addr(&self) -> TX_PKT_START_DSCR_ADDR_R {
        TX_PKT_START_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_TXPKTU_H_DSCR")
            .field(
                "tx_pkt_start_dscr_addr",
                &format_args!("{}", self.tx_pkt_start_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_TXPKTU_H_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_txpktu_h_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_TXPKTU_H_DSCR_SPEC;
impl crate::RegisterSpec for SLC0_TXPKTU_H_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_txpktu_h_dscr::R`](R) reader structure"]
impl crate::Readable for SLC0_TXPKTU_H_DSCR_SPEC {}
#[doc = "`reset()` method sets SLC0_TXPKTU_H_DSCR to value 0"]
impl crate::Resettable for SLC0_TXPKTU_H_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
