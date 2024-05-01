///Register `SHROM_MPU_TABLE6` reader
pub type R = crate::R<SHROM_MPU_TABLE6_SPEC>;
///Register `SHROM_MPU_TABLE6` writer
pub type W = crate::W<SHROM_MPU_TABLE6_SPEC>;
///Field `SHROM_MPU_TABLE6` reader -
pub type SHROM_MPU_TABLE6_R = crate::FieldReader;
///Field `SHROM_MPU_TABLE6` writer -
pub type SHROM_MPU_TABLE6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn shrom_mpu_table6(&self) -> SHROM_MPU_TABLE6_R {
        SHROM_MPU_TABLE6_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHROM_MPU_TABLE6")
            .field("shrom_mpu_table6", &self.shrom_mpu_table6())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn shrom_mpu_table6(&mut self) -> SHROM_MPU_TABLE6_W<SHROM_MPU_TABLE6_SPEC> {
        SHROM_MPU_TABLE6_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`shrom_mpu_table6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrom_mpu_table6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHROM_MPU_TABLE6_SPEC;
impl crate::RegisterSpec for SHROM_MPU_TABLE6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`shrom_mpu_table6::R`](R) reader structure
impl crate::Readable for SHROM_MPU_TABLE6_SPEC {}
///`write(|w| ..)` method takes [`shrom_mpu_table6::W`](W) writer structure
impl crate::Writable for SHROM_MPU_TABLE6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SHROM_MPU_TABLE6 to value 0x01
impl crate::Resettable for SHROM_MPU_TABLE6_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
