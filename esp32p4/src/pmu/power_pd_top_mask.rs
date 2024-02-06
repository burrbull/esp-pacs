#[doc = "Register `POWER_PD_TOP_MASK` reader"]
pub type R = crate::R<POWER_PD_TOP_MASK_SPEC>;
#[doc = "Register `POWER_PD_TOP_MASK` writer"]
pub type W = crate::W<POWER_PD_TOP_MASK_SPEC>;
#[doc = "Field `XPD_TOP_MASK` reader - need_des"]
pub type XPD_TOP_MASK_R = crate::FieldReader;
#[doc = "Field `XPD_TOP_MASK` writer - need_des"]
pub type XPD_TOP_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PD_TOP_MASK` reader - need_des"]
pub type PD_TOP_MASK_R = crate::FieldReader;
#[doc = "Field `PD_TOP_MASK` writer - need_des"]
pub type PD_TOP_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn xpd_top_mask(&self) -> XPD_TOP_MASK_R {
        XPD_TOP_MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_top_mask(&self) -> PD_TOP_MASK_R {
        PD_TOP_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_TOP_MASK")
            .field(
                "xpd_top_mask",
                &format_args!("{}", self.xpd_top_mask().bits()),
            )
            .field(
                "pd_top_mask",
                &format_args!("{}", self.pd_top_mask().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_TOP_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_top_mask(&mut self) -> XPD_TOP_MASK_W<POWER_PD_TOP_MASK_SPEC> {
        XPD_TOP_MASK_W::new(self, 0)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_top_mask(&mut self) -> PD_TOP_MASK_W<POWER_PD_TOP_MASK_SPEC> {
        PD_TOP_MASK_W::new(self, 27)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_pd_top_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_top_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_PD_TOP_MASK_SPEC;
impl crate::RegisterSpec for POWER_PD_TOP_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_top_mask::R`](R) reader structure"]
impl crate::Readable for POWER_PD_TOP_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_pd_top_mask::W`](W) writer structure"]
impl crate::Writable for POWER_PD_TOP_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_PD_TOP_MASK to value 0"]
impl crate::Resettable for POWER_PD_TOP_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
