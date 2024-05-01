///Register `WDTFEED` writer
pub type W = crate::W<WDTFEED_SPEC>;
///Field `WDT_FEED` writer - Write any value will feed SWDT
pub type WDT_FEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTFEED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Write any value will feed SWDT
    #[inline(always)]
    #[must_use]
    pub fn wdt_feed(&mut self) -> WDT_FEED_W<WDTFEED_SPEC> {
        WDT_FEED_W::new(self, 0)
    }
}
/**

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDTFEED_SPEC;
impl crate::RegisterSpec for WDTFEED_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wdtfeed::W`](W) writer structure
impl crate::Writable for WDTFEED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDTFEED to value 0
impl crate::Resettable for WDTFEED_SPEC {
    const RESET_VALUE: u32 = 0;
}
