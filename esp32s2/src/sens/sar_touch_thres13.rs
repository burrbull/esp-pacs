#[doc = "Register `SAR_TOUCH_THRES13` reader"]
pub type R = crate::R<SAR_TOUCH_THRES13_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES13` writer"]
pub type W = crate::W<SAR_TOUCH_THRES13_SPEC>;
#[doc = "Field `TOUCH_OUT_TH13` reader - Finger threshold for touch pad 13"]
pub type TOUCH_OUT_TH13_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_OUT_TH13` writer - Finger threshold for touch pad 13"]
pub type TOUCH_OUT_TH13_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 13"]
    #[inline(always)]
    pub fn touch_out_th13(&self) -> TOUCH_OUT_TH13_R {
        TOUCH_OUT_TH13_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES13")
            .field("touch_out_th13", &self.touch_out_th13().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 13"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th13(&mut self) -> TOUCH_OUT_TH13_W<SAR_TOUCH_THRES13_SPEC> {
        TOUCH_OUT_TH13_W::new(self, 0)
    }
}
#[doc = "Finger threshold for touch pad 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES13_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres13::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres13::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES13_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES13 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES13_SPEC {}
