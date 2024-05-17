///Register `SDA_HOLD_TIME` reader
pub type R = crate::R<SDA_HOLD_TIME_SPEC>;
///Register `SDA_HOLD_TIME` writer
pub type W = crate::W<SDA_HOLD_TIME_SPEC>;
///Field `REG_SDA_OD_TX_HOLD_TIME` reader - It is used to adjust sda drive point after scl neg under open drain speed
pub type REG_SDA_OD_TX_HOLD_TIME_R = crate::FieldReader<u16>;
///Field `REG_SDA_OD_TX_HOLD_TIME` writer - It is used to adjust sda drive point after scl neg under open drain speed
pub type REG_SDA_OD_TX_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `REG_SDA_PP_TX_HOLD_TIME` reader - It is used to adjust sda dirve point after scl neg under push pull speed
pub type REG_SDA_PP_TX_HOLD_TIME_R = crate::FieldReader;
///Field `REG_SDA_PP_TX_HOLD_TIME` writer - It is used to adjust sda dirve point after scl neg under push pull speed
pub type REG_SDA_PP_TX_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:8 - It is used to adjust sda drive point after scl neg under open drain speed
    #[inline(always)]
    pub fn reg_sda_od_tx_hold_time(&self) -> REG_SDA_OD_TX_HOLD_TIME_R {
        REG_SDA_OD_TX_HOLD_TIME_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:13 - It is used to adjust sda dirve point after scl neg under push pull speed
    #[inline(always)]
    pub fn reg_sda_pp_tx_hold_time(&self) -> REG_SDA_PP_TX_HOLD_TIME_R {
        REG_SDA_PP_TX_HOLD_TIME_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_HOLD_TIME")
            .field("reg_sda_od_tx_hold_time", &self.reg_sda_od_tx_hold_time())
            .field("reg_sda_pp_tx_hold_time", &self.reg_sda_pp_tx_hold_time())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - It is used to adjust sda drive point after scl neg under open drain speed
    #[inline(always)]
    #[must_use]
    pub fn reg_sda_od_tx_hold_time(
        &mut self,
    ) -> REG_SDA_OD_TX_HOLD_TIME_W<SDA_HOLD_TIME_SPEC> {
        REG_SDA_OD_TX_HOLD_TIME_W::new(self, 0)
    }
    ///Bits 9:13 - It is used to adjust sda dirve point after scl neg under push pull speed
    #[inline(always)]
    #[must_use]
    pub fn reg_sda_pp_tx_hold_time(
        &mut self,
    ) -> REG_SDA_PP_TX_HOLD_TIME_W<SDA_HOLD_TIME_SPEC> {
        REG_SDA_PP_TX_HOLD_TIME_W::new(self, 9)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`sda_hold_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDA_HOLD_TIME_SPEC;
impl crate::RegisterSpec for SDA_HOLD_TIME_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sda_hold_time::R`](R) reader structure
impl crate::Readable for SDA_HOLD_TIME_SPEC {}
///`write(|w| ..)` method takes [`sda_hold_time::W`](W) writer structure
impl crate::Writable for SDA_HOLD_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDA_HOLD_TIME to value 0x01
impl crate::Resettable for SDA_HOLD_TIME_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
