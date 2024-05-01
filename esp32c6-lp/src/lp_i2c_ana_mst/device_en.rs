///Register `DEVICE_EN` reader
pub type R = crate::R<DEVICE_EN_SPEC>;
///Register `DEVICE_EN` writer
pub type W = crate::W<DEVICE_EN_SPEC>;
///Field `LP_I2C_ANA_MAST_I2C_DEVICE_EN` reader - need_des
pub type LP_I2C_ANA_MAST_I2C_DEVICE_EN_R = crate::FieldReader<u16>;
///Field `LP_I2C_ANA_MAST_I2C_DEVICE_EN` writer - need_des
pub type LP_I2C_ANA_MAST_I2C_DEVICE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - need_des
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c_device_en(&self) -> LP_I2C_ANA_MAST_I2C_DEVICE_EN_R {
        LP_I2C_ANA_MAST_I2C_DEVICE_EN_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_EN")
            .field(
                "lp_i2c_ana_mast_i2c_device_en",
                &self.lp_i2c_ana_mast_i2c_device_en(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:11 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_device_en(
        &mut self,
    ) -> LP_I2C_ANA_MAST_I2C_DEVICE_EN_W<DEVICE_EN_SPEC> {
        LP_I2C_ANA_MAST_I2C_DEVICE_EN_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`device_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEVICE_EN_SPEC;
impl crate::RegisterSpec for DEVICE_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`device_en::R`](R) reader structure
impl crate::Readable for DEVICE_EN_SPEC {}
///`write(|w| ..)` method takes [`device_en::W`](W) writer structure
impl crate::Writable for DEVICE_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEVICE_EN to value 0
impl crate::Resettable for DEVICE_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
