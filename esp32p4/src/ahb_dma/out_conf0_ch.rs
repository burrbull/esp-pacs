#[doc = "Register `OUT_CONF0_CH%s` reader"]
pub type R = crate::R<OUT_CONF0_CH_SPEC>;
#[doc = "Register `OUT_CONF0_CH%s` writer"]
pub type W = crate::W<OUT_CONF0_CH_SPEC>;
#[doc = "Field `OUT_RST_CH` reader - This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH_R = crate::BitReader;
#[doc = "Field `OUT_RST_CH` writer - This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST_CH` reader - reserved"]
pub type OUT_LOOP_TEST_CH_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST_CH` writer - reserved"]
pub type OUT_LOOP_TEST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK_CH` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK_CH` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE_CH` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
pub type OUT_EOF_MODE_CH_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE_CH` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
pub type OUT_EOF_MODE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_EN_CH` reader - Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task."]
pub type OUT_ETM_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN_CH` writer - Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task."]
pub type OUT_ETM_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch(&self) -> OUT_RST_CH_R {
        OUT_RST_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch(&self) -> OUT_LOOP_TEST_CH_R {
        OUT_LOOP_TEST_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch(&self) -> OUT_AUTO_WRBACK_CH_R {
        OUT_AUTO_WRBACK_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch(&self) -> OUT_EOF_MODE_CH_R {
        OUT_EOF_MODE_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch(&self) -> OUTDSCR_BURST_EN_CH_R {
        OUTDSCR_BURST_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch(&self) -> OUT_DATA_BURST_EN_CH_R {
        OUT_DATA_BURST_EN_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task."]
    #[inline(always)]
    pub fn out_etm_en_ch(&self) -> OUT_ETM_EN_CH_R {
        OUT_ETM_EN_CH_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF0_CH")
            .field("out_rst_ch", &format_args!("{}", self.out_rst_ch().bit()))
            .field(
                "out_loop_test_ch",
                &format_args!("{}", self.out_loop_test_ch().bit()),
            )
            .field(
                "out_auto_wrback_ch",
                &format_args!("{}", self.out_auto_wrback_ch().bit()),
            )
            .field(
                "out_eof_mode_ch",
                &format_args!("{}", self.out_eof_mode_ch().bit()),
            )
            .field(
                "outdscr_burst_en_ch",
                &format_args!("{}", self.outdscr_burst_en_ch().bit()),
            )
            .field(
                "out_data_burst_en_ch",
                &format_args!("{}", self.out_data_burst_en_ch().bit()),
            )
            .field(
                "out_etm_en_ch",
                &format_args!("{}", self.out_etm_en_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CONF0_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset AHB_DMA channel 1 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst_ch(&mut self) -> OUT_RST_CH_W<OUT_CONF0_CH_SPEC> {
        OUT_RST_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test_ch(&mut self) -> OUT_LOOP_TEST_CH_W<OUT_CONF0_CH_SPEC> {
        OUT_LOOP_TEST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback_ch(&mut self) -> OUT_AUTO_WRBACK_CH_W<OUT_CONF0_CH_SPEC> {
        OUT_AUTO_WRBACK_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in AHB_DMA"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode_ch(&mut self) -> OUT_EOF_MODE_CH_W<OUT_CONF0_CH_SPEC> {
        OUT_EOF_MODE_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en_ch(&mut self) -> OUTDSCR_BURST_EN_CH_W<OUT_CONF0_CH_SPEC> {
        OUTDSCR_BURST_EN_CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn out_data_burst_en_ch(&mut self) -> OUT_DATA_BURST_EN_CH_W<OUT_CONF0_CH_SPEC> {
        OUT_DATA_BURST_EN_CH_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable etm control mode, dma Tx channel 1 is triggered by etm task."]
    #[inline(always)]
    #[must_use]
    pub fn out_etm_en_ch(&mut self) -> OUT_ETM_EN_CH_W<OUT_CONF0_CH_SPEC> {
        OUT_ETM_EN_CH_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure 0 register of Tx channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF0_CH_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf0_ch::R`](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf0_ch::W`](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CONF0_CH%s to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
