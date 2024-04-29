#[doc = "Register `TOKEN%s` reader"]
pub type R = crate::R<TOKEN_SPEC>;
#[doc = "Register `TOKEN%s` writer"]
pub type W = crate::W<TOKEN_SPEC>;
#[doc = "Field `WDATA` writer - "]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `WR` writer - "]
pub type WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INC` writer - "]
pub type INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INC_MODE` writer - "]
pub type INC_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOKEN` reader - "]
pub type TOKEN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn token(&self) -> TOKEN_R {
        TOKEN_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOKEN")
            .field("token", &format_args!("{}", self.token().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOKEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<TOKEN_SPEC> {
        WDATA_W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<TOKEN_SPEC> {
        WR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn inc(&mut self) -> INC_W<TOKEN_SPEC> {
        INC_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn inc_mode(&mut self) -> INC_MODE_W<TOKEN_SPEC> {
        INC_MODE_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`token::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOKEN_SPEC;
impl crate::RegisterSpec for TOKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`token::R`](R) reader structure"]
impl crate::Readable for TOKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`token::W`](W) writer structure"]
impl crate::Writable for TOKEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOKEN%s to value 0"]
impl crate::Resettable for TOKEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
