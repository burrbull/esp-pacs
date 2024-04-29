#[doc = "Register `SDIO_CRC_ST0` reader"]
pub type R = crate::R<SDIO_CRC_ST0_SPEC>;
#[doc = "Field `DAT_CRC_ERR_CNT(0-3)` reader - "]
pub type DAT_CRC_ERR_CNT_R = crate::FieldReader;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DAT0_CRC_ERR_CNT` field"]
    #[inline(always)]
    pub fn dat_crc_err_cnt(&self, n: u8) -> DAT_CRC_ERR_CNT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DAT_CRC_ERR_CNT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn dat_crc_err_cnt_iter(&self) -> impl Iterator<Item = DAT_CRC_ERR_CNT_R> + '_ {
        (0..4).map(move |n| DAT_CRC_ERR_CNT_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - DAT0_CRC_ERR_CNT"]
    #[inline(always)]
    pub fn dat0_crc_err_cnt(&self) -> DAT_CRC_ERR_CNT_R {
        DAT_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAT1_CRC_ERR_CNT"]
    #[inline(always)]
    pub fn dat1_crc_err_cnt(&self) -> DAT_CRC_ERR_CNT_R {
        DAT_CRC_ERR_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DAT2_CRC_ERR_CNT"]
    #[inline(always)]
    pub fn dat2_crc_err_cnt(&self) -> DAT_CRC_ERR_CNT_R {
        DAT_CRC_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DAT3_CRC_ERR_CNT"]
    #[inline(always)]
    pub fn dat3_crc_err_cnt(&self) -> DAT_CRC_ERR_CNT_R {
        DAT_CRC_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CRC_ST0")
            .field(
                "dat0_crc_err_cnt",
                &format_args!("{}", self.dat0_crc_err_cnt().bits()),
            )
            .field(
                "dat1_crc_err_cnt",
                &format_args!("{}", self.dat1_crc_err_cnt().bits()),
            )
            .field(
                "dat2_crc_err_cnt",
                &format_args!("{}", self.dat2_crc_err_cnt().bits()),
            )
            .field(
                "dat3_crc_err_cnt",
                &format_args!("{}", self.dat3_crc_err_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_CRC_ST0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_CRC_ST0_SPEC;
impl crate::RegisterSpec for SDIO_CRC_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_crc_st0::R`](R) reader structure"]
impl crate::Readable for SDIO_CRC_ST0_SPEC {}
#[doc = "`reset()` method sets SDIO_CRC_ST0 to value 0"]
impl crate::Resettable for SDIO_CRC_ST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
