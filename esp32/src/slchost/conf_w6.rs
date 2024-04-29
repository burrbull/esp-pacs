#[doc = "Register `CONF_W6` reader"]
pub type R = crate::R<CONF_W6_SPEC>;
#[doc = "Register `CONF_W6` writer"]
pub type W = crate::W<CONF_W6_SPEC>;
#[doc = "Field `CONF24` reader - "]
pub type CONF24_R = crate::FieldReader;
#[doc = "Field `CONF24` writer - "]
pub type CONF24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF25` reader - "]
pub type CONF25_R = crate::FieldReader;
#[doc = "Field `CONF25` writer - "]
pub type CONF25_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF26` reader - "]
pub type CONF26_R = crate::FieldReader;
#[doc = "Field `CONF26` writer - "]
pub type CONF26_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF27` reader - "]
pub type CONF27_R = crate::FieldReader;
#[doc = "Field `CONF27` writer - "]
pub type CONF27_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn conf24(&self) -> CONF24_R {
        CONF24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn conf25(&self) -> CONF25_R {
        CONF25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf26(&self) -> CONF26_R {
        CONF26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn conf27(&self) -> CONF27_R {
        CONF27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W6")
            .field("conf24", &format_args!("{}", self.conf24().bits()))
            .field("conf25", &format_args!("{}", self.conf25().bits()))
            .field("conf26", &format_args!("{}", self.conf26().bits()))
            .field("conf27", &format_args!("{}", self.conf27().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn conf24(&mut self) -> CONF24_W<CONF_W6_SPEC> {
        CONF24_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn conf25(&mut self) -> CONF25_W<CONF_W6_SPEC> {
        CONF25_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf26(&mut self) -> CONF26_W<CONF_W6_SPEC> {
        CONF26_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn conf27(&mut self) -> CONF27_W<CONF_W6_SPEC> {
        CONF27_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W6_SPEC;
impl crate::RegisterSpec for CONF_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w6::R`](R) reader structure"]
impl crate::Readable for CONF_W6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w6::W`](W) writer structure"]
impl crate::Writable for CONF_W6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W6 to value 0"]
impl crate::Resettable for CONF_W6_SPEC {
    const RESET_VALUE: u32 = 0;
}
