#[doc = "Register `CONF_W15` reader"]
pub type R = crate::R<CONF_W15_SPEC>;
#[doc = "Register `CONF_W15` writer"]
pub type W = crate::W<CONF_W15_SPEC>;
#[doc = "Field `CONF60` reader - "]
pub type CONF60_R = crate::FieldReader;
#[doc = "Field `CONF60` writer - "]
pub type CONF60_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF61` reader - "]
pub type CONF61_R = crate::FieldReader;
#[doc = "Field `CONF61` writer - "]
pub type CONF61_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF62` reader - "]
pub type CONF62_R = crate::FieldReader;
#[doc = "Field `CONF62` writer - "]
pub type CONF62_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF63` reader - "]
pub type CONF63_R = crate::FieldReader;
#[doc = "Field `CONF63` writer - "]
pub type CONF63_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn conf60(&self) -> CONF60_R {
        CONF60_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn conf61(&self) -> CONF61_R {
        CONF61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf62(&self) -> CONF62_R {
        CONF62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn conf63(&self) -> CONF63_R {
        CONF63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W15")
            .field("conf60", &format_args!("{}", self.conf60().bits()))
            .field("conf61", &format_args!("{}", self.conf61().bits()))
            .field("conf62", &format_args!("{}", self.conf62().bits()))
            .field("conf63", &format_args!("{}", self.conf63().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn conf60(&mut self) -> CONF60_W<CONF_W15_SPEC> {
        CONF60_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn conf61(&mut self) -> CONF61_W<CONF_W15_SPEC> {
        CONF61_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf62(&mut self) -> CONF62_W<CONF_W15_SPEC> {
        CONF62_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn conf63(&mut self) -> CONF63_W<CONF_W15_SPEC> {
        CONF63_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W15_SPEC;
impl crate::RegisterSpec for CONF_W15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w15::R`](R) reader structure"]
impl crate::Readable for CONF_W15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w15::W`](W) writer structure"]
impl crate::Writable for CONF_W15_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W15 to value 0"]
impl crate::Resettable for CONF_W15_SPEC {
    const RESET_VALUE: u32 = 0;
}
