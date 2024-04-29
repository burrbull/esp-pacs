#[doc = "Register `TX_STATUS` reader"]
pub type R = crate::R<TX_STATUS_SPEC>;
#[doc = "Field `SLC_TX_FULL(0-1)` reader - "]
pub type SLC_TX_FULL_R = crate::BitReader;
#[doc = "Field `SLC_TX_EMPTY(0-1)` reader - "]
pub type SLC_TX_EMPTY_R = crate::BitReader;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_FULL` field"]
    #[inline(always)]
    pub fn slc_tx_full(&self, n: u8) -> SLC_TX_FULL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_FULL_R::new(((self.bits >> (n * 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_tx_full_iter(&self) -> impl Iterator<Item = SLC_TX_FULL_R> + '_ {
        (0..2).map(move |n| SLC_TX_FULL_R::new(((self.bits >> (n * 16)) & 1) != 0))
    }
    #[doc = "Bit 0 - SLC0_TX_FULL"]
    #[inline(always)]
    pub fn slc0_tx_full(&self) -> SLC_TX_FULL_R {
        SLC_TX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - SLC1_TX_FULL"]
    #[inline(always)]
    pub fn slc1_tx_full(&self) -> SLC_TX_FULL_R {
        SLC_TX_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_EMPTY` field"]
    #[inline(always)]
    pub fn slc_tx_empty(&self, n: u8) -> SLC_TX_EMPTY_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_EMPTY_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_tx_empty_iter(&self) -> impl Iterator<Item = SLC_TX_EMPTY_R> + '_ {
        (0..2).map(move |n| SLC_TX_EMPTY_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - SLC0_TX_EMPTY"]
    #[inline(always)]
    pub fn slc0_tx_empty(&self) -> SLC_TX_EMPTY_R {
        SLC_TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - SLC1_TX_EMPTY"]
    #[inline(always)]
    pub fn slc1_tx_empty(&self) -> SLC_TX_EMPTY_R {
        SLC_TX_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_STATUS")
            .field(
                "slc0_tx_full",
                &format_args!("{}", self.slc0_tx_full().bit()),
            )
            .field(
                "slc1_tx_full",
                &format_args!("{}", self.slc1_tx_full().bit()),
            )
            .field(
                "slc0_tx_empty",
                &format_args!("{}", self.slc0_tx_empty().bit()),
            )
            .field(
                "slc1_tx_empty",
                &format_args!("{}", self.slc1_tx_empty().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_STATUS_SPEC;
impl crate::RegisterSpec for TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_status::R`](R) reader structure"]
impl crate::Readable for TX_STATUS_SPEC {}
#[doc = "`reset()` method sets TX_STATUS to value 0x0002_0002"]
impl crate::Resettable for TX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0002_0002;
}
