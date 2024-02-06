#[doc = "Register `ICACHE_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Register `ICACHE_AUTOLOAD_CTRL` writer"]
pub type W = crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the first section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the first section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the second section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the second section for autoload operation."]
pub type ICACHE_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
pub type ICACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
pub type ICACHE_AUTOLOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_AUTOLOAD_DONE` reader - The bit is used to indicate autoload operation is finished."]
pub type ICACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `ICACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub type ICACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `ICACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub type ICACHE_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type ICACHE_AUTOLOAD_RQST_R = crate::FieldReader;
#[doc = "Field `ICACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type ICACHE_AUTOLOAD_RQST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
pub type ICACHE_AUTOLOAD_SIZE_R = crate::FieldReader;
#[doc = "Field `ICACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
pub type ICACHE_AUTOLOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICACHE_AUTOLOAD_BUFFER_CLEAR` reader - The bit is used to clear autoload buffer in icache."]
pub type ICACHE_AUTOLOAD_BUFFER_CLEAR_R = crate::BitReader;
#[doc = "Field `ICACHE_AUTOLOAD_BUFFER_CLEAR` writer - The bit is used to clear autoload buffer in icache."]
pub type ICACHE_AUTOLOAD_BUFFER_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_sct0_ena(&self) -> ICACHE_AUTOLOAD_SCT0_ENA_R {
        ICACHE_AUTOLOAD_SCT0_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_sct1_ena(&self) -> ICACHE_AUTOLOAD_SCT1_ENA_R {
        ICACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn icache_autoload_ena(&self) -> ICACHE_AUTOLOAD_ENA_R {
        ICACHE_AUTOLOAD_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate autoload operation is finished."]
    #[inline(always)]
    pub fn icache_autoload_done(&self) -> ICACHE_AUTOLOAD_DONE_R {
        ICACHE_AUTOLOAD_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn icache_autoload_order(&self) -> ICACHE_AUTOLOAD_ORDER_R {
        ICACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn icache_autoload_rqst(&self) -> ICACHE_AUTOLOAD_RQST_R {
        ICACHE_AUTOLOAD_RQST_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
    #[inline(always)]
    pub fn icache_autoload_size(&self) -> ICACHE_AUTOLOAD_SIZE_R {
        ICACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - The bit is used to clear autoload buffer in icache."]
    #[inline(always)]
    pub fn icache_autoload_buffer_clear(&self) -> ICACHE_AUTOLOAD_BUFFER_CLEAR_R {
        ICACHE_AUTOLOAD_BUFFER_CLEAR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_AUTOLOAD_CTRL")
            .field(
                "icache_autoload_sct0_ena",
                &format_args!("{}", self.icache_autoload_sct0_ena().bit()),
            )
            .field(
                "icache_autoload_sct1_ena",
                &format_args!("{}", self.icache_autoload_sct1_ena().bit()),
            )
            .field(
                "icache_autoload_ena",
                &format_args!("{}", self.icache_autoload_ena().bit()),
            )
            .field(
                "icache_autoload_done",
                &format_args!("{}", self.icache_autoload_done().bit()),
            )
            .field(
                "icache_autoload_order",
                &format_args!("{}", self.icache_autoload_order().bit()),
            )
            .field(
                "icache_autoload_rqst",
                &format_args!("{}", self.icache_autoload_rqst().bits()),
            )
            .field(
                "icache_autoload_size",
                &format_args!("{}", self.icache_autoload_size().bits()),
            )
            .field(
                "icache_autoload_buffer_clear",
                &format_args!("{}", self.icache_autoload_buffer_clear().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_AUTOLOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_sct0_ena(
        &mut self,
    ) -> ICACHE_AUTOLOAD_SCT0_ENA_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_SCT0_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_sct1_ena(
        &mut self,
    ) -> ICACHE_AUTOLOAD_SCT1_ENA_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_SCT1_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_ena(&mut self) -> ICACHE_AUTOLOAD_ENA_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_ENA_W::new(self, 2)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_order(&mut self) -> ICACHE_AUTOLOAD_ORDER_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_ORDER_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_rqst(&mut self) -> ICACHE_AUTOLOAD_RQST_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_RQST_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_size(&mut self) -> ICACHE_AUTOLOAD_SIZE_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_SIZE_W::new(self, 7)
    }
    #[doc = "Bit 9 - The bit is used to clear autoload buffer in icache."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_buffer_clear(
        &mut self,
    ) -> ICACHE_AUTOLOAD_BUFFER_CLEAR_W<ICACHE_AUTOLOAD_CTRL_SPEC> {
        ICACHE_AUTOLOAD_BUFFER_CLEAR_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for ICACHE_AUTOLOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_autoload_ctrl::W`](W) writer structure"]
impl crate::Writable for ICACHE_AUTOLOAD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICACHE_AUTOLOAD_CTRL to value 0x08"]
impl crate::Resettable for ICACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
