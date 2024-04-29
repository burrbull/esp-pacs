#[doc = "Register `CONF_W11` reader"]
pub type R = crate::R<CONF_W11_SPEC>;
#[doc = "Register `CONF_W11` writer"]
pub type W = crate::W<CONF_W11_SPEC>;
#[doc = "Field `CONF44` reader - *******Description***********"]
pub type CONF44_R = crate::FieldReader;
#[doc = "Field `CONF44` writer - *******Description***********"]
pub type CONF44_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF45` reader - *******Description***********"]
pub type CONF45_R = crate::FieldReader;
#[doc = "Field `CONF45` writer - *******Description***********"]
pub type CONF45_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF46` reader - *******Description***********"]
pub type CONF46_R = crate::FieldReader;
#[doc = "Field `CONF46` writer - *******Description***********"]
pub type CONF46_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF47` reader - *******Description***********"]
pub type CONF47_R = crate::FieldReader;
#[doc = "Field `CONF47` writer - *******Description***********"]
pub type CONF47_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn conf44(&self) -> CONF44_R {
        CONF44_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn conf45(&self) -> CONF45_R {
        CONF45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn conf46(&self) -> CONF46_R {
        CONF46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn conf47(&self) -> CONF47_R {
        CONF47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W11")
            .field("conf44", &format_args!("{}", self.conf44().bits()))
            .field("conf45", &format_args!("{}", self.conf45().bits()))
            .field("conf46", &format_args!("{}", self.conf46().bits()))
            .field("conf47", &format_args!("{}", self.conf47().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf44(&mut self) -> CONF44_W<CONF_W11_SPEC> {
        CONF44_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf45(&mut self) -> CONF45_W<CONF_W11_SPEC> {
        CONF45_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf46(&mut self) -> CONF46_W<CONF_W11_SPEC> {
        CONF46_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf47(&mut self) -> CONF47_W<CONF_W11_SPEC> {
        CONF47_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W11_SPEC;
impl crate::RegisterSpec for CONF_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w11::R`](R) reader structure"]
impl crate::Readable for CONF_W11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w11::W`](W) writer structure"]
impl crate::Writable for CONF_W11_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W11 to value 0"]
impl crate::Resettable for CONF_W11_SPEC {
    const RESET_VALUE: u32 = 0;
}
