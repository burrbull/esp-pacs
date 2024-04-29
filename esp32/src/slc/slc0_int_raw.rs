#[doc = "Register `SLC0_INT_RAW` reader"]
pub type R = crate::R<SLC0_INT_RAW_SPEC>;
#[doc = "Field `FRHOST_BIT(0-7)` reader - "]
pub type FRHOST_BIT_R = crate::BitReader;
#[doc = "Field `RX_START` reader - "]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `TX_START` reader - "]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `RX_UDF` reader - "]
pub type RX_UDF_R = crate::BitReader;
#[doc = "Field `TX_OVF` reader - "]
pub type TX_OVF_R = crate::BitReader;
#[doc = "Field `TOKEN0_1TO0` reader - "]
pub type TOKEN0_1TO0_R = crate::BitReader;
#[doc = "Field `TOKEN1_1TO0` reader - "]
pub type TOKEN1_1TO0_R = crate::BitReader;
#[doc = "Field `TX_DONE` reader - "]
pub type TX_DONE_R = crate::BitReader;
#[doc = "Field `TX_SUC_EOF` reader - "]
pub type TX_SUC_EOF_R = crate::BitReader;
#[doc = "Field `RX_DONE` reader - "]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `RX_EOF` reader - "]
pub type RX_EOF_R = crate::BitReader;
#[doc = "Field `TOHOST` reader - "]
pub type TOHOST_R = crate::BitReader;
#[doc = "Field `TX_DSCR_ERR` reader - "]
pub type TX_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `RX_DSCR_ERR` reader - "]
pub type RX_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `TX_DSCR_EMPTY` reader - "]
pub type TX_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `HOST_RD_ACK` reader - "]
pub type HOST_RD_ACK_R = crate::BitReader;
#[doc = "Field `WR_RETRY_DONE` reader - "]
pub type WR_RETRY_DONE_R = crate::BitReader;
#[doc = "Field `TX_ERR_EOF` reader - "]
pub type TX_ERR_EOF_R = crate::BitReader;
#[doc = "Field `CMD_DTC` reader - "]
pub type CMD_DTC_R = crate::BitReader;
#[doc = "Field `RX_QUICK_EOF` reader - "]
pub type RX_QUICK_EOF_R = crate::BitReader;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FRHOST_BIT0` field"]
    #[inline(always)]
    pub fn frhost_bit(&self, n: u8) -> FRHOST_BIT_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        FRHOST_BIT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn frhost_bit_iter(&self) -> impl Iterator<Item = FRHOST_BIT_R> + '_ {
        (0..8).map(move |n| FRHOST_BIT_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - FRHOST_BIT0"]
    #[inline(always)]
    pub fn frhost_bit0(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FRHOST_BIT1"]
    #[inline(always)]
    pub fn frhost_bit1(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRHOST_BIT2"]
    #[inline(always)]
    pub fn frhost_bit2(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FRHOST_BIT3"]
    #[inline(always)]
    pub fn frhost_bit3(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FRHOST_BIT4"]
    #[inline(always)]
    pub fn frhost_bit4(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FRHOST_BIT5"]
    #[inline(always)]
    pub fn frhost_bit5(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FRHOST_BIT6"]
    #[inline(always)]
    pub fn frhost_bit6(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FRHOST_BIT7"]
    #[inline(always)]
    pub fn frhost_bit7(&self) -> FRHOST_BIT_R {
        FRHOST_BIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rx_udf(&self) -> RX_UDF_R {
        RX_UDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_ovf(&self) -> TX_OVF_R {
        TX_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn token0_1to0(&self) -> TOKEN0_1TO0_R {
        TOKEN0_1TO0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn token1_1to0(&self) -> TOKEN1_1TO0_R {
        TOKEN1_1TO0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_suc_eof(&self) -> TX_SUC_EOF_R {
        TX_SUC_EOF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_eof(&self) -> RX_EOF_R {
        RX_EOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tohost(&self) -> TOHOST_R {
        TOHOST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_dscr_err(&self) -> TX_DSCR_ERR_R {
        TX_DSCR_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_dscr_err(&self) -> RX_DSCR_ERR_R {
        RX_DSCR_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tx_dscr_empty(&self) -> TX_DSCR_EMPTY_R {
        TX_DSCR_EMPTY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_rd_ack(&self) -> HOST_RD_ACK_R {
        HOST_RD_ACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_retry_done(&self) -> WR_RETRY_DONE_R {
        WR_RETRY_DONE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_err_eof(&self) -> TX_ERR_EOF_R {
        TX_ERR_EOF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmd_dtc(&self) -> CMD_DTC_R {
        CMD_DTC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_quick_eof(&self) -> RX_QUICK_EOF_R {
        RX_QUICK_EOF_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_INT_RAW")
            .field("frhost_bit0", &format_args!("{}", self.frhost_bit0().bit()))
            .field("frhost_bit1", &format_args!("{}", self.frhost_bit1().bit()))
            .field("frhost_bit2", &format_args!("{}", self.frhost_bit2().bit()))
            .field("frhost_bit3", &format_args!("{}", self.frhost_bit3().bit()))
            .field("frhost_bit4", &format_args!("{}", self.frhost_bit4().bit()))
            .field("frhost_bit5", &format_args!("{}", self.frhost_bit5().bit()))
            .field("frhost_bit6", &format_args!("{}", self.frhost_bit6().bit()))
            .field("frhost_bit7", &format_args!("{}", self.frhost_bit7().bit()))
            .field("rx_start", &format_args!("{}", self.rx_start().bit()))
            .field("tx_start", &format_args!("{}", self.tx_start().bit()))
            .field("rx_udf", &format_args!("{}", self.rx_udf().bit()))
            .field("tx_ovf", &format_args!("{}", self.tx_ovf().bit()))
            .field("token0_1to0", &format_args!("{}", self.token0_1to0().bit()))
            .field("token1_1to0", &format_args!("{}", self.token1_1to0().bit()))
            .field("tx_done", &format_args!("{}", self.tx_done().bit()))
            .field("tx_suc_eof", &format_args!("{}", self.tx_suc_eof().bit()))
            .field("rx_done", &format_args!("{}", self.rx_done().bit()))
            .field("rx_eof", &format_args!("{}", self.rx_eof().bit()))
            .field("tohost", &format_args!("{}", self.tohost().bit()))
            .field("tx_dscr_err", &format_args!("{}", self.tx_dscr_err().bit()))
            .field("rx_dscr_err", &format_args!("{}", self.rx_dscr_err().bit()))
            .field(
                "tx_dscr_empty",
                &format_args!("{}", self.tx_dscr_empty().bit()),
            )
            .field("host_rd_ack", &format_args!("{}", self.host_rd_ack().bit()))
            .field(
                "wr_retry_done",
                &format_args!("{}", self.wr_retry_done().bit()),
            )
            .field("tx_err_eof", &format_args!("{}", self.tx_err_eof().bit()))
            .field("cmd_dtc", &format_args!("{}", self.cmd_dtc().bit()))
            .field(
                "rx_quick_eof",
                &format_args!("{}", self.rx_quick_eof().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_INT_RAW_SPEC;
impl crate::RegisterSpec for SLC0_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_int_raw::R`](R) reader structure"]
impl crate::Readable for SLC0_INT_RAW_SPEC {}
#[doc = "`reset()` method sets SLC0_INT_RAW to value 0"]
impl crate::Resettable for SLC0_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
