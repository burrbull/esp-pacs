#[doc = "Register `CONF_W4` reader"]
pub type R = crate::R<CONF_W4_SPEC>;
#[doc = "Register `CONF_W4` writer"]
pub type W = crate::W<CONF_W4_SPEC>;
#[doc = "Field `CONF16` reader - SLC timeout value"]
pub type CONF16_R = crate::FieldReader;
#[doc = "Field `CONF16` writer - SLC timeout value"]
pub type CONF16_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF17` reader - SLC timeout enable"]
pub type CONF17_R = crate::FieldReader;
#[doc = "Field `CONF17` writer - SLC timeout enable"]
pub type CONF17_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF18` reader - "]
pub type CONF18_R = crate::FieldReader;
#[doc = "Field `CONF18` writer - "]
pub type CONF18_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF19` reader - Interrupt to target CPU"]
pub type CONF19_R = crate::FieldReader;
#[doc = "Field `CONF19` writer - Interrupt to target CPU"]
pub type CONF19_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    pub fn conf16(&self) -> CONF16_R {
        CONF16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    pub fn conf17(&self) -> CONF17_R {
        CONF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf18(&self) -> CONF18_R {
        CONF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    pub fn conf19(&self) -> CONF19_R {
        CONF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W4")
            .field("conf16", &format_args!("{}", self.conf16().bits()))
            .field("conf17", &format_args!("{}", self.conf17().bits()))
            .field("conf18", &format_args!("{}", self.conf18().bits()))
            .field("conf19", &format_args!("{}", self.conf19().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn conf16(&mut self) -> CONF16_W<CONF_W4_SPEC> {
        CONF16_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn conf17(&mut self) -> CONF17_W<CONF_W4_SPEC> {
        CONF17_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf18(&mut self) -> CONF18_W<CONF_W4_SPEC> {
        CONF18_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    #[must_use]
    pub fn conf19(&mut self) -> CONF19_W<CONF_W4_SPEC> {
        CONF19_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W4_SPEC;
impl crate::RegisterSpec for CONF_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w4::R`](R) reader structure"]
impl crate::Readable for CONF_W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w4::W`](W) writer structure"]
impl crate::Writable for CONF_W4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W4 to value 0x01ff"]
impl crate::Resettable for CONF_W4_SPEC {
    const RESET_VALUE: u32 = 0x01ff;
}
