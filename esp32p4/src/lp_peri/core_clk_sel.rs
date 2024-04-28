#[doc = "Register `CORE_CLK_SEL` reader"]
pub type R = crate::R<CORE_CLK_SEL_SPEC>;
#[doc = "Register `CORE_CLK_SEL` writer"]
pub type W = crate::W<CORE_CLK_SEL_SPEC>;
#[doc = "Field `LP_I2S_TX` reader - need_des"]
pub type LP_I2S_TX_R = crate::FieldReader;
#[doc = "Field `LP_I2S_TX` writer - need_des"]
pub type LP_I2S_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_I2S_RX` reader - need_des"]
pub type LP_I2S_RX_R = crate::FieldReader;
#[doc = "Field `LP_I2S_RX` writer - need_des"]
pub type LP_I2S_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_I2C` reader - need_des"]
pub type LP_I2C_R = crate::FieldReader;
#[doc = "Field `LP_I2C` writer - need_des"]
pub type LP_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_UART` reader - need_des"]
pub type LP_UART_R = crate::FieldReader;
#[doc = "Field `LP_UART` writer - need_des"]
pub type LP_UART_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx(&self) -> LP_I2S_TX_R {
        LP_I2S_TX_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_rx(&self) -> LP_I2S_RX_R {
        LP_I2S_RX_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_i2c(&self) -> LP_I2C_R {
        LP_I2C_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn lp_uart(&self) -> LP_UART_R {
        LP_UART_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_CLK_SEL")
            .field("lp_i2s_tx", &format_args!("{}", self.lp_i2s_tx().bits()))
            .field("lp_i2s_rx", &format_args!("{}", self.lp_i2s_rx().bits()))
            .field("lp_i2c", &format_args!("{}", self.lp_i2c().bits()))
            .field("lp_uart", &format_args!("{}", self.lp_uart().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_CLK_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_tx(&mut self) -> LP_I2S_TX_W<CORE_CLK_SEL_SPEC> {
        LP_I2S_TX_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2s_rx(&mut self) -> LP_I2S_RX_W<CORE_CLK_SEL_SPEC> {
        LP_I2S_RX_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c(&mut self) -> LP_I2C_W<CORE_CLK_SEL_SPEC> {
        LP_I2C_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart(&mut self) -> LP_UART_W<CORE_CLK_SEL_SPEC> {
        LP_UART_W::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_clk_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_clk_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_CLK_SEL_SPEC;
impl crate::RegisterSpec for CORE_CLK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_clk_sel::R`](R) reader structure"]
impl crate::Readable for CORE_CLK_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_clk_sel::W`](W) writer structure"]
impl crate::Writable for CORE_CLK_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_CLK_SEL to value 0"]
impl crate::Resettable for CORE_CLK_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
