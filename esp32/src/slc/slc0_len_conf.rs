#[doc = "Register `SLC0_LEN_CONF` reader"]
pub type R = crate::R<SLC0_LEN_CONF_SPEC>;
#[doc = "Register `SLC0_LEN_CONF` writer"]
pub type W = crate::W<SLC0_LEN_CONF_SPEC>;
#[doc = "Field `LEN_WDATA` writer - "]
pub type LEN_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `LEN_WR` writer - "]
pub type LEN_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_INC` writer - "]
pub type LEN_INC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_INC_MORE` writer - "]
pub type LEN_INC_MORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PACKET_LOAD_EN` reader - "]
pub type RX_PACKET_LOAD_EN_R = crate::BitReader;
#[doc = "Field `RX_PACKET_LOAD_EN` writer - "]
pub type RX_PACKET_LOAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PACKET_LOAD_EN` reader - "]
pub type TX_PACKET_LOAD_EN_R = crate::BitReader;
#[doc = "Field `TX_PACKET_LOAD_EN` writer - "]
pub type TX_PACKET_LOAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_GET_USED_DSCR` writer - "]
pub type RX_GET_USED_DSCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_GET_USED_DSCR` writer - "]
pub type TX_GET_USED_DSCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NEW_PKT_IND` reader - "]
pub type RX_NEW_PKT_IND_R = crate::BitReader;
#[doc = "Field `TX_NEW_PKT_IND` reader - "]
pub type TX_NEW_PKT_IND_R = crate::BitReader;
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_packet_load_en(&self) -> RX_PACKET_LOAD_EN_R {
        RX_PACKET_LOAD_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_packet_load_en(&self) -> TX_PACKET_LOAD_EN_R {
        TX_PACKET_LOAD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rx_new_pkt_ind(&self) -> RX_NEW_PKT_IND_R {
        RX_NEW_PKT_IND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tx_new_pkt_ind(&self) -> TX_NEW_PKT_IND_R {
        TX_NEW_PKT_IND_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_LEN_CONF")
            .field(
                "rx_packet_load_en",
                &format_args!("{}", self.rx_packet_load_en().bit()),
            )
            .field(
                "tx_packet_load_en",
                &format_args!("{}", self.tx_packet_load_en().bit()),
            )
            .field(
                "rx_new_pkt_ind",
                &format_args!("{}", self.rx_new_pkt_ind().bit()),
            )
            .field(
                "tx_new_pkt_ind",
                &format_args!("{}", self.tx_new_pkt_ind().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_LEN_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn len_wdata(&mut self) -> LEN_WDATA_W<SLC0_LEN_CONF_SPEC> {
        LEN_WDATA_W::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn len_wr(&mut self) -> LEN_WR_W<SLC0_LEN_CONF_SPEC> {
        LEN_WR_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn len_inc(&mut self) -> LEN_INC_W<SLC0_LEN_CONF_SPEC> {
        LEN_INC_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn len_inc_more(&mut self) -> LEN_INC_MORE_W<SLC0_LEN_CONF_SPEC> {
        LEN_INC_MORE_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_packet_load_en(&mut self) -> RX_PACKET_LOAD_EN_W<SLC0_LEN_CONF_SPEC> {
        RX_PACKET_LOAD_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tx_packet_load_en(&mut self) -> TX_PACKET_LOAD_EN_W<SLC0_LEN_CONF_SPEC> {
        TX_PACKET_LOAD_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rx_get_used_dscr(&mut self) -> RX_GET_USED_DSCR_W<SLC0_LEN_CONF_SPEC> {
        RX_GET_USED_DSCR_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tx_get_used_dscr(&mut self) -> TX_GET_USED_DSCR_W<SLC0_LEN_CONF_SPEC> {
        TX_GET_USED_DSCR_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_len_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_len_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_LEN_CONF_SPEC;
impl crate::RegisterSpec for SLC0_LEN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_len_conf::R`](R) reader structure"]
impl crate::Readable for SLC0_LEN_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_len_conf::W`](W) writer structure"]
impl crate::Writable for SLC0_LEN_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_LEN_CONF to value 0"]
impl crate::Resettable for SLC0_LEN_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
