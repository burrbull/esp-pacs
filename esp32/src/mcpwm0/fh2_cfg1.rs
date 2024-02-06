#[doc = "Register `FH2_CFG1` reader"]
pub type R = crate::R<FH2_CFG1_SPEC>;
#[doc = "Register `FH2_CFG1` writer"]
pub type W = crate::W<FH2_CFG1_SPEC>;
#[doc = "Field `FH2_CLR_OST` reader - "]
pub type FH2_CLR_OST_R = crate::BitReader;
#[doc = "Field `FH2_CLR_OST` writer - "]
pub type FH2_CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_CBCPULSE` reader - "]
pub type FH2_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `FH2_CBCPULSE` writer - "]
pub type FH2_CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FH2_FORCE_CBC` reader - "]
pub type FH2_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `FH2_FORCE_CBC` writer - "]
pub type FH2_FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FH2_FORCE_OST` reader - "]
pub type FH2_FORCE_OST_R = crate::BitReader;
#[doc = "Field `FH2_FORCE_OST` writer - "]
pub type FH2_FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_clr_ost(&self) -> FH2_CLR_OST_R {
        FH2_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh2_cbcpulse(&self) -> FH2_CBCPULSE_R {
        FH2_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_force_cbc(&self) -> FH2_FORCE_CBC_R {
        FH2_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_force_ost(&self) -> FH2_FORCE_OST_R {
        FH2_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH2_CFG1")
            .field("fh2_clr_ost", &format_args!("{}", self.fh2_clr_ost().bit()))
            .field(
                "fh2_cbcpulse",
                &format_args!("{}", self.fh2_cbcpulse().bits()),
            )
            .field(
                "fh2_force_cbc",
                &format_args!("{}", self.fh2_force_cbc().bit()),
            )
            .field(
                "fh2_force_ost",
                &format_args!("{}", self.fh2_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH2_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_clr_ost(&mut self) -> FH2_CLR_OST_W<FH2_CFG1_SPEC> {
        FH2_CLR_OST_W::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_cbcpulse(&mut self) -> FH2_CBCPULSE_W<FH2_CFG1_SPEC> {
        FH2_CBCPULSE_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_force_cbc(&mut self) -> FH2_FORCE_CBC_W<FH2_CFG1_SPEC> {
        FH2_FORCE_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn fh2_force_ost(&mut self) -> FH2_FORCE_OST_W<FH2_CFG1_SPEC> {
        FH2_FORCE_OST_W::new(self, 4)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh2_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh2_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH2_CFG1_SPEC;
impl crate::RegisterSpec for FH2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh2_cfg1::R`](R) reader structure"]
impl crate::Readable for FH2_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh2_cfg1::W`](W) writer structure"]
impl crate::Writable for FH2_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FH2_CFG1 to value 0"]
impl crate::Resettable for FH2_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
