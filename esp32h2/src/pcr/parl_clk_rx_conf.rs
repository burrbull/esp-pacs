///Register `PARL_CLK_RX_CONF` reader
pub type R = crate::R<PARL_CLK_RX_CONF_SPEC>;
///Register `PARL_CLK_RX_CONF` writer
pub type W = crate::W<PARL_CLK_RX_CONF_SPEC>;
///Field `PARL_CLK_RX_DIV_NUM` reader - The integral part of the frequency divider factor of the parl rx clock.
pub type PARL_CLK_RX_DIV_NUM_R = crate::FieldReader<u16>;
///Field `PARL_CLK_RX_DIV_NUM` writer - The integral part of the frequency divider factor of the parl rx clock.
pub type PARL_CLK_RX_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PARL_CLK_RX_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad.
pub type PARL_CLK_RX_SEL_R = crate::FieldReader;
///Field `PARL_CLK_RX_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad.
pub type PARL_CLK_RX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PARL_CLK_RX_EN` reader - Set 1 to enable parl rx clock
pub type PARL_CLK_RX_EN_R = crate::BitReader;
///Field `PARL_CLK_RX_EN` writer - Set 1 to enable parl rx clock
pub type PARL_CLK_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARL_RX_RST_EN` reader - Set 0 to reset parl rx module
pub type PARL_RX_RST_EN_R = crate::BitReader;
///Field `PARL_RX_RST_EN` writer - Set 0 to reset parl rx module
pub type PARL_RX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - The integral part of the frequency divider factor of the parl rx clock.
    #[inline(always)]
    pub fn parl_clk_rx_div_num(&self) -> PARL_CLK_RX_DIV_NUM_R {
        PARL_CLK_RX_DIV_NUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad.
    #[inline(always)]
    pub fn parl_clk_rx_sel(&self) -> PARL_CLK_RX_SEL_R {
        PARL_CLK_RX_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Set 1 to enable parl rx clock
    #[inline(always)]
    pub fn parl_clk_rx_en(&self) -> PARL_CLK_RX_EN_R {
        PARL_CLK_RX_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Set 0 to reset parl rx module
    #[inline(always)]
    pub fn parl_rx_rst_en(&self) -> PARL_RX_RST_EN_R {
        PARL_RX_RST_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARL_CLK_RX_CONF")
            .field("parl_clk_rx_div_num", &self.parl_clk_rx_div_num())
            .field("parl_clk_rx_sel", &self.parl_clk_rx_sel())
            .field("parl_clk_rx_en", &self.parl_clk_rx_en())
            .field("parl_rx_rst_en", &self.parl_rx_rst_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - The integral part of the frequency divider factor of the parl rx clock.
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_div_num(&mut self) -> PARL_CLK_RX_DIV_NUM_W<PARL_CLK_RX_CONF_SPEC> {
        PARL_CLK_RX_DIV_NUM_W::new(self, 0)
    }
    ///Bits 16:17 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad.
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_sel(&mut self) -> PARL_CLK_RX_SEL_W<PARL_CLK_RX_CONF_SPEC> {
        PARL_CLK_RX_SEL_W::new(self, 16)
    }
    ///Bit 18 - Set 1 to enable parl rx clock
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_en(&mut self) -> PARL_CLK_RX_EN_W<PARL_CLK_RX_CONF_SPEC> {
        PARL_CLK_RX_EN_W::new(self, 18)
    }
    ///Bit 19 - Set 0 to reset parl rx module
    #[inline(always)]
    #[must_use]
    pub fn parl_rx_rst_en(&mut self) -> PARL_RX_RST_EN_W<PARL_CLK_RX_CONF_SPEC> {
        PARL_RX_RST_EN_W::new(self, 19)
    }
}
/**PARL_CLK_RX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`parl_clk_rx_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_clk_rx_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PARL_CLK_RX_CONF_SPEC;
impl crate::RegisterSpec for PARL_CLK_RX_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`parl_clk_rx_conf::R`](R) reader structure
impl crate::Readable for PARL_CLK_RX_CONF_SPEC {}
///`write(|w| ..)` method takes [`parl_clk_rx_conf::W`](W) writer structure
impl crate::Writable for PARL_CLK_RX_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PARL_CLK_RX_CONF to value 0x0004_0000
impl crate::Resettable for PARL_CLK_RX_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0004_0000;
}
