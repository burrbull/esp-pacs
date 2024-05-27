///Register `STATUS_NEXT1` reader
pub type R = crate::R<STATUS_NEXT1_SPEC>;
///Field `STATUS_INTERRUPT_NEXT1` reader - GPIO interrupt source register for GPIO32-56
pub type STATUS_INTERRUPT_NEXT1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:24 - GPIO interrupt source register for GPIO32-56
    #[inline(always)]
    pub fn status_interrupt_next1(&self) -> STATUS_INTERRUPT_NEXT1_R {
        STATUS_INTERRUPT_NEXT1_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_NEXT1")
            .field("status_interrupt_next1", &self.status_interrupt_next1())
            .finish()
    }
}
/**GPIO interrupt source register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_NEXT1_SPEC;
impl crate::RegisterSpec for STATUS_NEXT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status_next1::R`](R) reader structure
impl crate::Readable for STATUS_NEXT1_SPEC {}
///`reset()` method sets STATUS_NEXT1 to value 0
impl crate::Resettable for STATUS_NEXT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
