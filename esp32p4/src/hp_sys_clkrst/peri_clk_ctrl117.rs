#[doc = "Register `PERI_CLK_CTRL117` reader"]
pub type R = crate::R<PERI_CLK_CTRL117_SPEC>;
#[doc = "Register `PERI_CLK_CTRL117` writer"]
pub type W = crate::W<PERI_CLK_CTRL117_SPEC>;
#[doc = "Field `REG_GPSPI3_HS_CLK_DIV_NUM` reader - Reserved"]
pub type REG_GPSPI3_HS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_GPSPI3_HS_CLK_DIV_NUM` writer - Reserved"]
pub type REG_GPSPI3_HS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_GPSPI3_MST_CLK_DIV_NUM` reader - Reserved"]
pub type REG_GPSPI3_MST_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_GPSPI3_MST_CLK_DIV_NUM` writer - Reserved"]
pub type REG_GPSPI3_MST_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_GPSPI3_MST_CLK_EN` reader - Reserved"]
pub type REG_GPSPI3_MST_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_GPSPI3_MST_CLK_EN` writer - Reserved"]
pub type REG_GPSPI3_MST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PARLIO_RX_CLK_SRC_SEL` reader - Reserved"]
pub type REG_PARLIO_RX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PARLIO_RX_CLK_SRC_SEL` writer - Reserved"]
pub type REG_PARLIO_RX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PARLIO_RX_CLK_EN` reader - Reserved"]
pub type REG_PARLIO_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_PARLIO_RX_CLK_EN` writer - Reserved"]
pub type REG_PARLIO_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PARLIO_RX_CLK_DIV_NUM` reader - Reserved"]
pub type REG_PARLIO_RX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_PARLIO_RX_CLK_DIV_NUM` writer - Reserved"]
pub type REG_PARLIO_RX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpspi3_hs_clk_div_num(&self) -> REG_GPSPI3_HS_CLK_DIV_NUM_R {
        REG_GPSPI3_HS_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpspi3_mst_clk_div_num(&self) -> REG_GPSPI3_MST_CLK_DIV_NUM_R {
        REG_GPSPI3_MST_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reg_gpspi3_mst_clk_en(&self) -> REG_GPSPI3_MST_CLK_EN_R {
        REG_GPSPI3_MST_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Reserved"]
    #[inline(always)]
    pub fn reg_parlio_rx_clk_src_sel(&self) -> REG_PARLIO_RX_CLK_SRC_SEL_R {
        REG_PARLIO_RX_CLK_SRC_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reg_parlio_rx_clk_en(&self) -> REG_PARLIO_RX_CLK_EN_R {
        REG_PARLIO_RX_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Reserved"]
    #[inline(always)]
    pub fn reg_parlio_rx_clk_div_num(&self) -> REG_PARLIO_RX_CLK_DIV_NUM_R {
        REG_PARLIO_RX_CLK_DIV_NUM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL117")
            .field(
                "reg_gpspi3_hs_clk_div_num",
                &format_args!("{}", self.reg_gpspi3_hs_clk_div_num().bits()),
            )
            .field(
                "reg_gpspi3_mst_clk_div_num",
                &format_args!("{}", self.reg_gpspi3_mst_clk_div_num().bits()),
            )
            .field(
                "reg_gpspi3_mst_clk_en",
                &format_args!("{}", self.reg_gpspi3_mst_clk_en().bit()),
            )
            .field(
                "reg_parlio_rx_clk_src_sel",
                &format_args!("{}", self.reg_parlio_rx_clk_src_sel().bits()),
            )
            .field(
                "reg_parlio_rx_clk_en",
                &format_args!("{}", self.reg_parlio_rx_clk_en().bit()),
            )
            .field(
                "reg_parlio_rx_clk_div_num",
                &format_args!("{}", self.reg_parlio_rx_clk_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL117_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpspi3_hs_clk_div_num(
        &mut self,
    ) -> REG_GPSPI3_HS_CLK_DIV_NUM_W<PERI_CLK_CTRL117_SPEC> {
        REG_GPSPI3_HS_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpspi3_mst_clk_div_num(
        &mut self,
    ) -> REG_GPSPI3_MST_CLK_DIV_NUM_W<PERI_CLK_CTRL117_SPEC> {
        REG_GPSPI3_MST_CLK_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpspi3_mst_clk_en(&mut self) -> REG_GPSPI3_MST_CLK_EN_W<PERI_CLK_CTRL117_SPEC> {
        REG_GPSPI3_MST_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_parlio_rx_clk_src_sel(
        &mut self,
    ) -> REG_PARLIO_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL117_SPEC> {
        REG_PARLIO_RX_CLK_SRC_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_parlio_rx_clk_en(&mut self) -> REG_PARLIO_RX_CLK_EN_W<PERI_CLK_CTRL117_SPEC> {
        REG_PARLIO_RX_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bits 20:27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_parlio_rx_clk_div_num(
        &mut self,
    ) -> REG_PARLIO_RX_CLK_DIV_NUM_W<PERI_CLK_CTRL117_SPEC> {
        REG_PARLIO_RX_CLK_DIV_NUM_W::new(self, 20)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl117::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl117::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL117_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL117_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl117::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL117_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl117::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL117_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL117 to value 0x0001_0000"]
impl crate::Resettable for PERI_CLK_CTRL117_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}