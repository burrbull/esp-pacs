#[doc = "Register `RC32K_CNTL` reader"]
pub type R = crate::R<RC32K_CNTL_SPEC>;
#[doc = "Register `RC32K_CNTL` writer"]
pub type W = crate::W<RC32K_CNTL_SPEC>;
#[doc = "Field `RC32K_DFREQ` reader - need_des"]
pub type RC32K_DFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `RC32K_DFREQ` writer - need_des"]
pub type RC32K_DFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn rc32k_dfreq(&self) -> RC32K_DFREQ_R {
        RC32K_DFREQ_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RC32K_CNTL")
            .field(
                "rc32k_dfreq",
                &format_args!("{}", self.rc32k_dfreq().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RC32K_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rc32k_dfreq(&mut self) -> RC32K_DFREQ_W<RC32K_CNTL_SPEC, 22> {
        RC32K_DFREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32k_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32k_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC32K_CNTL_SPEC;
impl crate::RegisterSpec for RC32K_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32k_cntl::R`](R) reader structure"]
impl crate::Readable for RC32K_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc32k_cntl::W`](W) writer structure"]
impl crate::Writable for RC32K_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC32K_CNTL to value 0x2b00_0000"]
impl crate::Resettable for RC32K_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b00_0000;
}