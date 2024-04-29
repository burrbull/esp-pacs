#[doc = "Register `CONF_W2` reader"]
pub type R = crate::R<CONF_W2_SPEC>;
#[doc = "Register `CONF_W2` writer"]
pub type W = crate::W<CONF_W2_SPEC>;
#[doc = "Field `CONF8` reader - "]
pub type CONF8_R = crate::FieldReader;
#[doc = "Field `CONF8` writer - "]
pub type CONF8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF9` reader - "]
pub type CONF9_R = crate::FieldReader;
#[doc = "Field `CONF9` writer - "]
pub type CONF9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF10` reader - "]
pub type CONF10_R = crate::FieldReader;
#[doc = "Field `CONF10` writer - "]
pub type CONF10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF11` reader - "]
pub type CONF11_R = crate::FieldReader;
#[doc = "Field `CONF11` writer - "]
pub type CONF11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn conf8(&self) -> CONF8_R {
        CONF8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn conf9(&self) -> CONF9_R {
        CONF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf10(&self) -> CONF10_R {
        CONF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn conf11(&self) -> CONF11_R {
        CONF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W2")
            .field("conf8", &format_args!("{}", self.conf8().bits()))
            .field("conf9", &format_args!("{}", self.conf9().bits()))
            .field("conf10", &format_args!("{}", self.conf10().bits()))
            .field("conf11", &format_args!("{}", self.conf11().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn conf8(&mut self) -> CONF8_W<CONF_W2_SPEC> {
        CONF8_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn conf9(&mut self) -> CONF9_W<CONF_W2_SPEC> {
        CONF9_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf10(&mut self) -> CONF10_W<CONF_W2_SPEC> {
        CONF10_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn conf11(&mut self) -> CONF11_W<CONF_W2_SPEC> {
        CONF11_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W2_SPEC;
impl crate::RegisterSpec for CONF_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w2::R`](R) reader structure"]
impl crate::Readable for CONF_W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w2::W`](W) writer structure"]
impl crate::Writable for CONF_W2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W2 to value 0"]
impl crate::Resettable for CONF_W2_SPEC {
    const RESET_VALUE: u32 = 0;
}
