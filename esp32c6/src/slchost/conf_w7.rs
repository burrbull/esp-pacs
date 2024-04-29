#[doc = "Register `CONF_W7` reader"]
pub type R = crate::R<CONF_W7_SPEC>;
#[doc = "Register `CONF_W7` writer"]
pub type W = crate::W<CONF_W7_SPEC>;
#[doc = "Field `CONF28` reader - *******Description***********"]
pub type CONF28_R = crate::FieldReader;
#[doc = "Field `CONF28` writer - *******Description***********"]
pub type CONF28_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF29` reader - *******Description***********"]
pub type CONF29_R = crate::FieldReader;
#[doc = "Field `CONF29` writer - *******Description***********"]
pub type CONF29_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF30` reader - *******Description***********"]
pub type CONF30_R = crate::FieldReader;
#[doc = "Field `CONF30` writer - *******Description***********"]
pub type CONF30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF31` reader - *******Description***********"]
pub type CONF31_R = crate::FieldReader;
#[doc = "Field `CONF31` writer - *******Description***********"]
pub type CONF31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn conf28(&self) -> CONF28_R {
        CONF28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn conf29(&self) -> CONF29_R {
        CONF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn conf30(&self) -> CONF30_R {
        CONF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn conf31(&self) -> CONF31_R {
        CONF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W7")
            .field("conf28", &format_args!("{}", self.conf28().bits()))
            .field("conf29", &format_args!("{}", self.conf29().bits()))
            .field("conf30", &format_args!("{}", self.conf30().bits()))
            .field("conf31", &format_args!("{}", self.conf31().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf28(&mut self) -> CONF28_W<CONF_W7_SPEC> {
        CONF28_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf29(&mut self) -> CONF29_W<CONF_W7_SPEC> {
        CONF29_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf30(&mut self) -> CONF30_W<CONF_W7_SPEC> {
        CONF30_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf31(&mut self) -> CONF31_W<CONF_W7_SPEC> {
        CONF31_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W7_SPEC;
impl crate::RegisterSpec for CONF_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w7::R`](R) reader structure"]
impl crate::Readable for CONF_W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w7::W`](W) writer structure"]
impl crate::Writable for CONF_W7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W7 to value 0"]
impl crate::Resettable for CONF_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
