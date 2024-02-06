#[doc = "Register `LP_ANA_LP_INT_ENA` reader"]
pub type R = crate::R<LP_ANA_LP_INT_ENA_SPEC>;
#[doc = "Register `LP_ANA_LP_INT_ENA` writer"]
pub type W = crate::W<LP_ANA_LP_INT_ENA_SPEC>;
#[doc = "Field `LP_ANA_BOD_MODE0_LP_INT_ENA` reader - need_des"]
pub type LP_ANA_BOD_MODE0_LP_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_ANA_BOD_MODE0_LP_INT_ENA` writer - need_des"]
pub type LP_ANA_BOD_MODE0_LP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_bod_mode0_lp_int_ena(&self) -> LP_ANA_BOD_MODE0_LP_INT_ENA_R {
        LP_ANA_BOD_MODE0_LP_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_LP_INT_ENA")
            .field(
                "lp_ana_bod_mode0_lp_int_ena",
                &format_args!("{}", self.lp_ana_bod_mode0_lp_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_LP_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_bod_mode0_lp_int_ena(
        &mut self,
    ) -> LP_ANA_BOD_MODE0_LP_INT_ENA_W<LP_ANA_LP_INT_ENA_SPEC> {
        LP_ANA_BOD_MODE0_LP_INT_ENA_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_lp_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_lp_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_LP_INT_ENA_SPEC;
impl crate::RegisterSpec for LP_ANA_LP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LP_ANA_LP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LP_ANA_LP_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_LP_INT_ENA to value 0"]
impl crate::Resettable for LP_ANA_LP_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
