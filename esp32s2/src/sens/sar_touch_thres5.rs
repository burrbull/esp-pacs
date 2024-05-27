///Register `SAR_TOUCH_THRES5` reader
pub type R = crate::R<SAR_TOUCH_THRES5_SPEC>;
///Register `SAR_TOUCH_THRES5` writer
pub type W = crate::W<SAR_TOUCH_THRES5_SPEC>;
///Field `TOUCH_OUT_TH5` reader - Finger threshold for touch pad 5
pub type TOUCH_OUT_TH5_R = crate::FieldReader<u32>;
///Field `TOUCH_OUT_TH5` writer - Finger threshold for touch pad 5
pub type TOUCH_OUT_TH5_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - Finger threshold for touch pad 5
    #[inline(always)]
    pub fn touch_out_th5(&self) -> TOUCH_OUT_TH5_R {
        TOUCH_OUT_TH5_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES5")
            .field("touch_out_th5", &self.touch_out_th5())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - Finger threshold for touch pad 5
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th5(&mut self) -> TOUCH_OUT_TH5_W<SAR_TOUCH_THRES5_SPEC> {
        TOUCH_OUT_TH5_W::new(self, 0)
    }
}
/**Finger threshold for touch pad 5

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_THRES5_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_thres5::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_THRES5_SPEC {}
///`write(|w| ..)` method takes [`sar_touch_thres5::W`](W) writer structure
impl crate::Writable for SAR_TOUCH_THRES5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TOUCH_THRES5 to value 0
impl crate::Resettable for SAR_TOUCH_THRES5_SPEC {
    const RESET_VALUE: u32 = 0;
}
