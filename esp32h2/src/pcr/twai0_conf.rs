///Register `TWAI0_CONF` reader
pub type R = crate::R<TWAI0_CONF_SPEC>;
///Register `TWAI0_CONF` writer
pub type W = crate::W<TWAI0_CONF_SPEC>;
///Field `TWAI0_CLK_EN` reader - Set 1 to enable twai0 apb clock
pub type TWAI0_CLK_EN_R = crate::BitReader;
///Field `TWAI0_CLK_EN` writer - Set 1 to enable twai0 apb clock
pub type TWAI0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TWAI0_RST_EN` reader - Set 0 to reset twai0 module
pub type TWAI0_RST_EN_R = crate::BitReader;
///Field `TWAI0_RST_EN` writer - Set 0 to reset twai0 module
pub type TWAI0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TWAI0_READY` reader - Query this field after reset twai0 module
pub type TWAI0_READY_R = crate::BitReader;
impl R {
    ///Bit 0 - Set 1 to enable twai0 apb clock
    #[inline(always)]
    pub fn twai0_clk_en(&self) -> TWAI0_CLK_EN_R {
        TWAI0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 0 to reset twai0 module
    #[inline(always)]
    pub fn twai0_rst_en(&self) -> TWAI0_RST_EN_R {
        TWAI0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Query this field after reset twai0 module
    #[inline(always)]
    pub fn twai0_ready(&self) -> TWAI0_READY_R {
        TWAI0_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI0_CONF")
            .field("twai0_clk_en", &self.twai0_clk_en())
            .field("twai0_rst_en", &self.twai0_rst_en())
            .field("twai0_ready", &self.twai0_ready())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable twai0 apb clock
    #[inline(always)]
    #[must_use]
    pub fn twai0_clk_en(&mut self) -> TWAI0_CLK_EN_W<TWAI0_CONF_SPEC> {
        TWAI0_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 0 to reset twai0 module
    #[inline(always)]
    #[must_use]
    pub fn twai0_rst_en(&mut self) -> TWAI0_RST_EN_W<TWAI0_CONF_SPEC> {
        TWAI0_RST_EN_W::new(self, 1)
    }
}
/**TWAI0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`twai0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twai0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TWAI0_CONF_SPEC;
impl crate::RegisterSpec for TWAI0_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`twai0_conf::R`](R) reader structure
impl crate::Readable for TWAI0_CONF_SPEC {}
///`write(|w| ..)` method takes [`twai0_conf::W`](W) writer structure
impl crate::Writable for TWAI0_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TWAI0_CONF to value 0x05
impl crate::Resettable for TWAI0_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
