#[doc = "Register `DMA_IN_CH2_INTR_MAP` reader"]
pub type R = crate::R<DMA_IN_CH2_INTR_MAP_SPEC>;
#[doc = "Register `DMA_IN_CH2_INTR_MAP` writer"]
pub type W = crate::W<DMA_IN_CH2_INTR_MAP_SPEC>;
#[doc = "Field `DMA_IN_CH2_INTR_MAP` reader - Need add description"]
pub type DMA_IN_CH2_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `DMA_IN_CH2_INTR_MAP` writer - Need add description"]
pub type DMA_IN_CH2_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn dma_in_ch2_intr_map(&self) -> DMA_IN_CH2_INTR_MAP_R {
        DMA_IN_CH2_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_CH2_INTR_MAP")
            .field("dma_in_ch2_intr_map", &self.dma_in_ch2_intr_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_CH2_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_ch2_intr_map(&mut self) -> DMA_IN_CH2_INTR_MAP_W<DMA_IN_CH2_INTR_MAP_SPEC> {
        DMA_IN_CH2_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch2_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch2_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_CH2_INTR_MAP_SPEC;
impl crate::RegisterSpec for DMA_IN_CH2_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_ch2_intr_map::R`](R) reader structure"]
impl crate::Readable for DMA_IN_CH2_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_in_ch2_intr_map::W`](W) writer structure"]
impl crate::Writable for DMA_IN_CH2_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IN_CH2_INTR_MAP to value 0"]
impl crate::Resettable for DMA_IN_CH2_INTR_MAP_SPEC {}
