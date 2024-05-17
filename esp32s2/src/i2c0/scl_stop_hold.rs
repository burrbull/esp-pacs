///Register `SCL_STOP_HOLD` reader
pub type R = crate::R<SCL_STOP_HOLD_SPEC>;
///Register `SCL_STOP_HOLD` writer
pub type W = crate::W<SCL_STOP_HOLD_SPEC>;
///Field `TIME` reader - This register is used to configure the delay after the STOP condition, in I2C module clock cycles.
pub type TIME_R = crate::FieldReader<u16>;
///Field `TIME` writer - This register is used to configure the delay after the STOP condition, in I2C module clock cycles.
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - This register is used to configure the delay after the STOP condition, in I2C module clock cycles.
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STOP_HOLD").field("time", &self.time()).finish()
    }
}
impl W {
    ///Bits 0:13 - This register is used to configure the delay after the STOP condition, in I2C module clock cycles.
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SCL_STOP_HOLD_SPEC> {
        TIME_W::new(self, 0)
    }
}
/**Configures the delay after the SCL clock edge for a stop condition

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCL_STOP_HOLD_SPEC;
impl crate::RegisterSpec for SCL_STOP_HOLD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scl_stop_hold::R`](R) reader structure
impl crate::Readable for SCL_STOP_HOLD_SPEC {}
///`write(|w| ..)` method takes [`scl_stop_hold::W`](W) writer structure
impl crate::Writable for SCL_STOP_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCL_STOP_HOLD to value 0
impl crate::Resettable for SCL_STOP_HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
