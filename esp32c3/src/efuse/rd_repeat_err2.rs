///Register `RD_REPEAT_ERR2` reader
pub type R = crate::R<RD_REPEAT_ERR2_SPEC>;
///Field `KEY_PURPOSE_2_ERR` reader - If any bit in KEY_PURPOSE_2 is 1, then it indicates a programming error.
pub type KEY_PURPOSE_2_ERR_R = crate::FieldReader;
///Field `KEY_PURPOSE_3_ERR` reader - If any bit in KEY_PURPOSE_3 is 1, then it indicates a programming error.
pub type KEY_PURPOSE_3_ERR_R = crate::FieldReader;
///Field `KEY_PURPOSE_4_ERR` reader - If any bit in KEY_PURPOSE_4 is 1, then it indicates a programming error.
pub type KEY_PURPOSE_4_ERR_R = crate::FieldReader;
///Field `KEY_PURPOSE_5_ERR` reader - If any bit in KEY_PURPOSE_5 is 1, then it indicates a programming error.
pub type KEY_PURPOSE_5_ERR_R = crate::FieldReader;
///Field `RPT4_RESERVED3_ERR` reader - Reserved.
pub type RPT4_RESERVED3_ERR_R = crate::FieldReader;
///Field `SECURE_BOOT_EN_ERR` reader - If SECURE_BOOT_EN is 1, then it indicates a programming error.
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader;
///Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - If SECURE_BOOT_AGGRESSIVE_REVOKE is 1, then it indicates a programming error.
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::BitReader;
///Field `RPT4_RESERVED0_ERR` reader - Reserved.
pub type RPT4_RESERVED0_ERR_R = crate::FieldReader;
///Field `FLASH_TPUW_ERR` reader - If any bit in FLASH_TPUM is 1, then it indicates a programming error.
pub type FLASH_TPUW_ERR_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - If any bit in KEY_PURPOSE_2 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KEY_PURPOSE_2_ERR_R {
        KEY_PURPOSE_2_ERR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - If any bit in KEY_PURPOSE_3 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KEY_PURPOSE_3_ERR_R {
        KEY_PURPOSE_3_ERR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - If any bit in KEY_PURPOSE_4 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KEY_PURPOSE_4_ERR_R {
        KEY_PURPOSE_4_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - If any bit in KEY_PURPOSE_5 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KEY_PURPOSE_5_ERR_R {
        KEY_PURPOSE_5_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Reserved.
    #[inline(always)]
    pub fn rpt4_reserved3_err(&self) -> RPT4_RESERVED3_ERR_R {
        RPT4_RESERVED3_ERR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - If SECURE_BOOT_EN is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - If SECURE_BOOT_AGGRESSIVE_REVOKE is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(
        &self,
    ) -> SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:27 - Reserved.
    #[inline(always)]
    pub fn rpt4_reserved0_err(&self) -> RPT4_RESERVED0_ERR_R {
        RPT4_RESERVED0_ERR_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    ///Bits 28:31 - If any bit in FLASH_TPUM is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR2")
            .field("key_purpose_2_err", &self.key_purpose_2_err())
            .field("key_purpose_3_err", &self.key_purpose_3_err())
            .field("key_purpose_4_err", &self.key_purpose_4_err())
            .field("key_purpose_5_err", &self.key_purpose_5_err())
            .field("rpt4_reserved3_err", &self.rpt4_reserved3_err())
            .field("secure_boot_en_err", &self.secure_boot_en_err())
            .field(
                "secure_boot_aggressive_revoke_err",
                &self.secure_boot_aggressive_revoke_err(),
            )
            .field("rpt4_reserved0_err", &self.rpt4_reserved0_err())
            .field("flash_tpuw_err", &self.flash_tpuw_err())
            .finish()
    }
}
/**Programming error record register 2 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_REPEAT_ERR2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_repeat_err2::R`](R) reader structure
impl crate::Readable for RD_REPEAT_ERR2_SPEC {}
///`reset()` method sets RD_REPEAT_ERR2 to value 0
impl crate::Resettable for RD_REPEAT_ERR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
