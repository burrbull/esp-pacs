#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_3` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_3` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_2` reader - BackUp access spi_2 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_2` writer - BackUp access spi_2 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_3` reader - BackUp access spi_3 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SPI_3_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SPI_3` writer - BackUp access spi_3 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SPI_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL` reader - BackUp access apb_ctrl permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL` writer - BackUp access apb_ctrl permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1` reader - BackUp access i2c_ext1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1` writer - BackUp access i2c_ext1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST` reader - BackUp access sdio_host permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST` writer - BackUp access sdio_host permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CAN` reader - BackUp access can permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CAN_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_CAN` writer - BackUp access can permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_CAN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM1` reader - BackUp access pwm1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWM1` writer - BackUp access pwm1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWM1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S1` reader - BackUp access i2s1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2S1_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2S1` writer - BackUp access i2s1 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_I2S1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART2` reader - BackUp access uart2 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART2_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART2` writer - BackUp access uart2 permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_UART2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RWBT` reader - BackUp access rwbt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RWBT_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RWBT` writer - BackUp access rwbt permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RWBT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC` reader - BackUp access wifimac permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC` writer - BackUp access wifimac permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWR` reader - BackUp access pwr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWR_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_PWR` writer - BackUp access pwr permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_PWR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - BackUp access spi_2 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_spi_2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BackUp access spi_3 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_spi_3(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_3_R {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - BackUp access apb_ctrl permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_apb_ctrl(&self) -> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BackUp access i2c_ext1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c_ext1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - BackUp access sdio_host permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_sdio_host(&self) -> BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST_R {
        BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BackUp access can permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_can(&self) -> BACKUP_BUS_PMS_CONSTRAIN_CAN_R {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BackUp access pwm1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwm1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWM1_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWM1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BackUp access i2s1 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2s1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2S1_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - BackUp access uart2 permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART2_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BackUp access rwbt permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rwbt(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RWBT_R {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - BackUp access wifimac permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wifimac(&self) -> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BackUp access pwr permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_pwr(&self) -> BACKUP_BUS_PMS_CONSTRAIN_PWR_R {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_3")
            .field(
                "backup_bus_pms_constrain_spi_2",
                &self.backup_bus_pms_constrain_spi_2(),
            )
            .field(
                "backup_bus_pms_constrain_spi_3",
                &self.backup_bus_pms_constrain_spi_3(),
            )
            .field(
                "backup_bus_pms_constrain_apb_ctrl",
                &self.backup_bus_pms_constrain_apb_ctrl(),
            )
            .field(
                "backup_bus_pms_constrain_i2c_ext1",
                &self.backup_bus_pms_constrain_i2c_ext1(),
            )
            .field(
                "backup_bus_pms_constrain_sdio_host",
                &self.backup_bus_pms_constrain_sdio_host(),
            )
            .field(
                "backup_bus_pms_constrain_can",
                &self.backup_bus_pms_constrain_can(),
            )
            .field(
                "backup_bus_pms_constrain_pwm1",
                &self.backup_bus_pms_constrain_pwm1(),
            )
            .field(
                "backup_bus_pms_constrain_i2s1",
                &self.backup_bus_pms_constrain_i2s1(),
            )
            .field(
                "backup_bus_pms_constrain_uart2",
                &self.backup_bus_pms_constrain_uart2(),
            )
            .field(
                "backup_bus_pms_constrain_rwbt",
                &self.backup_bus_pms_constrain_rwbt(),
            )
            .field(
                "backup_bus_pms_constrain_wifimac",
                &self.backup_bus_pms_constrain_wifimac(),
            )
            .field(
                "backup_bus_pms_constrain_pwr",
                &self.backup_bus_pms_constrain_pwr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - BackUp access spi_2 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_spi_2(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_2_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - BackUp access spi_3 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_spi_3(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SPI_3_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SPI_3_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - BackUp access apb_ctrl permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_apb_ctrl(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - BackUp access i2c_ext1 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2c_ext1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT1_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - BackUp access sdio_host permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_sdio_host(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_SDIO_HOST_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - BackUp access can permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_can(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_CAN_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_CAN_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - BackUp access pwm1 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pwm1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_PWM1_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_PWM1_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - BackUp access i2s1 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_i2s1(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_I2S1_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_I2S1_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - BackUp access uart2 permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_uart2(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_UART2_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_UART2_W::new(self, 16)
    }
    #[doc = "Bits 22:23 - BackUp access rwbt permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rwbt(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RWBT_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_RWBT_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - BackUp access wifimac permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_wifimac(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - BackUp access pwr permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_pwr(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_PWR_W<BACKUP_BUS_PMS_CONSTRAIN_3_SPEC> {
        BACKUP_BUS_PMS_CONSTRAIN_PWR_W::new(self, 28)
    }
}
#[doc = "BackUp access peripherals permission configuration register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_3::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_3::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_3 to value 0x3cc3_ffff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: u32 = 0x3cc3_ffff;
}
