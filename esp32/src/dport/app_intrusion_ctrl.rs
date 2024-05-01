///Register `APP_INTRUSION_CTRL` reader
pub type R = crate::R<APP_INTRUSION_CTRL_SPEC>;
///Register `APP_INTRUSION_CTRL` writer
pub type W = crate::W<APP_INTRUSION_CTRL_SPEC>;
///Field `APP_INTRUSION_RECORD_RESET_N` reader -
pub type APP_INTRUSION_RECORD_RESET_N_R = crate::BitReader;
///Field `APP_INTRUSION_RECORD_RESET_N` writer -
pub type APP_INTRUSION_RECORD_RESET_N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn app_intrusion_record_reset_n(&self) -> APP_INTRUSION_RECORD_RESET_N_R {
        APP_INTRUSION_RECORD_RESET_N_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_INTRUSION_CTRL")
            .field(
                "app_intrusion_record_reset_n",
                &self.app_intrusion_record_reset_n(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn app_intrusion_record_reset_n(
        &mut self,
    ) -> APP_INTRUSION_RECORD_RESET_N_W<APP_INTRUSION_CTRL_SPEC> {
        APP_INTRUSION_RECORD_RESET_N_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_intrusion_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_intrusion_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_INTRUSION_CTRL_SPEC;
impl crate::RegisterSpec for APP_INTRUSION_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_intrusion_ctrl::R`](R) reader structure
impl crate::Readable for APP_INTRUSION_CTRL_SPEC {}
///`write(|w| ..)` method takes [`app_intrusion_ctrl::W`](W) writer structure
impl crate::Writable for APP_INTRUSION_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APP_INTRUSION_CTRL to value 0x01
impl crate::Resettable for APP_INTRUSION_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
