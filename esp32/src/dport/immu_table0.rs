#[doc = "Register `IMMU_TABLE0` reader"]
pub type R = crate::R<IMMU_TABLE0_SPEC>;
#[doc = "Register `IMMU_TABLE0` writer"]
pub type W = crate::W<IMMU_TABLE0_SPEC>;
#[doc = "Field `IMMU_TABLE0` reader - "]
pub type IMMU_TABLE0_R = crate::FieldReader;
#[doc = "Field `IMMU_TABLE0` writer - "]
pub type IMMU_TABLE0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table0(&self) -> IMMU_TABLE0_R {
        IMMU_TABLE0_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_TABLE0")
            .field(
                "immu_table0",
                &format_args!("{}", self.immu_table0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMMU_TABLE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn immu_table0(&mut self) -> IMMU_TABLE0_W<IMMU_TABLE0_SPEC> {
        IMMU_TABLE0_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`immu_table0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_table0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMMU_TABLE0_SPEC;
impl crate::RegisterSpec for IMMU_TABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`immu_table0::R`](R) reader structure"]
impl crate::Readable for IMMU_TABLE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`immu_table0::W`](W) writer structure"]
impl crate::Writable for IMMU_TABLE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMMU_TABLE0 to value 0"]
impl crate::Resettable for IMMU_TABLE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
