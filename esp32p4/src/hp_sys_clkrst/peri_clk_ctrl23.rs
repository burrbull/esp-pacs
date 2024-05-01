///Register `PERI_CLK_CTRL23` reader
pub type R = crate::R<PERI_CLK_CTRL23_SPEC>;
///Register `PERI_CLK_CTRL23` writer
pub type W = crate::W<PERI_CLK_CTRL23_SPEC>;
///Field `ADC_CLK_EN` reader - Reserved
pub type ADC_CLK_EN_R = crate::BitReader;
///Field `ADC_CLK_EN` writer - Reserved
pub type ADC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_CLK_DIV_NUM` reader - Reserved
pub type ADC_CLK_DIV_NUM_R = crate::FieldReader;
///Field `ADC_CLK_DIV_NUM` writer - Reserved
pub type ADC_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADC_CLK_DIV_NUMERATOR` reader - Reserved
pub type ADC_CLK_DIV_NUMERATOR_R = crate::FieldReader;
///Field `ADC_CLK_DIV_NUMERATOR` writer - Reserved
pub type ADC_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADC_CLK_DIV_DENOMINATOR` reader - Reserved
pub type ADC_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
///Field `ADC_CLK_DIV_DENOMINATOR` writer - Reserved
pub type ADC_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Reserved
    #[inline(always)]
    pub fn adc_clk_en(&self) -> ADC_CLK_EN_R {
        ADC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:8 - Reserved
    #[inline(always)]
    pub fn adc_clk_div_num(&self) -> ADC_CLK_DIV_NUM_R {
        ADC_CLK_DIV_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    ///Bits 9:16 - Reserved
    #[inline(always)]
    pub fn adc_clk_div_numerator(&self) -> ADC_CLK_DIV_NUMERATOR_R {
        ADC_CLK_DIV_NUMERATOR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    ///Bits 17:24 - Reserved
    #[inline(always)]
    pub fn adc_clk_div_denominator(&self) -> ADC_CLK_DIV_DENOMINATOR_R {
        ADC_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 17) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL23")
            .field("adc_clk_en", &self.adc_clk_en())
            .field("adc_clk_div_num", &self.adc_clk_div_num())
            .field("adc_clk_div_numerator", &self.adc_clk_div_numerator())
            .field("adc_clk_div_denominator", &self.adc_clk_div_denominator())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_en(&mut self) -> ADC_CLK_EN_W<PERI_CLK_CTRL23_SPEC> {
        ADC_CLK_EN_W::new(self, 0)
    }
    ///Bits 1:8 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_div_num(&mut self) -> ADC_CLK_DIV_NUM_W<PERI_CLK_CTRL23_SPEC> {
        ADC_CLK_DIV_NUM_W::new(self, 1)
    }
    ///Bits 9:16 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_div_numerator(&mut self) -> ADC_CLK_DIV_NUMERATOR_W<PERI_CLK_CTRL23_SPEC> {
        ADC_CLK_DIV_NUMERATOR_W::new(self, 9)
    }
    ///Bits 17:24 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_div_denominator(&mut self) -> ADC_CLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL23_SPEC> {
        ADC_CLK_DIV_DENOMINATOR_W::new(self, 17)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL23_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL23_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl23::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL23_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl23::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL23_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL23 to value 0x08
impl crate::Resettable for PERI_CLK_CTRL23_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
