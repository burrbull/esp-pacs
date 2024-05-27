///Register `DTM_CONFIG` reader
pub type R = crate::R<DTM_CONFIG_SPEC>;
///Register `DTM_CONFIG` writer
pub type W = crate::W<DTM_CONFIG_SPEC>;
///Field `DTMCH_TX_LENGTH` reader -
pub type DTMCH_TX_LENGTH_R = crate::FieldReader;
///Field `DTMCH_TX_LENGTH` writer -
pub type DTMCH_TX_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTM_TX_PLD_TYPE` reader -
pub type DTM_TX_PLD_TYPE_R = crate::FieldReader;
///Field `DTM_TX_PLD_TYPE` writer -
pub type DTM_TX_PLD_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DTM_HOP_FREQ` reader -
pub type DTM_HOP_FREQ_R = crate::FieldReader;
///Field `DTM_HOP_FREQ` writer -
pub type DTM_HOP_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DTM_CONTRX_EN` reader -
pub type DTM_CONTRX_EN_R = crate::BitReader;
///Field `DTM_CONTRX_EN` writer -
pub type DTM_CONTRX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTM_ON` reader -
pub type DTM_ON_R = crate::BitReader;
///Field `DTM_ON` writer -
pub type DTM_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn dtmch_tx_length(&self) -> DTMCH_TX_LENGTH_R {
        DTMCH_TX_LENGTH_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn dtm_tx_pld_type(&self) -> DTM_TX_PLD_TYPE_R {
        DTM_TX_PLD_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:21
    #[inline(always)]
    pub fn dtm_hop_freq(&self) -> DTM_HOP_FREQ_R {
        DTM_HOP_FREQ_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 24
    #[inline(always)]
    pub fn dtm_contrx_en(&self) -> DTM_CONTRX_EN_R {
        DTM_CONTRX_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25
    #[inline(always)]
    pub fn dtm_on(&self) -> DTM_ON_R {
        DTM_ON_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTM_CONFIG")
            .field("dtmch_tx_length", &self.dtmch_tx_length())
            .field("dtm_tx_pld_type", &self.dtm_tx_pld_type())
            .field("dtm_hop_freq", &self.dtm_hop_freq())
            .field("dtm_contrx_en", &self.dtm_contrx_en())
            .field("dtm_on", &self.dtm_on())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn dtmch_tx_length(&mut self) -> DTMCH_TX_LENGTH_W<DTM_CONFIG_SPEC> {
        DTMCH_TX_LENGTH_W::new(self, 0)
    }
    ///Bits 8:11
    #[inline(always)]
    #[must_use]
    pub fn dtm_tx_pld_type(&mut self) -> DTM_TX_PLD_TYPE_W<DTM_CONFIG_SPEC> {
        DTM_TX_PLD_TYPE_W::new(self, 8)
    }
    ///Bits 16:21
    #[inline(always)]
    #[must_use]
    pub fn dtm_hop_freq(&mut self) -> DTM_HOP_FREQ_W<DTM_CONFIG_SPEC> {
        DTM_HOP_FREQ_W::new(self, 16)
    }
    ///Bit 24
    #[inline(always)]
    #[must_use]
    pub fn dtm_contrx_en(&mut self) -> DTM_CONTRX_EN_W<DTM_CONFIG_SPEC> {
        DTM_CONTRX_EN_W::new(self, 24)
    }
    ///Bit 25
    #[inline(always)]
    #[must_use]
    pub fn dtm_on(&mut self) -> DTM_ON_W<DTM_CONFIG_SPEC> {
        DTM_ON_W::new(self, 25)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dtm_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtm_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DTM_CONFIG_SPEC;
impl crate::RegisterSpec for DTM_CONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dtm_config::R`](R) reader structure
impl crate::Readable for DTM_CONFIG_SPEC {}
///`write(|w| ..)` method takes [`dtm_config::W`](W) writer structure
impl crate::Writable for DTM_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTM_CONFIG to value 0
impl crate::Resettable for DTM_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
