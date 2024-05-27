///Register `SDA_HOLD` reader
pub type R = crate::R<SDA_HOLD_SPEC>;
///Register `SDA_HOLD` writer
pub type W = crate::W<SDA_HOLD_SPEC>;
///Field `TIME` reader - reg_sda_hold_time
pub type TIME_R = crate::FieldReader<u16>;
///Field `TIME` writer - reg_sda_hold_time
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - reg_sda_hold_time
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_HOLD")
            .field("time", &self.time())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - reg_sda_hold_time
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SDA_HOLD_SPEC> {
        TIME_W::new(self, 0)
    }
}
/**I2C_SDA_HOLD_REG

You can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDA_HOLD_SPEC;
impl crate::RegisterSpec for SDA_HOLD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sda_hold::R`](R) reader structure
impl crate::Readable for SDA_HOLD_SPEC {}
///`write(|w| ..)` method takes [`sda_hold::W`](W) writer structure
impl crate::Writable for SDA_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDA_HOLD to value 0
impl crate::Resettable for SDA_HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
