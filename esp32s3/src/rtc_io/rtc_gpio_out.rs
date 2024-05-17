///Register `RTC_GPIO_OUT` reader
pub type R = crate::R<RTC_GPIO_OUT_SPEC>;
///Register `RTC_GPIO_OUT` writer
pub type W = crate::W<RTC_GPIO_OUT_SPEC>;
///Field `DATA` reader - RTC GPIO 0 ~ 21 output data
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - RTC GPIO 0 ~ 21 output data
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 10:31 - RTC GPIO 0 ~ 21 output data
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_OUT").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 10:31 - RTC GPIO 0 ~ 21 output data
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<RTC_GPIO_OUT_SPEC> {
        DATA_W::new(self, 10)
    }
}
/**RTC GPIO 0 ~ 21 output data register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_gpio_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_gpio_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTC_GPIO_OUT_SPEC;
impl crate::RegisterSpec for RTC_GPIO_OUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rtc_gpio_out::R`](R) reader structure
impl crate::Readable for RTC_GPIO_OUT_SPEC {}
///`write(|w| ..)` method takes [`rtc_gpio_out::W`](W) writer structure
impl crate::Writable for RTC_GPIO_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_GPIO_OUT to value 0
impl crate::Resettable for RTC_GPIO_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
