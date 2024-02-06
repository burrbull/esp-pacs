#[doc = "Register `IN_CONF0_CH%s` reader"]
pub type R = crate::R<IN_CONF0_CH_SPEC>;
#[doc = "Register `IN_CONF0_CH%s` writer"]
pub type W = crate::W<IN_CONF0_CH_SPEC>;
#[doc = "Field `IN_RST_CH` reader - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_CH_R = crate::BitReader;
#[doc = "Field `IN_RST_CH` writer - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST_CH` reader - reserved"]
pub type IN_LOOP_TEST_CH_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST_CH` writer - reserved"]
pub type IN_LOOP_TEST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN_CH` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
pub type MEM_TRANS_EN_CH_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN_CH` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
pub type MEM_TRANS_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_EN_CH` reader - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
pub type IN_ETM_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN_CH` writer - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
pub type IN_ETM_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_BURST_SIZE_SEL_CH` reader - 3'b000-3'b100:burst length 8byte~128byte"]
pub type IN_BURST_SIZE_SEL_CH_R = crate::FieldReader;
#[doc = "Field `IN_BURST_SIZE_SEL_CH` writer - 3'b000-3'b100:burst length 8byte~128byte"]
pub type IN_BURST_SIZE_SEL_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IN_CMD_DISABLE_CH` reader - 1:mean disable cmd of this ch0"]
pub type IN_CMD_DISABLE_CH_R = crate::BitReader;
#[doc = "Field `IN_CMD_DISABLE_CH` writer - 1:mean disable cmd of this ch0"]
pub type IN_CMD_DISABLE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ECC_AEC_EN_CH` reader - 1: mean access ecc or aes domain,0: mean not"]
pub type IN_ECC_AEC_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_ECC_AEC_EN_CH` writer - 1: mean access ecc or aes domain,0: mean not"]
pub type IN_ECC_AEC_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN_CH` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_CH_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN_CH` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch(&self) -> IN_RST_CH_R {
        IN_RST_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch(&self) -> IN_LOOP_TEST_CH_R {
        IN_LOOP_TEST_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch(&self) -> MEM_TRANS_EN_CH_R {
        MEM_TRANS_EN_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
    #[inline(always)]
    pub fn in_etm_en_ch(&self) -> IN_ETM_EN_CH_R {
        IN_ETM_EN_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    pub fn in_burst_size_sel_ch(&self) -> IN_BURST_SIZE_SEL_CH_R {
        IN_BURST_SIZE_SEL_CH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 1:mean disable cmd of this ch0"]
    #[inline(always)]
    pub fn in_cmd_disable_ch(&self) -> IN_CMD_DISABLE_CH_R {
        IN_CMD_DISABLE_CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    pub fn in_ecc_aec_en_ch(&self) -> IN_ECC_AEC_EN_CH_R {
        IN_ECC_AEC_EN_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch(&self) -> INDSCR_BURST_EN_CH_R {
        INDSCR_BURST_EN_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0_CH")
            .field("in_rst_ch", &format_args!("{}", self.in_rst_ch().bit()))
            .field(
                "in_loop_test_ch",
                &format_args!("{}", self.in_loop_test_ch().bit()),
            )
            .field(
                "mem_trans_en_ch",
                &format_args!("{}", self.mem_trans_en_ch().bit()),
            )
            .field(
                "in_etm_en_ch",
                &format_args!("{}", self.in_etm_en_ch().bit()),
            )
            .field(
                "in_burst_size_sel_ch",
                &format_args!("{}", self.in_burst_size_sel_ch().bits()),
            )
            .field(
                "in_cmd_disable_ch",
                &format_args!("{}", self.in_cmd_disable_ch().bit()),
            )
            .field(
                "in_ecc_aec_en_ch",
                &format_args!("{}", self.in_ecc_aec_en_ch().bit()),
            )
            .field(
                "indscr_burst_en_ch",
                &format_args!("{}", self.indscr_burst_en_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF0_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset AXI_DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst_ch(&mut self) -> IN_RST_CH_W<IN_CONF0_CH_SPEC> {
        IN_RST_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test_ch(&mut self) -> IN_LOOP_TEST_CH_W<IN_CONF0_CH_SPEC> {
        IN_LOOP_TEST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit 1 to enable automatic transmitting data from memory to memory via AXI_DMA."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en_ch(&mut self) -> MEM_TRANS_EN_CH_W<IN_CONF0_CH_SPEC> {
        MEM_TRANS_EN_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable etm control mode, dma Rx channel 0 is triggered by etm task."]
    #[inline(always)]
    #[must_use]
    pub fn in_etm_en_ch(&mut self) -> IN_ETM_EN_CH_W<IN_CONF0_CH_SPEC> {
        IN_ETM_EN_CH_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 3'b000-3'b100:burst length 8byte~128byte"]
    #[inline(always)]
    #[must_use]
    pub fn in_burst_size_sel_ch(&mut self) -> IN_BURST_SIZE_SEL_CH_W<IN_CONF0_CH_SPEC> {
        IN_BURST_SIZE_SEL_CH_W::new(self, 4)
    }
    #[doc = "Bit 7 - 1:mean disable cmd of this ch0"]
    #[inline(always)]
    #[must_use]
    pub fn in_cmd_disable_ch(&mut self) -> IN_CMD_DISABLE_CH_W<IN_CONF0_CH_SPEC> {
        IN_CMD_DISABLE_CH_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: mean access ecc or aes domain,0: mean not"]
    #[inline(always)]
    #[must_use]
    pub fn in_ecc_aec_en_ch(&mut self) -> IN_ECC_AEC_EN_CH_W<IN_CONF0_CH_SPEC> {
        IN_ECC_AEC_EN_CH_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en_ch(&mut self) -> INDSCR_BURST_EN_CH_W<IN_CONF0_CH_SPEC> {
        INDSCR_BURST_EN_CH_W::new(self, 9)
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
#[doc = "Configure 0 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_CH_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0_ch::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0_ch::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF0_CH%s to value 0"]
impl crate::Resettable for IN_CONF0_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
