///Register `APP_RWBLE_NMI_MAP` reader
pub type R = crate::R<APP_RWBLE_NMI_MAP_SPEC>;
///Register `APP_RWBLE_NMI_MAP` writer
pub type W = crate::W<APP_RWBLE_NMI_MAP_SPEC>;
///Field `APP_RWBLE_NMI_MAP` reader -
pub type APP_RWBLE_NMI_MAP_R = crate::FieldReader;
///Field `APP_RWBLE_NMI_MAP` writer -
pub type APP_RWBLE_NMI_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn app_rwble_nmi_map(&self) -> APP_RWBLE_NMI_MAP_R {
        APP_RWBLE_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_RWBLE_NMI_MAP")
            .field("app_rwble_nmi_map", &self.app_rwble_nmi_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    #[must_use]
    pub fn app_rwble_nmi_map(&mut self) -> APP_RWBLE_NMI_MAP_W<APP_RWBLE_NMI_MAP_SPEC> {
        APP_RWBLE_NMI_MAP_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_rwble_nmi_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_rwble_nmi_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_RWBLE_NMI_MAP_SPEC;
impl crate::RegisterSpec for APP_RWBLE_NMI_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_rwble_nmi_map::R`](R) reader structure
impl crate::Readable for APP_RWBLE_NMI_MAP_SPEC {}
///`write(|w| ..)` method takes [`app_rwble_nmi_map::W`](W) writer structure
impl crate::Writable for APP_RWBLE_NMI_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APP_RWBLE_NMI_MAP to value 0x10
impl crate::Resettable for APP_RWBLE_NMI_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
