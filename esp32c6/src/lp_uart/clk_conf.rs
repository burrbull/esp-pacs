///Register `CLK_CONF` reader
pub type R = crate::R<CLK_CONF_SPEC>;
///Register `CLK_CONF` writer
pub type W = crate::W<CLK_CONF_SPEC>;
///Field `SCLK_DIV_B` reader - The denominator of the frequency divider factor.
pub type SCLK_DIV_B_R = crate::FieldReader;
///Field `SCLK_DIV_B` writer - The denominator of the frequency divider factor.
pub type SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SCLK_DIV_A` reader - The numerator of the frequency divider factor.
pub type SCLK_DIV_A_R = crate::FieldReader;
///Field `SCLK_DIV_A` writer - The numerator of the frequency divider factor.
pub type SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SCLK_DIV_NUM` reader - The integral part of the frequency divider factor.
pub type SCLK_DIV_NUM_R = crate::FieldReader;
///Field `SCLK_DIV_NUM` writer - The integral part of the frequency divider factor.
pub type SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLK_SEL` reader - UART clock source select. 1: 80Mhz. 2: 8Mhz. 3: XTAL.
pub type SCLK_SEL_R = crate::FieldReader;
///Field `SCLK_SEL` writer - UART clock source select. 1: 80Mhz. 2: 8Mhz. 3: XTAL.
pub type SCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SCLK_EN` reader - Set this bit to enable UART Tx/Rx clock.
pub type SCLK_EN_R = crate::BitReader;
///Field `SCLK_EN` writer - Set this bit to enable UART Tx/Rx clock.
pub type SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Tx/Rx.
pub type RST_CORE_R = crate::BitReader;
///Field `RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Tx/Rx.
pub type RST_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_SCLK_EN` reader - Set this bit to enable UART Tx clock.
pub type TX_SCLK_EN_R = crate::BitReader;
///Field `TX_SCLK_EN` writer - Set this bit to enable UART Tx clock.
pub type TX_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_SCLK_EN` reader - Set this bit to enable UART Rx clock.
pub type RX_SCLK_EN_R = crate::BitReader;
///Field `RX_SCLK_EN` writer - Set this bit to enable UART Rx clock.
pub type RX_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Tx.
pub type TX_RST_CORE_R = crate::BitReader;
///Field `TX_RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Tx.
pub type TX_RST_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Rx.
pub type RX_RST_CORE_R = crate::BitReader;
///Field `RX_RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Rx.
pub type RX_RST_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - The denominator of the frequency divider factor.
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:11 - The numerator of the frequency divider factor.
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bits 12:19 - The integral part of the frequency divider factor.
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:21 - UART clock source select. 1: 80Mhz. 2: 8Mhz. 3: XTAL.
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Set this bit to enable UART Tx/Rx clock.
    #[inline(always)]
    pub fn sclk_en(&self) -> SCLK_EN_R {
        SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Write 1 then write 0 to this bit to reset UART Tx/Rx.
    #[inline(always)]
    pub fn rst_core(&self) -> RST_CORE_R {
        RST_CORE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Set this bit to enable UART Tx clock.
    #[inline(always)]
    pub fn tx_sclk_en(&self) -> TX_SCLK_EN_R {
        TX_SCLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Set this bit to enable UART Rx clock.
    #[inline(always)]
    pub fn rx_sclk_en(&self) -> RX_SCLK_EN_R {
        RX_SCLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Write 1 then write 0 to this bit to reset UART Tx.
    #[inline(always)]
    pub fn tx_rst_core(&self) -> TX_RST_CORE_R {
        TX_RST_CORE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Write 1 then write 0 to this bit to reset UART Rx.
    #[inline(always)]
    pub fn rx_rst_core(&self) -> RX_RST_CORE_R {
        RX_RST_CORE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("sclk_div_b", &self.sclk_div_b())
            .field("sclk_div_a", &self.sclk_div_a())
            .field("sclk_div_num", &self.sclk_div_num())
            .field("sclk_sel", &self.sclk_sel())
            .field("sclk_en", &self.sclk_en())
            .field("rst_core", &self.rst_core())
            .field("tx_sclk_en", &self.tx_sclk_en())
            .field("rx_sclk_en", &self.rx_sclk_en())
            .field("tx_rst_core", &self.tx_rst_core())
            .field("rx_rst_core", &self.rx_rst_core())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - The denominator of the frequency divider factor.
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<CLK_CONF_SPEC> {
        SCLK_DIV_B_W::new(self, 0)
    }
    ///Bits 6:11 - The numerator of the frequency divider factor.
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<CLK_CONF_SPEC> {
        SCLK_DIV_A_W::new(self, 6)
    }
    ///Bits 12:19 - The integral part of the frequency divider factor.
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<CLK_CONF_SPEC> {
        SCLK_DIV_NUM_W::new(self, 12)
    }
    ///Bits 20:21 - UART clock source select. 1: 80Mhz. 2: 8Mhz. 3: XTAL.
    #[inline(always)]
    #[must_use]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<CLK_CONF_SPEC> {
        SCLK_SEL_W::new(self, 20)
    }
    ///Bit 22 - Set this bit to enable UART Tx/Rx clock.
    #[inline(always)]
    #[must_use]
    pub fn sclk_en(&mut self) -> SCLK_EN_W<CLK_CONF_SPEC> {
        SCLK_EN_W::new(self, 22)
    }
    ///Bit 23 - Write 1 then write 0 to this bit to reset UART Tx/Rx.
    #[inline(always)]
    #[must_use]
    pub fn rst_core(&mut self) -> RST_CORE_W<CLK_CONF_SPEC> {
        RST_CORE_W::new(self, 23)
    }
    ///Bit 24 - Set this bit to enable UART Tx clock.
    #[inline(always)]
    #[must_use]
    pub fn tx_sclk_en(&mut self) -> TX_SCLK_EN_W<CLK_CONF_SPEC> {
        TX_SCLK_EN_W::new(self, 24)
    }
    ///Bit 25 - Set this bit to enable UART Rx clock.
    #[inline(always)]
    #[must_use]
    pub fn rx_sclk_en(&mut self) -> RX_SCLK_EN_W<CLK_CONF_SPEC> {
        RX_SCLK_EN_W::new(self, 25)
    }
    ///Bit 26 - Write 1 then write 0 to this bit to reset UART Tx.
    #[inline(always)]
    #[must_use]
    pub fn tx_rst_core(&mut self) -> TX_RST_CORE_W<CLK_CONF_SPEC> {
        TX_RST_CORE_W::new(self, 26)
    }
    ///Bit 27 - Write 1 then write 0 to this bit to reset UART Rx.
    #[inline(always)]
    #[must_use]
    pub fn rx_rst_core(&mut self) -> RX_RST_CORE_W<CLK_CONF_SPEC> {
        RX_RST_CORE_W::new(self, 27)
    }
}
/**UART core clock configuration

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_conf::R`](R) reader structure
impl crate::Readable for CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_CONF to value 0x0370_1000
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0370_1000;
}
