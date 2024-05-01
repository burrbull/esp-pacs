///Register `CORE_1_AREA_PIF_1_MAX` reader
pub type R = crate::R<CORE_1_AREA_PIF_1_MAX_SPEC>;
///Register `CORE_1_AREA_PIF_1_MAX` writer
pub type W = crate::W<CORE_1_AREA_PIF_1_MAX_SPEC>;
///Field `CORE_1_AREA_PIF_1_MAX` reader - Core1 PIF region1 end addr
pub type CORE_1_AREA_PIF_1_MAX_R = crate::FieldReader<u32>;
///Field `CORE_1_AREA_PIF_1_MAX` writer - Core1 PIF region1 end addr
pub type CORE_1_AREA_PIF_1_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Core1 PIF region1 end addr
    #[inline(always)]
    pub fn core_1_area_pif_1_max(&self) -> CORE_1_AREA_PIF_1_MAX_R {
        CORE_1_AREA_PIF_1_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_AREA_PIF_1_MAX")
            .field("core_1_area_pif_1_max", &self.core_1_area_pif_1_max())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Core1 PIF region1 end addr
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_max(&mut self) -> CORE_1_AREA_PIF_1_MAX_W<CORE_1_AREA_PIF_1_MAX_SPEC> {
        CORE_1_AREA_PIF_1_MAX_W::new(self, 0)
    }
}
/**Core1 PIF region1 addr configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_pif_1_max::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_pif_1_max::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_AREA_PIF_1_MAX_SPEC;
impl crate::RegisterSpec for CORE_1_AREA_PIF_1_MAX_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_area_pif_1_max::R`](R) reader structure
impl crate::Readable for CORE_1_AREA_PIF_1_MAX_SPEC {}
///`write(|w| ..)` method takes [`core_1_area_pif_1_max::W`](W) writer structure
impl crate::Writable for CORE_1_AREA_PIF_1_MAX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_1_AREA_PIF_1_MAX to value 0
impl crate::Resettable for CORE_1_AREA_PIF_1_MAX_SPEC {
    const RESET_VALUE: u32 = 0;
}
