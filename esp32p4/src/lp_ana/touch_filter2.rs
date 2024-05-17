///Register `TOUCH_FILTER2` reader
pub type R = crate::R<TOUCH_FILTER2_SPEC>;
///Register `TOUCH_FILTER2` writer
pub type W = crate::W<TOUCH_FILTER2_SPEC>;
///Field `TOUCH_OUTEN` reader - need_des
pub type TOUCH_OUTEN_R = crate::FieldReader<u16>;
///Field `TOUCH_OUTEN` writer - need_des
pub type TOUCH_OUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `TOUCH_BYPASS_NOISE_THRES` reader - need_des
pub type TOUCH_BYPASS_NOISE_THRES_R = crate::BitReader;
///Field `TOUCH_BYPASS_NOISE_THRES` writer - need_des
pub type TOUCH_BYPASS_NOISE_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_BYPASS_NEG_NOISE_THRES` reader - need_des
pub type TOUCH_BYPASS_NEG_NOISE_THRES_R = crate::BitReader;
///Field `TOUCH_BYPASS_NEG_NOISE_THRES` writer - need_des
pub type TOUCH_BYPASS_NEG_NOISE_THRES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 15:29 - need_des
    #[inline(always)]
    pub fn touch_outen(&self) -> TOUCH_OUTEN_R {
        TOUCH_OUTEN_R::new(((self.bits >> 15) & 0x7fff) as u16)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn touch_bypass_noise_thres(&self) -> TOUCH_BYPASS_NOISE_THRES_R {
        TOUCH_BYPASS_NOISE_THRES_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn touch_bypass_neg_noise_thres(&self) -> TOUCH_BYPASS_NEG_NOISE_THRES_R {
        TOUCH_BYPASS_NEG_NOISE_THRES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_FILTER2")
            .field("touch_outen", &self.touch_outen())
            .field("touch_bypass_noise_thres", &self.touch_bypass_noise_thres())
            .field("touch_bypass_neg_noise_thres", &self.touch_bypass_neg_noise_thres())
            .finish()
    }
}
impl W {
    ///Bits 15:29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_outen(&mut self) -> TOUCH_OUTEN_W<TOUCH_FILTER2_SPEC> {
        TOUCH_OUTEN_W::new(self, 15)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_bypass_noise_thres(
        &mut self,
    ) -> TOUCH_BYPASS_NOISE_THRES_W<TOUCH_FILTER2_SPEC> {
        TOUCH_BYPASS_NOISE_THRES_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_bypass_neg_noise_thres(
        &mut self,
    ) -> TOUCH_BYPASS_NEG_NOISE_THRES_W<TOUCH_FILTER2_SPEC> {
        TOUCH_BYPASS_NEG_NOISE_THRES_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`touch_filter2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_FILTER2_SPEC;
impl crate::RegisterSpec for TOUCH_FILTER2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_filter2::R`](R) reader structure
impl crate::Readable for TOUCH_FILTER2_SPEC {}
///`write(|w| ..)` method takes [`touch_filter2::W`](W) writer structure
impl crate::Writable for TOUCH_FILTER2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_FILTER2 to value 0x1fff_8000
impl crate::Resettable for TOUCH_FILTER2_SPEC {
    const RESET_VALUE: u32 = 0x1fff_8000;
}
