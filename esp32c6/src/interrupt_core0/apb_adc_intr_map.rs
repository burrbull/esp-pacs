///Register `APB_ADC_INTR_MAP` reader
pub type R = crate::R<APB_ADC_INTR_MAP_SPEC>;
///Register `APB_ADC_INTR_MAP` writer
pub type W = crate::W<APB_ADC_INTR_MAP_SPEC>;
///Field `APB_ADC_INTR_MAP` reader - Need add description
pub type APB_ADC_INTR_MAP_R = crate::FieldReader;
///Field `APB_ADC_INTR_MAP` writer - Need add description
pub type APB_ADC_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    pub fn apb_adc_intr_map(&self) -> APB_ADC_INTR_MAP_R {
        APB_ADC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_ADC_INTR_MAP")
            .field("apb_adc_intr_map", &self.apb_adc_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn apb_adc_intr_map(&mut self) -> APB_ADC_INTR_MAP_W<APB_ADC_INTR_MAP_SPEC> {
        APB_ADC_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`apb_adc_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_adc_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB_ADC_INTR_MAP_SPEC;
impl crate::RegisterSpec for APB_ADC_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb_adc_intr_map::R`](R) reader structure
impl crate::Readable for APB_ADC_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`apb_adc_intr_map::W`](W) writer structure
impl crate::Writable for APB_ADC_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_ADC_INTR_MAP to value 0
impl crate::Resettable for APB_ADC_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
