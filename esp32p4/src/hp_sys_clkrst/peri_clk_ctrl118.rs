///Register `PERI_CLK_CTRL118` reader
pub type R = crate::R<PERI_CLK_CTRL118_SPEC>;
///Register `PERI_CLK_CTRL118` writer
pub type W = crate::W<PERI_CLK_CTRL118_SPEC>;
///Field `PARLIO_RX_CLK_DIV_NUMERATOR` reader - Reserved
pub type PARLIO_RX_CLK_DIV_NUMERATOR_R = crate::FieldReader;
///Field `PARLIO_RX_CLK_DIV_NUMERATOR` writer - Reserved
pub type PARLIO_RX_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PARLIO_RX_CLK_DIV_DENOMINATOR` reader - Reserved
pub type PARLIO_RX_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
///Field `PARLIO_RX_CLK_DIV_DENOMINATOR` writer - Reserved
pub type PARLIO_RX_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PARLIO_TX_CLK_SRC_SEL` reader - Reserved
pub type PARLIO_TX_CLK_SRC_SEL_R = crate::FieldReader;
///Field `PARLIO_TX_CLK_SRC_SEL` writer - Reserved
pub type PARLIO_TX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PARLIO_TX_CLK_EN` reader - Reserved
pub type PARLIO_TX_CLK_EN_R = crate::BitReader;
///Field `PARLIO_TX_CLK_EN` writer - Reserved
pub type PARLIO_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARLIO_TX_CLK_DIV_NUM` reader - Reserved
pub type PARLIO_TX_CLK_DIV_NUM_R = crate::FieldReader;
///Field `PARLIO_TX_CLK_DIV_NUM` writer - Reserved
pub type PARLIO_TX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    pub fn parlio_rx_clk_div_numerator(&self) -> PARLIO_RX_CLK_DIV_NUMERATOR_R {
        PARLIO_RX_CLK_DIV_NUMERATOR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Reserved
    #[inline(always)]
    pub fn parlio_rx_clk_div_denominator(&self) -> PARLIO_RX_CLK_DIV_DENOMINATOR_R {
        PARLIO_RX_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Reserved
    #[inline(always)]
    pub fn parlio_tx_clk_src_sel(&self) -> PARLIO_TX_CLK_SRC_SEL_R {
        PARLIO_TX_CLK_SRC_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Reserved
    #[inline(always)]
    pub fn parlio_tx_clk_en(&self) -> PARLIO_TX_CLK_EN_R {
        PARLIO_TX_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:26 - Reserved
    #[inline(always)]
    pub fn parlio_tx_clk_div_num(&self) -> PARLIO_TX_CLK_DIV_NUM_R {
        PARLIO_TX_CLK_DIV_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL118")
            .field(
                "parlio_rx_clk_div_numerator",
                &self.parlio_rx_clk_div_numerator(),
            )
            .field(
                "parlio_rx_clk_div_denominator",
                &self.parlio_rx_clk_div_denominator(),
            )
            .field("parlio_tx_clk_src_sel", &self.parlio_tx_clk_src_sel())
            .field("parlio_tx_clk_en", &self.parlio_tx_clk_en())
            .field("parlio_tx_clk_div_num", &self.parlio_tx_clk_div_num())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn parlio_rx_clk_div_numerator(
        &mut self,
    ) -> PARLIO_RX_CLK_DIV_NUMERATOR_W<PERI_CLK_CTRL118_SPEC> {
        PARLIO_RX_CLK_DIV_NUMERATOR_W::new(self, 0)
    }
    ///Bits 8:15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn parlio_rx_clk_div_denominator(
        &mut self,
    ) -> PARLIO_RX_CLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL118_SPEC> {
        PARLIO_RX_CLK_DIV_DENOMINATOR_W::new(self, 8)
    }
    ///Bits 16:17 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn parlio_tx_clk_src_sel(&mut self) -> PARLIO_TX_CLK_SRC_SEL_W<PERI_CLK_CTRL118_SPEC> {
        PARLIO_TX_CLK_SRC_SEL_W::new(self, 16)
    }
    ///Bit 18 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn parlio_tx_clk_en(&mut self) -> PARLIO_TX_CLK_EN_W<PERI_CLK_CTRL118_SPEC> {
        PARLIO_TX_CLK_EN_W::new(self, 18)
    }
    ///Bits 19:26 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn parlio_tx_clk_div_num(&mut self) -> PARLIO_TX_CLK_DIV_NUM_W<PERI_CLK_CTRL118_SPEC> {
        PARLIO_TX_CLK_DIV_NUM_W::new(self, 19)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl118::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl118::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL118_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL118_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl118::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL118_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl118::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL118_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL118 to value 0
impl crate::Resettable for PERI_CLK_CTRL118_SPEC {
    const RESET_VALUE: u32 = 0;
}
