///Register `PERI_CLK_CTRL114` reader
pub type R = crate::R<PERI_CLK_CTRL114_SPEC>;
///Register `PERI_CLK_CTRL114` writer
pub type W = crate::W<PERI_CLK_CTRL114_SPEC>;
///Field `UART3_SCLK_DIV_NUM` reader - Reserved
pub type UART3_SCLK_DIV_NUM_R = crate::FieldReader;
///Field `UART3_SCLK_DIV_NUM` writer - Reserved
pub type UART3_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UART3_SCLK_DIV_NUMERATOR` reader - Reserved
pub type UART3_SCLK_DIV_NUMERATOR_R = crate::FieldReader;
///Field `UART3_SCLK_DIV_NUMERATOR` writer - Reserved
pub type UART3_SCLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UART3_SCLK_DIV_DENOMINATOR` reader - Reserved
pub type UART3_SCLK_DIV_DENOMINATOR_R = crate::FieldReader;
///Field `UART3_SCLK_DIV_DENOMINATOR` writer - Reserved
pub type UART3_SCLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UART4_CLK_SRC_SEL` reader - Reserved
pub type UART4_CLK_SRC_SEL_R = crate::FieldReader;
///Field `UART4_CLK_SRC_SEL` writer - Reserved
pub type UART4_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UART4_CLK_EN` reader - Reserved
pub type UART4_CLK_EN_R = crate::BitReader;
///Field `UART4_CLK_EN` writer - Reserved
pub type UART4_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    pub fn uart3_sclk_div_num(&self) -> UART3_SCLK_DIV_NUM_R {
        UART3_SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Reserved
    #[inline(always)]
    pub fn uart3_sclk_div_numerator(&self) -> UART3_SCLK_DIV_NUMERATOR_R {
        UART3_SCLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Reserved
    #[inline(always)]
    pub fn uart3_sclk_div_denominator(&self) -> UART3_SCLK_DIV_DENOMINATOR_R {
        UART3_SCLK_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:25 - Reserved
    #[inline(always)]
    pub fn uart4_clk_src_sel(&self) -> UART4_CLK_SRC_SEL_R {
        UART4_CLK_SRC_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Reserved
    #[inline(always)]
    pub fn uart4_clk_en(&self) -> UART4_CLK_EN_R {
        UART4_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL114")
            .field("uart3_sclk_div_num", &self.uart3_sclk_div_num())
            .field("uart3_sclk_div_numerator", &self.uart3_sclk_div_numerator())
            .field("uart3_sclk_div_denominator", &self.uart3_sclk_div_denominator())
            .field("uart4_clk_src_sel", &self.uart4_clk_src_sel())
            .field("uart4_clk_en", &self.uart4_clk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn uart3_sclk_div_num(&mut self) -> UART3_SCLK_DIV_NUM_W<PERI_CLK_CTRL114_SPEC> {
        UART3_SCLK_DIV_NUM_W::new(self, 0)
    }
    ///Bits 8:15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn uart3_sclk_div_numerator(
        &mut self,
    ) -> UART3_SCLK_DIV_NUMERATOR_W<PERI_CLK_CTRL114_SPEC> {
        UART3_SCLK_DIV_NUMERATOR_W::new(self, 8)
    }
    ///Bits 16:23 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn uart3_sclk_div_denominator(
        &mut self,
    ) -> UART3_SCLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL114_SPEC> {
        UART3_SCLK_DIV_DENOMINATOR_W::new(self, 16)
    }
    ///Bits 24:25 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn uart4_clk_src_sel(&mut self) -> UART4_CLK_SRC_SEL_W<PERI_CLK_CTRL114_SPEC> {
        UART4_CLK_SRC_SEL_W::new(self, 24)
    }
    ///Bit 26 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn uart4_clk_en(&mut self) -> UART4_CLK_EN_W<PERI_CLK_CTRL114_SPEC> {
        UART4_CLK_EN_W::new(self, 26)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl114::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl114::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL114_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL114_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl114::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL114_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl114::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL114_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL114 to value 0x0400_0000
impl crate::Resettable for PERI_CLK_CTRL114_SPEC {
    const RESET_VALUE: u32 = 0x0400_0000;
}
