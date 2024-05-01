///Register `RTCCALICFG` reader
pub type R = crate::R<RTCCALICFG_SPEC>;
///Register `RTCCALICFG` writer
pub type W = crate::W<RTCCALICFG_SPEC>;
///Field `RTC_CALI_START_CYCLING` reader - reg_rtc_cali_start_cycling.
pub type RTC_CALI_START_CYCLING_R = crate::BitReader;
///Field `RTC_CALI_START_CYCLING` writer - reg_rtc_cali_start_cycling.
pub type RTC_CALI_START_CYCLING_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_CALI_CLK_SEL` reader - reg_rtc_cali_clk_sel.0:rtcslowclock.1:clk_80m.2:xtal_32k
pub type RTC_CALI_CLK_SEL_R = crate::FieldReader;
///Field `RTC_CALI_CLK_SEL` writer - reg_rtc_cali_clk_sel.0:rtcslowclock.1:clk_80m.2:xtal_32k
pub type RTC_CALI_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTC_CALI_RDY` reader - rtc_cali_rdy
pub type RTC_CALI_RDY_R = crate::BitReader;
///Field `RTC_CALI_MAX` reader - reg_rtc_cali_max.
pub type RTC_CALI_MAX_R = crate::FieldReader<u16>;
///Field `RTC_CALI_MAX` writer - reg_rtc_cali_max.
pub type RTC_CALI_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `RTC_CALI_START` reader - reg_rtc_cali_start.
pub type RTC_CALI_START_R = crate::BitReader;
///Field `RTC_CALI_START` writer - reg_rtc_cali_start.
pub type RTC_CALI_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - reg_rtc_cali_start_cycling.
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> RTC_CALI_START_CYCLING_R {
        RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - reg_rtc_cali_clk_sel.0:rtcslowclock.1:clk_80m.2:xtal_32k
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> RTC_CALI_CLK_SEL_R {
        RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - rtc_cali_rdy
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> RTC_CALI_RDY_R {
        RTC_CALI_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:30 - reg_rtc_cali_max.
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> RTC_CALI_MAX_R {
        RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - reg_rtc_cali_start.
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> RTC_CALI_START_R {
        RTC_CALI_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG")
            .field("rtc_cali_start_cycling", &self.rtc_cali_start_cycling())
            .field("rtc_cali_clk_sel", &self.rtc_cali_clk_sel())
            .field("rtc_cali_rdy", &self.rtc_cali_rdy())
            .field("rtc_cali_max", &self.rtc_cali_max())
            .field("rtc_cali_start", &self.rtc_cali_start())
            .finish()
    }
}
impl W {
    ///Bit 12 - reg_rtc_cali_start_cycling.
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_start_cycling(&mut self) -> RTC_CALI_START_CYCLING_W<RTCCALICFG_SPEC> {
        RTC_CALI_START_CYCLING_W::new(self, 12)
    }
    ///Bits 13:14 - reg_rtc_cali_clk_sel.0:rtcslowclock.1:clk_80m.2:xtal_32k
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_clk_sel(&mut self) -> RTC_CALI_CLK_SEL_W<RTCCALICFG_SPEC> {
        RTC_CALI_CLK_SEL_W::new(self, 13)
    }
    ///Bits 16:30 - reg_rtc_cali_max.
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_max(&mut self) -> RTC_CALI_MAX_W<RTCCALICFG_SPEC> {
        RTC_CALI_MAX_W::new(self, 16)
    }
    ///Bit 31 - reg_rtc_cali_start.
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_start(&mut self) -> RTC_CALI_START_W<RTCCALICFG_SPEC> {
        RTC_CALI_START_W::new(self, 31)
    }
}
/**TIMG_RTCCALICFG_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTCCALICFG_SPEC;
impl crate::RegisterSpec for RTCCALICFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rtccalicfg::R`](R) reader structure
impl crate::Readable for RTCCALICFG_SPEC {}
///`write(|w| ..)` method takes [`rtccalicfg::W`](W) writer structure
impl crate::Writable for RTCCALICFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTCCALICFG to value 0x0001_3000
impl crate::Resettable for RTCCALICFG_SPEC {
    const RESET_VALUE: u32 = 0x0001_3000;
}
