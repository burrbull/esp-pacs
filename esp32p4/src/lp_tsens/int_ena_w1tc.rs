///Register `INT_ENA_W1TC` writer
pub type W = crate::W<INT_ENA_W1TC_SPEC>;
///Field `COCPU_TSENS_WAKE_INT_ENA_W1TC` writer - Write 1 to this field to deassert interrupt enable.
pub type COCPU_TSENS_WAKE_INT_ENA_W1TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Write 1 to this field to deassert interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn cocpu_tsens_wake_int_ena_w1tc(
        &mut self,
    ) -> COCPU_TSENS_WAKE_INT_ENA_W1TC_W<INT_ENA_W1TC_SPEC> {
        COCPU_TSENS_WAKE_INT_ENA_W1TC_W::new(self, 0)
    }
}
/**Tsens wakeup interrupt enable deassert.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_W1TC_SPEC;
impl crate::RegisterSpec for INT_ENA_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_ena_w1tc::W`](W) writer structure
impl crate::Writable for INT_ENA_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA_W1TC to value 0
impl crate::Resettable for INT_ENA_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
