///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `TSENS` reader - saradc tsens interrupt raw
pub type TSENS_R = crate::BitReader;
///Field `TSENS` writer - saradc tsens interrupt raw
pub type TSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES1_LOW` reader - saradc thres1 low interrupt raw
pub type THRES1_LOW_R = crate::BitReader;
///Field `THRES1_LOW` writer - saradc thres1 low interrupt raw
pub type THRES1_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES0_LOW` reader - saradc thres0 low interrupt raw
pub type THRES0_LOW_R = crate::BitReader;
///Field `THRES0_LOW` writer - saradc thres0 low interrupt raw
pub type THRES0_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES1_HIGH` reader - saradc thres1 high interrupt raw
pub type THRES1_HIGH_R = crate::BitReader;
///Field `THRES1_HIGH` writer - saradc thres1 high interrupt raw
pub type THRES1_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES0_HIGH` reader - saradc thres0 high interrupt raw
pub type THRES0_HIGH_R = crate::BitReader;
///Field `THRES0_HIGH` writer - saradc thres0 high interrupt raw
pub type THRES0_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC2_DONE` reader - saradc2 done interrupt raw
pub type ADC2_DONE_R = crate::BitReader;
///Field `ADC2_DONE` writer - saradc2 done interrupt raw
pub type ADC2_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1_DONE` reader - saradc1 done interrupt raw
pub type ADC1_DONE_R = crate::BitReader;
///Field `ADC1_DONE` writer - saradc1 done interrupt raw
pub type ADC1_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 25 - saradc tsens interrupt raw
    #[inline(always)]
    pub fn tsens(&self) -> TSENS_R {
        TSENS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - saradc thres1 low interrupt raw
    #[inline(always)]
    pub fn thres1_low(&self) -> THRES1_LOW_R {
        THRES1_LOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - saradc thres0 low interrupt raw
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - saradc thres1 high interrupt raw
    #[inline(always)]
    pub fn thres1_high(&self) -> THRES1_HIGH_R {
        THRES1_HIGH_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - saradc thres0 high interrupt raw
    #[inline(always)]
    pub fn thres0_high(&self) -> THRES0_HIGH_R {
        THRES0_HIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - saradc2 done interrupt raw
    #[inline(always)]
    pub fn adc2_done(&self) -> ADC2_DONE_R {
        ADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - saradc1 done interrupt raw
    #[inline(always)]
    pub fn adc1_done(&self) -> ADC1_DONE_R {
        ADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("tsens", &self.tsens())
            .field("thres1_low", &self.thres1_low())
            .field("thres0_low", &self.thres0_low())
            .field("thres1_high", &self.thres1_high())
            .field("thres0_high", &self.thres0_high())
            .field("adc2_done", &self.adc2_done())
            .field("adc1_done", &self.adc1_done())
            .finish()
    }
}
impl W {
    ///Bit 25 - saradc tsens interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn tsens(&mut self) -> TSENS_W<INT_RAW_SPEC> {
        TSENS_W::new(self, 25)
    }
    ///Bit 26 - saradc thres1 low interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn thres1_low(&mut self) -> THRES1_LOW_W<INT_RAW_SPEC> {
        THRES1_LOW_W::new(self, 26)
    }
    ///Bit 27 - saradc thres0 low interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W<INT_RAW_SPEC> {
        THRES0_LOW_W::new(self, 27)
    }
    ///Bit 28 - saradc thres1 high interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn thres1_high(&mut self) -> THRES1_HIGH_W<INT_RAW_SPEC> {
        THRES1_HIGH_W::new(self, 28)
    }
    ///Bit 29 - saradc thres0 high interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W<INT_RAW_SPEC> {
        THRES0_HIGH_W::new(self, 29)
    }
    ///Bit 30 - saradc2 done interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn adc2_done(&mut self) -> ADC2_DONE_W<INT_RAW_SPEC> {
        ADC2_DONE_W::new(self, 30)
    }
    ///Bit 31 - saradc1 done interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn adc1_done(&mut self) -> ADC1_DONE_W<INT_RAW_SPEC> {
        ADC1_DONE_W::new(self, 31)
    }
}
/**digital saradc int register

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
