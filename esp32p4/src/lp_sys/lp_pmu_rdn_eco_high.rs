///Register `LP_PMU_RDN_ECO_HIGH` reader
pub type R = crate::R<LP_PMU_RDN_ECO_HIGH_SPEC>;
///Register `LP_PMU_RDN_ECO_HIGH` writer
pub type W = crate::W<LP_PMU_RDN_ECO_HIGH_SPEC>;
///Field `PMU_RDN_ECO_HIGH` reader - need_des
pub type PMU_RDN_ECO_HIGH_R = crate::FieldReader<u32>;
///Field `PMU_RDN_ECO_HIGH` writer - need_des
pub type PMU_RDN_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn pmu_rdn_eco_high(&self) -> PMU_RDN_ECO_HIGH_R {
        PMU_RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PMU_RDN_ECO_HIGH")
            .field("pmu_rdn_eco_high", &self.pmu_rdn_eco_high())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pmu_rdn_eco_high(&mut self) -> PMU_RDN_ECO_HIGH_W<LP_PMU_RDN_ECO_HIGH_SPEC> {
        PMU_RDN_ECO_HIGH_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_pmu_rdn_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_pmu_rdn_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_PMU_RDN_ECO_HIGH_SPEC;
impl crate::RegisterSpec for LP_PMU_RDN_ECO_HIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_pmu_rdn_eco_high::R`](R) reader structure
impl crate::Readable for LP_PMU_RDN_ECO_HIGH_SPEC {}
///`write(|w| ..)` method takes [`lp_pmu_rdn_eco_high::W`](W) writer structure
impl crate::Writable for LP_PMU_RDN_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_PMU_RDN_ECO_HIGH to value 0xffff_ffff
impl crate::Resettable for LP_PMU_RDN_ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
