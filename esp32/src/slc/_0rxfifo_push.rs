#[doc = "Register `_0RXFIFO_PUSH` reader"]
pub type R = crate::R<_0RXFIFO_PUSH_SPEC>;
#[doc = "Register `_0RXFIFO_PUSH` writer"]
pub type W = crate::W<_0RXFIFO_PUSH_SPEC>;
#[doc = "Field `SLC0_RXFIFO_WDATA` reader - "]
pub type SLC0_RXFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_RXFIFO_WDATA` writer - "]
pub type SLC0_RXFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SLC0_RXFIFO_PUSH` reader - "]
pub type SLC0_RXFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `SLC0_RXFIFO_PUSH` writer - "]
pub type SLC0_RXFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc0_rxfifo_wdata(&self) -> SLC0_RXFIFO_WDATA_R {
        SLC0_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rxfifo_push(&self) -> SLC0_RXFIFO_PUSH_R {
        SLC0_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0RXFIFO_PUSH")
            .field("slc0_rxfifo_wdata", &self.slc0_rxfifo_wdata())
            .field("slc0_rxfifo_push", &self.slc0_rxfifo_push())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxfifo_wdata(&mut self) -> SLC0_RXFIFO_WDATA_W<_0RXFIFO_PUSH_SPEC> {
        SLC0_RXFIFO_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rxfifo_push(&mut self) -> SLC0_RXFIFO_PUSH_W<_0RXFIFO_PUSH_SPEC> {
        SLC0_RXFIFO_PUSH_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0rxfifo_push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0rxfifo_push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for _0RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0rxfifo_push::R`](R) reader structure"]
impl crate::Readable for _0RXFIFO_PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0rxfifo_push::W`](W) writer structure"]
impl crate::Writable for _0RXFIFO_PUSH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _0RXFIFO_PUSH to value 0"]
impl crate::Resettable for _0RXFIFO_PUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
