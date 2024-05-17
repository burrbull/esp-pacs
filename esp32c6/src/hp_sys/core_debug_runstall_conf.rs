///Register `CORE_DEBUG_RUNSTALL_CONF` reader
pub type R = crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
///Register `CORE_DEBUG_RUNSTALL_CONF` writer
pub type W = crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
///Field `CORE_DEBUG_RUNSTALL_ENABLE` reader - Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
pub type CORE_DEBUG_RUNSTALL_ENABLE_R = crate::BitReader;
///Field `CORE_DEBUG_RUNSTALL_ENABLE` writer - Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
pub type CORE_DEBUG_RUNSTALL_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
    #[inline(always)]
    pub fn core_debug_runstall_enable(&self) -> CORE_DEBUG_RUNSTALL_ENABLE_R {
        CORE_DEBUG_RUNSTALL_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_DEBUG_RUNSTALL_CONF")
            .field("core_debug_runstall_enable", &self.core_debug_runstall_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
    #[inline(always)]
    #[must_use]
    pub fn core_debug_runstall_enable(
        &mut self,
    ) -> CORE_DEBUG_RUNSTALL_ENABLE_W<CORE_DEBUG_RUNSTALL_CONF_SPEC> {
        CORE_DEBUG_RUNSTALL_ENABLE_W::new(self, 0)
    }
}
/**Core Debug runstall configure register

You can [`read`](crate::generic::Reg::read) this register and get [`core_debug_runstall_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_DEBUG_RUNSTALL_CONF_SPEC;
impl crate::RegisterSpec for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_debug_runstall_conf::R`](R) reader structure
impl crate::Readable for CORE_DEBUG_RUNSTALL_CONF_SPEC {}
///`write(|w| ..)` method takes [`core_debug_runstall_conf::W`](W) writer structure
impl crate::Writable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_DEBUG_RUNSTALL_CONF to value 0
impl crate::Resettable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
