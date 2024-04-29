#[doc = "Register `SLC1HOST_INT_ST` reader"]
pub type R = crate::R<SLC1HOST_INT_ST_SPEC>;
#[doc = "Field `SLC1_TOHOST_BIT0_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT0_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT1_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT1_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT2_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT2_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT3_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT3_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT4_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT4_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT5_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT5_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT6_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT6_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOHOST_BIT7_INT_ST` reader - "]
pub type SLC1_TOHOST_BIT7_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN0_1TO0_INT_ST` reader - "]
pub type SLC1_TOKEN0_1TO0_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN1_1TO0_INT_ST` reader - "]
pub type SLC1_TOKEN1_1TO0_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN0_0TO1_INT_ST` reader - "]
pub type SLC1_TOKEN0_0TO1_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TOKEN1_0TO1_INT_ST` reader - "]
pub type SLC1_TOKEN1_0TO1_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1HOST_RX_SOF_INT_ST` reader - "]
pub type SLC1HOST_RX_SOF_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1HOST_RX_EOF_INT_ST` reader - "]
pub type SLC1HOST_RX_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1HOST_RX_START_INT_ST` reader - "]
pub type SLC1HOST_RX_START_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1HOST_TX_START_INT_ST` reader - "]
pub type SLC1HOST_TX_START_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_RX_UDF_INT_ST` reader - "]
pub type SLC1_RX_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_TX_OVF_INT_ST` reader - "]
pub type SLC1_TX_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_RX_PF_VALID_INT_ST` reader - "]
pub type SLC1_RX_PF_VALID_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT0_INT_ST` reader - "]
pub type SLC1_EXT_BIT0_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT1_INT_ST` reader - "]
pub type SLC1_EXT_BIT1_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT2_INT_ST` reader - "]
pub type SLC1_EXT_BIT2_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_EXT_BIT3_INT_ST` reader - "]
pub type SLC1_EXT_BIT3_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_WIFI_RX_NEW_PACKET_INT_ST` reader - "]
pub type SLC1_WIFI_RX_NEW_PACKET_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_HOST_RD_RETRY_INT_ST` reader - "]
pub type SLC1_HOST_RD_RETRY_INT_ST_R = crate::BitReader;
#[doc = "Field `SLC1_BT_RX_NEW_PACKET_INT_ST` reader - "]
pub type SLC1_BT_RX_NEW_PACKET_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc1_tohost_bit0_int_st(&self) -> SLC1_TOHOST_BIT0_INT_ST_R {
        SLC1_TOHOST_BIT0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc1_tohost_bit1_int_st(&self) -> SLC1_TOHOST_BIT1_INT_ST_R {
        SLC1_TOHOST_BIT1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc1_tohost_bit2_int_st(&self) -> SLC1_TOHOST_BIT2_INT_ST_R {
        SLC1_TOHOST_BIT2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc1_tohost_bit3_int_st(&self) -> SLC1_TOHOST_BIT3_INT_ST_R {
        SLC1_TOHOST_BIT3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc1_tohost_bit4_int_st(&self) -> SLC1_TOHOST_BIT4_INT_ST_R {
        SLC1_TOHOST_BIT4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc1_tohost_bit5_int_st(&self) -> SLC1_TOHOST_BIT5_INT_ST_R {
        SLC1_TOHOST_BIT5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc1_tohost_bit6_int_st(&self) -> SLC1_TOHOST_BIT6_INT_ST_R {
        SLC1_TOHOST_BIT6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc1_tohost_bit7_int_st(&self) -> SLC1_TOHOST_BIT7_INT_ST_R {
        SLC1_TOHOST_BIT7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc1_token0_1to0_int_st(&self) -> SLC1_TOKEN0_1TO0_INT_ST_R {
        SLC1_TOKEN0_1TO0_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc1_token1_1to0_int_st(&self) -> SLC1_TOKEN1_1TO0_INT_ST_R {
        SLC1_TOKEN1_1TO0_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc1_token0_0to1_int_st(&self) -> SLC1_TOKEN0_0TO1_INT_ST_R {
        SLC1_TOKEN0_0TO1_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc1_token1_0to1_int_st(&self) -> SLC1_TOKEN1_0TO1_INT_ST_R {
        SLC1_TOKEN1_0TO1_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1host_rx_sof_int_st(&self) -> SLC1HOST_RX_SOF_INT_ST_R {
        SLC1HOST_RX_SOF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1host_rx_eof_int_st(&self) -> SLC1HOST_RX_EOF_INT_ST_R {
        SLC1HOST_RX_EOF_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1host_rx_start_int_st(&self) -> SLC1HOST_RX_START_INT_ST_R {
        SLC1HOST_RX_START_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc1host_tx_start_int_st(&self) -> SLC1HOST_TX_START_INT_ST_R {
        SLC1HOST_TX_START_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rx_udf_int_st(&self) -> SLC1_RX_UDF_INT_ST_R {
        SLC1_RX_UDF_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_ovf_int_st(&self) -> SLC1_TX_OVF_INT_ST_R {
        SLC1_TX_OVF_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_pf_valid_int_st(&self) -> SLC1_RX_PF_VALID_INT_ST_R {
        SLC1_RX_PF_VALID_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_ext_bit0_int_st(&self) -> SLC1_EXT_BIT0_INT_ST_R {
        SLC1_EXT_BIT0_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_ext_bit1_int_st(&self) -> SLC1_EXT_BIT1_INT_ST_R {
        SLC1_EXT_BIT1_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_ext_bit2_int_st(&self) -> SLC1_EXT_BIT2_INT_ST_R {
        SLC1_EXT_BIT2_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_ext_bit3_int_st(&self) -> SLC1_EXT_BIT3_INT_ST_R {
        SLC1_EXT_BIT3_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_wifi_rx_new_packet_int_st(&self) -> SLC1_WIFI_RX_NEW_PACKET_INT_ST_R {
        SLC1_WIFI_RX_NEW_PACKET_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_host_rd_retry_int_st(&self) -> SLC1_HOST_RD_RETRY_INT_ST_R {
        SLC1_HOST_RD_RETRY_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc1_bt_rx_new_packet_int_st(&self) -> SLC1_BT_RX_NEW_PACKET_INT_ST_R {
        SLC1_BT_RX_NEW_PACKET_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC1HOST_INT_ST")
            .field(
                "slc1_tohost_bit0_int_st",
                &format_args!("{}", self.slc1_tohost_bit0_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit1_int_st",
                &format_args!("{}", self.slc1_tohost_bit1_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit2_int_st",
                &format_args!("{}", self.slc1_tohost_bit2_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit3_int_st",
                &format_args!("{}", self.slc1_tohost_bit3_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit4_int_st",
                &format_args!("{}", self.slc1_tohost_bit4_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit5_int_st",
                &format_args!("{}", self.slc1_tohost_bit5_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit6_int_st",
                &format_args!("{}", self.slc1_tohost_bit6_int_st().bit()),
            )
            .field(
                "slc1_tohost_bit7_int_st",
                &format_args!("{}", self.slc1_tohost_bit7_int_st().bit()),
            )
            .field(
                "slc1_token0_1to0_int_st",
                &format_args!("{}", self.slc1_token0_1to0_int_st().bit()),
            )
            .field(
                "slc1_token1_1to0_int_st",
                &format_args!("{}", self.slc1_token1_1to0_int_st().bit()),
            )
            .field(
                "slc1_token0_0to1_int_st",
                &format_args!("{}", self.slc1_token0_0to1_int_st().bit()),
            )
            .field(
                "slc1_token1_0to1_int_st",
                &format_args!("{}", self.slc1_token1_0to1_int_st().bit()),
            )
            .field(
                "slc1host_rx_sof_int_st",
                &format_args!("{}", self.slc1host_rx_sof_int_st().bit()),
            )
            .field(
                "slc1host_rx_eof_int_st",
                &format_args!("{}", self.slc1host_rx_eof_int_st().bit()),
            )
            .field(
                "slc1host_rx_start_int_st",
                &format_args!("{}", self.slc1host_rx_start_int_st().bit()),
            )
            .field(
                "slc1host_tx_start_int_st",
                &format_args!("{}", self.slc1host_tx_start_int_st().bit()),
            )
            .field(
                "slc1_rx_udf_int_st",
                &format_args!("{}", self.slc1_rx_udf_int_st().bit()),
            )
            .field(
                "slc1_tx_ovf_int_st",
                &format_args!("{}", self.slc1_tx_ovf_int_st().bit()),
            )
            .field(
                "slc1_rx_pf_valid_int_st",
                &format_args!("{}", self.slc1_rx_pf_valid_int_st().bit()),
            )
            .field(
                "slc1_ext_bit0_int_st",
                &format_args!("{}", self.slc1_ext_bit0_int_st().bit()),
            )
            .field(
                "slc1_ext_bit1_int_st",
                &format_args!("{}", self.slc1_ext_bit1_int_st().bit()),
            )
            .field(
                "slc1_ext_bit2_int_st",
                &format_args!("{}", self.slc1_ext_bit2_int_st().bit()),
            )
            .field(
                "slc1_ext_bit3_int_st",
                &format_args!("{}", self.slc1_ext_bit3_int_st().bit()),
            )
            .field(
                "slc1_wifi_rx_new_packet_int_st",
                &format_args!("{}", self.slc1_wifi_rx_new_packet_int_st().bit()),
            )
            .field(
                "slc1_host_rd_retry_int_st",
                &format_args!("{}", self.slc1_host_rd_retry_int_st().bit()),
            )
            .field(
                "slc1_bt_rx_new_packet_int_st",
                &format_args!("{}", self.slc1_bt_rx_new_packet_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC1HOST_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1host_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC1HOST_INT_ST_SPEC;
impl crate::RegisterSpec for SLC1HOST_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc1host_int_st::R`](R) reader structure"]
impl crate::Readable for SLC1HOST_INT_ST_SPEC {}
#[doc = "`reset()` method sets SLC1HOST_INT_ST to value 0"]
impl crate::Resettable for SLC1HOST_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
