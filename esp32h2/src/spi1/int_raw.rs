#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `PER_END` reader - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
pub type PER_END_R = crate::BitReader;
#[doc = "Field `PER_END` writer - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
pub type PER_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END` reader - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
pub type PES_END_R = crate::BitReader;
#[doc = "Field `PES_END` writer - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
pub type PES_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPE_END` reader - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type WPE_END_R = crate::BitReader;
#[doc = "Field `WPE_END` writer - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub type WPE_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_ST_END` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SLV_ST_END_R = crate::BitReader;
#[doc = "Field `SLV_ST_END` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_ST_END` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MST_ST_END_R = crate::BitReader;
#[doc = "Field `MST_ST_END` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MST_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
    #[inline(always)]
    pub fn per_end(&self) -> PER_END_R {
        PER_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
    #[inline(always)]
    pub fn pes_end(&self) -> PES_END_R {
        PES_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    pub fn wpe_end(&self) -> WPE_END_R {
        WPE_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn slv_st_end(&self) -> SLV_ST_END_R {
        SLV_ST_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mst_st_end(&self) -> MST_ST_END_R {
        MST_ST_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("per_end", &self.per_end().bit())
            .field("pes_end", &self.pes_end().bit())
            .field("wpe_end", &self.wpe_end().bit())
            .field("slv_st_end", &self.slv_st_end().bit())
            .field("mst_st_end", &self.mst_st_end().bit())
            .field("brown_out", &self.brown_out().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed successfully. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn per_end(&mut self) -> PER_END_W<INT_RAW_SPEC> {
        PER_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended successfully. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn pes_end(&mut self) -> PES_END_W<INT_RAW_SPEC> {
        PES_END_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn wpe_end(&mut self) -> WPE_END_W<INT_RAW_SPEC> {
        WPE_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<INT_RAW_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<INT_RAW_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    #[doc = "Bit 10 - The raw bit for SPI_MEM_BROWN_OUT_INT interrupt. 1: Triggered condition is that chip is loosing power and RTC module sends out brown out close flash request to SPI1. After SPI1 sends out suspend command to flash, this interrupt is triggered and MSPI returns to idle state. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_RAW_SPEC> {
        BROWN_OUT_W::new(self, 10)
    }
}
#[doc = "SPI1 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
