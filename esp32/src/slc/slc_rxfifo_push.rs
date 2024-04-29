#[doc = "Register `SLC%s_RXFIFO_PUSH` reader"]
pub type R = crate::R<SLC_RXFIFO_PUSH_SPEC>;
#[doc = "Register `SLC%s_RXFIFO_PUSH` writer"]
pub type W = crate::W<SLC_RXFIFO_PUSH_SPEC>;
#[doc = "Field `RXFIFO_WDATA` reader - "]
pub type RXFIFO_WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `RXFIFO_WDATA` writer - "]
pub type RXFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RXFIFO_PUSH` reader - "]
pub type RXFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `RXFIFO_PUSH` writer - "]
pub type RXFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxfifo_wdata(&self) -> RXFIFO_WDATA_R {
        RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxfifo_push(&self) -> RXFIFO_PUSH_R {
        RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_RXFIFO_PUSH")
            .field(
                "rxfifo_wdata",
                &format_args!("{}", self.rxfifo_wdata().bits()),
            )
            .field("rxfifo_push", &format_args!("{}", self.rxfifo_push().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_RXFIFO_PUSH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wdata(&mut self) -> RXFIFO_WDATA_W<SLC_RXFIFO_PUSH_SPEC> {
        RXFIFO_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_push(&mut self) -> RXFIFO_PUSH_W<SLC_RXFIFO_PUSH_SPEC> {
        RXFIFO_PUSH_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rxfifo_push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rxfifo_push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for SLC_RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_rxfifo_push::R`](R) reader structure"]
impl crate::Readable for SLC_RXFIFO_PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_rxfifo_push::W`](W) writer structure"]
impl crate::Writable for SLC_RXFIFO_PUSH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC%s_RXFIFO_PUSH to value 0"]
impl crate::Resettable for SLC_RXFIFO_PUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
