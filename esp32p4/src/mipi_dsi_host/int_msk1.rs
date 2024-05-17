///Register `INT_MSK1` reader
pub type R = crate::R<INT_MSK1_SPEC>;
///Register `INT_MSK1` writer
pub type W = crate::W<INT_MSK1_SPEC>;
///Field `MASK_TO_HS_TX` reader - NA
pub type MASK_TO_HS_TX_R = crate::BitReader;
///Field `MASK_TO_HS_TX` writer - NA
pub type MASK_TO_HS_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_TO_LP_RX` reader - NA
pub type MASK_TO_LP_RX_R = crate::BitReader;
///Field `MASK_TO_LP_RX` writer - NA
pub type MASK_TO_LP_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_ECC_SINGLE_ERR` reader - NA
pub type MASK_ECC_SINGLE_ERR_R = crate::BitReader;
///Field `MASK_ECC_SINGLE_ERR` writer - NA
pub type MASK_ECC_SINGLE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_ECC_MILTI_ERR` reader - NA
pub type MASK_ECC_MILTI_ERR_R = crate::BitReader;
///Field `MASK_ECC_MILTI_ERR` writer - NA
pub type MASK_ECC_MILTI_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_CRC_ERR` reader - NA
pub type MASK_CRC_ERR_R = crate::BitReader;
///Field `MASK_CRC_ERR` writer - NA
pub type MASK_CRC_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_PKT_SIZE_ERR` reader - NA
pub type MASK_PKT_SIZE_ERR_R = crate::BitReader;
///Field `MASK_PKT_SIZE_ERR` writer - NA
pub type MASK_PKT_SIZE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_EOPT_ERR` reader - NA
pub type MASK_EOPT_ERR_R = crate::BitReader;
///Field `MASK_EOPT_ERR` writer - NA
pub type MASK_EOPT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_DPI_PLD_WR_ERR` reader - NA
pub type MASK_DPI_PLD_WR_ERR_R = crate::BitReader;
///Field `MASK_DPI_PLD_WR_ERR` writer - NA
pub type MASK_DPI_PLD_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_GEN_CMD_WR_ERR` reader - NA
pub type MASK_GEN_CMD_WR_ERR_R = crate::BitReader;
///Field `MASK_GEN_CMD_WR_ERR` writer - NA
pub type MASK_GEN_CMD_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_GEN_PLD_WR_ERR` reader - NA
pub type MASK_GEN_PLD_WR_ERR_R = crate::BitReader;
///Field `MASK_GEN_PLD_WR_ERR` writer - NA
pub type MASK_GEN_PLD_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_GEN_PLD_SEND_ERR` reader - NA
pub type MASK_GEN_PLD_SEND_ERR_R = crate::BitReader;
///Field `MASK_GEN_PLD_SEND_ERR` writer - NA
pub type MASK_GEN_PLD_SEND_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_GEN_PLD_RD_ERR` reader - NA
pub type MASK_GEN_PLD_RD_ERR_R = crate::BitReader;
///Field `MASK_GEN_PLD_RD_ERR` writer - NA
pub type MASK_GEN_PLD_RD_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_GEN_PLD_RECEV_ERR` reader - NA
pub type MASK_GEN_PLD_RECEV_ERR_R = crate::BitReader;
///Field `MASK_GEN_PLD_RECEV_ERR` writer - NA
pub type MASK_GEN_PLD_RECEV_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASK_DPI_BUFF_PLD_UNDER` reader - NA
pub type MASK_DPI_BUFF_PLD_UNDER_R = crate::BitReader;
///Field `MASK_DPI_BUFF_PLD_UNDER` writer - NA
pub type MASK_DPI_BUFF_PLD_UNDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn mask_to_hs_tx(&self) -> MASK_TO_HS_TX_R {
        MASK_TO_HS_TX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn mask_to_lp_rx(&self) -> MASK_TO_LP_RX_R {
        MASK_TO_LP_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn mask_ecc_single_err(&self) -> MASK_ECC_SINGLE_ERR_R {
        MASK_ECC_SINGLE_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NA
    #[inline(always)]
    pub fn mask_ecc_milti_err(&self) -> MASK_ECC_MILTI_ERR_R {
        MASK_ECC_MILTI_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NA
    #[inline(always)]
    pub fn mask_crc_err(&self) -> MASK_CRC_ERR_R {
        MASK_CRC_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NA
    #[inline(always)]
    pub fn mask_pkt_size_err(&self) -> MASK_PKT_SIZE_ERR_R {
        MASK_PKT_SIZE_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NA
    #[inline(always)]
    pub fn mask_eopt_err(&self) -> MASK_EOPT_ERR_R {
        MASK_EOPT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NA
    #[inline(always)]
    pub fn mask_dpi_pld_wr_err(&self) -> MASK_DPI_PLD_WR_ERR_R {
        MASK_DPI_PLD_WR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - NA
    #[inline(always)]
    pub fn mask_gen_cmd_wr_err(&self) -> MASK_GEN_CMD_WR_ERR_R {
        MASK_GEN_CMD_WR_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NA
    #[inline(always)]
    pub fn mask_gen_pld_wr_err(&self) -> MASK_GEN_PLD_WR_ERR_R {
        MASK_GEN_PLD_WR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NA
    #[inline(always)]
    pub fn mask_gen_pld_send_err(&self) -> MASK_GEN_PLD_SEND_ERR_R {
        MASK_GEN_PLD_SEND_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - NA
    #[inline(always)]
    pub fn mask_gen_pld_rd_err(&self) -> MASK_GEN_PLD_RD_ERR_R {
        MASK_GEN_PLD_RD_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - NA
    #[inline(always)]
    pub fn mask_gen_pld_recev_err(&self) -> MASK_GEN_PLD_RECEV_ERR_R {
        MASK_GEN_PLD_RECEV_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 19 - NA
    #[inline(always)]
    pub fn mask_dpi_buff_pld_under(&self) -> MASK_DPI_BUFF_PLD_UNDER_R {
        MASK_DPI_BUFF_PLD_UNDER_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MSK1")
            .field("mask_to_hs_tx", &self.mask_to_hs_tx())
            .field("mask_to_lp_rx", &self.mask_to_lp_rx())
            .field("mask_ecc_single_err", &self.mask_ecc_single_err())
            .field("mask_ecc_milti_err", &self.mask_ecc_milti_err())
            .field("mask_crc_err", &self.mask_crc_err())
            .field("mask_pkt_size_err", &self.mask_pkt_size_err())
            .field("mask_eopt_err", &self.mask_eopt_err())
            .field("mask_dpi_pld_wr_err", &self.mask_dpi_pld_wr_err())
            .field("mask_gen_cmd_wr_err", &self.mask_gen_cmd_wr_err())
            .field("mask_gen_pld_wr_err", &self.mask_gen_pld_wr_err())
            .field("mask_gen_pld_send_err", &self.mask_gen_pld_send_err())
            .field("mask_gen_pld_rd_err", &self.mask_gen_pld_rd_err())
            .field("mask_gen_pld_recev_err", &self.mask_gen_pld_recev_err())
            .field("mask_dpi_buff_pld_under", &self.mask_dpi_buff_pld_under())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_to_hs_tx(&mut self) -> MASK_TO_HS_TX_W<INT_MSK1_SPEC> {
        MASK_TO_HS_TX_W::new(self, 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_to_lp_rx(&mut self) -> MASK_TO_LP_RX_W<INT_MSK1_SPEC> {
        MASK_TO_LP_RX_W::new(self, 1)
    }
    ///Bit 2 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_ecc_single_err(&mut self) -> MASK_ECC_SINGLE_ERR_W<INT_MSK1_SPEC> {
        MASK_ECC_SINGLE_ERR_W::new(self, 2)
    }
    ///Bit 3 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_ecc_milti_err(&mut self) -> MASK_ECC_MILTI_ERR_W<INT_MSK1_SPEC> {
        MASK_ECC_MILTI_ERR_W::new(self, 3)
    }
    ///Bit 4 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_crc_err(&mut self) -> MASK_CRC_ERR_W<INT_MSK1_SPEC> {
        MASK_CRC_ERR_W::new(self, 4)
    }
    ///Bit 5 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_pkt_size_err(&mut self) -> MASK_PKT_SIZE_ERR_W<INT_MSK1_SPEC> {
        MASK_PKT_SIZE_ERR_W::new(self, 5)
    }
    ///Bit 6 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_eopt_err(&mut self) -> MASK_EOPT_ERR_W<INT_MSK1_SPEC> {
        MASK_EOPT_ERR_W::new(self, 6)
    }
    ///Bit 7 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_dpi_pld_wr_err(&mut self) -> MASK_DPI_PLD_WR_ERR_W<INT_MSK1_SPEC> {
        MASK_DPI_PLD_WR_ERR_W::new(self, 7)
    }
    ///Bit 8 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_gen_cmd_wr_err(&mut self) -> MASK_GEN_CMD_WR_ERR_W<INT_MSK1_SPEC> {
        MASK_GEN_CMD_WR_ERR_W::new(self, 8)
    }
    ///Bit 9 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_gen_pld_wr_err(&mut self) -> MASK_GEN_PLD_WR_ERR_W<INT_MSK1_SPEC> {
        MASK_GEN_PLD_WR_ERR_W::new(self, 9)
    }
    ///Bit 10 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_gen_pld_send_err(&mut self) -> MASK_GEN_PLD_SEND_ERR_W<INT_MSK1_SPEC> {
        MASK_GEN_PLD_SEND_ERR_W::new(self, 10)
    }
    ///Bit 11 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_gen_pld_rd_err(&mut self) -> MASK_GEN_PLD_RD_ERR_W<INT_MSK1_SPEC> {
        MASK_GEN_PLD_RD_ERR_W::new(self, 11)
    }
    ///Bit 12 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_gen_pld_recev_err(&mut self) -> MASK_GEN_PLD_RECEV_ERR_W<INT_MSK1_SPEC> {
        MASK_GEN_PLD_RECEV_ERR_W::new(self, 12)
    }
    ///Bit 19 - NA
    #[inline(always)]
    #[must_use]
    pub fn mask_dpi_buff_pld_under(
        &mut self,
    ) -> MASK_DPI_BUFF_PLD_UNDER_W<INT_MSK1_SPEC> {
        MASK_DPI_BUFF_PLD_UNDER_W::new(self, 19)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_msk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_MSK1_SPEC;
impl crate::RegisterSpec for INT_MSK1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_msk1::R`](R) reader structure
impl crate::Readable for INT_MSK1_SPEC {}
///`write(|w| ..)` method takes [`int_msk1::W`](W) writer structure
impl crate::Writable for INT_MSK1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_MSK1 to value 0
impl crate::Resettable for INT_MSK1_SPEC {
    const RESET_VALUE: u32 = 0;
}
