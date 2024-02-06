#[doc = "Register `DCACHE_PRELOCK_SCT1_ADDR` reader"]
pub type R = crate::R<DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Register `DCACHE_PRELOCK_SCT1_ADDR` writer"]
pub type W = crate::W<DCACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_ADDR` reader - The bits are used to configure the second start virtual address of data prelock, which is combined with DCACHE_PRELOCK_SCT1_SIZE_REG"]
pub type DCACHE_PRELOCK_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_ADDR` writer - The bits are used to configure the second start virtual address of data prelock, which is combined with DCACHE_PRELOCK_SCT1_SIZE_REG"]
pub type DCACHE_PRELOCK_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the second start virtual address of data prelock, which is combined with DCACHE_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct1_addr(&self) -> DCACHE_PRELOCK_SCT1_ADDR_R {
        DCACHE_PRELOCK_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOCK_SCT1_ADDR")
            .field(
                "dcache_prelock_sct1_addr",
                &format_args!("{}", self.dcache_prelock_sct1_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_PRELOCK_SCT1_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the second start virtual address of data prelock, which is combined with DCACHE_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct1_addr(
        &mut self,
    ) -> DCACHE_PRELOCK_SCT1_ADDR_W<DCACHE_PRELOCK_SCT1_ADDR_SPEC> {
        DCACHE_PRELOCK_SCT1_ADDR_W::new(self, 0)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_sct1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_sct1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOCK_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_prelock_sct1_addr::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_SCT1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_prelock_sct1_addr::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_SCT1_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_SCT1_ADDR to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_SCT1_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
