///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `DONE` writer - This clear interrupt bit turns to high level when JPEG finishes encoding a picture..
pub type DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RLE_PARALLEL_ERR` writer - The clear interrupt bit to sign that rle parallel error when decoding.
pub type RLE_PARALLEL_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CID_ERR` writer - The clear interrupt bit to sign that scan id check with component fails when decoding.
pub type CID_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `C_DHT_DC_ID_ERR` writer - The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
pub type C_DHT_DC_ID_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `C_DHT_AC_ID_ERR` writer - The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
pub type C_DHT_AC_ID_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `C_DQT_ID_ERR` writer - The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
pub type C_DQT_ID_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RST_UXP_ERR` writer - The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
pub type RST_UXP_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RST_CHECK_NONE_ERR` writer - The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
pub type RST_CHECK_NONE_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RST_CHECK_POS_ERR` writer - The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
pub type RST_CHECK_POS_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_EOF` writer - The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
pub type OUT_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SR_COLOR_MODE_ERR` writer - The clear interrupt bit to sign that the selected source color mode is not supported.
pub type SR_COLOR_MODE_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DCT_DONE` writer - The clear interrupt bit to sign that one dct calculation is finished.
pub type DCT_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `BS_LAST_BLOCK_EOF` writer - The clear interrupt bit to sign that the coding process for last block is finished.
pub type BS_LAST_BLOCK_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SCAN_CHECK_NONE_ERR` writer - The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
pub type SCAN_CHECK_NONE_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SCAN_CHECK_POS_ERR` writer - The clear interrupt bit to sign that SOS header marker position wrong when decoding.
pub type SCAN_CHECK_POS_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `UXP_DET` writer - The clear interrupt bit to sign that unsupported header marker is detected when decoding.
pub type UXP_DET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `EN_FRAME_EOF_ERR` writer - The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
pub type EN_FRAME_EOF_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `EN_FRAME_EOF_LACK` writer - The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
pub type EN_FRAME_EOF_LACK_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DE_FRAME_EOF_ERR` writer - The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding.
pub type DE_FRAME_EOF_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DE_FRAME_EOF_LACK` writer - The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
pub type DE_FRAME_EOF_LACK_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SOS_UNMATCH_ERR` writer - The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
pub type SOS_UNMATCH_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `MARKER_ERR_FST_SCAN` writer - The clear interrupt bit to sign that the first scan has header marker error when decoding.
pub type MARKER_ERR_FST_SCAN_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `MARKER_ERR_OTHER_SCAN` writer - The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
pub type MARKER_ERR_OTHER_SCAN_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `UNDET` writer - The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
pub type UNDET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DECODE_TIMEOUT` writer - The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
pub type DECODE_TIMEOUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - This clear interrupt bit turns to high level when JPEG finishes encoding a picture..
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INT_CLR_SPEC> {
        DONE_W::new(self, 0)
    }
    ///Bit 1 - The clear interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    #[must_use]
    pub fn rle_parallel_err(&mut self) -> RLE_PARALLEL_ERR_W<INT_CLR_SPEC> {
        RLE_PARALLEL_ERR_W::new(self, 1)
    }
    ///Bit 2 - The clear interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    #[must_use]
    pub fn cid_err(&mut self) -> CID_ERR_W<INT_CLR_SPEC> {
        CID_ERR_W::new(self, 2)
    }
    ///Bit 3 - The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    #[must_use]
    pub fn c_dht_dc_id_err(&mut self) -> C_DHT_DC_ID_ERR_W<INT_CLR_SPEC> {
        C_DHT_DC_ID_ERR_W::new(self, 3)
    }
    ///Bit 4 - The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    #[must_use]
    pub fn c_dht_ac_id_err(&mut self) -> C_DHT_AC_ID_ERR_W<INT_CLR_SPEC> {
        C_DHT_AC_ID_ERR_W::new(self, 4)
    }
    ///Bit 5 - The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    #[must_use]
    pub fn c_dqt_id_err(&mut self) -> C_DQT_ID_ERR_W<INT_CLR_SPEC> {
        C_DQT_ID_ERR_W::new(self, 5)
    }
    ///Bit 6 - The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    #[must_use]
    pub fn rst_uxp_err(&mut self) -> RST_UXP_ERR_W<INT_CLR_SPEC> {
        RST_UXP_ERR_W::new(self, 6)
    }
    ///Bit 7 - The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    #[must_use]
    pub fn rst_check_none_err(&mut self) -> RST_CHECK_NONE_ERR_W<INT_CLR_SPEC> {
        RST_CHECK_NONE_ERR_W::new(self, 7)
    }
    ///Bit 8 - The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    #[must_use]
    pub fn rst_check_pos_err(&mut self) -> RST_CHECK_POS_ERR_W<INT_CLR_SPEC> {
        RST_CHECK_POS_ERR_W::new(self, 8)
    }
    ///Bit 9 - The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<INT_CLR_SPEC> {
        OUT_EOF_W::new(self, 9)
    }
    ///Bit 10 - The clear interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    #[must_use]
    pub fn sr_color_mode_err(&mut self) -> SR_COLOR_MODE_ERR_W<INT_CLR_SPEC> {
        SR_COLOR_MODE_ERR_W::new(self, 10)
    }
    ///Bit 11 - The clear interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    #[must_use]
    pub fn dct_done(&mut self) -> DCT_DONE_W<INT_CLR_SPEC> {
        DCT_DONE_W::new(self, 11)
    }
    ///Bit 12 - The clear interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    #[must_use]
    pub fn bs_last_block_eof(&mut self) -> BS_LAST_BLOCK_EOF_W<INT_CLR_SPEC> {
        BS_LAST_BLOCK_EOF_W::new(self, 12)
    }
    ///Bit 13 - The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    #[must_use]
    pub fn scan_check_none_err(&mut self) -> SCAN_CHECK_NONE_ERR_W<INT_CLR_SPEC> {
        SCAN_CHECK_NONE_ERR_W::new(self, 13)
    }
    ///Bit 14 - The clear interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    #[must_use]
    pub fn scan_check_pos_err(&mut self) -> SCAN_CHECK_POS_ERR_W<INT_CLR_SPEC> {
        SCAN_CHECK_POS_ERR_W::new(self, 14)
    }
    ///Bit 15 - The clear interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    #[must_use]
    pub fn uxp_det(&mut self) -> UXP_DET_W<INT_CLR_SPEC> {
        UXP_DET_W::new(self, 15)
    }
    ///Bit 16 - The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_err(&mut self) -> EN_FRAME_EOF_ERR_W<INT_CLR_SPEC> {
        EN_FRAME_EOF_ERR_W::new(self, 16)
    }
    ///Bit 17 - The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_lack(&mut self) -> EN_FRAME_EOF_LACK_W<INT_CLR_SPEC> {
        EN_FRAME_EOF_LACK_W::new(self, 17)
    }
    ///Bit 18 - The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_err(&mut self) -> DE_FRAME_EOF_ERR_W<INT_CLR_SPEC> {
        DE_FRAME_EOF_ERR_W::new(self, 18)
    }
    ///Bit 19 - The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_lack(&mut self) -> DE_FRAME_EOF_LACK_W<INT_CLR_SPEC> {
        DE_FRAME_EOF_LACK_W::new(self, 19)
    }
    ///Bit 20 - The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    #[must_use]
    pub fn sos_unmatch_err(&mut self) -> SOS_UNMATCH_ERR_W<INT_CLR_SPEC> {
        SOS_UNMATCH_ERR_W::new(self, 20)
    }
    ///Bit 21 - The clear interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    #[must_use]
    pub fn marker_err_fst_scan(&mut self) -> MARKER_ERR_FST_SCAN_W<INT_CLR_SPEC> {
        MARKER_ERR_FST_SCAN_W::new(self, 21)
    }
    ///Bit 22 - The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    #[must_use]
    pub fn marker_err_other_scan(&mut self) -> MARKER_ERR_OTHER_SCAN_W<INT_CLR_SPEC> {
        MARKER_ERR_OTHER_SCAN_W::new(self, 22)
    }
    ///Bit 23 - The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    #[must_use]
    pub fn undet(&mut self) -> UNDET_W<INT_CLR_SPEC> {
        UNDET_W::new(self, 23)
    }
    ///Bit 24 - The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    #[must_use]
    pub fn decode_timeout(&mut self) -> DECODE_TIMEOUT_W<INT_CLR_SPEC> {
        DECODE_TIMEOUT_W::new(self, 24)
    }
}
/**Interrupt clear registers

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01ff_ffff;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
