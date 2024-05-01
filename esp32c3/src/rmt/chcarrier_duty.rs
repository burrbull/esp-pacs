///Register `CH%sCARRIER_DUTY` reader
pub type R = crate::R<CHCARRIER_DUTY_SPEC>;
///Register `CH%sCARRIER_DUTY` writer
pub type W = crate::W<CHCARRIER_DUTY_SPEC>;
///Field `CARRIER_LOW` reader - reg_carrier_low_ch0.
pub type CARRIER_LOW_R = crate::FieldReader<u16>;
///Field `CARRIER_LOW` writer - reg_carrier_low_ch0.
pub type CARRIER_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CARRIER_HIGH` reader - reg_carrier_high_ch0.
pub type CARRIER_HIGH_R = crate::FieldReader<u16>;
///Field `CARRIER_HIGH` writer - reg_carrier_high_ch0.
pub type CARRIER_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - reg_carrier_low_ch0.
    #[inline(always)]
    pub fn carrier_low(&self) -> CARRIER_LOW_R {
        CARRIER_LOW_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - reg_carrier_high_ch0.
    #[inline(always)]
    pub fn carrier_high(&self) -> CARRIER_HIGH_R {
        CARRIER_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCARRIER_DUTY")
            .field("carrier_low", &self.carrier_low())
            .field("carrier_high", &self.carrier_high())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - reg_carrier_low_ch0.
    #[inline(always)]
    #[must_use]
    pub fn carrier_low(&mut self) -> CARRIER_LOW_W<CHCARRIER_DUTY_SPEC> {
        CARRIER_LOW_W::new(self, 0)
    }
    ///Bits 16:31 - reg_carrier_high_ch0.
    #[inline(always)]
    #[must_use]
    pub fn carrier_high(&mut self) -> CARRIER_HIGH_W<CHCARRIER_DUTY_SPEC> {
        CARRIER_HIGH_W::new(self, 16)
    }
}
/**RMT_CH%sCARRIER_DUTY_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CHCARRIER_DUTY_SPEC;
impl crate::RegisterSpec for CHCARRIER_DUTY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`chcarrier_duty::R`](R) reader structure
impl crate::Readable for CHCARRIER_DUTY_SPEC {}
///`write(|w| ..)` method takes [`chcarrier_duty::W`](W) writer structure
impl crate::Writable for CHCARRIER_DUTY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH%sCARRIER_DUTY to value 0x0040_0040
impl crate::Resettable for CHCARRIER_DUTY_SPEC {
    const RESET_VALUE: u32 = 0x0040_0040;
}
