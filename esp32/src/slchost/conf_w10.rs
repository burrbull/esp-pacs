#[doc = "Register `CONF_W10` reader"]
pub type R = crate::R<CONF_W10_SPEC>;
#[doc = "Register `CONF_W10` writer"]
pub type W = crate::W<CONF_W10_SPEC>;
#[doc = "Field `CONF40` reader - "]
pub type CONF40_R = crate::FieldReader;
#[doc = "Field `CONF40` writer - "]
pub type CONF40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF41` reader - "]
pub type CONF41_R = crate::FieldReader;
#[doc = "Field `CONF41` writer - "]
pub type CONF41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF42` reader - "]
pub type CONF42_R = crate::FieldReader;
#[doc = "Field `CONF42` writer - "]
pub type CONF42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF43` reader - "]
pub type CONF43_R = crate::FieldReader;
#[doc = "Field `CONF43` writer - "]
pub type CONF43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn conf40(&self) -> CONF40_R {
        CONF40_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn conf41(&self) -> CONF41_R {
        CONF41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf42(&self) -> CONF42_R {
        CONF42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn conf43(&self) -> CONF43_R {
        CONF43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W10")
            .field("conf40", &format_args!("{}", self.conf40().bits()))
            .field("conf41", &format_args!("{}", self.conf41().bits()))
            .field("conf42", &format_args!("{}", self.conf42().bits()))
            .field("conf43", &format_args!("{}", self.conf43().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn conf40(&mut self) -> CONF40_W<CONF_W10_SPEC> {
        CONF40_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn conf41(&mut self) -> CONF41_W<CONF_W10_SPEC> {
        CONF41_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf42(&mut self) -> CONF42_W<CONF_W10_SPEC> {
        CONF42_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn conf43(&mut self) -> CONF43_W<CONF_W10_SPEC> {
        CONF43_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W10_SPEC;
impl crate::RegisterSpec for CONF_W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w10::R`](R) reader structure"]
impl crate::Readable for CONF_W10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w10::W`](W) writer structure"]
impl crate::Writable for CONF_W10_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W10 to value 0"]
impl crate::Resettable for CONF_W10_SPEC {
    const RESET_VALUE: u32 = 0;
}
