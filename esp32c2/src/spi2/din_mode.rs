#[doc = "Register `DIN_MODE` reader"]
pub type R = crate::R<DIN_MODE_SPEC>;
#[doc = "Field `DIN_MODE(0-7)` reader - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
pub type DIN_MODE_R = crate::FieldReader;
#[doc = "Field `TIMING_HCLK_ACTIVE` reader - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
pub type TIMING_HCLK_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DIN0_MODE` field"]
    #[inline(always)]
    pub fn din_mode(&self, n: u8) -> DIN_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din_mode_iter(&self) -> impl Iterator<Item = DIN_MODE_R> + '_ {
        (0..8).map(move |n| DIN_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din4_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din5_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din6_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn din7_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
    #[inline(always)]
    pub fn timing_hclk_active(&self) -> TIMING_HCLK_ACTIVE_R {
        TIMING_HCLK_ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_MODE")
            .field("din0_mode", &format_args!("{}", self.din0_mode().bits()))
            .field("din1_mode", &format_args!("{}", self.din1_mode().bits()))
            .field("din2_mode", &format_args!("{}", self.din2_mode().bits()))
            .field("din3_mode", &format_args!("{}", self.din3_mode().bits()))
            .field("din4_mode", &format_args!("{}", self.din4_mode().bits()))
            .field("din5_mode", &format_args!("{}", self.din5_mode().bits()))
            .field("din6_mode", &format_args!("{}", self.din6_mode().bits()))
            .field("din7_mode", &format_args!("{}", self.din7_mode().bits()))
            .field(
                "timing_hclk_active",
                &format_args!("{}", self.timing_hclk_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIN_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI input delay mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_mode::R`](R) reader structure"]
impl crate::Readable for DIN_MODE_SPEC {}
#[doc = "`reset()` method sets DIN_MODE to value 0"]
impl crate::Resettable for DIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
