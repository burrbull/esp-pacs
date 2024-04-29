#[doc = "Register `SLC0_TXPKT_E_DSCR` reader"]
pub type R = crate::R<SLC0_TXPKT_E_DSCR_SPEC>;
#[doc = "Register `SLC0_TXPKT_E_DSCR` writer"]
pub type W = crate::W<SLC0_TXPKT_E_DSCR_SPEC>;
#[doc = "Field `TX_PKT_E_DSCR_ADDR` reader - "]
pub type TX_PKT_E_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `TX_PKT_E_DSCR_ADDR` writer - "]
pub type TX_PKT_E_DSCR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_pkt_e_dscr_addr(&self) -> TX_PKT_E_DSCR_ADDR_R {
        TX_PKT_E_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_TXPKT_E_DSCR")
            .field(
                "tx_pkt_e_dscr_addr",
                &format_args!("{}", self.tx_pkt_e_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_TXPKT_E_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pkt_e_dscr_addr(&mut self) -> TX_PKT_E_DSCR_ADDR_W<SLC0_TXPKT_E_DSCR_SPEC> {
        TX_PKT_E_DSCR_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_txpkt_e_dscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_txpkt_e_dscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_TXPKT_E_DSCR_SPEC;
impl crate::RegisterSpec for SLC0_TXPKT_E_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_txpkt_e_dscr::R`](R) reader structure"]
impl crate::Readable for SLC0_TXPKT_E_DSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_txpkt_e_dscr::W`](W) writer structure"]
impl crate::Writable for SLC0_TXPKT_E_DSCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_TXPKT_E_DSCR to value 0"]
impl crate::Resettable for SLC0_TXPKT_E_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
