#[doc = "Register `SCL_MAIN_ST_TIME_OUT` reader"]
pub type R = crate::R<SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "Register `SCL_MAIN_ST_TIME_OUT` writer"]
pub type W = crate::W<SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "Field `SCL_MAIN_ST_TO_I2C` reader - Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23. Measurement unit: i2c_sclk"]
pub type SCL_MAIN_ST_TO_I2C_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_ST_TO_I2C` writer - Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23. Measurement unit: i2c_sclk"]
pub type SCL_MAIN_ST_TO_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23. Measurement unit: i2c_sclk"]
    #[inline(always)]
    pub fn scl_main_st_to_i2c(&self) -> SCL_MAIN_ST_TO_I2C_R {
        SCL_MAIN_ST_TO_I2C_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_MAIN_ST_TIME_OUT")
            .field("scl_main_st_to_i2c", &self.scl_main_st_to_i2c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the threshold value of SCL_MAIN_FSM state unchanged period.nIt should be no more than 23. Measurement unit: i2c_sclk"]
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to_i2c(&mut self) -> SCL_MAIN_ST_TO_I2C_W<SCL_MAIN_ST_TIME_OUT_SPEC> {
        SCL_MAIN_ST_TO_I2C_W::new(self, 0)
    }
}
#[doc = "SCL main status time out register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_main_st_time_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_main_st_time_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_MAIN_ST_TIME_OUT_SPEC;
impl crate::RegisterSpec for SCL_MAIN_ST_TIME_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_main_st_time_out::R`](R) reader structure"]
impl crate::Readable for SCL_MAIN_ST_TIME_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_main_st_time_out::W`](W) writer structure"]
impl crate::Writable for SCL_MAIN_ST_TIME_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_MAIN_ST_TIME_OUT to value 0x10"]
impl crate::Resettable for SCL_MAIN_ST_TIME_OUT_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
