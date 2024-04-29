#[doc = "Register `SLC0_RX_LINK` reader"]
pub type R = crate::R<SLC0_RX_LINK_SPEC>;
#[doc = "Register `SLC0_RX_LINK` writer"]
pub type W = crate::W<SLC0_RX_LINK_SPEC>;
#[doc = "Field `RXLINK_ADDR` reader - "]
pub type RXLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RXLINK_ADDR` writer - "]
pub type RXLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RXLINK_STOP` reader - "]
pub type RXLINK_STOP_R = crate::BitReader;
#[doc = "Field `RXLINK_STOP` writer - "]
pub type RXLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLINK_START` reader - "]
pub type RXLINK_START_R = crate::BitReader;
#[doc = "Field `RXLINK_START` writer - "]
pub type RXLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLINK_RESTART` reader - "]
pub type RXLINK_RESTART_R = crate::BitReader;
#[doc = "Field `RXLINK_RESTART` writer - "]
pub type RXLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLINK_PARK` reader - "]
pub type RXLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn rxlink_addr(&self) -> RXLINK_ADDR_R {
        RXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxlink_stop(&self) -> RXLINK_STOP_R {
        RXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rxlink_start(&self) -> RXLINK_START_R {
        RXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rxlink_restart(&self) -> RXLINK_RESTART_R {
        RXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rxlink_park(&self) -> RXLINK_PARK_R {
        RXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_RX_LINK")
            .field(
                "rxlink_addr",
                &format_args!("{}", self.rxlink_addr().bits()),
            )
            .field("rxlink_stop", &format_args!("{}", self.rxlink_stop().bit()))
            .field(
                "rxlink_start",
                &format_args!("{}", self.rxlink_start().bit()),
            )
            .field(
                "rxlink_restart",
                &format_args!("{}", self.rxlink_restart().bit()),
            )
            .field("rxlink_park", &format_args!("{}", self.rxlink_park().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_RX_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn rxlink_addr(&mut self) -> RXLINK_ADDR_W<SLC0_RX_LINK_SPEC> {
        RXLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rxlink_stop(&mut self) -> RXLINK_STOP_W<SLC0_RX_LINK_SPEC> {
        RXLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rxlink_start(&mut self) -> RXLINK_START_W<SLC0_RX_LINK_SPEC> {
        RXLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rxlink_restart(&mut self) -> RXLINK_RESTART_W<SLC0_RX_LINK_SPEC> {
        RXLINK_RESTART_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_rx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_RX_LINK_SPEC;
impl crate::RegisterSpec for SLC0_RX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_rx_link::R`](R) reader structure"]
impl crate::Readable for SLC0_RX_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_rx_link::W`](W) writer structure"]
impl crate::Writable for SLC0_RX_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_RX_LINK to value 0"]
impl crate::Resettable for SLC0_RX_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
