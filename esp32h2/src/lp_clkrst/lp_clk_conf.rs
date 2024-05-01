///Register `LP_CLK_CONF` reader
pub type R = crate::R<LP_CLK_CONF_SPEC>;
///Register `LP_CLK_CONF` writer
pub type W = crate::W<LP_CLK_CONF_SPEC>;
///Field `SLOW_CLK_SEL` reader - need_des
pub type SLOW_CLK_SEL_R = crate::FieldReader;
///Field `SLOW_CLK_SEL` writer - need_des
pub type SLOW_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FAST_CLK_SEL` reader - need_des
pub type FAST_CLK_SEL_R = crate::FieldReader;
///Field `FAST_CLK_SEL` writer - need_des
pub type FAST_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LP_PERI_DIV_NUM` reader - need_des
pub type LP_PERI_DIV_NUM_R = crate::FieldReader;
///Field `LP_PERI_DIV_NUM` writer - need_des
pub type LP_PERI_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - need_des
    #[inline(always)]
    pub fn slow_clk_sel(&self) -> SLOW_CLK_SEL_R {
        SLOW_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - need_des
    #[inline(always)]
    pub fn fast_clk_sel(&self) -> FAST_CLK_SEL_R {
        FAST_CLK_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:11 - need_des
    #[inline(always)]
    pub fn lp_peri_div_num(&self) -> LP_PERI_DIV_NUM_R {
        LP_PERI_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_CONF")
            .field("slow_clk_sel", &self.slow_clk_sel())
            .field("fast_clk_sel", &self.fast_clk_sel())
            .field("lp_peri_div_num", &self.lp_peri_div_num())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn slow_clk_sel(&mut self) -> SLOW_CLK_SEL_W<LP_CLK_CONF_SPEC> {
        SLOW_CLK_SEL_W::new(self, 0)
    }
    ///Bits 2:3 - need_des
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_sel(&mut self) -> FAST_CLK_SEL_W<LP_CLK_CONF_SPEC> {
        FAST_CLK_SEL_W::new(self, 2)
    }
    ///Bits 4:11 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_div_num(&mut self) -> LP_PERI_DIV_NUM_W<LP_CLK_CONF_SPEC> {
        LP_PERI_DIV_NUM_W::new(self, 4)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CLK_CONF_SPEC;
impl crate::RegisterSpec for LP_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_clk_conf::R`](R) reader structure
impl crate::Readable for LP_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`lp_clk_conf::W`](W) writer structure
impl crate::Writable for LP_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_CLK_CONF to value 0x04
impl crate::Resettable for LP_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
