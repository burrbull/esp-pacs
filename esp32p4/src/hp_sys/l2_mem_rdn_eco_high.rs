///Register `L2_MEM_RDN_ECO_HIGH` reader
pub type R = crate::R<L2_MEM_RDN_ECO_HIGH_SPEC>;
///Register `L2_MEM_RDN_ECO_HIGH` writer
pub type W = crate::W<L2_MEM_RDN_ECO_HIGH_SPEC>;
///Field `REG_L2_MEM_RDN_ECO_HIGH` reader - NA
pub type REG_L2_MEM_RDN_ECO_HIGH_R = crate::FieldReader<u32>;
///Field `REG_L2_MEM_RDN_ECO_HIGH` writer - NA
pub type REG_L2_MEM_RDN_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_high(&self) -> REG_L2_MEM_RDN_ECO_HIGH_R {
        REG_L2_MEM_RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_RDN_ECO_HIGH")
            .field("reg_l2_mem_rdn_eco_high", &self.reg_l2_mem_rdn_eco_high())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_rdn_eco_high(
        &mut self,
    ) -> REG_L2_MEM_RDN_ECO_HIGH_W<L2_MEM_RDN_ECO_HIGH_SPEC> {
        REG_L2_MEM_RDN_ECO_HIGH_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_rdn_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_MEM_RDN_ECO_HIGH_SPEC;
impl crate::RegisterSpec for L2_MEM_RDN_ECO_HIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_mem_rdn_eco_high::R`](R) reader structure
impl crate::Readable for L2_MEM_RDN_ECO_HIGH_SPEC {}
///`write(|w| ..)` method takes [`l2_mem_rdn_eco_high::W`](W) writer structure
impl crate::Writable for L2_MEM_RDN_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_MEM_RDN_ECO_HIGH to value 0xffff_ffff
impl crate::Resettable for L2_MEM_RDN_ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
