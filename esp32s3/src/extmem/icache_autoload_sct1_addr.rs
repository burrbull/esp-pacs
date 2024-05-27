///Register `ICACHE_AUTOLOAD_SCT1_ADDR` reader
pub type R = crate::R<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
///Register `ICACHE_AUTOLOAD_SCT1_ADDR` writer
pub type W = crate::W<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
///Field `ICACHE_AUTOLOAD_SCT1_ADDR` reader - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena.
pub type ICACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32>;
///Field `ICACHE_AUTOLOAD_SCT1_ADDR` writer - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena.
pub type ICACHE_AUTOLOAD_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena.
    #[inline(always)]
    pub fn icache_autoload_sct1_addr(&self) -> ICACHE_AUTOLOAD_SCT1_ADDR_R {
        ICACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_AUTOLOAD_SCT1_ADDR")
            .field(
                "icache_autoload_sct1_addr",
                &self.icache_autoload_sct1_addr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena.
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_sct1_addr(
        &mut self,
    ) -> ICACHE_AUTOLOAD_SCT1_ADDR_W<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC> {
        ICACHE_AUTOLOAD_SCT1_ADDR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_autoload_sct1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_autoload_sct1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_AUTOLOAD_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`icache_autoload_sct1_addr::R`](R) reader structure
impl crate::Readable for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {}
///`write(|w| ..)` method takes [`icache_autoload_sct1_addr::W`](W) writer structure
impl crate::Writable for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICACHE_AUTOLOAD_SCT1_ADDR to value 0
impl crate::Resettable for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
