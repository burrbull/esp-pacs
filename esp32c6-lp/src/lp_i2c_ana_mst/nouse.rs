#[doc = "Register `NOUSE` reader"]
pub type R = crate::R<NOUSE_SPEC>;
#[doc = "Register `NOUSE` writer"]
pub type W = crate::W<NOUSE_SPEC>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_NOUSE` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C_MST_NOUSE_R = crate::FieldReader<u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_NOUSE` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C_MST_NOUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c_mst_nouse(&self) -> LP_I2C_ANA_MAST_I2C_MST_NOUSE_R {
        LP_I2C_ANA_MAST_I2C_MST_NOUSE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NOUSE")
            .field(
                "lp_i2c_ana_mast_i2c_mst_nouse",
                &self.lp_i2c_ana_mast_i2c_mst_nouse(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_mst_nouse(&mut self) -> LP_I2C_ANA_MAST_I2C_MST_NOUSE_W<NOUSE_SPEC> {
        LP_I2C_ANA_MAST_I2C_MST_NOUSE_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nouse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nouse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NOUSE_SPEC;
impl crate::RegisterSpec for NOUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nouse::R`](R) reader structure"]
impl crate::Readable for NOUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nouse::W`](W) writer structure"]
impl crate::Writable for NOUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NOUSE to value 0"]
impl crate::Resettable for NOUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
