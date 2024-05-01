///Register `PRO_DCACHE_AUTOLOAD_SECTION1_SIZE` reader
pub type R = crate::R<PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
///Register `PRO_DCACHE_AUTOLOAD_SECTION1_SIZE` writer
pub type W = crate::W<PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC>;
///Field `PRO_DCACHE_AUTOLOAD_SCT1_SIZE` reader - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct1_ena.
pub type PRO_DCACHE_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32>;
///Field `PRO_DCACHE_AUTOLOAD_SCT1_SIZE` writer - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct1_ena.
pub type PRO_DCACHE_AUTOLOAD_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct1_ena.
    #[inline(always)]
    pub fn pro_dcache_autoload_sct1_size(&self) -> PRO_DCACHE_AUTOLOAD_SCT1_SIZE_R {
        PRO_DCACHE_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_AUTOLOAD_SECTION1_SIZE")
            .field(
                "pro_dcache_autoload_sct1_size",
                &self.pro_dcache_autoload_sct1_size(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:23 - The bits are used to configure the length of the second section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct1_ena.
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_sct1_size(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_SCT1_SIZE_W<PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC> {
        PRO_DCACHE_AUTOLOAD_SCT1_SIZE_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_autoload_section1_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_autoload_section1_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_dcache_autoload_section1_size::R`](R) reader structure
impl crate::Readable for PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC {}
///`write(|w| ..)` method takes [`pro_dcache_autoload_section1_size::W`](W) writer structure
impl crate::Writable for PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_DCACHE_AUTOLOAD_SECTION1_SIZE to value 0x8000
impl crate::Resettable for PRO_DCACHE_AUTOLOAD_SECTION1_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
