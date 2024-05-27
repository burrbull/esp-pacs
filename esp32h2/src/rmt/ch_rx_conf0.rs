///Register `CH%s_RX_CONF0` reader
pub type R = crate::R<CH_RX_CONF0_SPEC>;
///Register `CH%s_RX_CONF0` writer
pub type W = crate::W<CH_RX_CONF0_SPEC>;
///Field `DIV_CNT` reader - This register is used to configure the divider for clock of CHANNEL%s.
pub type DIV_CNT_R = crate::FieldReader;
///Field `DIV_CNT` writer - This register is used to configure the divider for clock of CHANNEL%s.
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IDLE_THRES` reader - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
pub type IDLE_THRES_R = crate::FieldReader<u16>;
///Field `IDLE_THRES` writer - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
pub type IDLE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `MEM_SIZE` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
pub type MEM_SIZE_R = crate::FieldReader;
///Field `MEM_SIZE` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CARRIER_EN` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
pub type CARRIER_EN_R = crate::BitReader;
///Field `CARRIER_EN` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARRIER_OUT_LV` reader - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level.
pub type CARRIER_OUT_LV_R = crate::BitReader;
///Field `CARRIER_OUT_LV` writer - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level.
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    ///Bits 23:25 - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF0")
            .field("div_cnt", &self.div_cnt())
            .field("idle_thres", &self.idle_thres())
            .field("mem_size", &self.mem_size())
            .field("carrier_en", &self.carrier_en())
            .field("carrier_out_lv", &self.carrier_out_lv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<CH_RX_CONF0_SPEC> {
        DIV_CNT_W::new(self, 0)
    }
    ///Bits 8:22 - When no edge is detected on the input signal and continuous clock cycles is longer than this register value, received process is finished.
    #[inline(always)]
    #[must_use]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W<CH_RX_CONF0_SPEC> {
        IDLE_THRES_W::new(self, 8)
    }
    ///Bits 23:25 - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<CH_RX_CONF0_SPEC> {
        MEM_SIZE_W::new(self, 23)
    }
    ///Bit 28 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<CH_RX_CONF0_SPEC> {
        CARRIER_EN_W::new(self, 28)
    }
    ///Bit 29 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level.
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<CH_RX_CONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 29)
    }
}
/**Channel %s configure register 0

You can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_RX_CONF0_SPEC;
impl crate::RegisterSpec for CH_RX_CONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ch_rx_conf0::R`](R) reader structure
impl crate::Readable for CH_RX_CONF0_SPEC {}
///`write(|w| ..)` method takes [`ch_rx_conf0::W`](W) writer structure
impl crate::Writable for CH_RX_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH%s_RX_CONF0 to value 0x30ff_ff02
impl crate::Resettable for CH_RX_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x30ff_ff02;
}
