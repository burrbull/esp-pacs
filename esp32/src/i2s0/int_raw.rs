#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RX_TAKE_DATA_INT_RAW` reader - "]
pub type RX_TAKE_DATA_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_PUT_DATA_INT_RAW` reader - "]
pub type TX_PUT_DATA_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_WFULL_INT_RAW` reader - "]
pub type RX_WFULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_REMPTY_INT_RAW` reader - "]
pub type RX_REMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_WFULL_INT_RAW` reader - "]
pub type TX_WFULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_REMPTY_INT_RAW` reader - "]
pub type TX_REMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` reader - "]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` reader - "]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_RAW` reader - "]
pub type IN_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_RAW` reader - "]
pub type IN_SUC_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_RAW` reader - "]
pub type IN_ERR_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_RAW` reader - "]
pub type OUT_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` reader - "]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_INT_RAW` reader - "]
pub type IN_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_INT_RAW` reader - "]
pub type OUT_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_INT_RAW` reader - "]
pub type IN_DSCR_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_RAW` reader - "]
pub type OUT_TOTAL_EOF_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_take_data_int_raw(&self) -> RX_TAKE_DATA_INT_RAW_R {
        RX_TAKE_DATA_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_put_data_int_raw(&self) -> TX_PUT_DATA_INT_RAW_R {
        TX_PUT_DATA_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_wfull_int_raw(&self) -> RX_WFULL_INT_RAW_R {
        RX_WFULL_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_rempty_int_raw(&self) -> RX_REMPTY_INT_RAW_R {
        RX_REMPTY_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_wfull_int_raw(&self) -> TX_WFULL_INT_RAW_R {
        TX_WFULL_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_rempty_int_raw(&self) -> TX_REMPTY_INT_RAW_R {
        TX_REMPTY_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn in_done_int_raw(&self) -> IN_DONE_INT_RAW_R {
        IN_DONE_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&self) -> IN_SUC_EOF_INT_RAW_R {
        IN_SUC_EOF_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&self) -> IN_ERR_EOF_INT_RAW_R {
        IN_ERR_EOF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn out_done_int_raw(&self) -> OUT_DONE_INT_RAW_R {
        OUT_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn in_dscr_err_int_raw(&self) -> IN_DSCR_ERR_INT_RAW_R {
        IN_DSCR_ERR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn out_dscr_err_int_raw(&self) -> OUT_DSCR_ERR_INT_RAW_R {
        OUT_DSCR_ERR_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn in_dscr_empty_int_raw(&self) -> IN_DSCR_EMPTY_INT_RAW_R {
        IN_DSCR_EMPTY_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&self) -> OUT_TOTAL_EOF_INT_RAW_R {
        OUT_TOTAL_EOF_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rx_take_data_int_raw",
                &format_args!("{}", self.rx_take_data_int_raw().bit()),
            )
            .field(
                "tx_put_data_int_raw",
                &format_args!("{}", self.tx_put_data_int_raw().bit()),
            )
            .field(
                "rx_wfull_int_raw",
                &format_args!("{}", self.rx_wfull_int_raw().bit()),
            )
            .field(
                "rx_rempty_int_raw",
                &format_args!("{}", self.rx_rempty_int_raw().bit()),
            )
            .field(
                "tx_wfull_int_raw",
                &format_args!("{}", self.tx_wfull_int_raw().bit()),
            )
            .field(
                "tx_rempty_int_raw",
                &format_args!("{}", self.tx_rempty_int_raw().bit()),
            )
            .field(
                "rx_hung_int_raw",
                &format_args!("{}", self.rx_hung_int_raw().bit()),
            )
            .field(
                "tx_hung_int_raw",
                &format_args!("{}", self.tx_hung_int_raw().bit()),
            )
            .field(
                "in_done_int_raw",
                &format_args!("{}", self.in_done_int_raw().bit()),
            )
            .field(
                "in_suc_eof_int_raw",
                &format_args!("{}", self.in_suc_eof_int_raw().bit()),
            )
            .field(
                "in_err_eof_int_raw",
                &format_args!("{}", self.in_err_eof_int_raw().bit()),
            )
            .field(
                "out_done_int_raw",
                &format_args!("{}", self.out_done_int_raw().bit()),
            )
            .field(
                "out_eof_int_raw",
                &format_args!("{}", self.out_eof_int_raw().bit()),
            )
            .field(
                "in_dscr_err_int_raw",
                &format_args!("{}", self.in_dscr_err_int_raw().bit()),
            )
            .field(
                "out_dscr_err_int_raw",
                &format_args!("{}", self.out_dscr_err_int_raw().bit()),
            )
            .field(
                "in_dscr_empty_int_raw",
                &format_args!("{}", self.in_dscr_empty_int_raw().bit()),
            )
            .field(
                "out_total_eof_int_raw",
                &format_args!("{}", self.out_total_eof_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
