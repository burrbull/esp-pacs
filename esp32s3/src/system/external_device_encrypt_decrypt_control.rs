///Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` reader
pub type R = crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
///Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` writer
pub type W = crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
///Field `ENABLE_SPI_MANUAL_ENCRYPT` reader - Set 1 to enable the SPI manual encrypt.
pub type ENABLE_SPI_MANUAL_ENCRYPT_R = crate::BitReader;
///Field `ENABLE_SPI_MANUAL_ENCRYPT` writer - Set 1 to enable the SPI manual encrypt.
pub type ENABLE_SPI_MANUAL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_DOWNLOAD_DB_ENCRYPT` reader - Set 1 to enable download DB encrypt.
pub type ENABLE_DOWNLOAD_DB_ENCRYPT_R = crate::BitReader;
///Field `ENABLE_DOWNLOAD_DB_ENCRYPT` writer - Set 1 to enable download DB encrypt.
pub type ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` reader - Set 1 to enable download G0CB decrypt
pub type ENABLE_DOWNLOAD_G0CB_DECRYPT_R = crate::BitReader;
///Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` writer - Set 1 to enable download G0CB decrypt
pub type ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` reader - Set 1 to enable download manual encrypt
pub type ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
///Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` writer - Set 1 to enable download manual encrypt
pub type ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable the SPI manual encrypt.
    #[inline(always)]
    pub fn enable_spi_manual_encrypt(&self) -> ENABLE_SPI_MANUAL_ENCRYPT_R {
        ENABLE_SPI_MANUAL_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to enable download DB encrypt.
    #[inline(always)]
    pub fn enable_download_db_encrypt(&self) -> ENABLE_DOWNLOAD_DB_ENCRYPT_R {
        ENABLE_DOWNLOAD_DB_ENCRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set 1 to enable download G0CB decrypt
    #[inline(always)]
    pub fn enable_download_g0cb_decrypt(&self) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to enable download manual encrypt
    #[inline(always)]
    pub fn enable_download_manual_encrypt(&self) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL")
            .field("enable_spi_manual_encrypt", &self.enable_spi_manual_encrypt())
            .field("enable_download_db_encrypt", &self.enable_download_db_encrypt())
            .field("enable_download_g0cb_decrypt", &self.enable_download_g0cb_decrypt())
            .field(
                "enable_download_manual_encrypt",
                &self.enable_download_manual_encrypt(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable the SPI manual encrypt.
    #[inline(always)]
    #[must_use]
    pub fn enable_spi_manual_encrypt(
        &mut self,
    ) -> ENABLE_SPI_MANUAL_ENCRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_SPI_MANUAL_ENCRYPT_W::new(self, 0)
    }
    ///Bit 1 - Set 1 to enable download DB encrypt.
    #[inline(always)]
    #[must_use]
    pub fn enable_download_db_encrypt(
        &mut self,
    ) -> ENABLE_DOWNLOAD_DB_ENCRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_DOWNLOAD_DB_ENCRYPT_W::new(self, 1)
    }
    ///Bit 2 - Set 1 to enable download G0CB decrypt
    #[inline(always)]
    #[must_use]
    pub fn enable_download_g0cb_decrypt(
        &mut self,
    ) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_W::new(self, 2)
    }
    ///Bit 3 - Set 1 to enable download manual encrypt
    #[inline(always)]
    #[must_use]
    pub fn enable_download_manual_encrypt(
        &mut self,
    ) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W::new(self, 3)
    }
}
/**External memory encrypt and decrypt control register

You can [`read`](crate::generic::Reg::read) this register and get [`external_device_encrypt_decrypt_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_device_encrypt_decrypt_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC;
impl crate::RegisterSpec for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`external_device_encrypt_decrypt_control::R`](R) reader structure
impl crate::Readable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {}
///`write(|w| ..)` method takes [`external_device_encrypt_decrypt_control::W`](W) writer structure
impl crate::Writable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL to value 0
impl crate::Resettable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
