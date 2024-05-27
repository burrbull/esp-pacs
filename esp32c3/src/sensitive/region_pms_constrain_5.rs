///Register `REGION_PMS_CONSTRAIN_5` reader
pub type R = crate::R<REGION_PMS_CONSTRAIN_5_SPEC>;
///Register `REGION_PMS_CONSTRAIN_5` writer
pub type W = crate::W<REGION_PMS_CONSTRAIN_5_SPEC>;
///Field `REGION_PMS_CONSTRAIN_ADDR_2` reader - region_pms_constrain_addr_2
pub type REGION_PMS_CONSTRAIN_ADDR_2_R = crate::FieldReader<u32>;
///Field `REGION_PMS_CONSTRAIN_ADDR_2` writer - region_pms_constrain_addr_2
pub type REGION_PMS_CONSTRAIN_ADDR_2_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - region_pms_constrain_addr_2
    #[inline(always)]
    pub fn region_pms_constrain_addr_2(&self) -> REGION_PMS_CONSTRAIN_ADDR_2_R {
        REGION_PMS_CONSTRAIN_ADDR_2_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_PMS_CONSTRAIN_5")
            .field(
                "region_pms_constrain_addr_2",
                &self.region_pms_constrain_addr_2(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:29 - region_pms_constrain_addr_2
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_addr_2(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_ADDR_2_W<REGION_PMS_CONSTRAIN_5_SPEC> {
        REGION_PMS_CONSTRAIN_ADDR_2_W::new(self, 0)
    }
}
/**SENSITIVE_REGION_PMS_CONSTRAIN_5_REG

You can [`read`](crate::generic::Reg::read) this register and get [`region_pms_constrain_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_pms_constrain_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGION_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`region_pms_constrain_5::R`](R) reader structure
impl crate::Readable for REGION_PMS_CONSTRAIN_5_SPEC {}
///`write(|w| ..)` method takes [`region_pms_constrain_5::W`](W) writer structure
impl crate::Writable for REGION_PMS_CONSTRAIN_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGION_PMS_CONSTRAIN_5 to value 0
impl crate::Resettable for REGION_PMS_CONSTRAIN_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
