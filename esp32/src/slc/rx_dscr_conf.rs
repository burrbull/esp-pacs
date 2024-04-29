#[doc = "Register `RX_DSCR_CONF` reader"]
pub type R = crate::R<RX_DSCR_CONF_SPEC>;
#[doc = "Register `RX_DSCR_CONF` writer"]
pub type W = crate::W<RX_DSCR_CONF_SPEC>;
#[doc = "Field `SLC_TOKEN_NO_REPLACE(0-1)` reader - "]
pub type SLC_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN_NO_REPLACE(0-1)` writer - "]
pub type SLC_TOKEN_NO_REPLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_INFOR_NO_REPLACE(0-1)` reader - "]
pub type SLC_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC_INFOR_NO_REPLACE(0-1)` writer - "]
pub type SLC_INFOR_NO_REPLACE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_FILL_MODE(0-1)` reader - "]
pub type SLC_RX_FILL_MODE_R = crate::BitReader;
#[doc = "Field `SLC_RX_FILL_MODE(0-1)` writer - "]
pub type SLC_RX_FILL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_EOF_MODE(0-1)` reader - "]
pub type SLC_RX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `SLC_RX_EOF_MODE(0-1)` writer - "]
pub type SLC_RX_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RX_FILL_EN(0-1)` reader - "]
pub type SLC_RX_FILL_EN_R = crate::BitReader;
#[doc = "Field `SLC_RX_FILL_EN(0-1)` writer - "]
pub type SLC_RX_FILL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_RD_RETRY_THRESHOLD(0-1)` reader - "]
pub type SLC_RD_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `SLC_RD_RETRY_THRESHOLD(0-1)` writer - "]
pub type SLC_RD_RETRY_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOKEN_NO_REPLACE` field"]
    #[inline(always)]
    pub fn slc_token_no_replace(&self, n: u8) -> SLC_TOKEN_NO_REPLACE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOKEN_NO_REPLACE_R::new(((self.bits >> (n * 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_token_no_replace_iter(&self) -> impl Iterator<Item = SLC_TOKEN_NO_REPLACE_R> + '_ {
        (0..2).map(move |n| SLC_TOKEN_NO_REPLACE_R::new(((self.bits >> (n * 16)) & 1) != 0))
    }
    #[doc = "Bit 0 - SLC0_TOKEN_NO_REPLACE"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&self) -> SLC_TOKEN_NO_REPLACE_R {
        SLC_TOKEN_NO_REPLACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - SLC1_TOKEN_NO_REPLACE"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&self) -> SLC_TOKEN_NO_REPLACE_R {
        SLC_TOKEN_NO_REPLACE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_INFOR_NO_REPLACE` field"]
    #[inline(always)]
    pub fn slc_infor_no_replace(&self, n: u8) -> SLC_INFOR_NO_REPLACE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_INFOR_NO_REPLACE_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_infor_no_replace_iter(&self) -> impl Iterator<Item = SLC_INFOR_NO_REPLACE_R> + '_ {
        (0..2).map(move |n| SLC_INFOR_NO_REPLACE_R::new(((self.bits >> (n * 16 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - SLC0_INFOR_NO_REPLACE"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&self) -> SLC_INFOR_NO_REPLACE_R {
        SLC_INFOR_NO_REPLACE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - SLC1_INFOR_NO_REPLACE"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&self) -> SLC_INFOR_NO_REPLACE_R {
        SLC_INFOR_NO_REPLACE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_FILL_MODE` field"]
    #[inline(always)]
    pub fn slc_rx_fill_mode(&self, n: u8) -> SLC_RX_FILL_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_FILL_MODE_R::new(((self.bits >> (n * 16 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_fill_mode_iter(&self) -> impl Iterator<Item = SLC_RX_FILL_MODE_R> + '_ {
        (0..2).map(move |n| SLC_RX_FILL_MODE_R::new(((self.bits >> (n * 16 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - SLC0_RX_FILL_MODE"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&self) -> SLC_RX_FILL_MODE_R {
        SLC_RX_FILL_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - SLC1_RX_FILL_MODE"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&self) -> SLC_RX_FILL_MODE_R {
        SLC_RX_FILL_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_EOF_MODE` field"]
    #[inline(always)]
    pub fn slc_rx_eof_mode(&self, n: u8) -> SLC_RX_EOF_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_EOF_MODE_R::new(((self.bits >> (n * 16 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_eof_mode_iter(&self) -> impl Iterator<Item = SLC_RX_EOF_MODE_R> + '_ {
        (0..2).map(move |n| SLC_RX_EOF_MODE_R::new(((self.bits >> (n * 16 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - SLC0_RX_EOF_MODE"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&self) -> SLC_RX_EOF_MODE_R {
        SLC_RX_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 19 - SLC1_RX_EOF_MODE"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&self) -> SLC_RX_EOF_MODE_R {
        SLC_RX_EOF_MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_FILL_EN` field"]
    #[inline(always)]
    pub fn slc_rx_fill_en(&self, n: u8) -> SLC_RX_FILL_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_FILL_EN_R::new(((self.bits >> (n * 16 + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rx_fill_en_iter(&self) -> impl Iterator<Item = SLC_RX_FILL_EN_R> + '_ {
        (0..2).map(move |n| SLC_RX_FILL_EN_R::new(((self.bits >> (n * 16 + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - SLC0_RX_FILL_EN"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&self) -> SLC_RX_FILL_EN_R {
        SLC_RX_FILL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 20 - SLC1_RX_FILL_EN"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&self) -> SLC_RX_FILL_EN_R {
        SLC_RX_FILL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RD_RETRY_THRESHOLD` field"]
    #[inline(always)]
    pub fn slc_rd_retry_threshold(&self, n: u8) -> SLC_RD_RETRY_THRESHOLD_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RD_RETRY_THRESHOLD_R::new(((self.bits >> (n * 16 + 5)) & 0x07ff) as u16)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_rd_retry_threshold_iter(
        &self,
    ) -> impl Iterator<Item = SLC_RD_RETRY_THRESHOLD_R> + '_ {
        (0..2).map(move |n| {
            SLC_RD_RETRY_THRESHOLD_R::new(((self.bits >> (n * 16 + 5)) & 0x07ff) as u16)
        })
    }
    #[doc = "Bits 5:15 - SLC0_RD_RETRY_THRESHOLD"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&self) -> SLC_RD_RETRY_THRESHOLD_R {
        SLC_RD_RETRY_THRESHOLD_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 21:31 - SLC1_RD_RETRY_THRESHOLD"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&self) -> SLC_RD_RETRY_THRESHOLD_R {
        SLC_RD_RETRY_THRESHOLD_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_DSCR_CONF")
            .field(
                "slc0_token_no_replace",
                &format_args!("{}", self.slc0_token_no_replace().bit()),
            )
            .field(
                "slc1_token_no_replace",
                &format_args!("{}", self.slc1_token_no_replace().bit()),
            )
            .field(
                "slc0_infor_no_replace",
                &format_args!("{}", self.slc0_infor_no_replace().bit()),
            )
            .field(
                "slc1_infor_no_replace",
                &format_args!("{}", self.slc1_infor_no_replace().bit()),
            )
            .field(
                "slc0_rx_fill_mode",
                &format_args!("{}", self.slc0_rx_fill_mode().bit()),
            )
            .field(
                "slc1_rx_fill_mode",
                &format_args!("{}", self.slc1_rx_fill_mode().bit()),
            )
            .field(
                "slc0_rx_eof_mode",
                &format_args!("{}", self.slc0_rx_eof_mode().bit()),
            )
            .field(
                "slc1_rx_eof_mode",
                &format_args!("{}", self.slc1_rx_eof_mode().bit()),
            )
            .field(
                "slc0_rx_fill_en",
                &format_args!("{}", self.slc0_rx_fill_en().bit()),
            )
            .field(
                "slc1_rx_fill_en",
                &format_args!("{}", self.slc1_rx_fill_en().bit()),
            )
            .field(
                "slc0_rd_retry_threshold",
                &format_args!("{}", self.slc0_rd_retry_threshold().bits()),
            )
            .field(
                "slc1_rd_retry_threshold",
                &format_args!("{}", self.slc1_rd_retry_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_DSCR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOKEN_NO_REPLACE` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token_no_replace(&mut self, n: u8) -> SLC_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOKEN_NO_REPLACE_W::new(self, n * 16)
    }
    #[doc = "Bit 0 - SLC0_TOKEN_NO_REPLACE"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_token_no_replace(&mut self) -> SLC_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC_TOKEN_NO_REPLACE_W::new(self, 0)
    }
    #[doc = "Bit 16 - SLC1_TOKEN_NO_REPLACE"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_token_no_replace(&mut self) -> SLC_TOKEN_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC_TOKEN_NO_REPLACE_W::new(self, 16)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_INFOR_NO_REPLACE` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_infor_no_replace(&mut self, n: u8) -> SLC_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_INFOR_NO_REPLACE_W::new(self, n * 16 + 1)
    }
    #[doc = "Bit 1 - SLC0_INFOR_NO_REPLACE"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_infor_no_replace(&mut self) -> SLC_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC_INFOR_NO_REPLACE_W::new(self, 1)
    }
    #[doc = "Bit 17 - SLC1_INFOR_NO_REPLACE"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_infor_no_replace(&mut self) -> SLC_INFOR_NO_REPLACE_W<RX_DSCR_CONF_SPEC> {
        SLC_INFOR_NO_REPLACE_W::new(self, 17)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_FILL_MODE` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_fill_mode(&mut self, n: u8) -> SLC_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_FILL_MODE_W::new(self, n * 16 + 2)
    }
    #[doc = "Bit 2 - SLC0_RX_FILL_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_fill_mode(&mut self) -> SLC_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC_RX_FILL_MODE_W::new(self, 2)
    }
    #[doc = "Bit 18 - SLC1_RX_FILL_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_fill_mode(&mut self) -> SLC_RX_FILL_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC_RX_FILL_MODE_W::new(self, 18)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_EOF_MODE` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_eof_mode(&mut self, n: u8) -> SLC_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_EOF_MODE_W::new(self, n * 16 + 3)
    }
    #[doc = "Bit 3 - SLC0_RX_EOF_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_eof_mode(&mut self) -> SLC_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC_RX_EOF_MODE_W::new(self, 3)
    }
    #[doc = "Bit 19 - SLC1_RX_EOF_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_eof_mode(&mut self) -> SLC_RX_EOF_MODE_W<RX_DSCR_CONF_SPEC> {
        SLC_RX_EOF_MODE_W::new(self, 19)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RX_FILL_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rx_fill_en(&mut self, n: u8) -> SLC_RX_FILL_EN_W<RX_DSCR_CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RX_FILL_EN_W::new(self, n * 16 + 4)
    }
    #[doc = "Bit 4 - SLC0_RX_FILL_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_fill_en(&mut self) -> SLC_RX_FILL_EN_W<RX_DSCR_CONF_SPEC> {
        SLC_RX_FILL_EN_W::new(self, 4)
    }
    #[doc = "Bit 20 - SLC1_RX_FILL_EN"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rx_fill_en(&mut self) -> SLC_RX_FILL_EN_W<RX_DSCR_CONF_SPEC> {
        SLC_RX_FILL_EN_W::new(self, 20)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_RD_RETRY_THRESHOLD` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_rd_retry_threshold(&mut self, n: u8) -> SLC_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_RD_RETRY_THRESHOLD_W::new(self, n * 16 + 5)
    }
    #[doc = "Bits 5:15 - SLC0_RD_RETRY_THRESHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rd_retry_threshold(&mut self) -> SLC_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC> {
        SLC_RD_RETRY_THRESHOLD_W::new(self, 5)
    }
    #[doc = "Bits 21:31 - SLC1_RD_RETRY_THRESHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rd_retry_threshold(&mut self) -> SLC_RD_RETRY_THRESHOLD_W<RX_DSCR_CONF_SPEC> {
        SLC_RD_RETRY_THRESHOLD_W::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_dscr_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_dscr_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_dscr_conf::R`](R) reader structure"]
impl crate::Readable for RX_DSCR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_dscr_conf::W`](W) writer structure"]
impl crate::Writable for RX_DSCR_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_DSCR_CONF to value 0x101b_101a"]
impl crate::Resettable for RX_DSCR_CONF_SPEC {
    const RESET_VALUE: u32 = 0x101b_101a;
}
