#[doc = "Register `CONF_W14` reader"]
pub type R = crate::R<CONF_W14_SPEC>;
#[doc = "Register `CONF_W14` writer"]
pub type W = crate::W<CONF_W14_SPEC>;
#[doc = "Field `CONF56` reader - "]
pub type CONF56_R = crate::FieldReader;
#[doc = "Field `CONF56` writer - "]
pub type CONF56_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF57` reader - "]
pub type CONF57_R = crate::FieldReader;
#[doc = "Field `CONF57` writer - "]
pub type CONF57_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF58` reader - "]
pub type CONF58_R = crate::FieldReader;
#[doc = "Field `CONF58` writer - "]
pub type CONF58_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONF59` reader - "]
pub type CONF59_R = crate::FieldReader;
#[doc = "Field `CONF59` writer - "]
pub type CONF59_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn conf56(&self) -> CONF56_R {
        CONF56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn conf57(&self) -> CONF57_R {
        CONF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn conf58(&self) -> CONF58_R {
        CONF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn conf59(&self) -> CONF59_R {
        CONF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W14")
            .field("conf56", &format_args!("{}", self.conf56().bits()))
            .field("conf57", &format_args!("{}", self.conf57().bits()))
            .field("conf58", &format_args!("{}", self.conf58().bits()))
            .field("conf59", &format_args!("{}", self.conf59().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn conf56(&mut self) -> CONF56_W<CONF_W14_SPEC> {
        CONF56_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn conf57(&mut self) -> CONF57_W<CONF_W14_SPEC> {
        CONF57_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn conf58(&mut self) -> CONF58_W<CONF_W14_SPEC> {
        CONF58_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn conf59(&mut self) -> CONF59_W<CONF_W14_SPEC> {
        CONF59_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W14_SPEC;
impl crate::RegisterSpec for CONF_W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w14::R`](R) reader structure"]
impl crate::Readable for CONF_W14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w14::W`](W) writer structure"]
impl crate::Writable for CONF_W14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF_W14 to value 0"]
impl crate::Resettable for CONF_W14_SPEC {
    const RESET_VALUE: u32 = 0;
}
