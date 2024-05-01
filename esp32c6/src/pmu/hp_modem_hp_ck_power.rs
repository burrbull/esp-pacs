///Register `HP_MODEM_HP_CK_POWER` reader
pub type R = crate::R<HP_MODEM_HP_CK_POWER_SPEC>;
///Register `HP_MODEM_HP_CK_POWER` writer
pub type W = crate::W<HP_MODEM_HP_CK_POWER_SPEC>;
///Field `HP_MODEM_I2C_ISO_EN` reader - need_des
pub type HP_MODEM_I2C_ISO_EN_R = crate::BitReader;
///Field `HP_MODEM_I2C_ISO_EN` writer - need_des
pub type HP_MODEM_I2C_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_MODEM_I2C_RETENTION` reader - need_des
pub type HP_MODEM_I2C_RETENTION_R = crate::BitReader;
///Field `HP_MODEM_I2C_RETENTION` writer - need_des
pub type HP_MODEM_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_MODEM_XPD_BB_I2C` reader - need_des
pub type HP_MODEM_XPD_BB_I2C_R = crate::BitReader;
///Field `HP_MODEM_XPD_BB_I2C` writer - need_des
pub type HP_MODEM_XPD_BB_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_MODEM_XPD_BBPLL_I2C` reader - need_des
pub type HP_MODEM_XPD_BBPLL_I2C_R = crate::BitReader;
///Field `HP_MODEM_XPD_BBPLL_I2C` writer - need_des
pub type HP_MODEM_XPD_BBPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_MODEM_XPD_BBPLL` reader - need_des
pub type HP_MODEM_XPD_BBPLL_R = crate::BitReader;
///Field `HP_MODEM_XPD_BBPLL` writer - need_des
pub type HP_MODEM_XPD_BBPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 26 - need_des
    #[inline(always)]
    pub fn hp_modem_i2c_iso_en(&self) -> HP_MODEM_I2C_ISO_EN_R {
        HP_MODEM_I2C_ISO_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn hp_modem_i2c_retention(&self) -> HP_MODEM_I2C_RETENTION_R {
        HP_MODEM_I2C_RETENTION_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn hp_modem_xpd_bb_i2c(&self) -> HP_MODEM_XPD_BB_I2C_R {
        HP_MODEM_XPD_BB_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn hp_modem_xpd_bbpll_i2c(&self) -> HP_MODEM_XPD_BBPLL_I2C_R {
        HP_MODEM_XPD_BBPLL_I2C_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn hp_modem_xpd_bbpll(&self) -> HP_MODEM_XPD_BBPLL_R {
        HP_MODEM_XPD_BBPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_HP_CK_POWER")
            .field("hp_modem_i2c_iso_en", &self.hp_modem_i2c_iso_en())
            .field("hp_modem_i2c_retention", &self.hp_modem_i2c_retention())
            .field("hp_modem_xpd_bb_i2c", &self.hp_modem_xpd_bb_i2c())
            .field("hp_modem_xpd_bbpll_i2c", &self.hp_modem_xpd_bbpll_i2c())
            .field("hp_modem_xpd_bbpll", &self.hp_modem_xpd_bbpll())
            .finish()
    }
}
impl W {
    ///Bit 26 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_i2c_iso_en(&mut self) -> HP_MODEM_I2C_ISO_EN_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_I2C_ISO_EN_W::new(self, 26)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_i2c_retention(
        &mut self,
    ) -> HP_MODEM_I2C_RETENTION_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_I2C_RETENTION_W::new(self, 27)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_xpd_bb_i2c(&mut self) -> HP_MODEM_XPD_BB_I2C_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_BB_I2C_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_xpd_bbpll_i2c(
        &mut self,
    ) -> HP_MODEM_XPD_BBPLL_I2C_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_BBPLL_I2C_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_xpd_bbpll(&mut self) -> HP_MODEM_XPD_BBPLL_W<HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_BBPLL_W::new(self, 30)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_modem_hp_ck_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_hp_ck_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_MODEM_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_CK_POWER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hp_modem_hp_ck_power::R`](R) reader structure
impl crate::Readable for HP_MODEM_HP_CK_POWER_SPEC {}
///`write(|w| ..)` method takes [`hp_modem_hp_ck_power::W`](W) writer structure
impl crate::Writable for HP_MODEM_HP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_MODEM_HP_CK_POWER to value 0
impl crate::Resettable for HP_MODEM_HP_CK_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
