#[doc = "Register `WIN_CMD` reader"]
pub type R = crate::R<WIN_CMD_SPEC>;
#[doc = "Register `WIN_CMD` writer"]
pub type W = crate::W<WIN_CMD_SPEC>;
#[doc = "Field `WIN_CMD` reader - *******Description***********"]
pub type WIN_CMD_R = crate::FieldReader<u16>;
#[doc = "Field `WIN_CMD` writer - *******Description***********"]
pub type WIN_CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - *******Description***********"]
    #[inline(always)]
    pub fn win_cmd(&self) -> WIN_CMD_R {
        WIN_CMD_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN_CMD")
            .field("win_cmd", &format_args!("{}", self.win_cmd().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WIN_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn win_cmd(&mut self) -> WIN_CMD_W<WIN_CMD_SPEC> {
        WIN_CMD_W::new(self, 0)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIN_CMD_SPEC;
impl crate::RegisterSpec for WIN_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win_cmd::R`](R) reader structure"]
impl crate::Readable for WIN_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`win_cmd::W`](W) writer structure"]
impl crate::Writable for WIN_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN_CMD to value 0"]
impl crate::Resettable for WIN_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
