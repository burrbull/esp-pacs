#[doc = "Register `RX_STATUS` reader"]
pub type R = crate::R<RX_STATUS_SPEC>;
#[doc = "Field `SLC_RX_FULL(0-1)` reader - "]
pub type SLC_RX_FULL_R = crate::BitReader;
#[doc = "Field `SLC_RX_EMPTY(0-1)` reader - "]
pub type SLC_RX_EMPTY_R = crate::BitReader;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_FULL` field"]
    #[inline(always)]
    pub fn slc_rx_full(&self, n: u8) -> SLC_RX_FULL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_FULL_R::new(((self.bits >> (n * 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_full_iter(&self) -> impl Iterator<Item = SLC_RX_FULL_R> + '_ {
        (0..2).map(move |n| SLC_RX_FULL_R::new(((self.bits >> (n * 16)) & 1) != 0))
    }
    #[doc = "Bit 0 - SLC0_RX_FULL"]
    #[inline(always)]
    pub fn slc0_rx_full(&self) -> SLC_RX_FULL_R {
        SLC_RX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - SLC1_RX_FULL"]
    #[inline(always)]
    pub fn slc1_rx_full(&self) -> SLC_RX_FULL_R {
        SLC_RX_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_EMPTY` field"]
    #[inline(always)]
    pub fn slc_rx_empty(&self, n: u8) -> SLC_RX_EMPTY_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_EMPTY_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_empty_iter(&self) -> impl Iterator<Item = SLC_RX_EMPTY_R> + '_ {
        (0..2).map(move |n| SLC_RX_EMPTY_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - SLC0_RX_EMPTY"]
    #[inline(always)]
    pub fn slc0_rx_empty(&self) -> SLC_RX_EMPTY_R {
        SLC_RX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - SLC1_RX_EMPTY"]
    #[inline(always)]
    pub fn slc1_rx_empty(&self) -> SLC_RX_EMPTY_R {
        SLC_RX_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_STATUS")
            .field(
                "slc0_rx_full",
                &format_args!("{}", self.slc0_rx_full().bit()),
            )
            .field(
                "slc1_rx_full",
                &format_args!("{}", self.slc1_rx_full().bit()),
            )
            .field(
                "slc0_rx_empty",
                &format_args!("{}", self.slc0_rx_empty().bit()),
            )
            .field(
                "slc1_rx_empty",
                &format_args!("{}", self.slc1_rx_empty().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_STATUS_SPEC;
impl crate::RegisterSpec for RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_status::R`](R) reader structure"]
impl crate::Readable for RX_STATUS_SPEC {}
#[doc = "`reset()` method sets RX_STATUS to value 0x0002_0002"]
impl crate::Resettable for RX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0002_0002;
}
