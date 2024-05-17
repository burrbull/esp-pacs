///Register `ZERO_DET0_FILTER_CNT` reader
pub type R = crate::R<ZERO_DET0_FILTER_CNT_SPEC>;
///Register `ZERO_DET0_FILTER_CNT` writer
pub type W = crate::W<ZERO_DET0_FILTER_CNT_SPEC>;
///Field `ZERO_DET0_FILTER_CNT` reader - GPIO analog comparator zero detect filter count
pub type ZERO_DET0_FILTER_CNT_R = crate::FieldReader<u32>;
///Field `ZERO_DET0_FILTER_CNT` writer - GPIO analog comparator zero detect filter count
pub type ZERO_DET0_FILTER_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO analog comparator zero detect filter count
    #[inline(always)]
    pub fn zero_det0_filter_cnt(&self) -> ZERO_DET0_FILTER_CNT_R {
        ZERO_DET0_FILTER_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZERO_DET0_FILTER_CNT")
            .field("zero_det0_filter_cnt", &self.zero_det0_filter_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - GPIO analog comparator zero detect filter count
    #[inline(always)]
    #[must_use]
    pub fn zero_det0_filter_cnt(
        &mut self,
    ) -> ZERO_DET0_FILTER_CNT_W<ZERO_DET0_FILTER_CNT_SPEC> {
        ZERO_DET0_FILTER_CNT_W::new(self, 0)
    }
}
/**GPIO analog comparator zero detect filter count

You can [`read`](crate::generic::Reg::read) this register and get [`zero_det0_filter_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zero_det0_filter_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ZERO_DET0_FILTER_CNT_SPEC;
impl crate::RegisterSpec for ZERO_DET0_FILTER_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`zero_det0_filter_cnt::R`](R) reader structure
impl crate::Readable for ZERO_DET0_FILTER_CNT_SPEC {}
///`write(|w| ..)` method takes [`zero_det0_filter_cnt::W`](W) writer structure
impl crate::Writable for ZERO_DET0_FILTER_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ZERO_DET0_FILTER_CNT to value 0xffff_ffff
impl crate::Resettable for ZERO_DET0_FILTER_CNT_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
