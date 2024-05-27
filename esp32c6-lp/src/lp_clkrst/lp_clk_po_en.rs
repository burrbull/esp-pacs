///Register `LP_CLK_PO_EN` reader
pub type R = crate::R<LP_CLK_PO_EN_SPEC>;
///Register `LP_CLK_PO_EN` writer
pub type W = crate::W<LP_CLK_PO_EN_SPEC>;
///Field `AON_SLOW_OEN` reader - need_des
pub type AON_SLOW_OEN_R = crate::BitReader;
///Field `AON_SLOW_OEN` writer - need_des
pub type AON_SLOW_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AON_FAST_OEN` reader - need_des
pub type AON_FAST_OEN_R = crate::BitReader;
///Field `AON_FAST_OEN` writer - need_des
pub type AON_FAST_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOSC_OEN` reader - need_des
pub type SOSC_OEN_R = crate::BitReader;
///Field `SOSC_OEN` writer - need_des
pub type SOSC_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FOSC_OEN` reader - need_des
pub type FOSC_OEN_R = crate::BitReader;
///Field `FOSC_OEN` writer - need_des
pub type FOSC_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSC32K_OEN` reader - need_des
pub type OSC32K_OEN_R = crate::BitReader;
///Field `OSC32K_OEN` writer - need_des
pub type OSC32K_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTAL32K_OEN` reader - need_des
pub type XTAL32K_OEN_R = crate::BitReader;
///Field `XTAL32K_OEN` writer - need_des
pub type XTAL32K_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_EFUSE_OEN` reader - need_des
pub type CORE_EFUSE_OEN_R = crate::BitReader;
///Field `CORE_EFUSE_OEN` writer - need_des
pub type CORE_EFUSE_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLOW_OEN` reader - need_des
pub type SLOW_OEN_R = crate::BitReader;
///Field `SLOW_OEN` writer - need_des
pub type SLOW_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FAST_OEN` reader - need_des
pub type FAST_OEN_R = crate::BitReader;
///Field `FAST_OEN` writer - need_des
pub type FAST_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG_OEN` reader - need_des
pub type RNG_OEN_R = crate::BitReader;
///Field `RNG_OEN` writer - need_des
pub type RNG_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPBUS_OEN` reader - need_des
pub type LPBUS_OEN_R = crate::BitReader;
///Field `LPBUS_OEN` writer - need_des
pub type LPBUS_OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn aon_slow_oen(&self) -> AON_SLOW_OEN_R {
        AON_SLOW_OEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn aon_fast_oen(&self) -> AON_FAST_OEN_R {
        AON_FAST_OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - need_des
    #[inline(always)]
    pub fn sosc_oen(&self) -> SOSC_OEN_R {
        SOSC_OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - need_des
    #[inline(always)]
    pub fn fosc_oen(&self) -> FOSC_OEN_R {
        FOSC_OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - need_des
    #[inline(always)]
    pub fn osc32k_oen(&self) -> OSC32K_OEN_R {
        OSC32K_OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    pub fn xtal32k_oen(&self) -> XTAL32K_OEN_R {
        XTAL32K_OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - need_des
    #[inline(always)]
    pub fn core_efuse_oen(&self) -> CORE_EFUSE_OEN_R {
        CORE_EFUSE_OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - need_des
    #[inline(always)]
    pub fn slow_oen(&self) -> SLOW_OEN_R {
        SLOW_OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - need_des
    #[inline(always)]
    pub fn fast_oen(&self) -> FAST_OEN_R {
        FAST_OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - need_des
    #[inline(always)]
    pub fn rng_oen(&self) -> RNG_OEN_R {
        RNG_OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - need_des
    #[inline(always)]
    pub fn lpbus_oen(&self) -> LPBUS_OEN_R {
        LPBUS_OEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_PO_EN")
            .field("aon_slow_oen", &self.aon_slow_oen())
            .field("aon_fast_oen", &self.aon_fast_oen())
            .field("sosc_oen", &self.sosc_oen())
            .field("fosc_oen", &self.fosc_oen())
            .field("osc32k_oen", &self.osc32k_oen())
            .field("xtal32k_oen", &self.xtal32k_oen())
            .field("core_efuse_oen", &self.core_efuse_oen())
            .field("slow_oen", &self.slow_oen())
            .field("fast_oen", &self.fast_oen())
            .field("rng_oen", &self.rng_oen())
            .field("lpbus_oen", &self.lpbus_oen())
            .finish()
    }
}
impl W {
    ///Bit 0 - need_des
    #[inline(always)]
    #[must_use]
    pub fn aon_slow_oen(&mut self) -> AON_SLOW_OEN_W<LP_CLK_PO_EN_SPEC> {
        AON_SLOW_OEN_W::new(self, 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn aon_fast_oen(&mut self) -> AON_FAST_OEN_W<LP_CLK_PO_EN_SPEC> {
        AON_FAST_OEN_W::new(self, 1)
    }
    ///Bit 2 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sosc_oen(&mut self) -> SOSC_OEN_W<LP_CLK_PO_EN_SPEC> {
        SOSC_OEN_W::new(self, 2)
    }
    ///Bit 3 - need_des
    #[inline(always)]
    #[must_use]
    pub fn fosc_oen(&mut self) -> FOSC_OEN_W<LP_CLK_PO_EN_SPEC> {
        FOSC_OEN_W::new(self, 3)
    }
    ///Bit 4 - need_des
    #[inline(always)]
    #[must_use]
    pub fn osc32k_oen(&mut self) -> OSC32K_OEN_W<LP_CLK_PO_EN_SPEC> {
        OSC32K_OEN_W::new(self, 4)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_oen(&mut self) -> XTAL32K_OEN_W<LP_CLK_PO_EN_SPEC> {
        XTAL32K_OEN_W::new(self, 5)
    }
    ///Bit 6 - need_des
    #[inline(always)]
    #[must_use]
    pub fn core_efuse_oen(&mut self) -> CORE_EFUSE_OEN_W<LP_CLK_PO_EN_SPEC> {
        CORE_EFUSE_OEN_W::new(self, 6)
    }
    ///Bit 7 - need_des
    #[inline(always)]
    #[must_use]
    pub fn slow_oen(&mut self) -> SLOW_OEN_W<LP_CLK_PO_EN_SPEC> {
        SLOW_OEN_W::new(self, 7)
    }
    ///Bit 8 - need_des
    #[inline(always)]
    #[must_use]
    pub fn fast_oen(&mut self) -> FAST_OEN_W<LP_CLK_PO_EN_SPEC> {
        FAST_OEN_W::new(self, 8)
    }
    ///Bit 9 - need_des
    #[inline(always)]
    #[must_use]
    pub fn rng_oen(&mut self) -> RNG_OEN_W<LP_CLK_PO_EN_SPEC> {
        RNG_OEN_W::new(self, 9)
    }
    ///Bit 10 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lpbus_oen(&mut self) -> LPBUS_OEN_W<LP_CLK_PO_EN_SPEC> {
        LPBUS_OEN_W::new(self, 10)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_po_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_po_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_CLK_PO_EN_SPEC;
impl crate::RegisterSpec for LP_CLK_PO_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_clk_po_en::R`](R) reader structure
impl crate::Readable for LP_CLK_PO_EN_SPEC {}
///`write(|w| ..)` method takes [`lp_clk_po_en::W`](W) writer structure
impl crate::Writable for LP_CLK_PO_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_CLK_PO_EN to value 0x07ff
impl crate::Resettable for LP_CLK_PO_EN_SPEC {
    const RESET_VALUE: u32 = 0x07ff;
}
