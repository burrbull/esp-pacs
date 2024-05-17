///Register `PRO_ICACHE_TAG_POWER_CTRL` reader
pub type R = crate::R<PRO_ICACHE_TAG_POWER_CTRL_SPEC>;
///Register `PRO_ICACHE_TAG_POWER_CTRL` writer
pub type W = crate::W<PRO_ICACHE_TAG_POWER_CTRL_SPEC>;
///Field `PRO_ICACHE_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of icache tag memory. 1: close gating, 0: open clock gating.
pub type PRO_ICACHE_TAG_MEM_FORCE_ON_R = crate::BitReader;
///Field `PRO_ICACHE_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of icache tag memory. 1: close gating, 0: open clock gating.
pub type PRO_ICACHE_TAG_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_ICACHE_TAG_MEM_FORCE_PD` reader - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power down
pub type PRO_ICACHE_TAG_MEM_FORCE_PD_R = crate::BitReader;
///Field `PRO_ICACHE_TAG_MEM_FORCE_PD` writer - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power down
pub type PRO_ICACHE_TAG_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_ICACHE_TAG_MEM_FORCE_PU` reader - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power up
pub type PRO_ICACHE_TAG_MEM_FORCE_PU_R = crate::BitReader;
///Field `PRO_ICACHE_TAG_MEM_FORCE_PU` writer - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power up
pub type PRO_ICACHE_TAG_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The bit is used to close clock gating of icache tag memory. 1: close gating, 0: open clock gating.
    #[inline(always)]
    pub fn pro_icache_tag_mem_force_on(&self) -> PRO_ICACHE_TAG_MEM_FORCE_ON_R {
        PRO_ICACHE_TAG_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power down
    #[inline(always)]
    pub fn pro_icache_tag_mem_force_pd(&self) -> PRO_ICACHE_TAG_MEM_FORCE_PD_R {
        PRO_ICACHE_TAG_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power up
    #[inline(always)]
    pub fn pro_icache_tag_mem_force_pu(&self) -> PRO_ICACHE_TAG_MEM_FORCE_PU_R {
        PRO_ICACHE_TAG_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_TAG_POWER_CTRL")
            .field("pro_icache_tag_mem_force_on", &self.pro_icache_tag_mem_force_on())
            .field("pro_icache_tag_mem_force_pd", &self.pro_icache_tag_mem_force_pd())
            .field("pro_icache_tag_mem_force_pu", &self.pro_icache_tag_mem_force_pu())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to close clock gating of icache tag memory. 1: close gating, 0: open clock gating.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_tag_mem_force_on(
        &mut self,
    ) -> PRO_ICACHE_TAG_MEM_FORCE_ON_W<PRO_ICACHE_TAG_POWER_CTRL_SPEC> {
        PRO_ICACHE_TAG_MEM_FORCE_ON_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power down
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_tag_mem_force_pd(
        &mut self,
    ) -> PRO_ICACHE_TAG_MEM_FORCE_PD_W<PRO_ICACHE_TAG_POWER_CTRL_SPEC> {
        PRO_ICACHE_TAG_MEM_FORCE_PD_W::new(self, 1)
    }
    ///Bit 2 - The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power up
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_tag_mem_force_pu(
        &mut self,
    ) -> PRO_ICACHE_TAG_MEM_FORCE_PU_W<PRO_ICACHE_TAG_POWER_CTRL_SPEC> {
        PRO_ICACHE_TAG_MEM_FORCE_PU_W::new(self, 2)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_tag_power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_tag_power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_ICACHE_TAG_POWER_CTRL_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_TAG_POWER_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_icache_tag_power_ctrl::R`](R) reader structure
impl crate::Readable for PRO_ICACHE_TAG_POWER_CTRL_SPEC {}
///`write(|w| ..)` method takes [`pro_icache_tag_power_ctrl::W`](W) writer structure
impl crate::Writable for PRO_ICACHE_TAG_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_ICACHE_TAG_POWER_CTRL to value 0x05
impl crate::Resettable for PRO_ICACHE_TAG_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
