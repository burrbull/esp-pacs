///Register `AHBLITE_MPU_TABLE_UART` reader
pub type R = crate::R<AHBLITE_MPU_TABLE_UART_SPEC>;
///Register `AHBLITE_MPU_TABLE_UART` writer
pub type W = crate::W<AHBLITE_MPU_TABLE_UART_SPEC>;
///Field `UART_ACCESS_GRANT_CONFIG` reader -
pub type UART_ACCESS_GRANT_CONFIG_R = crate::FieldReader;
///Field `UART_ACCESS_GRANT_CONFIG` writer -
pub type UART_ACCESS_GRANT_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5
    #[inline(always)]
    pub fn uart_access_grant_config(&self) -> UART_ACCESS_GRANT_CONFIG_R {
        UART_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLITE_MPU_TABLE_UART")
            .field("uart_access_grant_config", &self.uart_access_grant_config())
            .finish()
    }
}
impl W {
    ///Bits 0:5
    #[inline(always)]
    #[must_use]
    pub fn uart_access_grant_config(
        &mut self,
    ) -> UART_ACCESS_GRANT_CONFIG_W<AHBLITE_MPU_TABLE_UART_SPEC> {
        UART_ACCESS_GRANT_CONFIG_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ahblite_mpu_table_uart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblite_mpu_table_uart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AHBLITE_MPU_TABLE_UART_SPEC;
impl crate::RegisterSpec for AHBLITE_MPU_TABLE_UART_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ahblite_mpu_table_uart::R`](R) reader structure
impl crate::Readable for AHBLITE_MPU_TABLE_UART_SPEC {}
///`write(|w| ..)` method takes [`ahblite_mpu_table_uart::W`](W) writer structure
impl crate::Writable for AHBLITE_MPU_TABLE_UART_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHBLITE_MPU_TABLE_UART to value 0
impl crate::Resettable for AHBLITE_MPU_TABLE_UART_SPEC {
    const RESET_VALUE: u32 = 0;
}
