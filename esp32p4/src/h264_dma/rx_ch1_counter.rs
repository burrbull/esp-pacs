///Register `RX_CH1_COUNTER` reader
pub type R = crate::R<RX_CH1_COUNTER_SPEC>;
///Field `RX_CH1_CNT` reader - rx ch1 counter register
pub type RX_CH1_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:20 - rx ch1 counter register
    #[inline(always)]
    pub fn rx_ch1_cnt(&self) -> RX_CH1_CNT_R {
        RX_CH1_CNT_R::new(self.bits & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH1_COUNTER")
            .field("rx_ch1_cnt", &self.rx_ch1_cnt())
            .finish()
    }
}
/**rx ch1 counter register

You can [`read`](crate::generic::Reg::read) this register and get [`rx_ch1_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_CH1_COUNTER_SPEC;
impl crate::RegisterSpec for RX_CH1_COUNTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_ch1_counter::R`](R) reader structure
impl crate::Readable for RX_CH1_COUNTER_SPEC {}
///`reset()` method sets RX_CH1_COUNTER to value 0
impl crate::Resettable for RX_CH1_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
