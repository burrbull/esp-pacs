#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTL_SPEC>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTL_SPEC>;
#[doc = "Field `STOPPCLK` reader - "]
pub type STOPPCLK_R = crate::BitReader;
#[doc = "Field `STOPPCLK` writer - "]
pub type STOPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - "]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - "]
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRCLMP` reader - "]
pub type PWRCLMP_R = crate::BitReader;
#[doc = "Field `PWRCLMP` writer - "]
pub type PWRCLMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPDWNMODULE` reader - "]
pub type RSTPDWNMODULE_R = crate::BitReader;
#[doc = "Field `RSTPDWNMODULE` writer - "]
pub type RSTPDWNMODULE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSLEEP` reader - "]
pub type PHYSLEEP_R = crate::BitReader;
#[doc = "Field `L1SUSPENDED` reader - "]
pub type L1SUSPENDED_R = crate::BitReader;
#[doc = "Field `RESETAFTERSUSP` reader - "]
pub type RESETAFTERSUSP_R = crate::BitReader;
#[doc = "Field `RESETAFTERSUSP` writer - "]
pub type RESETAFTERSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stoppclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PWRCLMP_R {
        PWRCLMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RSTPDWNMODULE_R {
        RSTPDWNMODULE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn l1suspended(&self) -> L1SUSPENDED_R {
        L1SUSPENDED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn resetaftersusp(&self) -> RESETAFTERSUSP_R {
        RESETAFTERSUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stoppclk", &format_args!("{}", self.stoppclk().bit()))
            .field("gatehclk", &format_args!("{}", self.gatehclk().bit()))
            .field("pwrclmp", &format_args!("{}", self.pwrclmp().bit()))
            .field(
                "rstpdwnmodule",
                &format_args!("{}", self.rstpdwnmodule().bit()),
            )
            .field("physleep", &format_args!("{}", self.physleep().bit()))
            .field("l1suspended", &format_args!("{}", self.l1suspended().bit()))
            .field(
                "resetaftersusp",
                &format_args!("{}", self.resetaftersusp().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PCGCCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn stoppclk(&mut self) -> STOPPCLK_W<PCGCCTL_SPEC> {
        STOPPCLK_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<PCGCCTL_SPEC> {
        GATEHCLK_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrclmp(&mut self) -> PWRCLMP_W<PCGCCTL_SPEC> {
        PWRCLMP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rstpdwnmodule(&mut self) -> RSTPDWNMODULE_W<PCGCCTL_SPEC> {
        RSTPDWNMODULE_W::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn resetaftersusp(&mut self) -> RESETAFTERSUSP_W<PCGCCTL_SPEC> {
        RESETAFTERSUSP_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
