///Register `DIN_MODE` reader
pub type R = crate::R<DIN_MODE_SPEC>;
///Register `DIN_MODE` writer
pub type W = crate::W<DIN_MODE_SPEC>;
///Field `DIN0_MODE` reader - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN0_MODE_R = crate::FieldReader;
///Field `DIN0_MODE` writer - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN1_MODE` reader - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN1_MODE_R = crate::FieldReader;
///Field `DIN1_MODE` writer - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN2_MODE` reader - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN2_MODE_R = crate::FieldReader;
///Field `DIN2_MODE` writer - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN3_MODE` reader - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN3_MODE_R = crate::FieldReader;
///Field `DIN3_MODE` writer - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN4_MODE` reader - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN4_MODE_R = crate::FieldReader;
///Field `DIN4_MODE` writer - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN4_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN5_MODE` reader - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN5_MODE_R = crate::FieldReader;
///Field `DIN5_MODE` writer - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN5_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN6_MODE` reader - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN6_MODE_R = crate::FieldReader;
///Field `DIN6_MODE` writer - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN6_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DIN7_MODE` reader - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN7_MODE_R = crate::FieldReader;
///Field `DIN7_MODE` writer - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DIN7_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DINS_MODE` reader - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DINS_MODE_R = crate::FieldReader;
///Field `DINS_MODE` writer - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
pub type DINS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN0_MODE_R {
        DIN0_MODE_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN1_MODE_R {
        DIN1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN2_MODE_R {
        DIN2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN3_MODE_R {
        DIN3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din4_mode(&self) -> DIN4_MODE_R {
        DIN4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din5_mode(&self) -> DIN5_MODE_R {
        DIN5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din6_mode(&self) -> DIN6_MODE_R {
        DIN6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn din7_mode(&self) -> DIN7_MODE_R {
        DIN7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    pub fn dins_mode(&self) -> DINS_MODE_R {
        DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_MODE")
            .field("din0_mode", &self.din0_mode())
            .field("din1_mode", &self.din1_mode())
            .field("din2_mode", &self.din2_mode())
            .field("din3_mode", &self.din3_mode())
            .field("din4_mode", &self.din4_mode())
            .field("din5_mode", &self.din5_mode())
            .field("din6_mode", &self.din6_mode())
            .field("din7_mode", &self.din7_mode())
            .field("dins_mode", &self.dins_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI_D input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN0_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN0_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din0_mode(&mut self) -> DIN0_MODE_W<DIN_MODE_SPEC> {
        DIN0_MODE_W::new(self, 0)
    }
    ///Bits 3:5 - SPI_Q input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN3_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN3_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din1_mode(&mut self) -> DIN1_MODE_W<DIN_MODE_SPEC> {
        DIN1_MODE_W::new(self, 3)
    }
    ///Bits 6:8 - SPI_WP input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN6_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN6_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din2_mode(&mut self) -> DIN2_MODE_W<DIN_MODE_SPEC> {
        DIN2_MODE_W::new(self, 6)
    }
    ///Bits 9:11 - SPI_HD input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN9_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN9_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din3_mode(&mut self) -> DIN3_MODE_W<DIN_MODE_SPEC> {
        DIN3_MODE_W::new(self, 9)
    }
    ///Bits 12:14 - SPI_IO4 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN12_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN12_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din4_mode(&mut self) -> DIN4_MODE_W<DIN_MODE_SPEC> {
        DIN4_MODE_W::new(self, 12)
    }
    ///Bits 15:17 - SPI_IO5 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN15_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN15_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din5_mode(&mut self) -> DIN5_MODE_W<DIN_MODE_SPEC> {
        DIN5_MODE_W::new(self, 15)
    }
    ///Bits 18:20 - SPI_IO6 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN18_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN18_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din6_mode(&mut self) -> DIN6_MODE_W<DIN_MODE_SPEC> {
        DIN6_MODE_W::new(self, 18)
    }
    ///Bits 21:23 - SPI_IO7 input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DIN21_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DIN21_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn din7_mode(&mut self) -> DIN7_MODE_W<DIN_MODE_SPEC> {
        DIN7_MODE_W::new(self, 21)
    }
    ///Bits 24:26 - SPI_DQS input delay mode. 0: No delay. 1: Delay for (SPI_MEM_DINS_NUM+1) cycles at MSPI_CORE_CLK negative edge. 2: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK positive edge. 3: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK positive edge and one cycle at MSPI_CORE_CLK negative edge. 4: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK positive edge. 5: Delay for (SPI_MEM_DINS_NUM+1) cycles at HCLK negative edge and one cycle at MSPI_CORE_CLK negative edge.
    #[inline(always)]
    #[must_use]
    pub fn dins_mode(&mut self) -> DINS_MODE_W<DIN_MODE_SPEC> {
        DINS_MODE_W::new(self, 24)
    }
}
/**MSPI input timing delay mode control register when accesses to flash.

You can [`read`](crate::generic::Reg::read) this register and get [`din_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIN_MODE_SPEC;
impl crate::RegisterSpec for DIN_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`din_mode::R`](R) reader structure
impl crate::Readable for DIN_MODE_SPEC {}
///`write(|w| ..)` method takes [`din_mode::W`](W) writer structure
impl crate::Writable for DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIN_MODE to value 0
impl crate::Resettable for DIN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
