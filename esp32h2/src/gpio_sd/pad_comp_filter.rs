///Register `PAD_COMP_FILTER` reader
pub type R = crate::R<PAD_COMP_FILTER_SPEC>;
///Register `PAD_COMP_FILTER` writer
pub type W = crate::W<PAD_COMP_FILTER_SPEC>;
///Field `ZERO_DET_FILTER_CNT` reader - Zero Detect filter cycle length
pub type ZERO_DET_FILTER_CNT_R = crate::FieldReader<u32>;
///Field `ZERO_DET_FILTER_CNT` writer - Zero Detect filter cycle length
pub type ZERO_DET_FILTER_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Zero Detect filter cycle length
    #[inline(always)]
    pub fn zero_det_filter_cnt(&self) -> ZERO_DET_FILTER_CNT_R {
        ZERO_DET_FILTER_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP_FILTER")
            .field("zero_det_filter_cnt", &self.zero_det_filter_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Zero Detect filter cycle length
    #[inline(always)]
    #[must_use]
    pub fn zero_det_filter_cnt(
        &mut self,
    ) -> ZERO_DET_FILTER_CNT_W<PAD_COMP_FILTER_SPEC> {
        ZERO_DET_FILTER_CNT_W::new(self, 0)
    }
}
/**Zero Detect filter Register

You can [`read`](crate::generic::Reg::read) this register and get [`pad_comp_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_comp_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PAD_COMP_FILTER_SPEC;
impl crate::RegisterSpec for PAD_COMP_FILTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pad_comp_filter::R`](R) reader structure
impl crate::Readable for PAD_COMP_FILTER_SPEC {}
///`write(|w| ..)` method takes [`pad_comp_filter::W`](W) writer structure
impl crate::Writable for PAD_COMP_FILTER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAD_COMP_FILTER to value 0
impl crate::Resettable for PAD_COMP_FILTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
