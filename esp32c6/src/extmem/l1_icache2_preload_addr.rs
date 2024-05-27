///Register `L1_ICACHE2_PRELOAD_ADDR` reader
pub type R = crate::R<L1_ICACHE2_PRELOAD_ADDR_SPEC>;
///Field `L1_ICACHE2_PRELOAD_ADDR` reader - Those bits are used to configure the start virtual address of preload on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_SIZE_REG
pub type L1_ICACHE2_PRELOAD_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Those bits are used to configure the start virtual address of preload on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_SIZE_REG
    #[inline(always)]
    pub fn l1_icache2_preload_addr(&self) -> L1_ICACHE2_PRELOAD_ADDR_R {
        L1_ICACHE2_PRELOAD_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE2_PRELOAD_ADDR")
            .field("l1_icache2_preload_addr", &self.l1_icache2_preload_addr())
            .finish()
    }
}
/**L1 instruction Cache 2 preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_preload_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_ICACHE2_PRELOAD_ADDR_SPEC;
impl crate::RegisterSpec for L1_ICACHE2_PRELOAD_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_icache2_preload_addr::R`](R) reader structure
impl crate::Readable for L1_ICACHE2_PRELOAD_ADDR_SPEC {}
///`reset()` method sets L1_ICACHE2_PRELOAD_ADDR to value 0
impl crate::Resettable for L1_ICACHE2_PRELOAD_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
