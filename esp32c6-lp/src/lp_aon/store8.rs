#[doc = "Register `STORE8` reader"]
pub type R = crate::R<STORE8_SPEC>;
#[doc = "Register `STORE8` writer"]
pub type W = crate::W<STORE8_SPEC>;
#[doc = "Field `LP_AON_STORE8` reader - need_des"]
pub type LP_AON_STORE8_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AON_STORE8` writer - need_des"]
pub type LP_AON_STORE8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_aon_store8(&self) -> LP_AON_STORE8_R {
        LP_AON_STORE8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE8")
            .field(
                "lp_aon_store8",
                &format_args!("{}", self.lp_aon_store8().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store8(&mut self) -> LP_AON_STORE8_W<STORE8_SPEC, 0> {
        LP_AON_STORE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`store8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STORE8_SPEC;
impl crate::RegisterSpec for STORE8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store8::R`](R) reader structure"]
impl crate::Readable for STORE8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`store8::W`](W) writer structure"]
impl crate::Writable for STORE8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE8 to value 0"]
impl crate::Resettable for STORE8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}