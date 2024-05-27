///Register `IOMUX_CONF` reader
pub type R = crate::R<IOMUX_CONF_SPEC>;
///Register `IOMUX_CONF` writer
pub type W = crate::W<IOMUX_CONF_SPEC>;
///Field `IOMUX_CLK_EN` reader - Set 1 to enable iomux apb clock
pub type IOMUX_CLK_EN_R = crate::BitReader;
///Field `IOMUX_CLK_EN` writer - Set 1 to enable iomux apb clock
pub type IOMUX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOMUX_RST_EN` reader - Set 0 to reset iomux module
pub type IOMUX_RST_EN_R = crate::BitReader;
///Field `IOMUX_RST_EN` writer - Set 0 to reset iomux module
pub type IOMUX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable iomux apb clock
    #[inline(always)]
    pub fn iomux_clk_en(&self) -> IOMUX_CLK_EN_R {
        IOMUX_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset iomux module
    #[inline(always)]
    pub fn iomux_rst_en(&self) -> IOMUX_RST_EN_R {
        IOMUX_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUX_CONF")
            .field("iomux_clk_en", &self.iomux_clk_en())
            .field("iomux_rst_en", &self.iomux_rst_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable iomux apb clock
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_en(&mut self) -> IOMUX_CLK_EN_W<IOMUX_CONF_SPEC> {
        IOMUX_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset iomux module
    #[inline(always)]
    #[must_use]
    pub fn iomux_rst_en(&mut self) -> IOMUX_RST_EN_W<IOMUX_CONF_SPEC> {
        IOMUX_RST_EN_W::new(self, 1)
    }
}
/**IOMUX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`iomux_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomux_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IOMUX_CONF_SPEC;
impl crate::RegisterSpec for IOMUX_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`iomux_conf::R`](R) reader structure
impl crate::Readable for IOMUX_CONF_SPEC {}
///`write(|w| ..)` method takes [`iomux_conf::W`](W) writer structure
impl crate::Writable for IOMUX_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IOMUX_CONF to value 0x01
impl crate::Resettable for IOMUX_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
