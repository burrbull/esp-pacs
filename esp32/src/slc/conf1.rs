#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `SLC_CHECK_OWNER(0-1)` reader - "]
pub type SLC_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `SLC_CHECK_OWNER(0-1)` writer - "]
pub type SLC_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TX_CHECK_SUM_EN(0-1)` reader - "]
pub type SLC_TX_CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `SLC_TX_CHECK_SUM_EN(0-1)` writer - "]
pub type SLC_TX_CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_CHECK_SUM_EN(0-1)` reader - "]
pub type SLC_RX_CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `SLC_RX_CHECK_SUM_EN(0-1)` writer - "]
pub type SLC_RX_CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_HOLD_EN` reader - "]
pub type CMD_HOLD_EN_R = crate::BitReader;
#[doc = "Field `CMD_HOLD_EN` writer - "]
pub type CMD_HOLD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_LEN_AUTO_CLR(0-0)` reader - "]
pub type SLC_LEN_AUTO_CLR_R = crate::BitReader;
#[doc = "Field `SLC_LEN_AUTO_CLR(0-0)` writer - "]
pub type SLC_LEN_AUTO_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TX_STITCH_EN(0-1)` reader - "]
pub type SLC_TX_STITCH_EN_R = crate::BitReader;
#[doc = "Field `SLC_TX_STITCH_EN(0-1)` writer - "]
pub type SLC_TX_STITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_STITCH_EN(0-1)` reader - "]
pub type SLC_RX_STITCH_EN_R = crate::BitReader;
#[doc = "Field `SLC_RX_STITCH_EN(0-1)` writer - "]
pub type SLC_RX_STITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_INT_LEVEL_SEL` reader - "]
pub type HOST_INT_LEVEL_SEL_R = crate::BitReader;
#[doc = "Field `HOST_INT_LEVEL_SEL` writer - "]
pub type HOST_INT_LEVEL_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_CHECK_OWNER` field"]
    #[inline(always)]
    pub fn slc_check_owner(&self, n: u8) -> SLC_CHECK_OWNER_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_CHECK_OWNER_R::new(((self.bits >> (n * 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_check_owner_iter(&self) -> impl Iterator<Item = SLC_CHECK_OWNER_R> + '_ {
        (0..2).map(move |n| SLC_CHECK_OWNER_R::new(((self.bits >> (n * 16)) & 1) != 0))
    }
    #[doc = "Bit 0 - SLC0_CHECK_OWNER"]
    #[inline(always)]
    pub fn slc0_check_owner(&self) -> SLC_CHECK_OWNER_R {
        SLC_CHECK_OWNER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - SLC1_CHECK_OWNER"]
    #[inline(always)]
    pub fn slc1_check_owner(&self) -> SLC_CHECK_OWNER_R {
        SLC_CHECK_OWNER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_CHECK_SUM_EN` field"]
    #[inline(always)]
    pub fn slc_tx_check_sum_en(&self, n: u8) -> SLC_TX_CHECK_SUM_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_CHECK_SUM_EN_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_tx_check_sum_en_iter(&self) -> impl Iterator<Item = SLC_TX_CHECK_SUM_EN_R> + '_ {
        (0..2).map(move |n| SLC_TX_CHECK_SUM_EN_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - SLC0_TX_CHECK_SUM_EN"]
    #[inline(always)]
    pub fn slc0_tx_check_sum_en(&self) -> SLC_TX_CHECK_SUM_EN_R {
        SLC_TX_CHECK_SUM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - SLC1_TX_CHECK_SUM_EN"]
    #[inline(always)]
    pub fn slc1_tx_check_sum_en(&self) -> SLC_TX_CHECK_SUM_EN_R {
        SLC_TX_CHECK_SUM_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_CHECK_SUM_EN` field"]
    #[inline(always)]
    pub fn slc_rx_check_sum_en(&self, n: u8) -> SLC_RX_CHECK_SUM_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_CHECK_SUM_EN_R::new(((self.bits >> (n * 16 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_check_sum_en_iter(&self) -> impl Iterator<Item = SLC_RX_CHECK_SUM_EN_R> + '_ {
        (0..2).map(move |n| SLC_RX_CHECK_SUM_EN_R::new(((self.bits >> (n * 16 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - SLC0_RX_CHECK_SUM_EN"]
    #[inline(always)]
    pub fn slc0_rx_check_sum_en(&self) -> SLC_RX_CHECK_SUM_EN_R {
        SLC_RX_CHECK_SUM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - SLC1_RX_CHECK_SUM_EN"]
    #[inline(always)]
    pub fn slc1_rx_check_sum_en(&self) -> SLC_RX_CHECK_SUM_EN_R {
        SLC_RX_CHECK_SUM_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_hold_en(&self) -> CMD_HOLD_EN_R {
        CMD_HOLD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_LEN_AUTO_CLR` field"]
    #[inline(always)]
    pub fn slc_len_auto_clr(&self, n: u8) -> SLC_LEN_AUTO_CLR_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        SLC_LEN_AUTO_CLR_R::new(((self.bits >> (n * 0 + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_len_auto_clr_iter(&self) -> impl Iterator<Item = SLC_LEN_AUTO_CLR_R> + '_ {
        (0..1).map(move |n| SLC_LEN_AUTO_CLR_R::new(((self.bits >> (n * 0 + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - SLC0_LEN_AUTO_CLR"]
    #[inline(always)]
    pub fn slc0_len_auto_clr(&self) -> SLC_LEN_AUTO_CLR_R {
        SLC_LEN_AUTO_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_STITCH_EN` field"]
    #[inline(always)]
    pub fn slc_tx_stitch_en(&self, n: u8) -> SLC_TX_STITCH_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_STITCH_EN_R::new(((self.bits >> (n * 15 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_tx_stitch_en_iter(&self) -> impl Iterator<Item = SLC_TX_STITCH_EN_R> + '_ {
        (0..2).map(move |n| SLC_TX_STITCH_EN_R::new(((self.bits >> (n * 15 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - SLC0_TX_STITCH_EN"]
    #[inline(always)]
    pub fn slc0_tx_stitch_en(&self) -> SLC_TX_STITCH_EN_R {
        SLC_TX_STITCH_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 20 - SLC1_TX_STITCH_EN"]
    #[inline(always)]
    pub fn slc1_tx_stitch_en(&self) -> SLC_TX_STITCH_EN_R {
        SLC_TX_STITCH_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_STITCH_EN` field"]
    #[inline(always)]
    pub fn slc_rx_stitch_en(&self, n: u8) -> SLC_RX_STITCH_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_STITCH_EN_R::new(((self.bits >> (n * 15 + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_stitch_en_iter(&self) -> impl Iterator<Item = SLC_RX_STITCH_EN_R> + '_ {
        (0..2).map(move |n| SLC_RX_STITCH_EN_R::new(((self.bits >> (n * 15 + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - SLC0_RX_STITCH_EN"]
    #[inline(always)]
    pub fn slc0_rx_stitch_en(&self) -> SLC_RX_STITCH_EN_R {
        SLC_RX_STITCH_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 21 - SLC1_RX_STITCH_EN"]
    #[inline(always)]
    pub fn slc1_rx_stitch_en(&self) -> SLC_RX_STITCH_EN_R {
        SLC_RX_STITCH_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_int_level_sel(&self) -> HOST_INT_LEVEL_SEL_R {
        HOST_INT_LEVEL_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field(
                "slc0_check_owner",
                &format_args!("{}", self.slc0_check_owner().bit()),
            )
            .field(
                "slc1_check_owner",
                &format_args!("{}", self.slc1_check_owner().bit()),
            )
            .field(
                "slc0_tx_check_sum_en",
                &format_args!("{}", self.slc0_tx_check_sum_en().bit()),
            )
            .field(
                "slc1_tx_check_sum_en",
                &format_args!("{}", self.slc1_tx_check_sum_en().bit()),
            )
            .field(
                "slc0_rx_check_sum_en",
                &format_args!("{}", self.slc0_rx_check_sum_en().bit()),
            )
            .field(
                "slc1_rx_check_sum_en",
                &format_args!("{}", self.slc1_rx_check_sum_en().bit()),
            )
            .field("cmd_hold_en", &format_args!("{}", self.cmd_hold_en().bit()))
            .field(
                "slc0_len_auto_clr",
                &format_args!("{}", self.slc0_len_auto_clr().bit()),
            )
            .field(
                "slc0_tx_stitch_en",
                &format_args!("{}", self.slc0_tx_stitch_en().bit()),
            )
            .field(
                "slc1_tx_stitch_en",
                &format_args!("{}", self.slc1_tx_stitch_en().bit()),
            )
            .field(
                "slc0_rx_stitch_en",
                &format_args!("{}", self.slc0_rx_stitch_en().bit()),
            )
            .field(
                "slc1_rx_stitch_en",
                &format_args!("{}", self.slc1_rx_stitch_en().bit()),
            )
            .field(
                "host_int_level_sel",
                &format_args!("{}", self.host_int_level_sel().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_CHECK_OWNER` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_check_owner(&mut self, n: u8) -> SLC_CHECK_OWNER_W<CONF1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_CHECK_OWNER_W::new(self, n * 16)
    }
    #[doc = "Bit 0 - SLC0_CHECK_OWNER"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_check_owner(&mut self) -> SLC_CHECK_OWNER_W<CONF1_SPEC> {
        SLC_CHECK_OWNER_W::new(self, 0)
    }
    #[doc = "Bit 16 - SLC1_CHECK_OWNER"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_check_owner(&mut self) -> SLC_CHECK_OWNER_W<CONF1_SPEC> {
        SLC_CHECK_OWNER_W::new(self, 16)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_CHECK_SUM_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_check_sum_en(&mut self, n: u8) -> SLC_TX_CHECK_SUM_EN_W<CONF1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_CHECK_SUM_EN_W::new(self, n * 16 + 1)
    }
    #[doc = "Bit 1 - SLC0_TX_CHECK_SUM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_check_sum_en(&mut self) -> SLC_TX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC_TX_CHECK_SUM_EN_W::new(self, 1)
    }
    #[doc = "Bit 17 - SLC1_TX_CHECK_SUM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_check_sum_en(&mut self) -> SLC_TX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC_TX_CHECK_SUM_EN_W::new(self, 17)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_CHECK_SUM_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_check_sum_en(&mut self, n: u8) -> SLC_RX_CHECK_SUM_EN_W<CONF1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_CHECK_SUM_EN_W::new(self, n * 16 + 2)
    }
    #[doc = "Bit 2 - SLC0_RX_CHECK_SUM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_check_sum_en(&mut self) -> SLC_RX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC_RX_CHECK_SUM_EN_W::new(self, 2)
    }
    #[doc = "Bit 18 - SLC1_RX_CHECK_SUM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_check_sum_en(&mut self) -> SLC_RX_CHECK_SUM_EN_W<CONF1_SPEC> {
        SLC_RX_CHECK_SUM_EN_W::new(self, 18)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_hold_en(&mut self) -> CMD_HOLD_EN_W<CONF1_SPEC> {
        CMD_HOLD_EN_W::new(self, 3)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_LEN_AUTO_CLR` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_len_auto_clr(&mut self, n: u8) -> SLC_LEN_AUTO_CLR_W<CONF1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        SLC_LEN_AUTO_CLR_W::new(self, n * 0 + 4)
    }
    #[doc = "Bit 4 - SLC0_LEN_AUTO_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_auto_clr(&mut self) -> SLC_LEN_AUTO_CLR_W<CONF1_SPEC> {
        SLC_LEN_AUTO_CLR_W::new(self, 4)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_STITCH_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_stitch_en(&mut self, n: u8) -> SLC_TX_STITCH_EN_W<CONF1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_STITCH_EN_W::new(self, n * 15 + 5)
    }
    #[doc = "Bit 5 - SLC0_TX_STITCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_stitch_en(&mut self) -> SLC_TX_STITCH_EN_W<CONF1_SPEC> {
        SLC_TX_STITCH_EN_W::new(self, 5)
    }
    #[doc = "Bit 20 - SLC1_TX_STITCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_stitch_en(&mut self) -> SLC_TX_STITCH_EN_W<CONF1_SPEC> {
        SLC_TX_STITCH_EN_W::new(self, 20)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_STITCH_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_stitch_en(&mut self, n: u8) -> SLC_RX_STITCH_EN_W<CONF1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_STITCH_EN_W::new(self, n * 15 + 6)
    }
    #[doc = "Bit 6 - SLC0_RX_STITCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_stitch_en(&mut self) -> SLC_RX_STITCH_EN_W<CONF1_SPEC> {
        SLC_RX_STITCH_EN_W::new(self, 6)
    }
    #[doc = "Bit 21 - SLC1_RX_STITCH_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_stitch_en(&mut self) -> SLC_RX_STITCH_EN_W<CONF1_SPEC> {
        SLC_RX_STITCH_EN_W::new(self, 21)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn host_int_level_sel(&mut self) -> HOST_INT_LEVEL_SEL_W<CONF1_SPEC> {
        HOST_INT_LEVEL_SEL_W::new(self, 19)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF1_SPEC> {
        CLK_EN_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x0030_0078"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x0030_0078;
}
