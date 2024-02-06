#[doc = "Register `IO_MUX` reader"]
pub type R = crate::R<IO_MUX_SPEC>;
#[doc = "Register `IO_MUX` writer"]
pub type W = crate::W<IO_MUX_SPEC>;
#[doc = "Field `PULL_LDO` reader - need_des"]
pub type PULL_LDO_R = crate::FieldReader;
#[doc = "Field `PULL_LDO` writer - need_des"]
pub type PULL_LDO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESET_DISABLE` reader - need_des"]
pub type RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `RESET_DISABLE` writer - need_des"]
pub type RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    pub fn pull_ldo(&self) -> PULL_LDO_R {
        PULL_LDO_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn reset_disable(&self) -> RESET_DISABLE_R {
        RESET_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX")
            .field("pull_ldo", &format_args!("{}", self.pull_ldo().bits()))
            .field(
                "reset_disable",
                &format_args!("{}", self.reset_disable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IO_MUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pull_ldo(&mut self) -> PULL_LDO_W<IO_MUX_SPEC> {
        PULL_LDO_W::new(self, 28)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn reset_disable(&mut self) -> RESET_DISABLE_W<IO_MUX_SPEC> {
        RESET_DISABLE_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_mux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_MUX_SPEC;
impl crate::RegisterSpec for IO_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_mux::R`](R) reader structure"]
impl crate::Readable for IO_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io_mux::W`](W) writer structure"]
impl crate::Writable for IO_MUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO_MUX to value 0"]
impl crate::Resettable for IO_MUX_SPEC {
    const RESET_VALUE: u32 = 0;
}
