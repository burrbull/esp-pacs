#[doc = "Register `STORE4` reader"]
pub type R = crate::R<STORE4_SPEC>;
#[doc = "Register `STORE4` writer"]
pub type W = crate::W<STORE4_SPEC>;
#[doc = "Field `LP_AON_STORE4` reader - need_des"]
pub type LP_AON_STORE4_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE4` writer - need_des"]
pub type LP_AON_STORE4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store4(&self) -> LP_AON_STORE4_R {
        LP_AON_STORE4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE4")
            .field(
                "lp_aon_store4",
                &format_args!("{}", self.lp_aon_store4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store4(&mut self) -> LP_AON_STORE4_W<STORE4_SPEC, 0> {
        LP_AON_STORE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE4_SPEC;
impl crate::RegisterSpec for STORE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store4::R`](R) reader structure"]
impl crate::Readable for STORE4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store4::W`](W) writer structure"]
impl crate::Writable for STORE4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE4 to value 0"]
impl crate::Resettable for STORE4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}