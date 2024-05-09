#[doc = "Register `OUT_W1TS` writer"]
pub type W = crate::W<OUT_W1TS_SPEC>;
#[doc = "Field `OUT_DATA_W1TS` writer - GPIO0~17 output value write 1 to set"]
pub type OUT_DATA_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 14:31 - GPIO0~17 output value write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn out_data_w1ts(&mut self) -> OUT_DATA_W1TS_W<OUT_W1TS_SPEC> {
        OUT_DATA_W1TS_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_W1TS_SPEC;
impl crate::RegisterSpec for OUT_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1ts::W`](W) writer structure"]
impl crate::Writable for OUT_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_W1TS to value 0"]
impl crate::Resettable for OUT_W1TS_SPEC {}
