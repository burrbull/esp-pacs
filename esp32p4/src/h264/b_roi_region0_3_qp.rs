///Register `B_ROI_REGION0_3_QP` reader
pub type R = crate::R<B_ROI_REGION0_3_QP_SPEC>;
///Register `B_ROI_REGION0_3_QP` writer
pub type W = crate::W<B_ROI_REGION0_3_QP_SPEC>;
///Field `B_ROI_REGION0_QP` reader - Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION0_QP_R = crate::FieldReader;
///Field `B_ROI_REGION0_QP` writer - Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION0_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `B_ROI_REGION1_QP` reader - Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION1_QP_R = crate::FieldReader;
///Field `B_ROI_REGION1_QP` writer - Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION1_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `B_ROI_REGION2_QP` reader - Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION2_QP_R = crate::FieldReader;
///Field `B_ROI_REGION2_QP` writer - Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION2_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `B_ROI_REGION3_QP` reader - Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION3_QP_R = crate::FieldReader;
///Field `B_ROI_REGION3_QP` writer - Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
pub type B_ROI_REGION3_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region0_qp(&self) -> B_ROI_REGION0_QP_R {
        B_ROI_REGION0_QP_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region1_qp(&self) -> B_ROI_REGION1_QP_R {
        B_ROI_REGION1_QP_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:20 - Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region2_qp(&self) -> B_ROI_REGION2_QP_R {
        B_ROI_REGION2_QP_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    ///Bits 21:27 - Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region3_qp(&self) -> B_ROI_REGION3_QP_R {
        B_ROI_REGION3_QP_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_ROI_REGION0_3_QP")
            .field("b_roi_region0_qp", &self.b_roi_region0_qp())
            .field("b_roi_region1_qp", &self.b_roi_region1_qp())
            .field("b_roi_region2_qp", &self.b_roi_region2_qp())
            .field("b_roi_region3_qp", &self.b_roi_region3_qp())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    #[must_use]
    pub fn b_roi_region0_qp(&mut self) -> B_ROI_REGION0_QP_W<B_ROI_REGION0_3_QP_SPEC> {
        B_ROI_REGION0_QP_W::new(self, 0)
    }
    ///Bits 7:13 - Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    #[must_use]
    pub fn b_roi_region1_qp(&mut self) -> B_ROI_REGION1_QP_W<B_ROI_REGION0_3_QP_SPEC> {
        B_ROI_REGION1_QP_W::new(self, 7)
    }
    ///Bits 14:20 - Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    #[must_use]
    pub fn b_roi_region2_qp(&mut self) -> B_ROI_REGION2_QP_W<B_ROI_REGION0_3_QP_SPEC> {
        B_ROI_REGION2_QP_W::new(self, 14)
    }
    ///Bits 21:27 - Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    #[must_use]
    pub fn b_roi_region3_qp(&mut self) -> B_ROI_REGION3_QP_W<B_ROI_REGION0_3_QP_SPEC> {
        B_ROI_REGION3_QP_W::new(self, 21)
    }
}
/**Video B H264 ROI region0, region1,region2,region3 QP register.

You can [`read`](crate::generic::Reg::read) this register and get [`b_roi_region0_3_qp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_region0_3_qp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct B_ROI_REGION0_3_QP_SPEC;
impl crate::RegisterSpec for B_ROI_REGION0_3_QP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`b_roi_region0_3_qp::R`](R) reader structure
impl crate::Readable for B_ROI_REGION0_3_QP_SPEC {}
///`write(|w| ..)` method takes [`b_roi_region0_3_qp::W`](W) writer structure
impl crate::Writable for B_ROI_REGION0_3_QP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets B_ROI_REGION0_3_QP to value 0
impl crate::Resettable for B_ROI_REGION0_3_QP_SPEC {
    const RESET_VALUE: u32 = 0;
}
