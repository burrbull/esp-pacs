#[doc = "Register `HOST_SLC_APBWIN_WDATA` reader"]
pub type R = crate::R<HOST_SLC_APBWIN_WDATA_SPEC>;
#[doc = "Register `HOST_SLC_APBWIN_WDATA` writer"]
pub type W = crate::W<HOST_SLC_APBWIN_WDATA_SPEC>;
#[doc = "Field `HOST_SLC_APBWIN_WDATA` reader - "]
pub type HOST_SLC_APBWIN_WDATA_R = crate::FieldReader<u32>;
#[doc = "Field `HOST_SLC_APBWIN_WDATA` writer - "]
pub type HOST_SLC_APBWIN_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc_apbwin_wdata(&self) -> HOST_SLC_APBWIN_WDATA_R {
        HOST_SLC_APBWIN_WDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC_APBWIN_WDATA")
            .field(
                "host_slc_apbwin_wdata",
                &format_args!("{}", self.host_slc_apbwin_wdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC_APBWIN_WDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc_apbwin_wdata(&mut self) -> HOST_SLC_APBWIN_WDATA_W<HOST_SLC_APBWIN_WDATA_SPEC> {
        HOST_SLC_APBWIN_WDATA_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slc_apbwin_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc_apbwin_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC_APBWIN_WDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC_APBWIN_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc_apbwin_wdata::R`](R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_WDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slc_apbwin_wdata::W`](W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_WDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLC_APBWIN_WDATA to value 0"]
impl crate::Resettable for HOST_SLC_APBWIN_WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
