#[doc = "Register `PLL_DIV_CLK_EN` reader"]
pub type R = crate::R<PLL_DIV_CLK_EN_SPEC>;
#[doc = "Register `PLL_DIV_CLK_EN` writer"]
pub type W = crate::W<PLL_DIV_CLK_EN_SPEC>;
#[doc = "Field `PLL_240M_CLK_EN` reader - This field is used to open 96 MHz clock (SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_240M_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_240M_CLK_EN` writer - This field is used to open 96 MHz clock (SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_240M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_160M_CLK_EN` reader - This field is used to open 64 MHz clock (div3 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_160M_CLK_EN` writer - This field is used to open 64 MHz clock (div3 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_120M_CLK_EN` reader - This field is used to open 48 MHz clock (div4 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_120M_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_120M_CLK_EN` writer - This field is used to open 48 MHz clock (div4 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_120M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_80M_CLK_EN` reader - This field is used to open 32 MHz clock (div6 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_80M_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_80M_CLK_EN` writer - This field is used to open 32 MHz clock (div6 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_80M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_48M_CLK_EN` reader - This field is used to open 16 MHz clock (div10 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_48M_CLK_EN` writer - This field is used to open 16 MHz clock (div10 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_40M_CLK_EN` reader - This field is used to open 8 MHz clock (div12 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_40M_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_40M_CLK_EN` writer - This field is used to open 8 MHz clock (div12 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
pub type PLL_40M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to open 96 MHz clock (SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    pub fn pll_240m_clk_en(&self) -> PLL_240M_CLK_EN_R {
        PLL_240M_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field is used to open 64 MHz clock (div3 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    pub fn pll_160m_clk_en(&self) -> PLL_160M_CLK_EN_R {
        PLL_160M_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field is used to open 48 MHz clock (div4 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    pub fn pll_120m_clk_en(&self) -> PLL_120M_CLK_EN_R {
        PLL_120M_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This field is used to open 32 MHz clock (div6 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    pub fn pll_80m_clk_en(&self) -> PLL_80M_CLK_EN_R {
        PLL_80M_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This field is used to open 16 MHz clock (div10 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    pub fn pll_48m_clk_en(&self) -> PLL_48M_CLK_EN_R {
        PLL_48M_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This field is used to open 8 MHz clock (div12 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    pub fn pll_40m_clk_en(&self) -> PLL_40M_CLK_EN_R {
        PLL_40M_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_DIV_CLK_EN")
            .field("pll_240m_clk_en", &self.pll_240m_clk_en().bit())
            .field("pll_160m_clk_en", &self.pll_160m_clk_en().bit())
            .field("pll_120m_clk_en", &self.pll_120m_clk_en().bit())
            .field("pll_80m_clk_en", &self.pll_80m_clk_en().bit())
            .field("pll_48m_clk_en", &self.pll_48m_clk_en().bit())
            .field("pll_40m_clk_en", &self.pll_40m_clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL_DIV_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to open 96 MHz clock (SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    #[must_use]
    pub fn pll_240m_clk_en(&mut self) -> PLL_240M_CLK_EN_W<PLL_DIV_CLK_EN_SPEC> {
        PLL_240M_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - This field is used to open 64 MHz clock (div3 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    #[must_use]
    pub fn pll_160m_clk_en(&mut self) -> PLL_160M_CLK_EN_W<PLL_DIV_CLK_EN_SPEC> {
        PLL_160M_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - This field is used to open 48 MHz clock (div4 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    #[must_use]
    pub fn pll_120m_clk_en(&mut self) -> PLL_120M_CLK_EN_W<PLL_DIV_CLK_EN_SPEC> {
        PLL_120M_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - This field is used to open 32 MHz clock (div6 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    #[must_use]
    pub fn pll_80m_clk_en(&mut self) -> PLL_80M_CLK_EN_W<PLL_DIV_CLK_EN_SPEC> {
        PLL_80M_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - This field is used to open 16 MHz clock (div10 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    #[must_use]
    pub fn pll_48m_clk_en(&mut self) -> PLL_48M_CLK_EN_W<PLL_DIV_CLK_EN_SPEC> {
        PLL_48M_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - This field is used to open 8 MHz clock (div12 of SPLL) drived from SPLL. 0: close, 1: open(default). Only avaliable when high-speed clock-source SPLL is active."]
    #[inline(always)]
    #[must_use]
    pub fn pll_40m_clk_en(&mut self) -> PLL_40M_CLK_EN_W<PLL_DIV_CLK_EN_SPEC> {
        PLL_40M_CLK_EN_W::new(self, 5)
    }
}
#[doc = "SPLL DIV clock-gating configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_div_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_div_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_DIV_CLK_EN_SPEC;
impl crate::RegisterSpec for PLL_DIV_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_div_clk_en::R`](R) reader structure"]
impl crate::Readable for PLL_DIV_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_div_clk_en::W`](W) writer structure"]
impl crate::Writable for PLL_DIV_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_DIV_CLK_EN to value 0x3f"]
impl crate::Resettable for PLL_DIV_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
