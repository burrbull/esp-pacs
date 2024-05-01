///Register `IOMUX_CLK_CONF` reader
pub type R = crate::R<IOMUX_CLK_CONF_SPEC>;
///Register `IOMUX_CLK_CONF` writer
pub type W = crate::W<IOMUX_CLK_CONF_SPEC>;
///Field `IOMUX_FUNC_CLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
pub type IOMUX_FUNC_CLK_SEL_R = crate::FieldReader;
///Field `IOMUX_FUNC_CLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
pub type IOMUX_FUNC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IOMUX_FUNC_CLK_EN` reader - Set 1 to enable iomux function clock
pub type IOMUX_FUNC_CLK_EN_R = crate::BitReader;
///Field `IOMUX_FUNC_CLK_EN` writer - Set 1 to enable iomux function clock
pub type IOMUX_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
    #[inline(always)]
    pub fn iomux_func_clk_sel(&self) -> IOMUX_FUNC_CLK_SEL_R {
        IOMUX_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Set 1 to enable iomux function clock
    #[inline(always)]
    pub fn iomux_func_clk_en(&self) -> IOMUX_FUNC_CLK_EN_R {
        IOMUX_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUX_CLK_CONF")
            .field("iomux_func_clk_sel", &self.iomux_func_clk_sel())
            .field("iomux_func_clk_en", &self.iomux_func_clk_en())
            .finish()
    }
}
impl W {
    ///Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
    #[inline(always)]
    #[must_use]
    pub fn iomux_func_clk_sel(&mut self) -> IOMUX_FUNC_CLK_SEL_W<IOMUX_CLK_CONF_SPEC> {
        IOMUX_FUNC_CLK_SEL_W::new(self, 20)
    }
    ///Bit 22 - Set 1 to enable iomux function clock
    #[inline(always)]
    #[must_use]
    pub fn iomux_func_clk_en(&mut self) -> IOMUX_FUNC_CLK_EN_W<IOMUX_CLK_CONF_SPEC> {
        IOMUX_FUNC_CLK_EN_W::new(self, 22)
    }
}
/**IOMUX_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`iomux_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomux_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IOMUX_CLK_CONF_SPEC;
impl crate::RegisterSpec for IOMUX_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`iomux_clk_conf::R`](R) reader structure
impl crate::Readable for IOMUX_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`iomux_clk_conf::W`](W) writer structure
impl crate::Writable for IOMUX_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IOMUX_CLK_CONF to value 0x0070_0000
impl crate::Resettable for IOMUX_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0070_0000;
}
