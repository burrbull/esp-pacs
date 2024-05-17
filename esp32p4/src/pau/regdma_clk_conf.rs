///Register `REGDMA_CLK_CONF` reader
pub type R = crate::R<REGDMA_CLK_CONF_SPEC>;
///Register `REGDMA_CLK_CONF` writer
pub type W = crate::W<REGDMA_CLK_CONF_SPEC>;
///Field `CLK_EN` reader - clock enable
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - clock enable
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - clock enable
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CLK_CONF").field("clk_en", &self.clk_en()).finish()
    }
}
impl W {
    ///Bit 0 - clock enable
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<REGDMA_CLK_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
/**Clock control register

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGDMA_CLK_CONF_SPEC;
impl crate::RegisterSpec for REGDMA_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`regdma_clk_conf::R`](R) reader structure
impl crate::Readable for REGDMA_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`regdma_clk_conf::W`](W) writer structure
impl crate::Writable for REGDMA_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGDMA_CLK_CONF to value 0
impl crate::Resettable for REGDMA_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
