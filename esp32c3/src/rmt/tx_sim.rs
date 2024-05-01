///Register `TX_SIM` reader
pub type R = crate::R<TX_SIM_SPEC>;
///Register `TX_SIM` writer
pub type W = crate::W<TX_SIM_SPEC>;
///Field `TX_SIM_CH0` reader - reg_rmt_tx_sim_ch0.
pub type TX_SIM_CH0_R = crate::BitReader;
///Field `TX_SIM_CH0` writer - reg_rmt_tx_sim_ch0.
pub type TX_SIM_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_SIM_CH1` reader - reg_rmt_tx_sim_ch1.
pub type TX_SIM_CH1_R = crate::BitReader;
///Field `TX_SIM_CH1` writer - reg_rmt_tx_sim_ch1.
pub type TX_SIM_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_SIM_EN` reader - reg_rmt_tx_sim_en.
pub type TX_SIM_EN_R = crate::BitReader;
///Field `TX_SIM_EN` writer - reg_rmt_tx_sim_en.
pub type TX_SIM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reg_rmt_tx_sim_ch0.
    #[inline(always)]
    pub fn tx_sim_ch0(&self) -> TX_SIM_CH0_R {
        TX_SIM_CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - reg_rmt_tx_sim_ch1.
    #[inline(always)]
    pub fn tx_sim_ch1(&self) -> TX_SIM_CH1_R {
        TX_SIM_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - reg_rmt_tx_sim_en.
    #[inline(always)]
    pub fn tx_sim_en(&self) -> TX_SIM_EN_R {
        TX_SIM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_SIM")
            .field("tx_sim_ch0", &self.tx_sim_ch0())
            .field("tx_sim_ch1", &self.tx_sim_ch1())
            .field("tx_sim_en", &self.tx_sim_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - reg_rmt_tx_sim_ch0.
    #[inline(always)]
    #[must_use]
    pub fn tx_sim_ch0(&mut self) -> TX_SIM_CH0_W<TX_SIM_SPEC> {
        TX_SIM_CH0_W::new(self, 0)
    }
    ///Bit 1 - reg_rmt_tx_sim_ch1.
    #[inline(always)]
    #[must_use]
    pub fn tx_sim_ch1(&mut self) -> TX_SIM_CH1_W<TX_SIM_SPEC> {
        TX_SIM_CH1_W::new(self, 1)
    }
    ///Bit 2 - reg_rmt_tx_sim_en.
    #[inline(always)]
    #[must_use]
    pub fn tx_sim_en(&mut self) -> TX_SIM_EN_W<TX_SIM_SPEC> {
        TX_SIM_EN_W::new(self, 2)
    }
}
/**RMT_TX_SIM_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`tx_sim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_sim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_SIM_SPEC;
impl crate::RegisterSpec for TX_SIM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_sim::R`](R) reader structure
impl crate::Readable for TX_SIM_SPEC {}
///`write(|w| ..)` method takes [`tx_sim::W`](W) writer structure
impl crate::Writable for TX_SIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_SIM to value 0
impl crate::Resettable for TX_SIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
