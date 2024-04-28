#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<CLK_EN_SPEC>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<CLK_EN_SPEC>;
#[doc = "Field `RNG` reader - need_des"]
pub type RNG_R = crate::BitReader;
#[doc = "Field `RNG` writer - need_des"]
pub type RNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TSENS` reader - need_des"]
pub type LP_TSENS_R = crate::BitReader;
#[doc = "Field `LP_TSENS` writer - need_des"]
pub type LP_TSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PMS` reader - need_des"]
pub type LP_PMS_R = crate::BitReader;
#[doc = "Field `LP_PMS` writer - need_des"]
pub type LP_PMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_EFUSE` reader - need_des"]
pub type LP_EFUSE_R = crate::BitReader;
#[doc = "Field `LP_EFUSE` writer - need_des"]
pub type LP_EFUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_IOMUX` reader - need_des"]
pub type LP_IOMUX_R = crate::BitReader;
#[doc = "Field `LP_IOMUX` writer - need_des"]
pub type LP_IOMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TOUCH` reader - need_des"]
pub type LP_TOUCH_R = crate::BitReader;
#[doc = "Field `LP_TOUCH` writer - need_des"]
pub type LP_TOUCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SPI` reader - need_des"]
pub type LP_SPI_R = crate::BitReader;
#[doc = "Field `LP_SPI` writer - need_des"]
pub type LP_SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ADC` reader - need_des"]
pub type LP_ADC_R = crate::BitReader;
#[doc = "Field `LP_ADC` writer - need_des"]
pub type LP_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2S_TX` reader - need_des"]
pub type LP_I2S_TX_R = crate::BitReader;
#[doc = "Field `LP_I2S_TX` writer - need_des"]
pub type LP_I2S_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2S_RX` reader - need_des"]
pub type LP_I2S_RX_R = crate::BitReader;
#[doc = "Field `LP_I2S_RX` writer - need_des"]
pub type LP_I2S_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2S` reader - need_des"]
pub type LP_I2S_R = crate::BitReader;
#[doc = "Field `LP_I2S` writer - need_des"]
pub type LP_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2CMST` reader - need_des"]
pub type LP_I2CMST_R = crate::BitReader;
#[doc = "Field `LP_I2CMST` writer - need_des"]
pub type LP_I2CMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_I2C` reader - need_des"]
pub type LP_I2C_R = crate::BitReader;
#[doc = "Field `LP_I2C` writer - need_des"]
pub type LP_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART` reader - need_des"]
pub type LP_UART_R = crate::BitReader;
#[doc = "Field `LP_UART` writer - need_des"]
pub type LP_UART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_INTR` reader - need_des"]
pub type LP_INTR_R = crate::BitReader;
#[doc = "Field `LP_INTR` writer - need_des"]
pub type LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE` reader - write 1 to force on lp_core clk"]
pub type LP_CORE_R = crate::BitReader;
#[doc = "Field `LP_CORE` writer - write 1 to force on lp_core clk"]
pub type LP_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn lp_tsens(&self) -> LP_TSENS_R {
        LP_TSENS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn lp_pms(&self) -> LP_PMS_R {
        LP_PMS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn lp_efuse(&self) -> LP_EFUSE_R {
        LP_EFUSE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_iomux(&self) -> LP_IOMUX_R {
        LP_IOMUX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn lp_touch(&self) -> LP_TOUCH_R {
        LP_TOUCH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn lp_spi(&self) -> LP_SPI_R {
        LP_SPI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn lp_adc(&self) -> LP_ADC_R {
        LP_ADC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx(&self) -> LP_I2S_TX_R {
        LP_I2S_TX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_rx(&self) -> LP_I2S_RX_R {
        LP_I2S_RX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_i2s(&self) -> LP_I2S_R {
        LP_I2S_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_i2cmst(&self) -> LP_I2CMST_R {
        LP_I2CMST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_i2c(&self) -> LP_I2C_R {
        LP_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_uart(&self) -> LP_UART_R {
        LP_UART_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_intr(&self) -> LP_INTR_R {
        LP_INTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to force on lp_core clk"]
    #[inline(always)]
    pub fn lp_core(&self) -> LP_CORE_R {
        LP_CORE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field("rng", &format_args!("{}", self.rng().bit()))
            .field("lp_tsens", &format_args!("{}", self.lp_tsens().bit()))
            .field("lp_pms", &format_args!("{}", self.lp_pms().bit()))
            .field("lp_efuse", &format_args!("{}", self.lp_efuse().bit()))
            .field("lp_iomux", &format_args!("{}", self.lp_iomux().bit()))
            .field("lp_touch", &format_args!("{}", self.lp_touch().bit()))
            .field("lp_spi", &format_args!("{}", self.lp_spi().bit()))
            .field("lp_adc", &format_args!("{}", self.lp_adc().bit()))
            .field("lp_i2s_tx", &format_args!("{}", self.lp_i2s_tx().bit()))
            .field("lp_i2s_rx", &format_args!("{}", self.lp_i2s_rx().bit()))
            .field("lp_i2s", &format_args!("{}", self.lp_i2s().bit()))
            .field("lp_i2cmst", &format_args!("{}", self.lp_i2cmst().bit()))
            .field("lp_i2c", &format_args!("{}", self.lp_i2c().bit()))
            .field("lp_uart", &format_args!("{}", self.lp_uart().bit()))
            .field("lp_intr", &format_args!("{}", self.lp_intr().bit()))
            .field("lp_core", &format_args!("{}", self.lp_core().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<CLK_EN_SPEC> {
        RNG_W::new(self, 16)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_tsens(&mut self) -> LP_TSENS_W<CLK_EN_SPEC> {
        LP_TSENS_W::new(self, 17)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_pms(&mut self) -> LP_PMS_W<CLK_EN_SPEC> {
        LP_PMS_W::new(self, 18)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_efuse(&mut self) -> LP_EFUSE_W<CLK_EN_SPEC> {
        LP_EFUSE_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_iomux(&mut self) -> LP_IOMUX_W<CLK_EN_SPEC> {
        LP_IOMUX_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_touch(&mut self) -> LP_TOUCH_W<CLK_EN_SPEC> {
        LP_TOUCH_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_spi(&mut self) -> LP_SPI_W<CLK_EN_SPEC> {
        LP_SPI_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_adc(&mut self) -> LP_ADC_W<CLK_EN_SPEC> {
        LP_ADC_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_tx(&mut self) -> LP_I2S_TX_W<CLK_EN_SPEC> {
        LP_I2S_TX_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx(&mut self) -> LP_I2S_RX_W<CLK_EN_SPEC> {
        LP_I2S_RX_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s(&mut self) -> LP_I2S_W<CLK_EN_SPEC> {
        LP_I2S_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2cmst(&mut self) -> LP_I2CMST_W<CLK_EN_SPEC> {
        LP_I2CMST_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c(&mut self) -> LP_I2C_W<CLK_EN_SPEC> {
        LP_I2C_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart(&mut self) -> LP_UART_W<CLK_EN_SPEC> {
        LP_UART_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_intr(&mut self) -> LP_INTR_W<CLK_EN_SPEC> {
        LP_INTR_W::new(self, 30)
    }
    #[doc = "Bit 31 - write 1 to force on lp_core clk"]
    #[inline(always)]
    #[must_use]
    pub fn lp_core(&mut self) -> LP_CORE_W<CLK_EN_SPEC> {
        LP_CORE_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0x7fff_0000"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x7fff_0000;
}
