#[doc = "Register `DCACHE_FREEZE` reader"]
pub type R = crate::R<DCACHE_FREEZE_SPEC>;
#[doc = "Register `DCACHE_FREEZE` writer"]
pub type W = crate::W<DCACHE_FREEZE_SPEC>;
#[doc = "Field `ENA` reader - The bit is used to enable dcache freeze mode"]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - The bit is used to enable dcache freeze mode"]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - The bit is used to indicate dcache freeze success"]
pub type DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable dcache freeze mode"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate dcache freeze success"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_FREEZE")
            .field("ena", &self.ena().bit())
            .field("mode", &self.mode().bit())
            .field("done", &self.done().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_FREEZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable dcache freeze mode"]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<DCACHE_FREEZE_SPEC> {
        ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<DCACHE_FREEZE_SPEC> {
        MODE_W::new(self, 1)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_freeze::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_freeze::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_FREEZE_SPEC;
impl crate::RegisterSpec for DCACHE_FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_freeze::R`](R) reader structure"]
impl crate::Readable for DCACHE_FREEZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_freeze::W`](W) writer structure"]
impl crate::Writable for DCACHE_FREEZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_FREEZE to value 0x04"]
impl crate::Resettable for DCACHE_FREEZE_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
