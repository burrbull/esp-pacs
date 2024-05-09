#[doc = "Register `BLK0_BACKUP1_W5` reader"]
pub type R = crate::R<BLK0_BACKUP1_W5_SPEC>;
#[doc = "Field `OTP_BEBUG_BLOCK0_BACKUP1_W5` reader - Otp block0 backup1 word5 data."]
pub type OTP_BEBUG_BLOCK0_BACKUP1_W5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup1 word5 data."]
    #[inline(always)]
    pub fn otp_bebug_block0_backup1_w5(&self) -> OTP_BEBUG_BLOCK0_BACKUP1_W5_R {
        OTP_BEBUG_BLOCK0_BACKUP1_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_BACKUP1_W5")
            .field(
                "otp_bebug_block0_backup1_w5",
                &self.otp_bebug_block0_backup1_w5().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_BACKUP1_W5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Otp debuger block0 data register6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_backup1_w5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_BACKUP1_W5_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP1_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_backup1_w5::R`](R) reader structure"]
impl crate::Readable for BLK0_BACKUP1_W5_SPEC {}
#[doc = "`reset()` method sets BLK0_BACKUP1_W5 to value 0"]
impl crate::Resettable for BLK0_BACKUP1_W5_SPEC {}
