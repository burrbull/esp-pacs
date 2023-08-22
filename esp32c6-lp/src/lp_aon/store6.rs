#[doc = "Register `STORE6` reader"]
pub type R = crate::R<STORE6_SPEC>;
#[doc = "Register `STORE6` writer"]
pub type W = crate::W<STORE6_SPEC>;
#[doc = "Field `LP_AON_STORE6` reader - need_des"]
pub type LP_AON_STORE6_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE6` writer - need_des"]
pub type LP_AON_STORE6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store6(&self) -> LP_AON_STORE6_R {
        LP_AON_STORE6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE6")
            .field(
                "lp_aon_store6",
                &format_args!("{}", self.lp_aon_store6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store6(&mut self) -> LP_AON_STORE6_W<STORE6_SPEC, 0> {
        LP_AON_STORE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE6_SPEC;
impl crate::RegisterSpec for STORE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store6::R`](R) reader structure"]
impl crate::Readable for STORE6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store6::W`](W) writer structure"]
impl crate::Writable for STORE6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE6 to value 0"]
impl crate::Resettable for STORE6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}