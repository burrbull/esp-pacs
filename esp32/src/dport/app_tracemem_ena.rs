///Register `APP_TRACEMEM_ENA` reader
pub type R = crate::R<APP_TRACEMEM_ENA_SPEC>;
///Register `APP_TRACEMEM_ENA` writer
pub type W = crate::W<APP_TRACEMEM_ENA_SPEC>;
///Field `APP_TRACEMEM_ENA` reader -
pub type APP_TRACEMEM_ENA_R = crate::BitReader;
///Field `APP_TRACEMEM_ENA` writer -
pub type APP_TRACEMEM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn app_tracemem_ena(&self) -> APP_TRACEMEM_ENA_R {
        APP_TRACEMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_TRACEMEM_ENA")
            .field("app_tracemem_ena", &self.app_tracemem_ena())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn app_tracemem_ena(&mut self) -> APP_TRACEMEM_ENA_W<APP_TRACEMEM_ENA_SPEC> {
        APP_TRACEMEM_ENA_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_tracemem_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tracemem_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_TRACEMEM_ENA_SPEC;
impl crate::RegisterSpec for APP_TRACEMEM_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_tracemem_ena::R`](R) reader structure
impl crate::Readable for APP_TRACEMEM_ENA_SPEC {}
///`write(|w| ..)` method takes [`app_tracemem_ena::W`](W) writer structure
impl crate::Writable for APP_TRACEMEM_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APP_TRACEMEM_ENA to value 0
impl crate::Resettable for APP_TRACEMEM_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
