///Register `STATUS_W1TC` writer
pub type W = crate::W<STATUS_W1TC_SPEC>;
///Field `REG_GPIO_STATUS_DATA_W1TC` writer - Reserved
pub type REG_GPIO_STATUS_DATA_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_status_data_w1tc(
        &mut self,
    ) -> REG_GPIO_STATUS_DATA_W1TC_W<STATUS_W1TC_SPEC> {
        REG_GPIO_STATUS_DATA_W1TC_W::new(self, 0)
    }
}
/**Reserved

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_W1TC_SPEC;
impl crate::RegisterSpec for STATUS_W1TC_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`status_w1tc::W`](W) writer structure
impl crate::Writable for STATUS_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS_W1TC to value 0
impl crate::Resettable for STATUS_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
