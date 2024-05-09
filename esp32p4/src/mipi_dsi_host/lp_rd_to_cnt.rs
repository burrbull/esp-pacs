#[doc = "Register `LP_RD_TO_CNT` reader"]
pub type R = crate::R<LP_RD_TO_CNT_SPEC>;
#[doc = "Register `LP_RD_TO_CNT` writer"]
pub type W = crate::W<LP_RD_TO_CNT_SPEC>;
#[doc = "Field `LP_RD_TO_CNT` reader - NA"]
pub type LP_RD_TO_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `LP_RD_TO_CNT` writer - NA"]
pub type LP_RD_TO_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn lp_rd_to_cnt(&self) -> LP_RD_TO_CNT_R {
        LP_RD_TO_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RD_TO_CNT")
            .field("lp_rd_to_cnt", &self.lp_rd_to_cnt().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_RD_TO_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_rd_to_cnt(&mut self) -> LP_RD_TO_CNT_W<LP_RD_TO_CNT_SPEC> {
        LP_RD_TO_CNT_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rd_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rd_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_RD_TO_CNT_SPEC;
impl crate::RegisterSpec for LP_RD_TO_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rd_to_cnt::R`](R) reader structure"]
impl crate::Readable for LP_RD_TO_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rd_to_cnt::W`](W) writer structure"]
impl crate::Writable for LP_RD_TO_CNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_RD_TO_CNT to value 0"]
impl crate::Resettable for LP_RD_TO_CNT_SPEC {}
