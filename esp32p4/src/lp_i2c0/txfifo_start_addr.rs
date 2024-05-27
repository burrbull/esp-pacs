///Register `TXFIFO_START_ADDR` reader
pub type R = crate::R<TXFIFO_START_ADDR_SPEC>;
///Field `TXFIFO_START_ADDR` reader - Represents the I2C txfifo first address.
pub type TXFIFO_START_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Represents the I2C txfifo first address.
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> TXFIFO_START_ADDR_R {
        TXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFIFO_START_ADDR")
            .field("txfifo_start_addr", &self.txfifo_start_addr())
            .finish()
    }
}
/**I2C TXFIFO base address register

You can [`read`](crate::generic::Reg::read) this register and get [`txfifo_start_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for TXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`txfifo_start_addr::R`](R) reader structure
impl crate::Readable for TXFIFO_START_ADDR_SPEC {}
///`reset()` method sets TXFIFO_START_ADDR to value 0
impl crate::Resettable for TXFIFO_START_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
