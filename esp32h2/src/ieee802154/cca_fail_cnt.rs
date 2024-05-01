///Register `CCA_FAIL_CNT` reader
pub type R = crate::R<CCA_FAIL_CNT_SPEC>;
///Register `CCA_FAIL_CNT` writer
pub type W = crate::W<CCA_FAIL_CNT_SPEC>;
///Field `CCA_FAIL_CNT` reader -
pub type CCA_FAIL_CNT_R = crate::FieldReader<u16>;
///Field `CCA_FAIL_CNT` writer -
pub type CCA_FAIL_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn cca_fail_cnt(&self) -> CCA_FAIL_CNT_R {
        CCA_FAIL_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCA_FAIL_CNT")
            .field("cca_fail_cnt", &self.cca_fail_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn cca_fail_cnt(&mut self) -> CCA_FAIL_CNT_W<CCA_FAIL_CNT_SPEC> {
        CCA_FAIL_CNT_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`cca_fail_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cca_fail_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCA_FAIL_CNT_SPEC;
impl crate::RegisterSpec for CCA_FAIL_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cca_fail_cnt::R`](R) reader structure
impl crate::Readable for CCA_FAIL_CNT_SPEC {}
///`write(|w| ..)` method takes [`cca_fail_cnt::W`](W) writer structure
impl crate::Writable for CCA_FAIL_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCA_FAIL_CNT to value 0
impl crate::Resettable for CCA_FAIL_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
