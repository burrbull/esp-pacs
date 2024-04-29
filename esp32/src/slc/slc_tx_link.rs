#[doc = "Register `SLC%s_TX_LINK` reader"]
pub type R = crate::R<SLC_TX_LINK_SPEC>;
#[doc = "Register `SLC%s_TX_LINK` writer"]
pub type W = crate::W<SLC_TX_LINK_SPEC>;
#[doc = "Field `TXLINK_ADDR` reader - "]
pub type TXLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `TXLINK_ADDR` writer - "]
pub type TXLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `TXLINK_STOP` reader - "]
pub type TXLINK_STOP_R = crate::BitReader;
#[doc = "Field `TXLINK_STOP` writer - "]
pub type TXLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLINK_START` reader - "]
pub type TXLINK_START_R = crate::BitReader;
#[doc = "Field `TXLINK_START` writer - "]
pub type TXLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLINK_RESTART` reader - "]
pub type TXLINK_RESTART_R = crate::BitReader;
#[doc = "Field `TXLINK_RESTART` writer - "]
pub type TXLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLINK_PARK` reader - "]
pub type TXLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn txlink_addr(&self) -> TXLINK_ADDR_R {
        TXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn txlink_stop(&self) -> TXLINK_STOP_R {
        TXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn txlink_start(&self) -> TXLINK_START_R {
        TXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn txlink_restart(&self) -> TXLINK_RESTART_R {
        TXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn txlink_park(&self) -> TXLINK_PARK_R {
        TXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TX_LINK")
            .field(
                "txlink_addr",
                &format_args!("{}", self.txlink_addr().bits()),
            )
            .field("txlink_stop", &format_args!("{}", self.txlink_stop().bit()))
            .field(
                "txlink_start",
                &format_args!("{}", self.txlink_start().bit()),
            )
            .field(
                "txlink_restart",
                &format_args!("{}", self.txlink_restart().bit()),
            )
            .field("txlink_park", &format_args!("{}", self.txlink_park().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TX_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn txlink_addr(&mut self) -> TXLINK_ADDR_W<SLC_TX_LINK_SPEC> {
        TXLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn txlink_stop(&mut self) -> TXLINK_STOP_W<SLC_TX_LINK_SPEC> {
        TXLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn txlink_start(&mut self) -> TXLINK_START_W<SLC_TX_LINK_SPEC> {
        TXLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn txlink_restart(&mut self) -> TXLINK_RESTART_W<SLC_TX_LINK_SPEC> {
        TXLINK_RESTART_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TX_LINK_SPEC;
impl crate::RegisterSpec for SLC_TX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_tx_link::R`](R) reader structure"]
impl crate::Readable for SLC_TX_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_tx_link::W`](W) writer structure"]
impl crate::Writable for SLC_TX_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC%s_TX_LINK to value 0"]
impl crate::Resettable for SLC_TX_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
