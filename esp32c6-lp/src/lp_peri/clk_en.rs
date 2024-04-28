#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<CLK_EN_SPEC>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<CLK_EN_SPEC>;
#[doc = "Field `LP_TOUCH` reader - need_des"]
pub type LP_TOUCH_R = crate::BitReader;
#[doc = "Field `LP_TOUCH` writer - need_des"]
pub type LP_TOUCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG` reader - need_des"]
pub type RNG_R = crate::BitReader;
#[doc = "Field `RNG` writer - need_des"]
pub type RNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTP_DBG` reader - need_des"]
pub type OTP_DBG_R = crate::BitReader;
#[doc = "Field `OTP_DBG` writer - need_des"]
pub type OTP_DBG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART` reader - need_des"]
pub type LP_UART_R = crate::BitReader;
#[doc = "Field `LP_UART` writer - need_des"]
pub type LP_UART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_IO` reader - need_des"]
pub type LP_IO_R = crate::BitReader;
#[doc = "Field `LP_IO` writer - need_des"]
pub type LP_IO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_EXT_I2C` reader - need_des"]
pub type LP_EXT_I2C_R = crate::BitReader;
#[doc = "Field `LP_EXT_I2C` writer - need_des"]
pub type LP_EXT_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_I2C` reader - need_des"]
pub type LP_ANA_I2C_R = crate::BitReader;
#[doc = "Field `LP_ANA_I2C` writer - need_des"]
pub type LP_ANA_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE` reader - need_des"]
pub type EFUSE_R = crate::BitReader;
#[doc = "Field `EFUSE` writer - need_des"]
pub type EFUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CPU` reader - need_des"]
pub type LP_CPU_R = crate::BitReader;
#[doc = "Field `LP_CPU` writer - need_des"]
pub type LP_CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn lp_touch(&self) -> LP_TOUCH_R {
        LP_TOUCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn otp_dbg(&self) -> OTP_DBG_R {
        OTP_DBG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_uart(&self) -> LP_UART_R {
        LP_UART_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_io(&self) -> LP_IO_R {
        LP_IO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_ext_i2c(&self) -> LP_EXT_I2C_R {
        LP_EXT_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_i2c(&self) -> LP_ANA_I2C_R {
        LP_ANA_I2C_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn efuse(&self) -> EFUSE_R {
        EFUSE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu(&self) -> LP_CPU_R {
        LP_CPU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field("lp_touch", &format_args!("{}", self.lp_touch().bit()))
            .field("rng", &format_args!("{}", self.rng().bit()))
            .field("otp_dbg", &format_args!("{}", self.otp_dbg().bit()))
            .field("lp_uart", &format_args!("{}", self.lp_uart().bit()))
            .field("lp_io", &format_args!("{}", self.lp_io().bit()))
            .field("lp_ext_i2c", &format_args!("{}", self.lp_ext_i2c().bit()))
            .field("lp_ana_i2c", &format_args!("{}", self.lp_ana_i2c().bit()))
            .field("efuse", &format_args!("{}", self.efuse().bit()))
            .field("lp_cpu", &format_args!("{}", self.lp_cpu().bit()))
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
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_touch(&mut self) -> LP_TOUCH_W<CLK_EN_SPEC> {
        LP_TOUCH_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<CLK_EN_SPEC> {
        RNG_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn otp_dbg(&mut self) -> OTP_DBG_W<CLK_EN_SPEC> {
        OTP_DBG_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_uart(&mut self) -> LP_UART_W<CLK_EN_SPEC> {
        LP_UART_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_io(&mut self) -> LP_IO_W<CLK_EN_SPEC> {
        LP_IO_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ext_i2c(&mut self) -> LP_EXT_I2C_W<CLK_EN_SPEC> {
        LP_EXT_I2C_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_i2c(&mut self) -> LP_ANA_I2C_W<CLK_EN_SPEC> {
        LP_ANA_I2C_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn efuse(&mut self) -> EFUSE_W<CLK_EN_SPEC> {
        EFUSE_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu(&mut self) -> LP_CPU_W<CLK_EN_SPEC> {
        LP_CPU_W::new(self, 31)
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
#[doc = "`reset()` method sets CLK_EN to value 0x7f80_0000"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x7f80_0000;
}
