#[doc = "Register `DOUT_MODE` reader"]
pub type R = crate::R<DOUT_MODE_SPEC>;
#[doc = "Register `DOUT_MODE` writer"]
pub type W = crate::W<DOUT_MODE_SPEC>;
#[doc = "Configure the output mode for FSPID signal. Can be configured in CONF state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT_MODE {
    #[doc = "0: Output without delay"]
    NoDelay = 0,
    #[doc = "1: Output with a delay of a SPI module clock cycle at its falling edge"]
    SpiEdge = 1,
}
impl From<OUT_MODE> for bool {
    #[inline(always)]
    fn from(variant: OUT_MODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT0_MODE` reader - Configure the output mode for FSPID signal. Can be configured in CONF state"]
pub type DOUT0_MODE_R = crate::BitReader<OUT_MODE>;
impl DOUT0_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUT_MODE {
        match self.bits {
            false => OUT_MODE::NoDelay,
            true => OUT_MODE::SpiEdge,
        }
    }
    #[doc = "Output without delay"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == OUT_MODE::NoDelay
    }
    #[doc = "Output with a delay of a SPI module clock cycle at its falling edge"]
    #[inline(always)]
    pub fn is_spi_edge(&self) -> bool {
        *self == OUT_MODE::SpiEdge
    }
}
#[doc = "Field `DOUT0_MODE` writer - Configure the output mode for FSPID signal. Can be configured in CONF state"]
pub type DOUT0_MODE_W<'a, REG> = crate::BitWriter<'a, REG, OUT_MODE>;
impl<'a, REG> DOUT0_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output without delay"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(OUT_MODE::NoDelay)
    }
    #[doc = "Output with a delay of a SPI module clock cycle at its falling edge"]
    #[inline(always)]
    pub fn spi_edge(self) -> &'a mut crate::W<REG> {
        self.variant(OUT_MODE::SpiEdge)
    }
}
#[doc = "Field `DOUT1_MODE` reader - Configure the output mode for FSPIQ signal. Can be configured in CONF state"]
pub use DOUT0_MODE_R as DOUT1_MODE_R;
#[doc = "Field `DOUT2_MODE` reader - Configure the output mode for FSPIWP signal. Can be configured in CONF state"]
pub use DOUT0_MODE_R as DOUT2_MODE_R;
#[doc = "Field `DOUT3_MODE` reader - Configure the output mode for FSPIHF signal. Can be configured in CONF state"]
pub use DOUT0_MODE_R as DOUT3_MODE_R;
#[doc = "Field `DOUT1_MODE` writer - Configure the output mode for FSPIQ signal. Can be configured in CONF state"]
pub use DOUT0_MODE_W as DOUT1_MODE_W;
#[doc = "Field `DOUT2_MODE` writer - Configure the output mode for FSPIWP signal. Can be configured in CONF state"]
pub use DOUT0_MODE_W as DOUT2_MODE_W;
#[doc = "Field `DOUT3_MODE` writer - Configure the output mode for FSPIHF signal. Can be configured in CONF state"]
pub use DOUT0_MODE_W as DOUT3_MODE_W;
#[doc = "Field `DOUT4_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT4_MODE_R = crate::BitReader;
#[doc = "Field `DOUT4_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT4_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT5_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT5_MODE_R = crate::BitReader;
#[doc = "Field `DOUT5_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT5_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT6_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT6_MODE_R = crate::BitReader;
#[doc = "Field `DOUT6_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT6_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUT7_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT7_MODE_R = crate::BitReader;
#[doc = "Field `DOUT7_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUT7_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTS_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUTS_MODE_R = crate::BitReader;
#[doc = "Field `DOUTS_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
pub type DOUTS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure the output mode for FSPID signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure the output mode for FSPIQ signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure the output mode for FSPIWP signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure the output mode for FSPIHF signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT4_MODE_R {
        DOUT4_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT5_MODE_R {
        DOUT5_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT6_MODE_R {
        DOUT6_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT7_MODE_R {
        DOUT7_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    pub fn douts_mode(&self) -> DOUTS_MODE_R {
        DOUTS_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT_MODE")
            .field("dout0_mode", &format_args!("{}", self.dout0_mode().bit()))
            .field("dout1_mode", &format_args!("{}", self.dout1_mode().bit()))
            .field("dout2_mode", &format_args!("{}", self.dout2_mode().bit()))
            .field("dout3_mode", &format_args!("{}", self.dout3_mode().bit()))
            .field("dout4_mode", &format_args!("{}", self.dout4_mode().bit()))
            .field("dout5_mode", &format_args!("{}", self.dout5_mode().bit()))
            .field("dout6_mode", &format_args!("{}", self.dout6_mode().bit()))
            .field("dout7_mode", &format_args!("{}", self.dout7_mode().bit()))
            .field("douts_mode", &format_args!("{}", self.douts_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOUT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configure the output mode for FSPID signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W<DOUT_MODE_SPEC> {
        DOUT0_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure the output mode for FSPIQ signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W<DOUT_MODE_SPEC> {
        DOUT1_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configure the output mode for FSPIWP signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W<DOUT_MODE_SPEC> {
        DOUT2_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configure the output mode for FSPIHF signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W<DOUT_MODE_SPEC> {
        DOUT3_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn dout4_mode(&mut self) -> DOUT4_MODE_W<DOUT_MODE_SPEC> {
        DOUT4_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn dout5_mode(&mut self) -> DOUT5_MODE_W<DOUT_MODE_SPEC> {
        DOUT5_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn dout6_mode(&mut self) -> DOUT6_MODE_W<DOUT_MODE_SPEC> {
        DOUT6_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn dout7_mode(&mut self) -> DOUT7_MODE_W<DOUT_MODE_SPEC> {
        DOUT7_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk"]
    #[inline(always)]
    #[must_use]
    pub fn douts_mode(&mut self) -> DOUTS_MODE_W<DOUT_MODE_SPEC> {
        DOUTS_MODE_W::new(self, 8)
    }
}
#[doc = "MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_mode::R`](R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dout_mode::W`](W) writer structure"]
impl crate::Writable for DOUT_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
