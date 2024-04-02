#[doc = "Register `DIN_MODE` reader"]
pub type R = crate::R<DIN_MODE_SPEC>;
#[doc = "Register `DIN_MODE` writer"]
pub type W = crate::W<DIN_MODE_SPEC>;
#[doc = "Configure the input mode for FSPID signal. Can be configured in CONF state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN_MODE {
    #[doc = "0: Input without delay"]
    NoDelay = 0,
    #[doc = "1: Input at the rising edge of APB_CLK"]
    ApbRisingEdge = 1,
    #[doc = "2: Input at the falling edge of APB_CLK"]
    ApbFallingEdge = 2,
    #[doc = "3: Input at the edge of SPI_CLK"]
    SpiEdge = 3,
}
impl From<IN_MODE> for u8 {
    #[inline(always)]
    fn from(variant: IN_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IN_MODE {
    type Ux = u8;
}
impl crate::IsEnum for IN_MODE {}
#[doc = "Field `DIN0_MODE` reader - Configure the input mode for FSPID signal. Can be configured in CONF state"]
pub type DIN0_MODE_R = crate::FieldReader<IN_MODE>;
impl DIN0_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IN_MODE {
        match self.bits {
            0 => IN_MODE::NoDelay,
            1 => IN_MODE::ApbRisingEdge,
            2 => IN_MODE::ApbFallingEdge,
            3 => IN_MODE::SpiEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "Input without delay"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == IN_MODE::NoDelay
    }
    #[doc = "Input at the rising edge of APB_CLK"]
    #[inline(always)]
    pub fn is_apb_rising_edge(&self) -> bool {
        *self == IN_MODE::ApbRisingEdge
    }
    #[doc = "Input at the falling edge of APB_CLK"]
    #[inline(always)]
    pub fn is_apb_falling_edge(&self) -> bool {
        *self == IN_MODE::ApbFallingEdge
    }
    #[doc = "Input at the edge of SPI_CLK"]
    #[inline(always)]
    pub fn is_spi_edge(&self) -> bool {
        *self == IN_MODE::SpiEdge
    }
}
#[doc = "Field `DIN0_MODE` writer - Configure the input mode for FSPID signal. Can be configured in CONF state"]
pub type DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IN_MODE, crate::Safe>;
impl<'a, REG> DIN0_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input without delay"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(IN_MODE::NoDelay)
    }
    #[doc = "Input at the rising edge of APB_CLK"]
    #[inline(always)]
    pub fn apb_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(IN_MODE::ApbRisingEdge)
    }
    #[doc = "Input at the falling edge of APB_CLK"]
    #[inline(always)]
    pub fn apb_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(IN_MODE::ApbFallingEdge)
    }
    #[doc = "Input at the edge of SPI_CLK"]
    #[inline(always)]
    pub fn spi_edge(self) -> &'a mut crate::W<REG> {
        self.variant(IN_MODE::SpiEdge)
    }
}
#[doc = "Field `DIN1_MODE` reader - Configure the input mode for FSPIQ signal. Can be configured in CONF state"]
pub use DIN0_MODE_R as DIN1_MODE_R;
#[doc = "Field `DIN2_MODE` reader - Configure the input mode for FSPIWP signal. Can be configured in CONF state"]
pub use DIN0_MODE_R as DIN2_MODE_R;
#[doc = "Field `DIN3_MODE` reader - Configure the input mode for FSPIHF signal. Can be configured in CONF state"]
pub use DIN0_MODE_R as DIN3_MODE_R;
#[doc = "Field `DIN1_MODE` writer - Configure the input mode for FSPIQ signal. Can be configured in CONF state"]
pub use DIN0_MODE_W as DIN1_MODE_W;
#[doc = "Field `DIN2_MODE` writer - Configure the input mode for FSPIWP signal. Can be configured in CONF state"]
pub use DIN0_MODE_W as DIN2_MODE_W;
#[doc = "Field `DIN3_MODE` writer - Configure the input mode for FSPIHF signal. Can be configured in CONF state"]
pub use DIN0_MODE_W as DIN3_MODE_W;
impl R {
    #[doc = "Bits 0:1 - Configure the input mode for FSPID signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configure the input mode for FSPIQ signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configure the input mode for FSPIWP signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configure the input mode for FSPIHF signal. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 6) & 3) as u8)
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
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIN_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the input mode for FSPID signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din0_mode(&mut self) -> DIN0_MODE_W<DIN_MODE_SPEC> {
        DIN0_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configure the input mode for FSPIQ signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din1_mode(&mut self) -> DIN1_MODE_W<DIN_MODE_SPEC> {
        DIN1_MODE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configure the input mode for FSPIWP signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din2_mode(&mut self) -> DIN2_MODE_W<DIN_MODE_SPEC> {
        DIN2_MODE_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configure the input mode for FSPIHF signal. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din3_mode(&mut self) -> DIN3_MODE_W<DIN_MODE_SPEC> {
        DIN3_MODE_W::new(self, 6)
    }
}
#[doc = "SPI0 input delay mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_mode::R`](R) reader structure"]
impl crate::Readable for DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`din_mode::W`](W) writer structure"]
impl crate::Writable for DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIN_MODE to value 0"]
impl crate::Resettable for DIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
