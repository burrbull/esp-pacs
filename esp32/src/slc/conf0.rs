#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `SLC_TX_RST(0-1)` reader - "]
pub type SLC_TX_RST_R = crate::BitReader;
#[doc = "Field `SLC_TX_RST(0-1)` writer - "]
pub type SLC_TX_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_RST(0-1)` reader - "]
pub type SLC_RX_RST_R = crate::BitReader;
#[doc = "Field `SLC_RX_RST(0-1)` writer - "]
pub type SLC_RX_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_FIFO_RST` reader - "]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - "]
pub type AHBM_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_RST` reader - "]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - "]
pub type AHBM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TX_LOOP_TEST(0-1)` reader - "]
pub type SLC_TX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC_TX_LOOP_TEST(0-1)` writer - "]
pub type SLC_TX_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_LOOP_TEST(0-1)` reader - "]
pub type SLC_RX_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `SLC_RX_LOOP_TEST(0-1)` writer - "]
pub type SLC_RX_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_AUTO_WRBACK(0-1)` reader - "]
pub type SLC_RX_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `SLC_RX_AUTO_WRBACK(0-1)` writer - "]
pub type SLC_RX_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_NO_RESTART_CLR(0-1)` reader - "]
pub type SLC_RX_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `SLC_RX_NO_RESTART_CLR(0-1)` writer - "]
pub type SLC_RX_NO_RESTART_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RXDSCR_BURST_EN(0-1)` reader - "]
pub type SLC_RXDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC_RXDSCR_BURST_EN(0-1)` writer - "]
pub type SLC_RXDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RXDATA_BURST_EN(0-1)` reader - "]
pub type SLC_RXDATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC_RXDATA_BURST_EN(0-1)` writer - "]
pub type SLC_RXDATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RXLINK_AUTO_RET(0-1)` reader - "]
pub type SLC_RXLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `SLC_RXLINK_AUTO_RET(0-1)` writer - "]
pub type SLC_RXLINK_AUTO_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TXLINK_AUTO_RET(0-1)` reader - "]
pub type SLC_TXLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `SLC_TXLINK_AUTO_RET(0-1)` writer - "]
pub type SLC_TXLINK_AUTO_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TXDSCR_BURST_EN(0-1)` reader - "]
pub type SLC_TXDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC_TXDSCR_BURST_EN(0-1)` writer - "]
pub type SLC_TXDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TXDATA_BURST_EN(0-1)` reader - "]
pub type SLC_TXDATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `SLC_TXDATA_BURST_EN(0-1)` writer - "]
pub type SLC_TXDATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TOKEN_AUTO_CLR(0-1)` reader - "]
pub type SLC_TOKEN_AUTO_CLR_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN_AUTO_CLR(0-1)` writer - "]
pub type SLC_TOKEN_AUTO_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_TOKEN_SEL(0-1)` reader - "]
pub type SLC_TOKEN_SEL_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN_SEL(0-1)` writer - "]
pub type SLC_TOKEN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_WR_RETRY_MASK_EN(0-1)` reader - "]
pub type SLC_WR_RETRY_MASK_EN_R = crate::BitReader;
#[doc = "Field `SLC_WR_RETRY_MASK_EN(0-1)` writer - "]
pub type SLC_WR_RETRY_MASK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_RST` field"]
    #[inline(always)]
    pub fn slc_tx_rst(&self, n: u8) -> SLC_TX_RST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_RST_R::new(((self.bits >> (n * 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_tx_rst_iter(&self) -> impl Iterator<Item = SLC_TX_RST_R> + '_ {
        (0..2).map(move |n| SLC_TX_RST_R::new(((self.bits >> (n * 16)) & 1) != 0))
    }
    #[doc = "Bit 0 - SLC0_TX_RST"]
    #[inline(always)]
    pub fn slc0_tx_rst(&self) -> SLC_TX_RST_R {
        SLC_TX_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - SLC1_TX_RST"]
    #[inline(always)]
    pub fn slc1_tx_rst(&self) -> SLC_TX_RST_R {
        SLC_TX_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_RST` field"]
    #[inline(always)]
    pub fn slc_rx_rst(&self, n: u8) -> SLC_RX_RST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_RST_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_rst_iter(&self) -> impl Iterator<Item = SLC_RX_RST_R> + '_ {
        (0..2).map(move |n| SLC_RX_RST_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - SLC0_RX_RST"]
    #[inline(always)]
    pub fn slc0_rx_rst(&self) -> SLC_RX_RST_R {
        SLC_RX_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - SLC1_RX_RST"]
    #[inline(always)]
    pub fn slc1_rx_rst(&self) -> SLC_RX_RST_R {
        SLC_RX_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_LOOP_TEST` field"]
    #[inline(always)]
    pub fn slc_tx_loop_test(&self, n: u8) -> SLC_TX_LOOP_TEST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_LOOP_TEST_R::new(((self.bits >> (n * 16 + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_tx_loop_test_iter(&self) -> impl Iterator<Item = SLC_TX_LOOP_TEST_R> + '_ {
        (0..2).map(move |n| SLC_TX_LOOP_TEST_R::new(((self.bits >> (n * 16 + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - SLC0_TX_LOOP_TEST"]
    #[inline(always)]
    pub fn slc0_tx_loop_test(&self) -> SLC_TX_LOOP_TEST_R {
        SLC_TX_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 20 - SLC1_TX_LOOP_TEST"]
    #[inline(always)]
    pub fn slc1_tx_loop_test(&self) -> SLC_TX_LOOP_TEST_R {
        SLC_TX_LOOP_TEST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_LOOP_TEST` field"]
    #[inline(always)]
    pub fn slc_rx_loop_test(&self, n: u8) -> SLC_RX_LOOP_TEST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_LOOP_TEST_R::new(((self.bits >> (n * 16 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_loop_test_iter(&self) -> impl Iterator<Item = SLC_RX_LOOP_TEST_R> + '_ {
        (0..2).map(move |n| SLC_RX_LOOP_TEST_R::new(((self.bits >> (n * 16 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - SLC0_RX_LOOP_TEST"]
    #[inline(always)]
    pub fn slc0_rx_loop_test(&self) -> SLC_RX_LOOP_TEST_R {
        SLC_RX_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 21 - SLC1_RX_LOOP_TEST"]
    #[inline(always)]
    pub fn slc1_rx_loop_test(&self) -> SLC_RX_LOOP_TEST_R {
        SLC_RX_LOOP_TEST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_AUTO_WRBACK` field"]
    #[inline(always)]
    pub fn slc_rx_auto_wrback(&self, n: u8) -> SLC_RX_AUTO_WRBACK_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_AUTO_WRBACK_R::new(((self.bits >> (n * 16 + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_auto_wrback_iter(&self) -> impl Iterator<Item = SLC_RX_AUTO_WRBACK_R> + '_ {
        (0..2).map(move |n| SLC_RX_AUTO_WRBACK_R::new(((self.bits >> (n * 16 + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - SLC0_RX_AUTO_WRBACK"]
    #[inline(always)]
    pub fn slc0_rx_auto_wrback(&self) -> SLC_RX_AUTO_WRBACK_R {
        SLC_RX_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 22 - SLC1_RX_AUTO_WRBACK"]
    #[inline(always)]
    pub fn slc1_rx_auto_wrback(&self) -> SLC_RX_AUTO_WRBACK_R {
        SLC_RX_AUTO_WRBACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_NO_RESTART_CLR` field"]
    #[inline(always)]
    pub fn slc_rx_no_restart_clr(&self, n: u8) -> SLC_RX_NO_RESTART_CLR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_NO_RESTART_CLR_R::new(((self.bits >> (n * 16 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_no_restart_clr_iter(&self) -> impl Iterator<Item = SLC_RX_NO_RESTART_CLR_R> + '_ {
        (0..2).map(move |n| SLC_RX_NO_RESTART_CLR_R::new(((self.bits >> (n * 16 + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - SLC0_RX_NO_RESTART_CLR"]
    #[inline(always)]
    pub fn slc0_rx_no_restart_clr(&self) -> SLC_RX_NO_RESTART_CLR_R {
        SLC_RX_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 23 - SLC1_RX_NO_RESTART_CLR"]
    #[inline(always)]
    pub fn slc1_rx_no_restart_clr(&self) -> SLC_RX_NO_RESTART_CLR_R {
        SLC_RX_NO_RESTART_CLR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RXDSCR_BURST_EN` field"]
    #[inline(always)]
    pub fn slc_rxdscr_burst_en(&self, n: u8) -> SLC_RXDSCR_BURST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RXDSCR_BURST_EN_R::new(((self.bits >> (n * 16 + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rxdscr_burst_en_iter(&self) -> impl Iterator<Item = SLC_RXDSCR_BURST_EN_R> + '_ {
        (0..2).map(move |n| SLC_RXDSCR_BURST_EN_R::new(((self.bits >> (n * 16 + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - SLC0_RXDSCR_BURST_EN"]
    #[inline(always)]
    pub fn slc0_rxdscr_burst_en(&self) -> SLC_RXDSCR_BURST_EN_R {
        SLC_RXDSCR_BURST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - SLC1_RXDSCR_BURST_EN"]
    #[inline(always)]
    pub fn slc1_rxdscr_burst_en(&self) -> SLC_RXDSCR_BURST_EN_R {
        SLC_RXDSCR_BURST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RXDATA_BURST_EN` field"]
    #[inline(always)]
    pub fn slc_rxdata_burst_en(&self, n: u8) -> SLC_RXDATA_BURST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RXDATA_BURST_EN_R::new(((self.bits >> (n * 16 + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rxdata_burst_en_iter(&self) -> impl Iterator<Item = SLC_RXDATA_BURST_EN_R> + '_ {
        (0..2).map(move |n| SLC_RXDATA_BURST_EN_R::new(((self.bits >> (n * 16 + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - SLC0_RXDATA_BURST_EN"]
    #[inline(always)]
    pub fn slc0_rxdata_burst_en(&self) -> SLC_RXDATA_BURST_EN_R {
        SLC_RXDATA_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 25 - SLC1_RXDATA_BURST_EN"]
    #[inline(always)]
    pub fn slc1_rxdata_burst_en(&self) -> SLC_RXDATA_BURST_EN_R {
        SLC_RXDATA_BURST_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RXLINK_AUTO_RET` field"]
    #[inline(always)]
    pub fn slc_rxlink_auto_ret(&self, n: u8) -> SLC_RXLINK_AUTO_RET_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RXLINK_AUTO_RET_R::new(((self.bits >> (n * 16 + 10)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rxlink_auto_ret_iter(&self) -> impl Iterator<Item = SLC_RXLINK_AUTO_RET_R> + '_ {
        (0..2).map(move |n| SLC_RXLINK_AUTO_RET_R::new(((self.bits >> (n * 16 + 10)) & 1) != 0))
    }
    #[doc = "Bit 10 - SLC0_RXLINK_AUTO_RET"]
    #[inline(always)]
    pub fn slc0_rxlink_auto_ret(&self) -> SLC_RXLINK_AUTO_RET_R {
        SLC_RXLINK_AUTO_RET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 26 - SLC1_RXLINK_AUTO_RET"]
    #[inline(always)]
    pub fn slc1_rxlink_auto_ret(&self) -> SLC_RXLINK_AUTO_RET_R {
        SLC_RXLINK_AUTO_RET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TXLINK_AUTO_RET` field"]
    #[inline(always)]
    pub fn slc_txlink_auto_ret(&self, n: u8) -> SLC_TXLINK_AUTO_RET_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TXLINK_AUTO_RET_R::new(((self.bits >> (n * 16 + 11)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_txlink_auto_ret_iter(&self) -> impl Iterator<Item = SLC_TXLINK_AUTO_RET_R> + '_ {
        (0..2).map(move |n| SLC_TXLINK_AUTO_RET_R::new(((self.bits >> (n * 16 + 11)) & 1) != 0))
    }
    #[doc = "Bit 11 - SLC0_TXLINK_AUTO_RET"]
    #[inline(always)]
    pub fn slc0_txlink_auto_ret(&self) -> SLC_TXLINK_AUTO_RET_R {
        SLC_TXLINK_AUTO_RET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 27 - SLC1_TXLINK_AUTO_RET"]
    #[inline(always)]
    pub fn slc1_txlink_auto_ret(&self) -> SLC_TXLINK_AUTO_RET_R {
        SLC_TXLINK_AUTO_RET_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TXDSCR_BURST_EN` field"]
    #[inline(always)]
    pub fn slc_txdscr_burst_en(&self, n: u8) -> SLC_TXDSCR_BURST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TXDSCR_BURST_EN_R::new(((self.bits >> (n * 16 + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_txdscr_burst_en_iter(&self) -> impl Iterator<Item = SLC_TXDSCR_BURST_EN_R> + '_ {
        (0..2).map(move |n| SLC_TXDSCR_BURST_EN_R::new(((self.bits >> (n * 16 + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - SLC0_TXDSCR_BURST_EN"]
    #[inline(always)]
    pub fn slc0_txdscr_burst_en(&self) -> SLC_TXDSCR_BURST_EN_R {
        SLC_TXDSCR_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 28 - SLC1_TXDSCR_BURST_EN"]
    #[inline(always)]
    pub fn slc1_txdscr_burst_en(&self) -> SLC_TXDSCR_BURST_EN_R {
        SLC_TXDSCR_BURST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TXDATA_BURST_EN` field"]
    #[inline(always)]
    pub fn slc_txdata_burst_en(&self, n: u8) -> SLC_TXDATA_BURST_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TXDATA_BURST_EN_R::new(((self.bits >> (n * 16 + 13)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_txdata_burst_en_iter(&self) -> impl Iterator<Item = SLC_TXDATA_BURST_EN_R> + '_ {
        (0..2).map(move |n| SLC_TXDATA_BURST_EN_R::new(((self.bits >> (n * 16 + 13)) & 1) != 0))
    }
    #[doc = "Bit 13 - SLC0_TXDATA_BURST_EN"]
    #[inline(always)]
    pub fn slc0_txdata_burst_en(&self) -> SLC_TXDATA_BURST_EN_R {
        SLC_TXDATA_BURST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 29 - SLC1_TXDATA_BURST_EN"]
    #[inline(always)]
    pub fn slc1_txdata_burst_en(&self) -> SLC_TXDATA_BURST_EN_R {
        SLC_TXDATA_BURST_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOKEN_AUTO_CLR` field"]
    #[inline(always)]
    pub fn slc_token_auto_clr(&self, n: u8) -> SLC_TOKEN_AUTO_CLR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOKEN_AUTO_CLR_R::new(((self.bits >> (n * 16 + 14)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_token_auto_clr_iter(&self) -> impl Iterator<Item = SLC_TOKEN_AUTO_CLR_R> + '_ {
        (0..2).map(move |n| SLC_TOKEN_AUTO_CLR_R::new(((self.bits >> (n * 16 + 14)) & 1) != 0))
    }
    #[doc = "Bit 14 - SLC0_TOKEN_AUTO_CLR"]
    #[inline(always)]
    pub fn slc0_token_auto_clr(&self) -> SLC_TOKEN_AUTO_CLR_R {
        SLC_TOKEN_AUTO_CLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 30 - SLC1_TOKEN_AUTO_CLR"]
    #[inline(always)]
    pub fn slc1_token_auto_clr(&self) -> SLC_TOKEN_AUTO_CLR_R {
        SLC_TOKEN_AUTO_CLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOKEN_SEL` field"]
    #[inline(always)]
    pub fn slc_token_sel(&self, n: u8) -> SLC_TOKEN_SEL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOKEN_SEL_R::new(((self.bits >> (n * 16 + 15)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_token_sel_iter(&self) -> impl Iterator<Item = SLC_TOKEN_SEL_R> + '_ {
        (0..2).map(move |n| SLC_TOKEN_SEL_R::new(((self.bits >> (n * 16 + 15)) & 1) != 0))
    }
    #[doc = "Bit 15 - SLC0_TOKEN_SEL"]
    #[inline(always)]
    pub fn slc0_token_sel(&self) -> SLC_TOKEN_SEL_R {
        SLC_TOKEN_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - SLC1_TOKEN_SEL"]
    #[inline(always)]
    pub fn slc1_token_sel(&self) -> SLC_TOKEN_SEL_R {
        SLC_TOKEN_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_WR_RETRY_MASK_EN` field"]
    #[inline(always)]
    pub fn slc_wr_retry_mask_en(&self, n: u8) -> SLC_WR_RETRY_MASK_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_WR_RETRY_MASK_EN_R::new(((self.bits >> (n + 18)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_wr_retry_mask_en_iter(&self) -> impl Iterator<Item = SLC_WR_RETRY_MASK_EN_R> + '_ {
        (0..2).map(move |n| SLC_WR_RETRY_MASK_EN_R::new(((self.bits >> (n + 18)) & 1) != 0))
    }
    #[doc = "Bit 18 - SLC0_WR_RETRY_MASK_EN"]
    #[inline(always)]
    pub fn slc0_wr_retry_mask_en(&self) -> SLC_WR_RETRY_MASK_EN_R {
        SLC_WR_RETRY_MASK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SLC1_WR_RETRY_MASK_EN"]
    #[inline(always)]
    pub fn slc1_wr_retry_mask_en(&self) -> SLC_WR_RETRY_MASK_EN_R {
        SLC_WR_RETRY_MASK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("slc0_tx_rst", &format_args!("{}", self.slc0_tx_rst().bit()))
            .field("slc1_tx_rst", &format_args!("{}", self.slc1_tx_rst().bit()))
            .field("slc0_rx_rst", &format_args!("{}", self.slc0_rx_rst().bit()))
            .field("slc1_rx_rst", &format_args!("{}", self.slc1_rx_rst().bit()))
            .field(
                "ahbm_fifo_rst",
                &format_args!("{}", self.ahbm_fifo_rst().bit()),
            )
            .field("ahbm_rst", &format_args!("{}", self.ahbm_rst().bit()))
            .field(
                "slc0_tx_loop_test",
                &format_args!("{}", self.slc0_tx_loop_test().bit()),
            )
            .field(
                "slc1_tx_loop_test",
                &format_args!("{}", self.slc1_tx_loop_test().bit()),
            )
            .field(
                "slc0_rx_loop_test",
                &format_args!("{}", self.slc0_rx_loop_test().bit()),
            )
            .field(
                "slc1_rx_loop_test",
                &format_args!("{}", self.slc1_rx_loop_test().bit()),
            )
            .field(
                "slc0_rx_auto_wrback",
                &format_args!("{}", self.slc0_rx_auto_wrback().bit()),
            )
            .field(
                "slc1_rx_auto_wrback",
                &format_args!("{}", self.slc1_rx_auto_wrback().bit()),
            )
            .field(
                "slc0_rx_no_restart_clr",
                &format_args!("{}", self.slc0_rx_no_restart_clr().bit()),
            )
            .field(
                "slc1_rx_no_restart_clr",
                &format_args!("{}", self.slc1_rx_no_restart_clr().bit()),
            )
            .field(
                "slc0_rxdscr_burst_en",
                &format_args!("{}", self.slc0_rxdscr_burst_en().bit()),
            )
            .field(
                "slc1_rxdscr_burst_en",
                &format_args!("{}", self.slc1_rxdscr_burst_en().bit()),
            )
            .field(
                "slc0_rxdata_burst_en",
                &format_args!("{}", self.slc0_rxdata_burst_en().bit()),
            )
            .field(
                "slc1_rxdata_burst_en",
                &format_args!("{}", self.slc1_rxdata_burst_en().bit()),
            )
            .field(
                "slc0_rxlink_auto_ret",
                &format_args!("{}", self.slc0_rxlink_auto_ret().bit()),
            )
            .field(
                "slc1_rxlink_auto_ret",
                &format_args!("{}", self.slc1_rxlink_auto_ret().bit()),
            )
            .field(
                "slc0_txlink_auto_ret",
                &format_args!("{}", self.slc0_txlink_auto_ret().bit()),
            )
            .field(
                "slc1_txlink_auto_ret",
                &format_args!("{}", self.slc1_txlink_auto_ret().bit()),
            )
            .field(
                "slc0_txdscr_burst_en",
                &format_args!("{}", self.slc0_txdscr_burst_en().bit()),
            )
            .field(
                "slc1_txdscr_burst_en",
                &format_args!("{}", self.slc1_txdscr_burst_en().bit()),
            )
            .field(
                "slc0_txdata_burst_en",
                &format_args!("{}", self.slc0_txdata_burst_en().bit()),
            )
            .field(
                "slc1_txdata_burst_en",
                &format_args!("{}", self.slc1_txdata_burst_en().bit()),
            )
            .field(
                "slc0_token_auto_clr",
                &format_args!("{}", self.slc0_token_auto_clr().bit()),
            )
            .field(
                "slc1_token_auto_clr",
                &format_args!("{}", self.slc1_token_auto_clr().bit()),
            )
            .field(
                "slc0_token_sel",
                &format_args!("{}", self.slc0_token_sel().bit()),
            )
            .field(
                "slc1_token_sel",
                &format_args!("{}", self.slc1_token_sel().bit()),
            )
            .field(
                "slc0_wr_retry_mask_en",
                &format_args!("{}", self.slc0_wr_retry_mask_en().bit()),
            )
            .field(
                "slc1_wr_retry_mask_en",
                &format_args!("{}", self.slc1_wr_retry_mask_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_RST` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_rst(&mut self, n: u8) -> SLC_TX_RST_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_RST_W::new(self, n * 16)
    }
    #[doc = "Bit 0 - SLC0_TX_RST"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_rst(&mut self) -> SLC_TX_RST_W<CONF0_SPEC> {
        SLC_TX_RST_W::new(self, 0)
    }
    #[doc = "Bit 16 - SLC1_TX_RST"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_rst(&mut self) -> SLC_TX_RST_W<CONF0_SPEC> {
        SLC_TX_RST_W::new(self, 16)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_RST` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_rst(&mut self, n: u8) -> SLC_RX_RST_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_RST_W::new(self, n * 16 + 1)
    }
    #[doc = "Bit 1 - SLC0_RX_RST"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_rst(&mut self) -> SLC_RX_RST_W<CONF0_SPEC> {
        SLC_RX_RST_W::new(self, 1)
    }
    #[doc = "Bit 17 - SLC1_RX_RST"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_rst(&mut self) -> SLC_RX_RST_W<CONF0_SPEC> {
        SLC_RX_RST_W::new(self, 17)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<CONF0_SPEC> {
        AHBM_FIFO_RST_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<CONF0_SPEC> {
        AHBM_RST_W::new(self, 3)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TX_LOOP_TEST` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tx_loop_test(&mut self, n: u8) -> SLC_TX_LOOP_TEST_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TX_LOOP_TEST_W::new(self, n * 16 + 4)
    }
    #[doc = "Bit 4 - SLC0_TX_LOOP_TEST"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_loop_test(&mut self) -> SLC_TX_LOOP_TEST_W<CONF0_SPEC> {
        SLC_TX_LOOP_TEST_W::new(self, 4)
    }
    #[doc = "Bit 20 - SLC1_TX_LOOP_TEST"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tx_loop_test(&mut self) -> SLC_TX_LOOP_TEST_W<CONF0_SPEC> {
        SLC_TX_LOOP_TEST_W::new(self, 20)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_LOOP_TEST` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_loop_test(&mut self, n: u8) -> SLC_RX_LOOP_TEST_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_LOOP_TEST_W::new(self, n * 16 + 5)
    }
    #[doc = "Bit 5 - SLC0_RX_LOOP_TEST"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_loop_test(&mut self) -> SLC_RX_LOOP_TEST_W<CONF0_SPEC> {
        SLC_RX_LOOP_TEST_W::new(self, 5)
    }
    #[doc = "Bit 21 - SLC1_RX_LOOP_TEST"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_loop_test(&mut self) -> SLC_RX_LOOP_TEST_W<CONF0_SPEC> {
        SLC_RX_LOOP_TEST_W::new(self, 21)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_AUTO_WRBACK` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_auto_wrback(&mut self, n: u8) -> SLC_RX_AUTO_WRBACK_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_AUTO_WRBACK_W::new(self, n * 16 + 6)
    }
    #[doc = "Bit 6 - SLC0_RX_AUTO_WRBACK"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_auto_wrback(&mut self) -> SLC_RX_AUTO_WRBACK_W<CONF0_SPEC> {
        SLC_RX_AUTO_WRBACK_W::new(self, 6)
    }
    #[doc = "Bit 22 - SLC1_RX_AUTO_WRBACK"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_auto_wrback(&mut self) -> SLC_RX_AUTO_WRBACK_W<CONF0_SPEC> {
        SLC_RX_AUTO_WRBACK_W::new(self, 22)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_NO_RESTART_CLR` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_no_restart_clr(&mut self, n: u8) -> SLC_RX_NO_RESTART_CLR_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_NO_RESTART_CLR_W::new(self, n * 16 + 7)
    }
    #[doc = "Bit 7 - SLC0_RX_NO_RESTART_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_no_restart_clr(&mut self) -> SLC_RX_NO_RESTART_CLR_W<CONF0_SPEC> {
        SLC_RX_NO_RESTART_CLR_W::new(self, 7)
    }
    #[doc = "Bit 23 - SLC1_RX_NO_RESTART_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_no_restart_clr(&mut self) -> SLC_RX_NO_RESTART_CLR_W<CONF0_SPEC> {
        SLC_RX_NO_RESTART_CLR_W::new(self, 23)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RXDSCR_BURST_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxdscr_burst_en(&mut self, n: u8) -> SLC_RXDSCR_BURST_EN_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RXDSCR_BURST_EN_W::new(self, n * 16 + 8)
    }
    #[doc = "Bit 8 - SLC0_RXDSCR_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxdscr_burst_en(&mut self) -> SLC_RXDSCR_BURST_EN_W<CONF0_SPEC> {
        SLC_RXDSCR_BURST_EN_W::new(self, 8)
    }
    #[doc = "Bit 24 - SLC1_RXDSCR_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxdscr_burst_en(&mut self) -> SLC_RXDSCR_BURST_EN_W<CONF0_SPEC> {
        SLC_RXDSCR_BURST_EN_W::new(self, 24)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RXDATA_BURST_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxdata_burst_en(&mut self, n: u8) -> SLC_RXDATA_BURST_EN_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RXDATA_BURST_EN_W::new(self, n * 16 + 9)
    }
    #[doc = "Bit 9 - SLC0_RXDATA_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxdata_burst_en(&mut self) -> SLC_RXDATA_BURST_EN_W<CONF0_SPEC> {
        SLC_RXDATA_BURST_EN_W::new(self, 9)
    }
    #[doc = "Bit 25 - SLC1_RXDATA_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxdata_burst_en(&mut self) -> SLC_RXDATA_BURST_EN_W<CONF0_SPEC> {
        SLC_RXDATA_BURST_EN_W::new(self, 25)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RXLINK_AUTO_RET` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rxlink_auto_ret(&mut self, n: u8) -> SLC_RXLINK_AUTO_RET_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RXLINK_AUTO_RET_W::new(self, n * 16 + 10)
    }
    #[doc = "Bit 10 - SLC0_RXLINK_AUTO_RET"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxlink_auto_ret(&mut self) -> SLC_RXLINK_AUTO_RET_W<CONF0_SPEC> {
        SLC_RXLINK_AUTO_RET_W::new(self, 10)
    }
    #[doc = "Bit 26 - SLC1_RXLINK_AUTO_RET"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxlink_auto_ret(&mut self) -> SLC_RXLINK_AUTO_RET_W<CONF0_SPEC> {
        SLC_RXLINK_AUTO_RET_W::new(self, 26)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TXLINK_AUTO_RET` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txlink_auto_ret(&mut self, n: u8) -> SLC_TXLINK_AUTO_RET_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TXLINK_AUTO_RET_W::new(self, n * 16 + 11)
    }
    #[doc = "Bit 11 - SLC0_TXLINK_AUTO_RET"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txlink_auto_ret(&mut self) -> SLC_TXLINK_AUTO_RET_W<CONF0_SPEC> {
        SLC_TXLINK_AUTO_RET_W::new(self, 11)
    }
    #[doc = "Bit 27 - SLC1_TXLINK_AUTO_RET"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txlink_auto_ret(&mut self) -> SLC_TXLINK_AUTO_RET_W<CONF0_SPEC> {
        SLC_TXLINK_AUTO_RET_W::new(self, 27)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TXDSCR_BURST_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txdscr_burst_en(&mut self, n: u8) -> SLC_TXDSCR_BURST_EN_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TXDSCR_BURST_EN_W::new(self, n * 16 + 12)
    }
    #[doc = "Bit 12 - SLC0_TXDSCR_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txdscr_burst_en(&mut self) -> SLC_TXDSCR_BURST_EN_W<CONF0_SPEC> {
        SLC_TXDSCR_BURST_EN_W::new(self, 12)
    }
    #[doc = "Bit 28 - SLC1_TXDSCR_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txdscr_burst_en(&mut self) -> SLC_TXDSCR_BURST_EN_W<CONF0_SPEC> {
        SLC_TXDSCR_BURST_EN_W::new(self, 28)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TXDATA_BURST_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txdata_burst_en(&mut self, n: u8) -> SLC_TXDATA_BURST_EN_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TXDATA_BURST_EN_W::new(self, n * 16 + 13)
    }
    #[doc = "Bit 13 - SLC0_TXDATA_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txdata_burst_en(&mut self) -> SLC_TXDATA_BURST_EN_W<CONF0_SPEC> {
        SLC_TXDATA_BURST_EN_W::new(self, 13)
    }
    #[doc = "Bit 29 - SLC1_TXDATA_BURST_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txdata_burst_en(&mut self) -> SLC_TXDATA_BURST_EN_W<CONF0_SPEC> {
        SLC_TXDATA_BURST_EN_W::new(self, 29)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOKEN_AUTO_CLR` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token_auto_clr(&mut self, n: u8) -> SLC_TOKEN_AUTO_CLR_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOKEN_AUTO_CLR_W::new(self, n * 16 + 14)
    }
    #[doc = "Bit 14 - SLC0_TOKEN_AUTO_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token_auto_clr(&mut self) -> SLC_TOKEN_AUTO_CLR_W<CONF0_SPEC> {
        SLC_TOKEN_AUTO_CLR_W::new(self, 14)
    }
    #[doc = "Bit 30 - SLC1_TOKEN_AUTO_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token_auto_clr(&mut self) -> SLC_TOKEN_AUTO_CLR_W<CONF0_SPEC> {
        SLC_TOKEN_AUTO_CLR_W::new(self, 30)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOKEN_SEL` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token_sel(&mut self, n: u8) -> SLC_TOKEN_SEL_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOKEN_SEL_W::new(self, n * 16 + 15)
    }
    #[doc = "Bit 15 - SLC0_TOKEN_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token_sel(&mut self) -> SLC_TOKEN_SEL_W<CONF0_SPEC> {
        SLC_TOKEN_SEL_W::new(self, 15)
    }
    #[doc = "Bit 31 - SLC1_TOKEN_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token_sel(&mut self) -> SLC_TOKEN_SEL_W<CONF0_SPEC> {
        SLC_TOKEN_SEL_W::new(self, 31)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_WR_RETRY_MASK_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_wr_retry_mask_en(&mut self, n: u8) -> SLC_WR_RETRY_MASK_EN_W<CONF0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_WR_RETRY_MASK_EN_W::new(self, n + 18)
    }
    #[doc = "Bit 18 - SLC0_WR_RETRY_MASK_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_wr_retry_mask_en(&mut self) -> SLC_WR_RETRY_MASK_EN_W<CONF0_SPEC> {
        SLC_WR_RETRY_MASK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - SLC1_WR_RETRY_MASK_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_wr_retry_mask_en(&mut self) -> SLC_WR_RETRY_MASK_EN_W<CONF0_SPEC> {
        SLC_WR_RETRY_MASK_EN_W::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0xff3c_ff30"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0xff3c_ff30;
}
