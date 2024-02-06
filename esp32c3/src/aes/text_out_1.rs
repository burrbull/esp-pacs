#[doc = "Register `TEXT_OUT_1` reader"]
pub type R = crate::R<TEXT_OUT_1_SPEC>;
#[doc = "Register `TEXT_OUT_1` writer"]
pub type W = crate::W<TEXT_OUT_1_SPEC>;
#[doc = "Field `TEXT_OUT_1` reader - This bits stores text_out_1 that is a part of result text material."]
pub type TEXT_OUT_1_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_OUT_1` writer - This bits stores text_out_1 that is a part of result text material."]
pub type TEXT_OUT_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_out_1 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_1(&self) -> TEXT_OUT_1_R {
        TEXT_OUT_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_OUT_1")
            .field("text_out_1", &format_args!("{}", self.text_out_1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEXT_OUT_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_out_1 that is a part of result text material."]
    #[inline(always)]
    #[must_use]
    pub fn text_out_1(&mut self) -> TEXT_OUT_1_W<TEXT_OUT_1_SPEC> {
        TEXT_OUT_1_W::new(self, 0)
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
#[doc = "result text material text_out_1 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEXT_OUT_1_SPEC;
impl crate::RegisterSpec for TEXT_OUT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_out_1::R`](R) reader structure"]
impl crate::Readable for TEXT_OUT_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`text_out_1::W`](W) writer structure"]
impl crate::Writable for TEXT_OUT_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEXT_OUT_1 to value 0"]
impl crate::Resettable for TEXT_OUT_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
