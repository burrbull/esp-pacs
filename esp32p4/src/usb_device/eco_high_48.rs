#[doc = "Register `ECO_HIGH_48` reader"]
pub type R = crate::R<ECO_HIGH_48_SPEC>;
#[doc = "Register `ECO_HIGH_48` writer"]
pub type W = crate::W<ECO_HIGH_48_SPEC>;
#[doc = "Field `RND_ECO_HIGH_48` reader - Reserved."]
pub type RND_ECO_HIGH_48_R = crate::FieldReader<u32>;
#[doc = "Field `RND_ECO_HIGH_48` writer - Reserved."]
pub type RND_ECO_HIGH_48_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn rnd_eco_high_48(&self) -> RND_ECO_HIGH_48_R {
        RND_ECO_HIGH_48_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_HIGH_48")
            .field(
                "rnd_eco_high_48",
                &format_args!("{}", self.rnd_eco_high_48().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECO_HIGH_48_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rnd_eco_high_48(&mut self) -> RND_ECO_HIGH_48_W<ECO_HIGH_48_SPEC> {
        RND_ECO_HIGH_48_W::new(self, 0)
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
#[doc = "Reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_high_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_high_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_HIGH_48_SPEC;
impl crate::RegisterSpec for ECO_HIGH_48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_high_48::R`](R) reader structure"]
impl crate::Readable for ECO_HIGH_48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_high_48::W`](W) writer structure"]
impl crate::Writable for ECO_HIGH_48_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECO_HIGH_48 to value 0xffff_ffff"]
impl crate::Resettable for ECO_HIGH_48_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
