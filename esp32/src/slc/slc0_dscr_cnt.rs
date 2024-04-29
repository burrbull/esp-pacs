#[doc = "Register `SLC0_DSCR_CNT` reader"]
pub type R = crate::R<SLC0_DSCR_CNT_SPEC>;
#[doc = "Field `RX_DSCR_CNT_LAT` reader - "]
pub type RX_DSCR_CNT_LAT_R = crate::FieldReader<u16>;
#[doc = "Field `RX_GET_EOF_OCC` reader - "]
pub type RX_GET_EOF_OCC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_dscr_cnt_lat(&self) -> RX_DSCR_CNT_LAT_R {
        RX_DSCR_CNT_LAT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_get_eof_occ(&self) -> RX_GET_EOF_OCC_R {
        RX_GET_EOF_OCC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_DSCR_CNT")
            .field(
                "rx_dscr_cnt_lat",
                &format_args!("{}", self.rx_dscr_cnt_lat().bits()),
            )
            .field(
                "rx_get_eof_occ",
                &format_args!("{}", self.rx_get_eof_occ().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_DSCR_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_dscr_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_DSCR_CNT_SPEC;
impl crate::RegisterSpec for SLC0_DSCR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_dscr_cnt::R`](R) reader structure"]
impl crate::Readable for SLC0_DSCR_CNT_SPEC {}
#[doc = "`reset()` method sets SLC0_DSCR_CNT to value 0"]
impl crate::Resettable for SLC0_DSCR_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
