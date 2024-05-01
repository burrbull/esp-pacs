///Register `PIN_CTRL` reader
pub type R = crate::R<PIN_CTRL_SPEC>;
///Register `PIN_CTRL` writer
pub type W = crate::W<PIN_CTRL_SPEC>;
///Field `CLK_OUT1` reader - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals.
pub type CLK_OUT1_R = crate::FieldReader;
///Field `CLK_OUT1` writer - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals.
pub type CLK_OUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CLK_OUT2` reader - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals.
pub type CLK_OUT2_R = crate::FieldReader;
///Field `CLK_OUT2` writer - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals.
pub type CLK_OUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CLK_OUT3` reader - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals.
pub type CLK_OUT3_R = crate::FieldReader;
///Field `CLK_OUT3` writer - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals.
pub type CLK_OUT3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals.
    #[inline(always)]
    pub fn clk_out1(&self) -> CLK_OUT1_R {
        CLK_OUT1_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals.
    #[inline(always)]
    pub fn clk_out2(&self) -> CLK_OUT2_R {
        CLK_OUT2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals.
    #[inline(always)]
    pub fn clk_out3(&self) -> CLK_OUT3_R {
        CLK_OUT3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN_CTRL")
            .field("clk_out1", &self.clk_out1())
            .field("clk_out2", &self.clk_out2())
            .field("clk_out3", &self.clk_out3())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals.
    #[inline(always)]
    #[must_use]
    pub fn clk_out1(&mut self) -> CLK_OUT1_W<PIN_CTRL_SPEC> {
        CLK_OUT1_W::new(self, 0)
    }
    ///Bits 5:9 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals.
    #[inline(always)]
    #[must_use]
    pub fn clk_out2(&mut self) -> CLK_OUT2_W<PIN_CTRL_SPEC> {
        CLK_OUT2_W::new(self, 5)
    }
    ///Bits 10:14 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals.
    #[inline(always)]
    #[must_use]
    pub fn clk_out3(&mut self) -> CLK_OUT3_W<PIN_CTRL_SPEC> {
        CLK_OUT3_W::new(self, 10)
    }
}
/**Clock Output Configuration Register

You can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pin_ctrl::R`](R) reader structure
impl crate::Readable for PIN_CTRL_SPEC {}
///`write(|w| ..)` method takes [`pin_ctrl::W`](W) writer structure
impl crate::Writable for PIN_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PIN_CTRL to value 0x1def
impl crate::Resettable for PIN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1def;
}
