///Register `CONF` reader
pub type R = crate::R<CONF_SPEC>;
///Register `CONF` writer
pub type W = crate::W<CONF_SPEC>;
///Field `DUTY_RES` reader - reg_lstimer0_duty_res.
pub type DUTY_RES_R = crate::FieldReader;
///Field `DUTY_RES` writer - reg_lstimer0_duty_res.
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLK_DIV` reader - reg_clk_div_lstimer0.
pub type CLK_DIV_R = crate::FieldReader<u32>;
///Field `CLK_DIV` writer - reg_clk_div_lstimer0.
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `PAUSE` reader - reg_lstimer0_pause.
pub type PAUSE_R = crate::BitReader;
///Field `PAUSE` writer - reg_lstimer0_pause.
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST` reader - reg_lstimer0_rst.
pub type RST_R = crate::BitReader;
///Field `RST` writer - reg_lstimer0_rst.
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TICK_SEL` reader - reg_tick_sel_lstimer0.
pub type TICK_SEL_R = crate::BitReader;
///Field `TICK_SEL` writer - reg_tick_sel_lstimer0.
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARA_UP` writer - reg_lstimer0_para_up.
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - reg_lstimer0_duty_res.
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:21 - reg_clk_div_lstimer0.
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits >> 4) & 0x0003_ffff)
    }
    ///Bit 22 - reg_lstimer0_pause.
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - reg_lstimer0_rst.
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - reg_tick_sel_lstimer0.
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("duty_res", &self.duty_res())
            .field("clk_div", &self.clk_div())
            .field("pause", &self.pause())
            .field("rst", &self.rst())
            .field("tick_sel", &self.tick_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - reg_lstimer0_duty_res.
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    ///Bits 4:21 - reg_clk_div_lstimer0.
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<CONF_SPEC> {
        CLK_DIV_W::new(self, 4)
    }
    ///Bit 22 - reg_lstimer0_pause.
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<CONF_SPEC> {
        PAUSE_W::new(self, 22)
    }
    ///Bit 23 - reg_lstimer0_rst.
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CONF_SPEC> {
        RST_W::new(self, 23)
    }
    ///Bit 24 - reg_tick_sel_lstimer0.
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<CONF_SPEC> {
        TICK_SEL_W::new(self, 24)
    }
    ///Bit 25 - reg_lstimer0_para_up.
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<CONF_SPEC> {
        PARA_UP_W::new(self, 25)
    }
}
/**LEDC_LSTIMER0_CONF.

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
///`reset()` method sets CONF to value 0x0080_0000
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
