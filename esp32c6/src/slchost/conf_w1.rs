#[doc = "Register `CONF_W1` reader"]
pub type R = crate::R<CONF_W1_SPEC>;
#[doc = "Register `CONF_W1` writer"]
pub type W = crate::W<CONF_W1_SPEC>;
#[doc = "Field `CONF4` reader - *******Description***********"]
pub type CONF4_R = crate::FieldReader;
#[doc = "Field `CONF4` writer - *******Description***********"]
pub type CONF4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF5` reader - *******Description***********"]
pub type CONF5_R = crate::FieldReader;
#[doc = "Field `CONF5` writer - *******Description***********"]
pub type CONF5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF6` reader - *******Description***********"]
pub type CONF6_R = crate::FieldReader;
#[doc = "Field `CONF6` writer - *******Description***********"]
pub type CONF6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF7` reader - *******Description***********"]
pub type CONF7_R = crate::FieldReader;
#[doc = "Field `CONF7` writer - *******Description***********"]
pub type CONF7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn conf4(&self) -> CONF4_R {
        CONF4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn conf5(&self) -> CONF5_R {
        CONF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn conf6(&self) -> CONF6_R {
        CONF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn conf7(&self) -> CONF7_R {
        CONF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W1")
            .field("conf4", &format_args!("{}", self.conf4().bits()))
            .field("conf5", &format_args!("{}", self.conf5().bits()))
            .field("conf6", &format_args!("{}", self.conf6().bits()))
            .field("conf7", &format_args!("{}", self.conf7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf4(&mut self) -> CONF4_W<CONF_W1_SPEC> {
        CONF4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf5(&mut self) -> CONF5_W<CONF_W1_SPEC> {
        CONF5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf6(&mut self) -> CONF6_W<CONF_W1_SPEC> {
        CONF6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf7(&mut self) -> CONF7_W<CONF_W1_SPEC> {
        CONF7_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W1_SPEC;
impl crate::RegisterSpec for CONF_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w1::R`](R) reader structure"]
impl crate::Readable for CONF_W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w1::W`](W) writer structure"]
impl crate::Writable for CONF_W1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W1 to value 0"]
impl crate::Resettable for CONF_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
