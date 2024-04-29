#[doc = "Register `SLC%s_TXFIFO_POP` reader"]
pub type R = crate::R<SLC_TXFIFO_POP_SPEC>;
#[doc = "Register `SLC%s_TXFIFO_POP` writer"]
pub type W = crate::W<SLC_TXFIFO_POP_SPEC>;
#[doc = "Field `TXFIFO_RDATA` reader - "]
pub type TXFIFO_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `TXFIFO_POP` reader - "]
pub type TXFIFO_POP_R = crate::BitReader;
#[doc = "Field `TXFIFO_POP` writer - "]
pub type TXFIFO_POP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn txfifo_rdata(&self) -> TXFIFO_RDATA_R {
        TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn txfifo_pop(&self) -> TXFIFO_POP_R {
        TXFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TXFIFO_POP")
            .field(
                "txfifo_rdata",
                &format_args!("{}", self.txfifo_rdata().bits()),
            )
            .field("txfifo_pop", &format_args!("{}", self.txfifo_pop().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TXFIFO_POP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_pop(&mut self) -> TXFIFO_POP_W<SLC_TXFIFO_POP_SPEC> {
        TXFIFO_POP_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_txfifo_pop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_txfifo_pop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TXFIFO_POP_SPEC;
impl crate::RegisterSpec for SLC_TXFIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_txfifo_pop::R`](R) reader structure"]
impl crate::Readable for SLC_TXFIFO_POP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_txfifo_pop::W`](W) writer structure"]
impl crate::Writable for SLC_TXFIFO_POP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC%s_TXFIFO_POP to value 0"]
impl crate::Resettable for SLC_TXFIFO_POP_SPEC {
    const RESET_VALUE: u32 = 0;
}
