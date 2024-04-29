#[doc = "Register `SLC1_INT_CLR` writer"]
pub type W = crate::W<SLC1_INT_CLR_SPEC>;
#[doc = "Field `FRHOST_BIT(8-15)` writer - "]
pub type FRHOST_BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` writer - "]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UDF` writer - "]
pub type RX_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OVF` writer - "]
pub type TX_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOKEN0_1TO0` writer - "]
pub type TOKEN0_1TO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOKEN1_1TO0` writer - "]
pub type TOKEN1_1TO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE` writer - "]
pub type TX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SUC_EOF` writer - "]
pub type TX_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DONE` writer - "]
pub type RX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF` writer - "]
pub type RX_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOHOST` writer - "]
pub type TOHOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DSCR_ERR` writer - "]
pub type TX_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DSCR_ERR` writer - "]
pub type RX_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DSCR_EMPTY` writer - "]
pub type TX_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_RD_ACK` writer - "]
pub type HOST_RD_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_RETRY_DONE` writer - "]
pub type WR_RETRY_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ERR_EOF` writer - "]
pub type TX_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC1_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FRHOST_BIT8` field"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit(&mut self, n: u8) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        FRHOST_BIT_W::new(self, n)
    }
    #[doc = "Bit 0 - FRHOST_BIT8"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit8(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - FRHOST_BIT9"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit9(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 1)
    }
    #[doc = "Bit 2 - FRHOST_BIT10"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit10(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - FRHOST_BIT11"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit11(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 3)
    }
    #[doc = "Bit 4 - FRHOST_BIT12"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit12(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 4)
    }
    #[doc = "Bit 5 - FRHOST_BIT13"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit13(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 5)
    }
    #[doc = "Bit 6 - FRHOST_BIT14"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit14(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 6)
    }
    #[doc = "Bit 7 - FRHOST_BIT15"]
    #[inline(always)]
    #[must_use]
    pub fn frhost_bit15(&mut self) -> FRHOST_BIT_W<SLC1_INT_CLR_SPEC> {
        FRHOST_BIT_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<SLC1_INT_CLR_SPEC> {
        RX_START_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<SLC1_INT_CLR_SPEC> {
        TX_START_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rx_udf(&mut self) -> RX_UDF_W<SLC1_INT_CLR_SPEC> {
        RX_UDF_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ovf(&mut self) -> TX_OVF_W<SLC1_INT_CLR_SPEC> {
        TX_OVF_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn token0_1to0(&mut self) -> TOKEN0_1TO0_W<SLC1_INT_CLR_SPEC> {
        TOKEN0_1TO0_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn token1_1to0(&mut self) -> TOKEN1_1TO0_W<SLC1_INT_CLR_SPEC> {
        TOKEN1_1TO0_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<SLC1_INT_CLR_SPEC> {
        TX_DONE_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_suc_eof(&mut self) -> TX_SUC_EOF_W<SLC1_INT_CLR_SPEC> {
        TX_SUC_EOF_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<SLC1_INT_CLR_SPEC> {
        RX_DONE_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof(&mut self) -> RX_EOF_W<SLC1_INT_CLR_SPEC> {
        RX_EOF_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tohost(&mut self) -> TOHOST_W<SLC1_INT_CLR_SPEC> {
        TOHOST_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dscr_err(&mut self) -> TX_DSCR_ERR_W<SLC1_INT_CLR_SPEC> {
        TX_DSCR_ERR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dscr_err(&mut self) -> RX_DSCR_ERR_W<SLC1_INT_CLR_SPEC> {
        RX_DSCR_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dscr_empty(&mut self) -> TX_DSCR_EMPTY_W<SLC1_INT_CLR_SPEC> {
        TX_DSCR_EMPTY_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn host_rd_ack(&mut self) -> HOST_RD_ACK_W<SLC1_INT_CLR_SPEC> {
        HOST_RD_ACK_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wr_retry_done(&mut self) -> WR_RETRY_DONE_W<SLC1_INT_CLR_SPEC> {
        WR_RETRY_DONE_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tx_err_eof(&mut self) -> TX_ERR_EOF_W<SLC1_INT_CLR_SPEC> {
        TX_ERR_EOF_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC1_INT_CLR_SPEC;
impl crate::RegisterSpec for SLC1_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slc1_int_clr::W`](W) writer structure"]
impl crate::Writable for SLC1_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC1_INT_CLR to value 0"]
impl crate::Resettable for SLC1_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
