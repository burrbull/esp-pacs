///Register `SAR_TOUCH_CHN_ST` reader
pub type R = crate::R<SAR_TOUCH_CHN_ST_SPEC>;
///Register `SAR_TOUCH_CHN_ST` writer
pub type W = crate::W<SAR_TOUCH_CHN_ST_SPEC>;
///Field `SAR_TOUCH_PAD_ACTIVE` reader - touch active status
pub type SAR_TOUCH_PAD_ACTIVE_R = crate::FieldReader<u16>;
///Field `SAR_TOUCH_CHANNEL_CLR` writer - Clear touch channel
pub type SAR_TOUCH_CHANNEL_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `SAR_TOUCH_MEAS_DONE` reader - get touch meas done
pub type SAR_TOUCH_MEAS_DONE_R = crate::BitReader;
impl R {
    ///Bits 0:14 - touch active status
    #[inline(always)]
    pub fn sar_touch_pad_active(&self) -> SAR_TOUCH_PAD_ACTIVE_R {
        SAR_TOUCH_PAD_ACTIVE_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bit 31 - get touch meas done
    #[inline(always)]
    pub fn sar_touch_meas_done(&self) -> SAR_TOUCH_MEAS_DONE_R {
        SAR_TOUCH_MEAS_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CHN_ST")
            .field("sar_touch_pad_active", &self.sar_touch_pad_active())
            .field("sar_touch_meas_done", &self.sar_touch_meas_done())
            .finish()
    }
}
impl W {
    ///Bits 15:29 - Clear touch channel
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_channel_clr(
        &mut self,
    ) -> SAR_TOUCH_CHANNEL_CLR_W<SAR_TOUCH_CHN_ST_SPEC> {
        SAR_TOUCH_CHANNEL_CLR_W::new(self, 15)
    }
}
/**Get touch channel status

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_chn_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_chn_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_CHN_ST_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CHN_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_chn_st::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_CHN_ST_SPEC {}
///`write(|w| ..)` method takes [`sar_touch_chn_st::W`](W) writer structure
impl crate::Writable for SAR_TOUCH_CHN_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TOUCH_CHN_ST to value 0
impl crate::Resettable for SAR_TOUCH_CHN_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
