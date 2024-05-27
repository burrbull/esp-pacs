///Register `OUTFIFO_STATUS1_CH%s` reader
pub type R = crate::R<OUTFIFO_STATUS1_CH_SPEC>;
///Field `L1OUTFIFO_CNT` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
pub type L1OUTFIFO_CNT_R = crate::FieldReader;
///Field `L2OUTFIFO_CNT` reader - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 0.
pub type L2OUTFIFO_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn l1outfifo_cnt(&self) -> L1OUTFIFO_CNT_R {
        L1OUTFIFO_CNT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:9 - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn l2outfifo_cnt(&self) -> L2OUTFIFO_CNT_R {
        L2OUTFIFO_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS1_CH")
            .field("l1outfifo_cnt", &self.l1outfifo_cnt())
            .field("l2outfifo_cnt", &self.l2outfifo_cnt())
            .finish()
    }
}
/**Receive FIFO status of Tx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status1_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUTFIFO_STATUS1_CH_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS1_CH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`outfifo_status1_ch::R`](R) reader structure
impl crate::Readable for OUTFIFO_STATUS1_CH_SPEC {}
///`reset()` method sets OUTFIFO_STATUS1_CH%s to value 0
impl crate::Resettable for OUTFIFO_STATUS1_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
