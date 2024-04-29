#[doc = "Register `CONF_W13` reader"]
pub type R = crate::R<CONF_W13_SPEC>;
#[doc = "Register `CONF_W13` writer"]
pub type W = crate::W<CONF_W13_SPEC>;
#[doc = "Field `CONF52` reader - *******Description***********"]
pub type CONF52_R = crate::FieldReader;
#[doc = "Field `CONF52` writer - *******Description***********"]
pub type CONF52_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF53` reader - *******Description***********"]
pub type CONF53_R = crate::FieldReader;
#[doc = "Field `CONF53` writer - *******Description***********"]
pub type CONF53_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF54` reader - *******Description***********"]
pub type CONF54_R = crate::FieldReader;
#[doc = "Field `CONF54` writer - *******Description***********"]
pub type CONF54_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF55` reader - *******Description***********"]
pub type CONF55_R = crate::FieldReader;
#[doc = "Field `CONF55` writer - *******Description***********"]
pub type CONF55_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn conf52(&self) -> CONF52_R {
        CONF52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn conf53(&self) -> CONF53_R {
        CONF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn conf54(&self) -> CONF54_R {
        CONF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn conf55(&self) -> CONF55_R {
        CONF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W13")
            .field("conf52", &format_args!("{}", self.conf52().bits()))
            .field("conf53", &format_args!("{}", self.conf53().bits()))
            .field("conf54", &format_args!("{}", self.conf54().bits()))
            .field("conf55", &format_args!("{}", self.conf55().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf52(&mut self) -> CONF52_W<CONF_W13_SPEC> {
        CONF52_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf53(&mut self) -> CONF53_W<CONF_W13_SPEC> {
        CONF53_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf54(&mut self) -> CONF54_W<CONF_W13_SPEC> {
        CONF54_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf55(&mut self) -> CONF55_W<CONF_W13_SPEC> {
        CONF55_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W13_SPEC;
impl crate::RegisterSpec for CONF_W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w13::R`](R) reader structure"]
impl crate::Readable for CONF_W13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w13::W`](W) writer structure"]
impl crate::Writable for CONF_W13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W13 to value 0"]
impl crate::Resettable for CONF_W13_SPEC {
    const RESET_VALUE: u32 = 0;
}
