#[doc = "Register `CONF_W8` reader"]
pub type R = crate::R<CONF_W8_SPEC>;
#[doc = "Register `CONF_W8` writer"]
pub type W = crate::W<CONF_W8_SPEC>;
#[doc = "Field `CONF32` reader - *******Description***********"]
pub type CONF32_R = crate::FieldReader;
#[doc = "Field `CONF32` writer - *******Description***********"]
pub type CONF32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF33` reader - *******Description***********"]
pub type CONF33_R = crate::FieldReader;
#[doc = "Field `CONF33` writer - *******Description***********"]
pub type CONF33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF34` reader - *******Description***********"]
pub type CONF34_R = crate::FieldReader;
#[doc = "Field `CONF34` writer - *******Description***********"]
pub type CONF34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF35` reader - *******Description***********"]
pub type CONF35_R = crate::FieldReader;
#[doc = "Field `CONF35` writer - *******Description***********"]
pub type CONF35_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn conf32(&self) -> CONF32_R {
        CONF32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn conf33(&self) -> CONF33_R {
        CONF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn conf34(&self) -> CONF34_R {
        CONF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn conf35(&self) -> CONF35_R {
        CONF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W8")
            .field("conf32", &format_args!("{}", self.conf32().bits()))
            .field("conf33", &format_args!("{}", self.conf33().bits()))
            .field("conf34", &format_args!("{}", self.conf34().bits()))
            .field("conf35", &format_args!("{}", self.conf35().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf32(&mut self) -> CONF32_W<CONF_W8_SPEC> {
        CONF32_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf33(&mut self) -> CONF33_W<CONF_W8_SPEC> {
        CONF33_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf34(&mut self) -> CONF34_W<CONF_W8_SPEC> {
        CONF34_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn conf35(&mut self) -> CONF35_W<CONF_W8_SPEC> {
        CONF35_W::new(self, 24)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W8_SPEC;
impl crate::RegisterSpec for CONF_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w8::R`](R) reader structure"]
impl crate::Readable for CONF_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w8::W`](W) writer structure"]
impl crate::Writable for CONF_W8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W8 to value 0"]
impl crate::Resettable for CONF_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
