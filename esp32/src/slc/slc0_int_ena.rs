#[doc = "Register `SLC0_INT_ENA` reader"]
pub type R = crate::R<SLC0_INT_ENA_SPEC>;
#[doc = "Register `SLC0_INT_ENA` writer"]
pub type W = crate::W<SLC0_INT_ENA_SPEC>;
#[doc = "Field `FRHOST_BIT(0-7)` reader - "]
pub type FRHOST_BIT_R = crate::BitReader;
#[doc = "Field `FRHOST_BIT(0-7)` writer - "]
pub type FRHOST_BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` reader - "]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - "]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` reader - "]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UDF` reader - "]
pub type RX_UDF_R = crate::BitReader;
#[doc = "Field `RX_UDF` writer - "]
pub type RX_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OVF` reader - "]
pub type TX_OVF_R = crate::BitReader;
#[doc = "Field `TX_OVF` writer - "]
pub type TX_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOKEN0_1TO0` reader - "]
pub type TOKEN0_1TO0_R = crate::BitReader;
#[doc = "Field `TOKEN0_1TO0` writer - "]
pub type TOKEN0_1TO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOKEN1_1TO0` reader - "]
pub type TOKEN1_1TO0_R = crate::BitReader;
#[doc = "Field `TOKEN1_1TO0` writer - "]
pub type TOKEN1_1TO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` reader - "]
pub type TX_DONE_R = crate::BitReader;
#[doc = "Field `TX_DONE` writer - "]
pub type TX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SUC_EOF` reader - "]
pub type TX_SUC_EOF_R = crate::BitReader;
#[doc = "Field `TX_SUC_EOF` writer - "]
pub type TX_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DONE` reader - "]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `RX_DONE` writer - "]
pub type RX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF` reader - "]
pub type RX_EOF_R = crate::BitReader;
#[doc = "Field `RX_EOF` writer - "]
pub type RX_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOHOST` reader - "]
pub type TOHOST_R = crate::BitReader;
#[doc = "Field `TOHOST` writer - "]
pub type TOHOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DSCR_ERR` reader - "]
pub type TX_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `TX_DSCR_ERR` writer - "]
pub type TX_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DSCR_ERR` reader - "]
pub type RX_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `RX_DSCR_ERR` writer - "]
pub type RX_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DSCR_EMPTY` reader - "]
pub type TX_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_DSCR_EMPTY` writer - "]
pub type TX_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_RD_ACK` reader - "]
pub type HOST_RD_ACK_R = crate::BitReader;
#[doc = "Field `HOST_RD_ACK` writer - "]
pub type HOST_RD_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_RETRY_DONE` reader - "]
pub type WR_RETRY_DONE_R = crate::BitReader;
#[doc = "Field `WR_RETRY_DONE` writer - "]
pub type WR_RETRY_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ERR_EOF` reader - "]
pub type TX_ERR_EOF_R = crate::BitReader;
#[doc = "Field `TX_ERR_EOF` writer - "]
pub type TX_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_DTC` reader - "]
pub type CMD_DTC_R = crate::BitReader;
#[doc = "Field `CMD_DTC` writer - "]
pub type CMD_DTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_QUICK_EOF` reader - "]
pub type RX_QUICK_EOF_R = crate::BitReader;
#[doc = "Field `RX_QUICK_EOF` writer - "]
pub type RX_QUICK_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("SLC0_INT_ENA")
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
impl core::fmt::Debug for crate::generic::Reg<SLC0_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FRHOST_BIT0` field"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit(&mut self, n: u8) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        FRHOST_BIT_W::new(self, n)
    }
    #[doc = "Bit 0 - FRHOST_BIT0"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit0(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - FRHOST_BIT1"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit1(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 1)
    }
    #[doc = "Bit 2 - FRHOST_BIT2"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit2(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - FRHOST_BIT3"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit3(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 3)
    }
    #[doc = "Bit 4 - FRHOST_BIT4"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit4(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 4)
    }
    #[doc = "Bit 5 - FRHOST_BIT5"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit5(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 5)
    }
    #[doc = "Bit 6 - FRHOST_BIT6"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit6(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 6)
    }
    #[doc = "Bit 7 - FRHOST_BIT7"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit7(&mut self) -> FRHOST_BIT_W<SLC0_INT_ENA_SPEC> {
        FRHOST_BIT_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<SLC0_INT_ENA_SPEC> {
        RX_START_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<SLC0_INT_ENA_SPEC> {
        TX_START_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rx_udf(&mut self) -> RX_UDF_W<SLC0_INT_ENA_SPEC> {
        RX_UDF_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ovf(&mut self) -> TX_OVF_W<SLC0_INT_ENA_SPEC> {
        TX_OVF_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn token0_1to0(&mut self) -> TOKEN0_1TO0_W<SLC0_INT_ENA_SPEC> {
        TOKEN0_1TO0_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn token1_1to0(&mut self) -> TOKEN1_1TO0_W<SLC0_INT_ENA_SPEC> {
        TOKEN1_1TO0_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<SLC0_INT_ENA_SPEC> {
        TX_DONE_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_suc_eof(&mut self) -> TX_SUC_EOF_W<SLC0_INT_ENA_SPEC> {
        TX_SUC_EOF_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<SLC0_INT_ENA_SPEC> {
        RX_DONE_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof(&mut self) -> RX_EOF_W<SLC0_INT_ENA_SPEC> {
        RX_EOF_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tohost(&mut self) -> TOHOST_W<SLC0_INT_ENA_SPEC> {
        TOHOST_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dscr_err(&mut self) -> TX_DSCR_ERR_W<SLC0_INT_ENA_SPEC> {
        TX_DSCR_ERR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dscr_err(&mut self) -> RX_DSCR_ERR_W<SLC0_INT_ENA_SPEC> {
        RX_DSCR_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dscr_empty(&mut self) -> TX_DSCR_EMPTY_W<SLC0_INT_ENA_SPEC> {
        TX_DSCR_EMPTY_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn host_rd_ack(&mut self) -> HOST_RD_ACK_W<SLC0_INT_ENA_SPEC> {
        HOST_RD_ACK_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wr_retry_done(&mut self) -> WR_RETRY_DONE_W<SLC0_INT_ENA_SPEC> {
        WR_RETRY_DONE_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tx_err_eof(&mut self) -> TX_ERR_EOF_W<SLC0_INT_ENA_SPEC> {
        TX_ERR_EOF_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_dtc(&mut self) -> CMD_DTC_W<SLC0_INT_ENA_SPEC> {
        CMD_DTC_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rx_quick_eof(&mut self) -> RX_QUICK_EOF_W<SLC0_INT_ENA_SPEC> {
        RX_QUICK_EOF_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_INT_ENA_SPEC;
impl crate::RegisterSpec for SLC0_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_int_ena::R`](R) reader structure"]
impl crate::Readable for SLC0_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_int_ena::W`](W) writer structure"]
impl crate::Writable for SLC0_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_INT_ENA to value 0"]
impl crate::Resettable for SLC0_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
