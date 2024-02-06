#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFG_SPEC>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFG_SPEC>;
#[doc = "Field `H_FSLSPCLKSEL` reader - "]
pub type H_FSLSPCLKSEL_R = crate::FieldReader;
#[doc = "Field `H_FSLSPCLKSEL` writer - "]
pub type H_FSLSPCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_FSLSSUPP` reader - "]
pub type H_FSLSSUPP_R = crate::BitReader;
#[doc = "Field `H_FSLSSUPP` writer - "]
pub type H_FSLSSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ENA32KHZS` reader - "]
pub type H_ENA32KHZS_R = crate::BitReader;
#[doc = "Field `H_ENA32KHZS` writer - "]
pub type H_ENA32KHZS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESCDMA` reader - "]
pub type H_DESCDMA_R = crate::BitReader;
#[doc = "Field `H_DESCDMA` writer - "]
pub type H_DESCDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRLISTEN` reader - "]
pub type H_FRLISTEN_R = crate::FieldReader;
#[doc = "Field `H_FRLISTEN` writer - "]
pub type H_FRLISTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `H_PERSCHEDENA` reader - "]
pub type H_PERSCHEDENA_R = crate::BitReader;
#[doc = "Field `H_PERSCHEDENA` writer - "]
pub type H_PERSCHEDENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_MODECHTIMEN` reader - "]
pub type H_MODECHTIMEN_R = crate::BitReader;
#[doc = "Field `H_MODECHTIMEN` writer - "]
pub type H_MODECHTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn h_fslspclksel(&self) -> H_FSLSPCLKSEL_R {
        H_FSLSPCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_fslssupp(&self) -> H_FSLSSUPP_R {
        H_FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_ena32khzs(&self) -> H_ENA32KHZS_R {
        H_ENA32KHZS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn h_descdma(&self) -> H_DESCDMA_R {
        H_DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn h_frlisten(&self) -> H_FRLISTEN_R {
        H_FRLISTEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn h_perschedena(&self) -> H_PERSCHEDENA_R {
        H_PERSCHEDENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_modechtimen(&self) -> H_MODECHTIMEN_R {
        H_MODECHTIMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFG")
            .field(
                "h_fslspclksel",
                &format_args!("{}", self.h_fslspclksel().bits()),
            )
            .field("h_fslssupp", &format_args!("{}", self.h_fslssupp().bit()))
            .field("h_ena32khzs", &format_args!("{}", self.h_ena32khzs().bit()))
            .field("h_descdma", &format_args!("{}", self.h_descdma().bit()))
            .field("h_frlisten", &format_args!("{}", self.h_frlisten().bits()))
            .field(
                "h_perschedena",
                &format_args!("{}", self.h_perschedena().bit()),
            )
            .field(
                "h_modechtimen",
                &format_args!("{}", self.h_modechtimen().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn h_fslspclksel(&mut self) -> H_FSLSPCLKSEL_W<HCFG_SPEC> {
        H_FSLSPCLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_fslssupp(&mut self) -> H_FSLSSUPP_W<HCFG_SPEC> {
        H_FSLSSUPP_W::new(self, 2)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_ena32khzs(&mut self) -> H_ENA32KHZS_W<HCFG_SPEC> {
        H_ENA32KHZS_W::new(self, 7)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn h_descdma(&mut self) -> H_DESCDMA_W<HCFG_SPEC> {
        H_DESCDMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn h_frlisten(&mut self) -> H_FRLISTEN_W<HCFG_SPEC> {
        H_FRLISTEN_W::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn h_perschedena(&mut self) -> H_PERSCHEDENA_W<HCFG_SPEC> {
        H_PERSCHEDENA_W::new(self, 26)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_modechtimen(&mut self) -> H_MODECHTIMEN_W<HCFG_SPEC> {
        H_MODECHTIMEN_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
