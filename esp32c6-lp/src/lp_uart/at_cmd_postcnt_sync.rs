#[doc = "Register `AT_CMD_POSTCNT_SYNC` reader"]
pub type R = crate::R<AT_CMD_POSTCNT_SYNC_SPEC>;
#[doc = "Register `AT_CMD_POSTCNT_SYNC` writer"]
pub type W = crate::W<AT_CMD_POSTCNT_SYNC_SPEC>;
#[doc = "Field `POST_IDLE_NUM` reader - This register is used to configure the duration time between the last at_cmd and the next data."]
pub type POST_IDLE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `POST_IDLE_NUM` writer - This register is used to configure the duration time between the last at_cmd and the next data."]
pub type POST_IDLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the last at_cmd and the next data."]
    #[inline(always)]
    pub fn post_idle_num(&self) -> POST_IDLE_NUM_R {
        POST_IDLE_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_POSTCNT_SYNC")
            .field("post_idle_num", &self.post_idle_num().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AT_CMD_POSTCNT_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the duration time between the last at_cmd and the next data."]
    #[inline(always)]
    #[must_use]
    pub fn post_idle_num(&mut self) -> POST_IDLE_NUM_W<AT_CMD_POSTCNT_SYNC_SPEC> {
        POST_IDLE_NUM_W::new(self, 0)
    }
}
#[doc = "Post-sequence timing configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_postcnt_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_postcnt_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AT_CMD_POSTCNT_SYNC_SPEC;
impl crate::RegisterSpec for AT_CMD_POSTCNT_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_postcnt_sync::R`](R) reader structure"]
impl crate::Readable for AT_CMD_POSTCNT_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_postcnt_sync::W`](W) writer structure"]
impl crate::Writable for AT_CMD_POSTCNT_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT_CMD_POSTCNT_SYNC to value 0x0901"]
impl crate::Resettable for AT_CMD_POSTCNT_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x0901;
}
