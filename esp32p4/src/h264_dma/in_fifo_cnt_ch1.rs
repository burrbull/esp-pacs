#[doc = "Register `IN_FIFO_CNT_CH1` reader"]
pub type R = crate::R<IN_FIFO_CNT_CH1_SPEC>;
#[doc = "Field `IN_CMDFIFO_INFIFO_CNT_CH1` reader - only for debug"]
pub type IN_CMDFIFO_INFIFO_CNT_CH1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_infifo_cnt_ch1(&self) -> IN_CMDFIFO_INFIFO_CNT_CH1_R {
        IN_CMDFIFO_INFIFO_CNT_CH1_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_FIFO_CNT_CH1")
            .field(
                "in_cmdfifo_infifo_cnt_ch1",
                &format_args!("{}", self.in_cmdfifo_infifo_cnt_ch1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_FIFO_CNT_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "rx CH1 fifo cnt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_fifo_cnt_ch1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_FIFO_CNT_CH1_SPEC;
impl crate::RegisterSpec for IN_FIFO_CNT_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_fifo_cnt_ch1::R`](R) reader structure"]
impl crate::Readable for IN_FIFO_CNT_CH1_SPEC {}
#[doc = "`reset()` method sets IN_FIFO_CNT_CH1 to value 0"]
impl crate::Resettable for IN_FIFO_CNT_CH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
