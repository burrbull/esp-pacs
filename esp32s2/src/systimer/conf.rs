///Register `CONF` reader
pub type R = crate::R<CONF_SPEC>;
///Register `CONF` writer
pub type W = crate::W<CONF_SPEC>;
///Field `CLK_FO` reader - System timer clock force enable.
pub type CLK_FO_R = crate::BitReader;
///Field `CLK_FO` writer - System timer clock force enable.
pub type CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - Register clock enable.
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - Register clock enable.
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - System timer clock force enable.
    #[inline(always)]
    pub fn clk_fo(&self) -> CLK_FO_R {
        CLK_FO_R::new((self.bits & 1) != 0)
    }
    ///Bit 31 - Register clock enable.
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("clk_fo", &self.clk_fo())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - System timer clock force enable.
    #[inline(always)]
    #[must_use]
    pub fn clk_fo(&mut self) -> CLK_FO_W<CONF_SPEC> {
        CLK_FO_W::new(self, 0)
    }
    ///Bit 31 - Register clock enable.
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
/**Configure system timer clock

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf::R`](R) reader structure
impl crate::Readable for CONF_SPEC {}
///`write(|w| ..)` method takes [`conf::W`](W) writer structure
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF to value 0
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
