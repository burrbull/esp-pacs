#[doc = "Register `FLASH_ACE1_ATTR` reader"]
pub type R = crate::R<FLASH_ACE1_ATTR_SPEC>;
#[doc = "Register `FLASH_ACE1_ATTR` writer"]
pub type W = crate::W<FLASH_ACE1_ATTR_SPEC>;
#[doc = "Field `FLASH_ACE1_ATTR` reader - ******* Description ***********"]
pub type FLASH_ACE1_ATTR_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_ACE1_ATTR` writer - ******* Description ***********"]
pub type FLASH_ACE1_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn flash_ace1_attr(&self) -> FLASH_ACE1_ATTR_R {
        FLASH_ACE1_ATTR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE1_ATTR")
            .field(
                "flash_ace1_attr",
                &format_args!("{}", self.flash_ace1_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_ACE1_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn flash_ace1_attr(&mut self) -> FLASH_ACE1_ATTR_W<FLASH_ACE1_ATTR_SPEC> {
        FLASH_ACE1_ATTR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE1_ATTR_SPEC;
impl crate::RegisterSpec for FLASH_ACE1_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace1_attr::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE1_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace1_attr::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE1_ATTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ACE1_ATTR to value 0xff"]
impl crate::Resettable for FLASH_ACE1_ATTR_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
