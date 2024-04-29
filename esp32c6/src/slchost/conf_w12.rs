#[doc = "Register `CONF_W12` reader"]
pub type R = crate::R<CONF_W12_SPEC>;
#[doc = "Register `CONF_W12` writer"]
pub type W = crate::W<CONF_W12_SPEC>;
#[doc = "Field `CONF48` reader - *******Description***********"]
pub type CONF48_R = crate::FieldReader;
#[doc = "Field `CONF48` writer - *******Description***********"]
pub type CONF48_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF49` reader - *******Description***********"]
pub type CONF49_R = crate::FieldReader;
#[doc = "Field `CONF49` writer - *******Description***********"]
pub type CONF49_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF50` reader - *******Description***********"]
pub type CONF50_R = crate::FieldReader;
#[doc = "Field `CONF50` writer - *******Description***********"]
pub type CONF50_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF51` reader - *******Description***********"]
pub type CONF51_R = crate::FieldReader;
#[doc = "Field `CONF51` writer - *******Description***********"]
pub type CONF51_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn conf48(&self) -> CONF48_R {
        CONF48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn conf49(&self) -> CONF49_R {
        CONF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn conf50(&self) -> CONF50_R {
        CONF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn conf51(&self) -> CONF51_R {
        CONF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W12")
            .field("conf48", &format_args!("{}", self.conf48().bits()))
            .field("conf49", &format_args!("{}", self.conf49().bits()))
            .field("conf50", &format_args!("{}", self.conf50().bits()))
            .field("conf51", &format_args!("{}", self.conf51().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf48(&mut self) -> CONF48_W<CONF_W12_SPEC> {
        CONF48_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf49(&mut self) -> CONF49_W<CONF_W12_SPEC> {
        CONF49_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf50(&mut self) -> CONF50_W<CONF_W12_SPEC> {
        CONF50_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf51(&mut self) -> CONF51_W<CONF_W12_SPEC> {
        CONF51_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W12_SPEC;
impl crate::RegisterSpec for CONF_W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w12::R`](R) reader structure"]
impl crate::Readable for CONF_W12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w12::W`](W) writer structure"]
impl crate::Writable for CONF_W12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W12 to value 0"]
impl crate::Resettable for CONF_W12_SPEC {
    const RESET_VALUE: u32 = 0;
}
