///Register `STATUS_W1TC` writer
pub type W = crate::W<STATUS_W1TC_SPEC>;
///Field `STATUS_INT_W1TC` writer - GPIO0~17 interrupt status write 1 to clear
pub type STATUS_INT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 14:31 - GPIO0~17 interrupt status write 1 to clear
    #[inline(always)]
    #[must_use]
    pub fn status_int_w1tc(&mut self) -> STATUS_INT_W1TC_W<STATUS_W1TC_SPEC> {
        STATUS_INT_W1TC_W::new(self, 14)
    }
}
/**

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
