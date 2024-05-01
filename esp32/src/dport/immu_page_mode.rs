///Register `IMMU_PAGE_MODE` reader
pub type R = crate::R<IMMU_PAGE_MODE_SPEC>;
///Register `IMMU_PAGE_MODE` writer
pub type W = crate::W<IMMU_PAGE_MODE_SPEC>;
///Field `INTERNAL_SRAM_IMMU_ENA` reader -
pub type INTERNAL_SRAM_IMMU_ENA_R = crate::BitReader;
///Field `INTERNAL_SRAM_IMMU_ENA` writer -
pub type INTERNAL_SRAM_IMMU_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IMMU_PAGE_MODE` reader -
pub type IMMU_PAGE_MODE_R = crate::FieldReader;
///Field `IMMU_PAGE_MODE` writer -
pub type IMMU_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn internal_sram_immu_ena(&self) -> INTERNAL_SRAM_IMMU_ENA_R {
        INTERNAL_SRAM_IMMU_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2
    #[inline(always)]
    pub fn immu_page_mode(&self) -> IMMU_PAGE_MODE_R {
        IMMU_PAGE_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMMU_PAGE_MODE")
            .field("internal_sram_immu_ena", &self.internal_sram_immu_ena())
            .field("immu_page_mode", &self.immu_page_mode())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_immu_ena(&mut self) -> INTERNAL_SRAM_IMMU_ENA_W<IMMU_PAGE_MODE_SPEC> {
        INTERNAL_SRAM_IMMU_ENA_W::new(self, 0)
    }
    ///Bits 1:2
    #[inline(always)]
    #[must_use]
    pub fn immu_page_mode(&mut self) -> IMMU_PAGE_MODE_W<IMMU_PAGE_MODE_SPEC> {
        IMMU_PAGE_MODE_W::new(self, 1)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`immu_page_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`immu_page_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for IMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`immu_page_mode::R`](R) reader structure
impl crate::Readable for IMMU_PAGE_MODE_SPEC {}
///`write(|w| ..)` method takes [`immu_page_mode::W`](W) writer structure
impl crate::Writable for IMMU_PAGE_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMMU_PAGE_MODE to value 0
impl crate::Resettable for IMMU_PAGE_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
