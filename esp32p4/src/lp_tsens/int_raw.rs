///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `COCPU_TSENS_WAKE` reader - Tsens wakeup interrupt raw.
pub type COCPU_TSENS_WAKE_R = crate::BitReader;
///Field `COCPU_TSENS_WAKE` writer - Tsens wakeup interrupt raw.
pub type COCPU_TSENS_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tsens wakeup interrupt raw.
    #[inline(always)]
    pub fn cocpu_tsens_wake(&self) -> COCPU_TSENS_WAKE_R {
        COCPU_TSENS_WAKE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("cocpu_tsens_wake", &self.cocpu_tsens_wake())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tsens wakeup interrupt raw.
    #[inline(always)]
    #[must_use]
    pub fn cocpu_tsens_wake(&mut self) -> COCPU_TSENS_WAKE_W<INT_RAW_SPEC> {
        COCPU_TSENS_WAKE_W::new(self, 0)
    }
}
/**Tsens interrupt raw registers.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`int_raw::W`](W) writer structure
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
