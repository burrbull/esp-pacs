///Register `I2C0_CONF` reader
pub type R = crate::R<I2C0_CONF_SPEC>;
///Register `I2C0_CONF` writer
pub type W = crate::W<I2C0_CONF_SPEC>;
///Field `I2C0_CLK_EN` reader - Set 1 to enable i2c apb clock
pub type I2C0_CLK_EN_R = crate::BitReader;
///Field `I2C0_CLK_EN` writer - Set 1 to enable i2c apb clock
pub type I2C0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C0_RST_EN` reader - Set 0 to reset i2c module
pub type I2C0_RST_EN_R = crate::BitReader;
///Field `I2C0_RST_EN` writer - Set 0 to reset i2c module
pub type I2C0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable i2c apb clock
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2C0_CLK_EN_R {
        I2C0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset i2c module
    #[inline(always)]
    pub fn i2c0_rst_en(&self) -> I2C0_RST_EN_R {
        I2C0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CONF")
            .field("i2c0_clk_en", &self.i2c0_clk_en())
            .field("i2c0_rst_en", &self.i2c0_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable i2c apb clock
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_en(&mut self) -> I2C0_CLK_EN_W<I2C0_CONF_SPEC> {
        I2C0_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset i2c module
    #[inline(always)]
    #[must_use]
    pub fn i2c0_rst_en(&mut self) -> I2C0_RST_EN_W<I2C0_CONF_SPEC> {
        I2C0_RST_EN_W::new(self, 1)
    }
}
/**I2C configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2C0_CONF_SPEC;
impl crate::RegisterSpec for I2C0_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`i2c0_conf::R`](R) reader structure
impl crate::Readable for I2C0_CONF_SPEC {}
///`write(|w| ..)` method takes [`i2c0_conf::W`](W) writer structure
impl crate::Writable for I2C0_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C0_CONF to value 0x01
impl crate::Resettable for I2C0_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
