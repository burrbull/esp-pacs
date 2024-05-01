///Register `TX_ABORT_INTERRUPT_CONTROL` reader
pub type R = crate::R<TX_ABORT_INTERRUPT_CONTROL_SPEC>;
///Register `TX_ABORT_INTERRUPT_CONTROL` writer
pub type W = crate::W<TX_ABORT_INTERRUPT_CONTROL_SPEC>;
///Field `TX_ABORT_INTERRUPT_CONTROL` reader -
pub type TX_ABORT_INTERRUPT_CONTROL_R = crate::FieldReader<u32>;
///Field `TX_ABORT_INTERRUPT_CONTROL` writer -
pub type TX_ABORT_INTERRUPT_CONTROL_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bits 0:30
    #[inline(always)]
    pub fn tx_abort_interrupt_control(&self) -> TX_ABORT_INTERRUPT_CONTROL_R {
        TX_ABORT_INTERRUPT_CONTROL_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ABORT_INTERRUPT_CONTROL")
            .field(
                "tx_abort_interrupt_control",
                &self.tx_abort_interrupt_control(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:30
    #[inline(always)]
    #[must_use]
    pub fn tx_abort_interrupt_control(
        &mut self,
    ) -> TX_ABORT_INTERRUPT_CONTROL_W<TX_ABORT_INTERRUPT_CONTROL_SPEC> {
        TX_ABORT_INTERRUPT_CONTROL_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`tx_abort_interrupt_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_abort_interrupt_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_ABORT_INTERRUPT_CONTROL_SPEC;
impl crate::RegisterSpec for TX_ABORT_INTERRUPT_CONTROL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_abort_interrupt_control::R`](R) reader structure
impl crate::Readable for TX_ABORT_INTERRUPT_CONTROL_SPEC {}
///`write(|w| ..)` method takes [`tx_abort_interrupt_control::W`](W) writer structure
impl crate::Writable for TX_ABORT_INTERRUPT_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_ABORT_INTERRUPT_CONTROL to value 0
impl crate::Resettable for TX_ABORT_INTERRUPT_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
