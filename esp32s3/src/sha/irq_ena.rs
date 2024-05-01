///Register `IRQ_ENA` reader
pub type R = crate::R<IRQ_ENA_SPEC>;
///Register `IRQ_ENA` writer
pub type W = crate::W<IRQ_ENA_SPEC>;
///Field `INTERRUPT_ENA` reader - sha interrupt enable register. 1'b0: disable(default) 1'b1: enable
pub type INTERRUPT_ENA_R = crate::BitReader;
///Field `INTERRUPT_ENA` writer - sha interrupt enable register. 1'b0: disable(default) 1'b1: enable
pub type INTERRUPT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - sha interrupt enable register. 1'b0: disable(default) 1'b1: enable
    #[inline(always)]
    pub fn interrupt_ena(&self) -> INTERRUPT_ENA_R {
        INTERRUPT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_ENA")
            .field("interrupt_ena", &self.interrupt_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - sha interrupt enable register. 1'b0: disable(default) 1'b1: enable
    #[inline(always)]
    #[must_use]
    pub fn interrupt_ena(&mut self) -> INTERRUPT_ENA_W<IRQ_ENA_SPEC> {
        INTERRUPT_ENA_W::new(self, 0)
    }
}
/**Interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`irq_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IRQ_ENA_SPEC;
impl crate::RegisterSpec for IRQ_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`irq_ena::R`](R) reader structure
impl crate::Readable for IRQ_ENA_SPEC {}
///`write(|w| ..)` method takes [`irq_ena::W`](W) writer structure
impl crate::Writable for IRQ_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IRQ_ENA to value 0
impl crate::Resettable for IRQ_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
