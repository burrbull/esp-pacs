///Register `CH%s_TX_LIM` reader
pub type R = crate::R<CH_TX_LIM_SPEC>;
///Register `CH%s_TX_LIM` writer
pub type W = crate::W<CH_TX_LIM_SPEC>;
///Field `TX_LIM_CH` reader - This register is used to configure the maximum entries that CHANNEL%s can send out.
pub type TX_LIM_CH_R = crate::FieldReader<u16>;
///Field `TX_LIM_CH` writer - This register is used to configure the maximum entries that CHANNEL%s can send out.
pub type TX_LIM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `TX_LOOP_NUM_CH` reader - This register is used to configure the maximum loop count when tx_conti_mode is valid.
pub type TX_LOOP_NUM_CH_R = crate::FieldReader<u16>;
///Field `TX_LOOP_NUM_CH` writer - This register is used to configure the maximum loop count when tx_conti_mode is valid.
pub type TX_LOOP_NUM_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TX_LOOP_CNT_EN_CH` reader - This register is the enabled bit for loop count.
pub type TX_LOOP_CNT_EN_CH_R = crate::BitReader;
///Field `TX_LOOP_CNT_EN_CH` writer - This register is the enabled bit for loop count.
pub type TX_LOOP_CNT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOP_COUNT_RESET_CH` writer - This register is used to reset the loop count when tx_conti_mode is valid.
pub type LOOP_COUNT_RESET_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOP_STOP_EN_CH` reader - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
pub type LOOP_STOP_EN_CH_R = crate::BitReader;
///Field `LOOP_STOP_EN_CH` writer - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
pub type LOOP_STOP_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    pub fn tx_lim_ch(&self) -> TX_LIM_CH_R {
        TX_LIM_CH_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    pub fn tx_loop_num_ch(&self) -> TX_LOOP_NUM_CH_R {
        TX_LOOP_NUM_CH_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    ///Bit 19 - This register is the enabled bit for loop count.
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&self) -> TX_LOOP_CNT_EN_CH_R {
        TX_LOOP_CNT_EN_CH_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    pub fn loop_stop_en_ch(&self) -> LOOP_STOP_EN_CH_R {
        LOOP_STOP_EN_CH_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_LIM")
            .field("tx_lim_ch", &self.tx_lim_ch())
            .field("tx_loop_num_ch", &self.tx_loop_num_ch())
            .field("tx_loop_cnt_en_ch", &self.tx_loop_cnt_en_ch())
            .field("loop_stop_en_ch", &self.loop_stop_en_ch())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out.
    #[inline(always)]
    #[must_use]
    pub fn tx_lim_ch(&mut self) -> TX_LIM_CH_W<CH_TX_LIM_SPEC> {
        TX_LIM_CH_W::new(self, 0)
    }
    ///Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid.
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_num_ch(&mut self) -> TX_LOOP_NUM_CH_W<CH_TX_LIM_SPEC> {
        TX_LOOP_NUM_CH_W::new(self, 9)
    }
    ///Bit 19 - This register is the enabled bit for loop count.
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_cnt_en_ch(&mut self) -> TX_LOOP_CNT_EN_CH_W<CH_TX_LIM_SPEC> {
        TX_LOOP_CNT_EN_CH_W::new(self, 19)
    }
    ///Bit 20 - This register is used to reset the loop count when tx_conti_mode is valid.
    #[inline(always)]
    #[must_use]
    pub fn loop_count_reset_ch(&mut self) -> LOOP_COUNT_RESET_CH_W<CH_TX_LIM_SPEC> {
        LOOP_COUNT_RESET_CH_W::new(self, 20)
    }
    ///Bit 21 - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn loop_stop_en_ch(&mut self) -> LOOP_STOP_EN_CH_W<CH_TX_LIM_SPEC> {
        LOOP_STOP_EN_CH_W::new(self, 21)
    }
}
/**Channel %s Tx event configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_TX_LIM_SPEC;
impl crate::RegisterSpec for CH_TX_LIM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ch_tx_lim::R`](R) reader structure
impl crate::Readable for CH_TX_LIM_SPEC {}
///`write(|w| ..)` method takes [`ch_tx_lim::W`](W) writer structure
impl crate::Writable for CH_TX_LIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH%s_TX_LIM to value 0x80
impl crate::Resettable for CH_TX_LIM_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
