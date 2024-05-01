///Register `SCL_LOW_PERIOD` reader
pub type R = crate::R<SCL_LOW_PERIOD_SPEC>;
///Register `SCL_LOW_PERIOD` writer
pub type W = crate::W<SCL_LOW_PERIOD_SPEC>;
///Field `SCL_LOW_PERIOD` reader - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles.
pub type SCL_LOW_PERIOD_R = crate::FieldReader<u16>;
///Field `SCL_LOW_PERIOD` writer - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles.
pub type SCL_LOW_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles.
    #[inline(always)]
    pub fn scl_low_period(&self) -> SCL_LOW_PERIOD_R {
        SCL_LOW_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_LOW_PERIOD")
            .field("scl_low_period", &self.scl_low_period())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles.
    #[inline(always)]
    #[must_use]
    pub fn scl_low_period(&mut self) -> SCL_LOW_PERIOD_W<SCL_LOW_PERIOD_SPEC> {
        SCL_LOW_PERIOD_W::new(self, 0)
    }
}
/**Configures the low level width of the SCL Clock

You can [`read`](crate::generic::Reg::read) this register and get [`scl_low_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCL_LOW_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_LOW_PERIOD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scl_low_period::R`](R) reader structure
impl crate::Readable for SCL_LOW_PERIOD_SPEC {}
///`write(|w| ..)` method takes [`scl_low_period::W`](W) writer structure
impl crate::Writable for SCL_LOW_PERIOD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCL_LOW_PERIOD to value 0
impl crate::Resettable for SCL_LOW_PERIOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
