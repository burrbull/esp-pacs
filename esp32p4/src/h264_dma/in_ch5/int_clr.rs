///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt.
pub type IN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `INFIFO_OVF_L1` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
pub type INFIFO_OVF_L1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `INFIFO_UDF_L1` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
pub type INFIFO_UDF_L1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `FETCH_MB_COL_CNT_OVF` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
pub type FETCH_MB_COL_CNT_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_CLR_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_CLR_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<INT_CLR_SPEC> {
        INFIFO_OVF_L1_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<INT_CLR_SPEC> {
        INFIFO_UDF_L1_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn fetch_mb_col_cnt_ovf(&mut self) -> FETCH_MB_COL_CNT_OVF_W<INT_CLR_SPEC> {
        FETCH_MB_COL_CNT_OVF_W::new(self, 4)
    }
}
/**RX CH5 interrupt clr register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
