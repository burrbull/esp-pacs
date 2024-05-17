///Register `HP_SLEEP_BACKUP` reader
pub type R = crate::R<HP_SLEEP_BACKUP_SPEC>;
///Register `HP_SLEEP_BACKUP` writer
pub type W = crate::W<HP_SLEEP_BACKUP_SPEC>;
///Field `HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE` reader - need_des
pub type HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
///Field `HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE` writer - need_des
pub type HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_W<'a, REG> = crate::FieldWriter<
    'a,
    REG,
    2,
>;
///Field `HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE` reader - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_R = crate::FieldReader;
///Field `HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE` writer - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_W<'a, REG> = crate::FieldWriter<
    'a,
    REG,
    2,
>;
///Field `HP_SLEEP_RETENTION_MODE` reader - need_des
pub type HP_SLEEP_RETENTION_MODE_R = crate::BitReader;
///Field `HP_SLEEP_RETENTION_MODE` writer - need_des
pub type HP_SLEEP_RETENTION_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_MODEM2SLEEP_RETENTION_EN` reader - need_des
pub type HP_MODEM2SLEEP_RETENTION_EN_R = crate::BitReader;
///Field `HP_MODEM2SLEEP_RETENTION_EN` writer - need_des
pub type HP_MODEM2SLEEP_RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_ACTIVE2SLEEP_RETENTION_EN` reader - need_des
pub type HP_ACTIVE2SLEEP_RETENTION_EN_R = crate::BitReader;
///Field `HP_ACTIVE2SLEEP_RETENTION_EN` writer - need_des
pub type HP_ACTIVE2SLEEP_RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_MODEM2SLEEP_BACKUP_CLK_SEL` reader - need_des
pub type HP_MODEM2SLEEP_BACKUP_CLK_SEL_R = crate::FieldReader;
///Field `HP_MODEM2SLEEP_BACKUP_CLK_SEL` writer - need_des
pub type HP_MODEM2SLEEP_BACKUP_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HP_ACTIVE2SLEEP_BACKUP_CLK_SEL` reader - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_R = crate::FieldReader;
///Field `HP_ACTIVE2SLEEP_BACKUP_CLK_SEL` writer - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HP_MODEM2SLEEP_BACKUP_MODE` reader - need_des
pub type HP_MODEM2SLEEP_BACKUP_MODE_R = crate::FieldReader;
///Field `HP_MODEM2SLEEP_BACKUP_MODE` writer - need_des
pub type HP_MODEM2SLEEP_BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HP_ACTIVE2SLEEP_BACKUP_MODE` reader - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_MODE_R = crate::FieldReader;
///Field `HP_ACTIVE2SLEEP_BACKUP_MODE` writer - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HP_MODEM2SLEEP_BACKUP_EN` reader - need_des
pub type HP_MODEM2SLEEP_BACKUP_EN_R = crate::BitReader;
///Field `HP_MODEM2SLEEP_BACKUP_EN` writer - need_des
pub type HP_MODEM2SLEEP_BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_ACTIVE2SLEEP_BACKUP_EN` reader - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_EN_R = crate::BitReader;
///Field `HP_ACTIVE2SLEEP_BACKUP_EN` writer - need_des
pub type HP_ACTIVE2SLEEP_BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 6:7 - need_des
    #[inline(always)]
    pub fn hp_modem2sleep_backup_modem_clk_code(
        &self,
    ) -> HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_R {
        HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - need_des
    #[inline(always)]
    pub fn hp_active2sleep_backup_modem_clk_code(
        &self,
    ) -> HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_R {
        HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - need_des
    #[inline(always)]
    pub fn hp_sleep_retention_mode(&self) -> HP_SLEEP_RETENTION_MODE_R {
        HP_SLEEP_RETENTION_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - need_des
    #[inline(always)]
    pub fn hp_modem2sleep_retention_en(&self) -> HP_MODEM2SLEEP_RETENTION_EN_R {
        HP_MODEM2SLEEP_RETENTION_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - need_des
    #[inline(always)]
    pub fn hp_active2sleep_retention_en(&self) -> HP_ACTIVE2SLEEP_RETENTION_EN_R {
        HP_ACTIVE2SLEEP_RETENTION_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - need_des
    #[inline(always)]
    pub fn hp_modem2sleep_backup_clk_sel(&self) -> HP_MODEM2SLEEP_BACKUP_CLK_SEL_R {
        HP_MODEM2SLEEP_BACKUP_CLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - need_des
    #[inline(always)]
    pub fn hp_active2sleep_backup_clk_sel(&self) -> HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_R {
        HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 23:25 - need_des
    #[inline(always)]
    pub fn hp_modem2sleep_backup_mode(&self) -> HP_MODEM2SLEEP_BACKUP_MODE_R {
        HP_MODEM2SLEEP_BACKUP_MODE_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:28 - need_des
    #[inline(always)]
    pub fn hp_active2sleep_backup_mode(&self) -> HP_ACTIVE2SLEEP_BACKUP_MODE_R {
        HP_ACTIVE2SLEEP_BACKUP_MODE_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn hp_modem2sleep_backup_en(&self) -> HP_MODEM2SLEEP_BACKUP_EN_R {
        HP_MODEM2SLEEP_BACKUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn hp_active2sleep_backup_en(&self) -> HP_ACTIVE2SLEEP_BACKUP_EN_R {
        HP_ACTIVE2SLEEP_BACKUP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_BACKUP")
            .field(
                "hp_modem2sleep_backup_modem_clk_code",
                &self.hp_modem2sleep_backup_modem_clk_code(),
            )
            .field(
                "hp_active2sleep_backup_modem_clk_code",
                &self.hp_active2sleep_backup_modem_clk_code(),
            )
            .field("hp_sleep_retention_mode", &self.hp_sleep_retention_mode())
            .field("hp_modem2sleep_retention_en", &self.hp_modem2sleep_retention_en())
            .field("hp_active2sleep_retention_en", &self.hp_active2sleep_retention_en())
            .field(
                "hp_modem2sleep_backup_clk_sel",
                &self.hp_modem2sleep_backup_clk_sel(),
            )
            .field(
                "hp_active2sleep_backup_clk_sel",
                &self.hp_active2sleep_backup_clk_sel(),
            )
            .field("hp_modem2sleep_backup_mode", &self.hp_modem2sleep_backup_mode())
            .field("hp_active2sleep_backup_mode", &self.hp_active2sleep_backup_mode())
            .field("hp_modem2sleep_backup_en", &self.hp_modem2sleep_backup_en())
            .field("hp_active2sleep_backup_en", &self.hp_active2sleep_backup_en())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_modem_clk_code(
        &mut self,
    ) -> HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_W<HP_SLEEP_BACKUP_SPEC> {
        HP_MODEM2SLEEP_BACKUP_MODEM_CLK_CODE_W::new(self, 6)
    }
    ///Bits 8:9 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_modem_clk_code(
        &mut self,
    ) -> HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_W<HP_SLEEP_BACKUP_SPEC> {
        HP_ACTIVE2SLEEP_BACKUP_MODEM_CLK_CODE_W::new(self, 8)
    }
    ///Bit 10 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_retention_mode(
        &mut self,
    ) -> HP_SLEEP_RETENTION_MODE_W<HP_SLEEP_BACKUP_SPEC> {
        HP_SLEEP_RETENTION_MODE_W::new(self, 10)
    }
    ///Bit 12 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_retention_en(
        &mut self,
    ) -> HP_MODEM2SLEEP_RETENTION_EN_W<HP_SLEEP_BACKUP_SPEC> {
        HP_MODEM2SLEEP_RETENTION_EN_W::new(self, 12)
    }
    ///Bit 13 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_retention_en(
        &mut self,
    ) -> HP_ACTIVE2SLEEP_RETENTION_EN_W<HP_SLEEP_BACKUP_SPEC> {
        HP_ACTIVE2SLEEP_RETENTION_EN_W::new(self, 13)
    }
    ///Bits 16:17 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_clk_sel(
        &mut self,
    ) -> HP_MODEM2SLEEP_BACKUP_CLK_SEL_W<HP_SLEEP_BACKUP_SPEC> {
        HP_MODEM2SLEEP_BACKUP_CLK_SEL_W::new(self, 16)
    }
    ///Bits 18:19 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_clk_sel(
        &mut self,
    ) -> HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_W<HP_SLEEP_BACKUP_SPEC> {
        HP_ACTIVE2SLEEP_BACKUP_CLK_SEL_W::new(self, 18)
    }
    ///Bits 23:25 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_mode(
        &mut self,
    ) -> HP_MODEM2SLEEP_BACKUP_MODE_W<HP_SLEEP_BACKUP_SPEC> {
        HP_MODEM2SLEEP_BACKUP_MODE_W::new(self, 23)
    }
    ///Bits 26:28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_mode(
        &mut self,
    ) -> HP_ACTIVE2SLEEP_BACKUP_MODE_W<HP_SLEEP_BACKUP_SPEC> {
        HP_ACTIVE2SLEEP_BACKUP_MODE_W::new(self, 26)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_modem2sleep_backup_en(
        &mut self,
    ) -> HP_MODEM2SLEEP_BACKUP_EN_W<HP_SLEEP_BACKUP_SPEC> {
        HP_MODEM2SLEEP_BACKUP_EN_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_active2sleep_backup_en(
        &mut self,
    ) -> HP_ACTIVE2SLEEP_BACKUP_EN_W<HP_SLEEP_BACKUP_SPEC> {
        HP_ACTIVE2SLEEP_BACKUP_EN_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_backup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_backup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_SLEEP_BACKUP_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BACKUP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hp_sleep_backup::R`](R) reader structure
impl crate::Readable for HP_SLEEP_BACKUP_SPEC {}
///`write(|w| ..)` method takes [`hp_sleep_backup::W`](W) writer structure
impl crate::Writable for HP_SLEEP_BACKUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_SLEEP_BACKUP to value 0
impl crate::Resettable for HP_SLEEP_BACKUP_SPEC {
    const RESET_VALUE: u32 = 0;
}
