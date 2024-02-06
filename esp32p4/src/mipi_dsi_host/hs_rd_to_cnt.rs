#[doc = "Register `HS_RD_TO_CNT` reader"]
pub type R = crate::R<HS_RD_TO_CNT_SPEC>;
#[doc = "Register `HS_RD_TO_CNT` writer"]
pub type W = crate::W<HS_RD_TO_CNT_SPEC>;
#[doc = "Field `HS_RD_TO_CNT` reader - NA"]
pub type HS_RD_TO_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `HS_RD_TO_CNT` writer - NA"]
pub type HS_RD_TO_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn hs_rd_to_cnt(&self) -> HS_RD_TO_CNT_R {
        HS_RD_TO_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HS_RD_TO_CNT")
            .field(
                "hs_rd_to_cnt",
                &format_args!("{}", self.hs_rd_to_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HS_RD_TO_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hs_rd_to_cnt(&mut self) -> HS_RD_TO_CNT_W<HS_RD_TO_CNT_SPEC> {
        HS_RD_TO_CNT_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_rd_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_rd_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HS_RD_TO_CNT_SPEC;
impl crate::RegisterSpec for HS_RD_TO_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_rd_to_cnt::R`](R) reader structure"]
impl crate::Readable for HS_RD_TO_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hs_rd_to_cnt::W`](W) writer structure"]
impl crate::Writable for HS_RD_TO_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HS_RD_TO_CNT to value 0"]
impl crate::Resettable for HS_RD_TO_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
