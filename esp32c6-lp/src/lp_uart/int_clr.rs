#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RXFIFO_FULL_INT_CLR` writer - Set this bit to clear the rxfifo_full_int_raw interrupt."]
pub type RXFIFO_FULL_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFIFO_EMPTY_INT_CLR` writer - Set this bit to clear txfifo_empty_int_raw interrupt."]
pub type TXFIFO_EMPTY_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PARITY_ERR_INT_CLR` writer - Set this bit to clear parity_err_int_raw interrupt."]
pub type PARITY_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FRM_ERR_INT_CLR` writer - Set this bit to clear frm_err_int_raw interrupt."]
pub type FRM_ERR_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - Set this bit to clear rxfifo_ovf_int_raw interrupt."]
pub type RXFIFO_OVF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSR_CHG_INT_CLR` writer - Set this bit to clear the dsr_chg_int_raw interrupt."]
pub type DSR_CHG_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTS_CHG_INT_CLR` writer - Set this bit to clear the cts_chg_int_raw interrupt."]
pub type CTS_CHG_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRK_DET_INT_CLR` writer - Set this bit to clear the brk_det_int_raw interrupt."]
pub type BRK_DET_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFIFO_TOUT_INT_CLR` writer - Set this bit to clear the rxfifo_tout_int_raw interrupt."]
pub type RXFIFO_TOUT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SW_XON_INT_CLR` writer - Set this bit to clear the sw_xon_int_raw interrupt."]
pub type SW_XON_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SW_XOFF_INT_CLR` writer - Set this bit to clear the sw_xoff_int_raw interrupt."]
pub type SW_XOFF_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GLITCH_DET_INT_CLR` writer - Set this bit to clear the glitch_det_int_raw interrupt."]
pub type GLITCH_DET_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_BRK_DONE_INT_CLR` writer - Set this bit to clear the tx_brk_done_int_raw interrupt.."]
pub type TX_BRK_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_CLR` writer - Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
pub type TX_BRK_IDLE_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_DONE_INT_CLR` writer - Set this bit to clear the tx_done_int_raw interrupt."]
pub type TX_DONE_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AT_CMD_CHAR_DET_INT_CLR` writer - Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
pub type AT_CMD_CHAR_DET_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUP_INT_CLR` writer - Set this bit to clear the uart_wakeup_int_raw interrupt."]
pub type WAKEUP_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the rxfifo_full_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_int_clr(&mut self) -> RXFIFO_FULL_INT_CLR_W<INT_CLR_SPEC, 0> {
        RXFIFO_FULL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear txfifo_empty_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_int_clr(&mut self) -> TXFIFO_EMPTY_INT_CLR_W<INT_CLR_SPEC, 1> {
        TXFIFO_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear parity_err_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn parity_err_int_clr(&mut self) -> PARITY_ERR_INT_CLR_W<INT_CLR_SPEC, 2> {
        PARITY_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear frm_err_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frm_err_int_clr(&mut self) -> FRM_ERR_INT_CLR_W<INT_CLR_SPEC, 3> {
        FRM_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear rxfifo_ovf_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W<INT_CLR_SPEC, 4> {
        RXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the dsr_chg_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dsr_chg_int_clr(&mut self) -> DSR_CHG_INT_CLR_W<INT_CLR_SPEC, 5> {
        DSR_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the cts_chg_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cts_chg_int_clr(&mut self) -> CTS_CHG_INT_CLR_W<INT_CLR_SPEC, 6> {
        CTS_CHG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the brk_det_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn brk_det_int_clr(&mut self) -> BRK_DET_INT_CLR_W<INT_CLR_SPEC, 7> {
        BRK_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the rxfifo_tout_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_tout_int_clr(&mut self) -> RXFIFO_TOUT_INT_CLR_W<INT_CLR_SPEC, 8> {
        RXFIFO_TOUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the sw_xon_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sw_xon_int_clr(&mut self) -> SW_XON_INT_CLR_W<INT_CLR_SPEC, 9> {
        SW_XON_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the sw_xoff_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sw_xoff_int_clr(&mut self) -> SW_XOFF_INT_CLR_W<INT_CLR_SPEC, 10> {
        SW_XOFF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the glitch_det_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W<INT_CLR_SPEC, 11> {
        GLITCH_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the tx_brk_done_int_raw interrupt.."]
    #[inline(always)]
    #[must_use]
    pub fn tx_brk_done_int_clr(&mut self) -> TX_BRK_DONE_INT_CLR_W<INT_CLR_SPEC, 12> {
        TX_BRK_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_brk_idle_done_int_clr(&mut self) -> TX_BRK_IDLE_DONE_INT_CLR_W<INT_CLR_SPEC, 13> {
        TX_BRK_IDLE_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear the tx_done_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_done_int_clr(&mut self) -> TX_DONE_INT_CLR_W<INT_CLR_SPEC, 14> {
        TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn at_cmd_char_det_int_clr(&mut self) -> AT_CMD_CHAR_DET_INT_CLR_W<INT_CLR_SPEC, 18> {
        AT_CMD_CHAR_DET_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear the uart_wakeup_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_int_clr(&mut self) -> WAKEUP_INT_CLR_W<INT_CLR_SPEC, 19> {
        WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}