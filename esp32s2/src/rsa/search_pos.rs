#[doc = "Register `SEARCH_POS` reader"]
pub type R = crate::R<SEARCH_POS_SPEC>;
#[doc = "Register `SEARCH_POS` writer"]
pub type W = crate::W<SEARCH_POS_SPEC>;
#[doc = "Field `SEARCH_POS` reader - Is used to configure the starting address when the acceleration option of search is used."]
pub type SEARCH_POS_R = crate::FieldReader<u16>;
#[doc = "Field `SEARCH_POS` writer - Is used to configure the starting address when the acceleration option of search is used."]
pub type SEARCH_POS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Is used to configure the starting address when the acceleration option of search is used."]
    #[inline(always)]
    pub fn search_pos(&self) -> SEARCH_POS_R {
        SEARCH_POS_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEARCH_POS")
            .field("search_pos", &self.search_pos().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEARCH_POS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Is used to configure the starting address when the acceleration option of search is used."]
    #[inline(always)]
    #[must_use]
    pub fn search_pos(&mut self) -> SEARCH_POS_W<SEARCH_POS_SPEC> {
        SEARCH_POS_W::new(self, 0)
    }
}
#[doc = "The search position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`search_pos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`search_pos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEARCH_POS_SPEC;
impl crate::RegisterSpec for SEARCH_POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`search_pos::R`](R) reader structure"]
impl crate::Readable for SEARCH_POS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`search_pos::W`](W) writer structure"]
impl crate::Writable for SEARCH_POS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEARCH_POS to value 0"]
impl crate::Resettable for SEARCH_POS_SPEC {}
