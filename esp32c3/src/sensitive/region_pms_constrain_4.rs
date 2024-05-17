///Register `REGION_PMS_CONSTRAIN_4` reader
pub type R = crate::R<REGION_PMS_CONSTRAIN_4_SPEC>;
///Register `REGION_PMS_CONSTRAIN_4` writer
pub type W = crate::W<REGION_PMS_CONSTRAIN_4_SPEC>;
///Field `REGION_PMS_CONSTRAIN_ADDR_1` reader - region_pms_constrain_addr_1
pub type REGION_PMS_CONSTRAIN_ADDR_1_R = crate::FieldReader<u32>;
///Field `REGION_PMS_CONSTRAIN_ADDR_1` writer - region_pms_constrain_addr_1
pub type REGION_PMS_CONSTRAIN_ADDR_1_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - region_pms_constrain_addr_1
    #[inline(always)]
    pub fn region_pms_constrain_addr_1(&self) -> REGION_PMS_CONSTRAIN_ADDR_1_R {
        REGION_PMS_CONSTRAIN_ADDR_1_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_PMS_CONSTRAIN_4")
            .field("region_pms_constrain_addr_1", &self.region_pms_constrain_addr_1())
            .finish()
    }
}
impl W {
    ///Bits 0:29 - region_pms_constrain_addr_1
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_addr_1(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_ADDR_1_W<REGION_PMS_CONSTRAIN_4_SPEC> {
        REGION_PMS_CONSTRAIN_ADDR_1_W::new(self, 0)
    }
}
/**SENSITIVE_REGION_PMS_CONSTRAIN_4_REG

You can [`read`](crate::generic::Reg::read) this register and get [`region_pms_constrain_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region_pms_constrain_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGION_PMS_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`region_pms_constrain_4::R`](R) reader structure
impl crate::Readable for REGION_PMS_CONSTRAIN_4_SPEC {}
///`write(|w| ..)` method takes [`region_pms_constrain_4::W`](W) writer structure
impl crate::Writable for REGION_PMS_CONSTRAIN_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REGION_PMS_CONSTRAIN_4 to value 0
impl crate::Resettable for REGION_PMS_CONSTRAIN_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
