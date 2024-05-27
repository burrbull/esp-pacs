///Register `OUTFIFO_STATUS` reader
pub type R = crate::R<OUTFIFO_STATUS_SPEC>;
///Field `OUTFIFO_FULL_L1` reader - L1 Tx FIFO full signal for Tx channel 0.
pub type OUTFIFO_FULL_L1_R = crate::BitReader;
///Field `OUTFIFO_EMPTY_L1` reader - L1 Tx FIFO empty signal for Tx channel 0.
pub type OUTFIFO_EMPTY_L1_R = crate::BitReader;
///Field `OUTFIFO_FULL_L2` reader - L2 Tx FIFO full signal for Tx channel 0.
pub type OUTFIFO_FULL_L2_R = crate::BitReader;
///Field `OUTFIFO_EMPTY_L2` reader - L2 Tx FIFO empty signal for Tx channel 0.
pub type OUTFIFO_EMPTY_L2_R = crate::BitReader;
///Field `OUTFIFO_FULL_L3` reader - L3 Tx FIFO full signal for Tx channel 0.
pub type OUTFIFO_FULL_L3_R = crate::BitReader;
///Field `OUTFIFO_EMPTY_L3` reader - L3 Tx FIFO empty signal for Tx channel 0.
pub type OUTFIFO_EMPTY_L3_R = crate::BitReader;
///Field `OUTFIFO_CNT_L1` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
pub type OUTFIFO_CNT_L1_R = crate::FieldReader;
///Field `OUTFIFO_CNT_L2` reader - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 0.
pub type OUTFIFO_CNT_L2_R = crate::FieldReader;
///Field `OUTFIFO_CNT_L3` reader - The register stores the byte number of the data in L3 Tx FIFO for Tx channel 0.
pub type OUTFIFO_CNT_L3_R = crate::FieldReader;
///Field `OUT_REMAIN_UNDER_1B_L3` reader - reserved
pub type OUT_REMAIN_UNDER_1B_L3_R = crate::BitReader;
///Field `OUT_REMAIN_UNDER_2B_L3` reader - reserved
pub type OUT_REMAIN_UNDER_2B_L3_R = crate::BitReader;
///Field `OUT_REMAIN_UNDER_3B_L3` reader - reserved
pub type OUT_REMAIN_UNDER_3B_L3_R = crate::BitReader;
///Field `OUT_REMAIN_UNDER_4B_L3` reader - reserved
pub type OUT_REMAIN_UNDER_4B_L3_R = crate::BitReader;
impl R {
    ///Bit 0 - L1 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_full_l1(&self) -> OUTFIFO_FULL_L1_R {
        OUTFIFO_FULL_L1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - L1 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_empty_l1(&self) -> OUTFIFO_EMPTY_L1_R {
        OUTFIFO_EMPTY_L1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - L2 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_full_l2(&self) -> OUTFIFO_FULL_L2_R {
        OUTFIFO_FULL_L2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - L2 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_empty_l2(&self) -> OUTFIFO_EMPTY_L2_R {
        OUTFIFO_EMPTY_L2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - L3 Tx FIFO full signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_full_l3(&self) -> OUTFIFO_FULL_L3_R {
        OUTFIFO_FULL_L3_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - L3 Tx FIFO empty signal for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_empty_l3(&self) -> OUTFIFO_EMPTY_L3_R {
        OUTFIFO_EMPTY_L3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_cnt_l1(&self) -> OUTFIFO_CNT_L1_R {
        OUTFIFO_CNT_L1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:17 - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_cnt_l2(&self) -> OUTFIFO_CNT_L2_R {
        OUTFIFO_CNT_L2_R::new(((self.bits >> 11) & 0x7f) as u8)
    }
    ///Bits 18:22 - The register stores the byte number of the data in L3 Tx FIFO for Tx channel 0.
    #[inline(always)]
    pub fn outfifo_cnt_l3(&self) -> OUTFIFO_CNT_L3_R {
        OUTFIFO_CNT_L3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - reserved
    #[inline(always)]
    pub fn out_remain_under_1b_l3(&self) -> OUT_REMAIN_UNDER_1B_L3_R {
        OUT_REMAIN_UNDER_1B_L3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - reserved
    #[inline(always)]
    pub fn out_remain_under_2b_l3(&self) -> OUT_REMAIN_UNDER_2B_L3_R {
        OUT_REMAIN_UNDER_2B_L3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - reserved
    #[inline(always)]
    pub fn out_remain_under_3b_l3(&self) -> OUT_REMAIN_UNDER_3B_L3_R {
        OUT_REMAIN_UNDER_3B_L3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - reserved
    #[inline(always)]
    pub fn out_remain_under_4b_l3(&self) -> OUT_REMAIN_UNDER_4B_L3_R {
        OUT_REMAIN_UNDER_4B_L3_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS")
            .field("outfifo_full_l1", &self.outfifo_full_l1())
            .field("outfifo_empty_l1", &self.outfifo_empty_l1())
            .field("outfifo_full_l2", &self.outfifo_full_l2())
            .field("outfifo_empty_l2", &self.outfifo_empty_l2())
            .field("outfifo_full_l3", &self.outfifo_full_l3())
            .field("outfifo_empty_l3", &self.outfifo_empty_l3())
            .field("outfifo_cnt_l1", &self.outfifo_cnt_l1())
            .field("outfifo_cnt_l2", &self.outfifo_cnt_l2())
            .field("outfifo_cnt_l3", &self.outfifo_cnt_l3())
            .field("out_remain_under_1b_l3", &self.out_remain_under_1b_l3())
            .field("out_remain_under_2b_l3", &self.out_remain_under_2b_l3())
            .field("out_remain_under_3b_l3", &self.out_remain_under_3b_l3())
            .field("out_remain_under_4b_l3", &self.out_remain_under_4b_l3())
            .finish()
    }
}
/**Transmit FIFO status of Tx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUTFIFO_STATUS_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`outfifo_status::R`](R) reader structure
impl crate::Readable for OUTFIFO_STATUS_SPEC {}
///`reset()` method sets OUTFIFO_STATUS to value 0x0780_002a
impl crate::Resettable for OUTFIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0780_002a;
}
