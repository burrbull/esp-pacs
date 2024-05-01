///Register `CONF` reader
pub type R = crate::R<CONF_SPEC>;
///Register `CONF` writer
pub type W = crate::W<CONF_SPEC>;
///Field `DUTY_RES` reader - Configures the range of the counter in timer %s.
pub type DUTY_RES_R = crate::FieldReader;
///Field `DUTY_RES` writer - Configures the range of the counter in timer %s.
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CLK_DIV` reader - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
pub type CLK_DIV_R = crate::FieldReader<u32>;
///Field `CLK_DIV` writer - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `PAUSE` reader - Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
pub type PAUSE_R = crate::BitReader;
///Field `PAUSE` writer - Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST` reader - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
pub type RST_R = crate::BitReader;
///Field `RST` writer - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TICK_SEL` reader - Configures which clock is timer %s selected. Unused.
pub type TICK_SEL_R = crate::BitReader;
///Field `TICK_SEL` writer - Configures which clock is timer %s selected. Unused.
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARA_UP` writer - Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Configures the range of the counter in timer %s.
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:22 - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    ///Bit 23 - Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("duty_res", &self.duty_res())
            .field("clk_div", &self.clk_div())
            .field("pause", &self.pause())
            .field("rst", &self.rst())
            .field("tick_sel", &self.tick_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Configures the range of the counter in timer %s.
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    ///Bits 5:22 - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part.
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<CONF_SPEC> {
        CLK_DIV_W::new(self, 5)
    }
    ///Bit 23 - Configures whether or not to pause the counter in timer %s.\\0: Normal\\1: Pause
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<CONF_SPEC> {
        PAUSE_W::new(self, 23)
    }
    ///Bit 24 - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\0: Not reset\\1: Reset
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CONF_SPEC> {
        RST_W::new(self, 24)
    }
    ///Bit 25 - Configures which clock is timer %s selected. Unused.
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<CONF_SPEC> {
        TICK_SEL_W::new(self, 25)
    }
    ///Bit 26 - Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\0: Invalid. No effect\\1: Update
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<CONF_SPEC> {
        PARA_UP_W::new(self, 26)
    }
}
/**Timer 0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf::R`](R) reader structure
impl crate::Readable for CONF_SPEC {}
///`write(|w| ..)` method takes [`conf::W`](W) writer structure
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF to value 0x0100_0000
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
