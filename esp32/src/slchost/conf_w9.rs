#[doc = "Register `CONF_W9` reader"]
pub type R = crate::R<CONF_W9_SPEC>;
#[doc = "Register `CONF_W9` writer"]
pub type W = crate::W<CONF_W9_SPEC>;
#[doc = "Field `CONF36` reader - "]
pub type CONF36_R = crate::FieldReader;
#[doc = "Field `CONF36` writer - "]
pub type CONF36_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF37` reader - "]
pub type CONF37_R = crate::FieldReader;
#[doc = "Field `CONF37` writer - "]
pub type CONF37_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF38` reader - "]
pub type CONF38_R = crate::FieldReader;
#[doc = "Field `CONF38` writer - "]
pub type CONF38_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF39` reader - "]
pub type CONF39_R = crate::FieldReader;
#[doc = "Field `CONF39` writer - "]
pub type CONF39_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn conf36(&self) -> CONF36_R {
        CONF36_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn conf37(&self) -> CONF37_R {
        CONF37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf38(&self) -> CONF38_R {
        CONF38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn conf39(&self) -> CONF39_R {
        CONF39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W9")
            .field("conf36", &format_args!("{}", self.conf36().bits()))
            .field("conf37", &format_args!("{}", self.conf37().bits()))
            .field("conf38", &format_args!("{}", self.conf38().bits()))
            .field("conf39", &format_args!("{}", self.conf39().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn conf36(&mut self) -> CONF36_W<CONF_W9_SPEC> {
        CONF36_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn conf37(&mut self) -> CONF37_W<CONF_W9_SPEC> {
        CONF37_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf38(&mut self) -> CONF38_W<CONF_W9_SPEC> {
        CONF38_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn conf39(&mut self) -> CONF39_W<CONF_W9_SPEC> {
        CONF39_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W9_SPEC;
impl crate::RegisterSpec for CONF_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w9::R`](R) reader structure"]
impl crate::Readable for CONF_W9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w9::W`](W) writer structure"]
impl crate::Writable for CONF_W9_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W9 to value 0"]
impl crate::Resettable for CONF_W9_SPEC {
    const RESET_VALUE: u32 = 0;
}
