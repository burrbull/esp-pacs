#[doc = "Register `REGION3_ADDR_END` reader"]
pub type R = crate::R<REGION3_ADDR_END_SPEC>;
#[doc = "Register `REGION3_ADDR_END` writer"]
pub type W = crate::W<REGION3_ADDR_END_SPEC>;
#[doc = "Field `REGION3_ADDR_END` reader - End address of region3"]
pub type REGION3_ADDR_END_R = crate::FieldReader<u32>;
#[doc = "Field `REGION3_ADDR_END` writer - End address of region3"]
pub type REGION3_ADDR_END_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - End address of region3"]
    #[inline(always)]
    pub fn region3_addr_end(&self) -> REGION3_ADDR_END_R {
        REGION3_ADDR_END_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION3_ADDR_END")
            .field(
                "region3_addr_end",
                &format_args!("{}", self.region3_addr_end().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION3_ADDR_END_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region3"]
    #[inline(always)]
    #[must_use]
    pub fn region3_addr_end(&mut self) -> REGION3_ADDR_END_W<REGION3_ADDR_END_SPEC, 0> {
        REGION3_ADDR_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region3_addr_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region3_addr_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION3_ADDR_END_SPEC;
impl crate::RegisterSpec for REGION3_ADDR_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region3_addr_end::R`](R) reader structure"]
impl crate::Readable for REGION3_ADDR_END_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region3_addr_end::W`](W) writer structure"]
impl crate::Writable for REGION3_ADDR_END_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION3_ADDR_END to value 0xffff_ffff"]
impl crate::Resettable for REGION3_ADDR_END_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}