///Register `PHY_IF_CFG` reader
pub type R = crate::R<PHY_IF_CFG_SPEC>;
///Register `PHY_IF_CFG` writer
pub type W = crate::W<PHY_IF_CFG_SPEC>;
///Field `N_LANES` reader - NA
pub type N_LANES_R = crate::FieldReader;
///Field `N_LANES` writer - NA
pub type N_LANES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PHY_STOP_WAIT_TIME` reader - NA
pub type PHY_STOP_WAIT_TIME_R = crate::FieldReader;
///Field `PHY_STOP_WAIT_TIME` writer - NA
pub type PHY_STOP_WAIT_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - NA
    #[inline(always)]
    pub fn n_lanes(&self) -> N_LANES_R {
        N_LANES_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    pub fn phy_stop_wait_time(&self) -> PHY_STOP_WAIT_TIME_R {
        PHY_STOP_WAIT_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_IF_CFG")
            .field("n_lanes", &self.n_lanes())
            .field("phy_stop_wait_time", &self.phy_stop_wait_time())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - NA
    #[inline(always)]
    #[must_use]
    pub fn n_lanes(&mut self) -> N_LANES_W<PHY_IF_CFG_SPEC> {
        N_LANES_W::new(self, 0)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    #[must_use]
    pub fn phy_stop_wait_time(&mut self) -> PHY_STOP_WAIT_TIME_W<PHY_IF_CFG_SPEC> {
        PHY_STOP_WAIT_TIME_W::new(self, 8)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_if_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_if_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHY_IF_CFG_SPEC;
impl crate::RegisterSpec for PHY_IF_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`phy_if_cfg::R`](R) reader structure
impl crate::Readable for PHY_IF_CFG_SPEC {}
///`write(|w| ..)` method takes [`phy_if_cfg::W`](W) writer structure
impl crate::Writable for PHY_IF_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PHY_IF_CFG to value 0x01
impl crate::Resettable for PHY_IF_CFG_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
