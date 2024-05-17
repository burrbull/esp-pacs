///Register `L2_CACHE_DATA_MEM_POWER_CTRL` reader
pub type R = crate::R<L2_CACHE_DATA_MEM_POWER_CTRL_SPEC>;
///Register `L2_CACHE_DATA_MEM_POWER_CTRL` writer
pub type W = crate::W<L2_CACHE_DATA_MEM_POWER_CTRL_SPEC>;
///Field `L2_CACHE_DATA_MEM_FORCE_ON` reader - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating.
pub type L2_CACHE_DATA_MEM_FORCE_ON_R = crate::BitReader;
///Field `L2_CACHE_DATA_MEM_FORCE_ON` writer - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating.
pub type L2_CACHE_DATA_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_CACHE_DATA_MEM_FORCE_PD` reader - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down
pub type L2_CACHE_DATA_MEM_FORCE_PD_R = crate::BitReader;
///Field `L2_CACHE_DATA_MEM_FORCE_PD` writer - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down
pub type L2_CACHE_DATA_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2_CACHE_DATA_MEM_FORCE_PU` reader - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up
pub type L2_CACHE_DATA_MEM_FORCE_PU_R = crate::BitReader;
///Field `L2_CACHE_DATA_MEM_FORCE_PU` writer - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up
pub type L2_CACHE_DATA_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 20 - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating.
    #[inline(always)]
    pub fn l2_cache_data_mem_force_on(&self) -> L2_CACHE_DATA_MEM_FORCE_ON_R {
        L2_CACHE_DATA_MEM_FORCE_ON_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down
    #[inline(always)]
    pub fn l2_cache_data_mem_force_pd(&self) -> L2_CACHE_DATA_MEM_FORCE_PD_R {
        L2_CACHE_DATA_MEM_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up
    #[inline(always)]
    pub fn l2_cache_data_mem_force_pu(&self) -> L2_CACHE_DATA_MEM_FORCE_PU_R {
        L2_CACHE_DATA_MEM_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_DATA_MEM_POWER_CTRL")
            .field("l2_cache_data_mem_force_on", &self.l2_cache_data_mem_force_on())
            .field("l2_cache_data_mem_force_pd", &self.l2_cache_data_mem_force_pd())
            .field("l2_cache_data_mem_force_pu", &self.l2_cache_data_mem_force_pu())
            .finish()
    }
}
impl W {
    ///Bit 20 - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating.
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_data_mem_force_on(
        &mut self,
    ) -> L2_CACHE_DATA_MEM_FORCE_ON_W<L2_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L2_CACHE_DATA_MEM_FORCE_ON_W::new(self, 20)
    }
    ///Bit 21 - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_data_mem_force_pd(
        &mut self,
    ) -> L2_CACHE_DATA_MEM_FORCE_PD_W<L2_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L2_CACHE_DATA_MEM_FORCE_PD_W::new(self, 21)
    }
    ///Bit 22 - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_data_mem_force_pu(
        &mut self,
    ) -> L2_CACHE_DATA_MEM_FORCE_PU_W<L2_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L2_CACHE_DATA_MEM_FORCE_PU_W::new(self, 22)
    }
}
/**Cache data memory power control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_data_mem_power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_data_mem_power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_DATA_MEM_POWER_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_DATA_MEM_POWER_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_data_mem_power_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_DATA_MEM_POWER_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_data_mem_power_ctrl::W`](W) writer structure
impl crate::Writable for L2_CACHE_DATA_MEM_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_DATA_MEM_POWER_CTRL to value 0x0050_0000
impl crate::Resettable for L2_CACHE_DATA_MEM_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0050_0000;
}
