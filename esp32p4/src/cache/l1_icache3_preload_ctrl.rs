///Register `L1_ICACHE3_PRELOAD_CTRL` reader
pub type R = crate::R<L1_ICACHE3_PRELOAD_CTRL_SPEC>;
///Field `L1_ICACHE3_PRELOAD_ENA` reader - The bit is used to enable preload operation on L1-ICache3. It will be cleared by hardware automatically after preload operation is done.
pub type L1_ICACHE3_PRELOAD_ENA_R = crate::BitReader;
///Field `L1_ICACHE3_PRELOAD_DONE` reader - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished.
pub type L1_ICACHE3_PRELOAD_DONE_R = crate::BitReader;
///Field `L1_ICACHE3_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending.
pub type L1_ICACHE3_PRELOAD_ORDER_R = crate::BitReader;
///Field `L1_ICACHE3_PRELOAD_RGID` reader - The bit is used to set the gid of l1 icache3 preload.
pub type L1_ICACHE3_PRELOAD_RGID_R = crate::FieldReader;
impl R {
    ///Bit 0 - The bit is used to enable preload operation on L1-ICache3. It will be cleared by hardware automatically after preload operation is done.
    #[inline(always)]
    pub fn l1_icache3_preload_ena(&self) -> L1_ICACHE3_PRELOAD_ENA_R {
        L1_ICACHE3_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished.
    #[inline(always)]
    pub fn l1_icache3_preload_done(&self) -> L1_ICACHE3_PRELOAD_DONE_R {
        L1_ICACHE3_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending.
    #[inline(always)]
    pub fn l1_icache3_preload_order(&self) -> L1_ICACHE3_PRELOAD_ORDER_R {
        L1_ICACHE3_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - The bit is used to set the gid of l1 icache3 preload.
    #[inline(always)]
    pub fn l1_icache3_preload_rgid(&self) -> L1_ICACHE3_PRELOAD_RGID_R {
        L1_ICACHE3_PRELOAD_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE3_PRELOAD_CTRL")
            .field("l1_icache3_preload_ena", &self.l1_icache3_preload_ena())
            .field("l1_icache3_preload_done", &self.l1_icache3_preload_done())
            .field("l1_icache3_preload_order", &self.l1_icache3_preload_order())
            .field("l1_icache3_preload_rgid", &self.l1_icache3_preload_rgid())
            .finish()
    }
}
/**L1 instruction Cache 3 preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_preload_ctrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_ICACHE3_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_ICACHE3_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_icache3_preload_ctrl::R`](R) reader structure
impl crate::Readable for L1_ICACHE3_PRELOAD_CTRL_SPEC {}
///`reset()` method sets L1_ICACHE3_PRELOAD_CTRL to value 0x02
impl crate::Resettable for L1_ICACHE3_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
