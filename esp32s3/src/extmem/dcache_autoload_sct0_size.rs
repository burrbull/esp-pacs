#[doc = "Register `DCACHE_AUTOLOAD_SCT0_SIZE` reader"]
pub type R = crate::R<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Register `DCACHE_AUTOLOAD_SCT0_SIZE` writer"]
pub type W = crate::W<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_SIZE` reader - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
pub type DCACHE_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_SIZE` writer - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
pub type DCACHE_AUTOLOAD_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_size(&self) -> DCACHE_AUTOLOAD_SCT0_SIZE_R {
        DCACHE_AUTOLOAD_SCT0_SIZE_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_AUTOLOAD_SCT0_SIZE")
            .field(
                "dcache_autoload_sct0_size",
                &format_args!("{}", self.dcache_autoload_sct0_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:26 - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_sct0_size(
        &mut self,
    ) -> DCACHE_AUTOLOAD_SCT0_SIZE_W<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC> {
        DCACHE_AUTOLOAD_SCT0_SIZE_W::new(self, 0)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct0_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct0_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_autoload_sct0_size::R`](R) reader structure"]
impl crate::Readable for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_autoload_sct0_size::W`](W) writer structure"]
impl crate::Writable for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
