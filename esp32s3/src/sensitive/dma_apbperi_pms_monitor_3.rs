#[doc = "Register `DMA_APBPERI_PMS_MONITOR_3` reader"]
pub struct R(crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR` reader - recorded dma's write status when dma access violated permission, 1(write), 0(read)"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R = crate::BitReader<bool>;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN` reader - recorded dma's byte enable status when dma access violated permission"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - recorded dma's write status when dma access violated permission, 1(write), 0(read)"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_wr(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - recorded dma's byte enable status when dma access violated permission"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_byteen(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[doc = "dma permission monitor configuration register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_pms_monitor_3](index.html) module"]
pub struct DMA_APBPERI_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_pms_monitor_3::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_APBPERI_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}