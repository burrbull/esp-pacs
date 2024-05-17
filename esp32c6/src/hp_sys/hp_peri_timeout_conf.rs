///Register `HP_PERI_TIMEOUT_CONF` reader
pub type R = crate::R<HP_PERI_TIMEOUT_CONF_SPEC>;
///Register `HP_PERI_TIMEOUT_CONF` writer
pub type W = crate::W<HP_PERI_TIMEOUT_CONF_SPEC>;
///Field `HP_PERI_TIMEOUT_THRES` reader - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain.
pub type HP_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
///Field `HP_PERI_TIMEOUT_THRES` writer - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain.
pub type HP_PERI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HP_PERI_TIMEOUT_INT_CLEAR` writer - Set this bit as 1 to clear timeout interrupt
pub type HP_PERI_TIMEOUT_INT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_PERI_TIMEOUT_PROTECT_EN` reader - Set this bit as 1 to enable timeout protection for accessing hp peripheral registers
pub type HP_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
///Field `HP_PERI_TIMEOUT_PROTECT_EN` writer - Set this bit as 1 to enable timeout protection for accessing hp peripheral registers
pub type HP_PERI_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain.
    #[inline(always)]
    pub fn hp_peri_timeout_thres(&self) -> HP_PERI_TIMEOUT_THRES_R {
        HP_PERI_TIMEOUT_THRES_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 17 - Set this bit as 1 to enable timeout protection for accessing hp peripheral registers
    #[inline(always)]
    pub fn hp_peri_timeout_protect_en(&self) -> HP_PERI_TIMEOUT_PROTECT_EN_R {
        HP_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PERI_TIMEOUT_CONF")
            .field("hp_peri_timeout_thres", &self.hp_peri_timeout_thres())
            .field("hp_peri_timeout_protect_en", &self.hp_peri_timeout_protect_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Set the timeout threshold for bus access, corresponding to the number of clock cycles of the clock domain.
    #[inline(always)]
    #[must_use]
    pub fn hp_peri_timeout_thres(
        &mut self,
    ) -> HP_PERI_TIMEOUT_THRES_W<HP_PERI_TIMEOUT_CONF_SPEC> {
        HP_PERI_TIMEOUT_THRES_W::new(self, 0)
    }
    ///Bit 16 - Set this bit as 1 to clear timeout interrupt
    #[inline(always)]
    #[must_use]
    pub fn hp_peri_timeout_int_clear(
        &mut self,
    ) -> HP_PERI_TIMEOUT_INT_CLEAR_W<HP_PERI_TIMEOUT_CONF_SPEC> {
        HP_PERI_TIMEOUT_INT_CLEAR_W::new(self, 16)
    }
    ///Bit 17 - Set this bit as 1 to enable timeout protection for accessing hp peripheral registers
    #[inline(always)]
    #[must_use]
    pub fn hp_peri_timeout_protect_en(
        &mut self,
    ) -> HP_PERI_TIMEOUT_PROTECT_EN_W<HP_PERI_TIMEOUT_CONF_SPEC> {
        HP_PERI_TIMEOUT_PROTECT_EN_W::new(self, 17)
    }
}
/**HP_PERI_TIMEOUT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`hp_peri_timeout_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_peri_timeout_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_PERI_TIMEOUT_CONF_SPEC;
impl crate::RegisterSpec for HP_PERI_TIMEOUT_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hp_peri_timeout_conf::R`](R) reader structure
impl crate::Readable for HP_PERI_TIMEOUT_CONF_SPEC {}
///`write(|w| ..)` method takes [`hp_peri_timeout_conf::W`](W) writer structure
impl crate::Writable for HP_PERI_TIMEOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_PERI_TIMEOUT_CONF to value 0x0002_ffff
impl crate::Resettable for HP_PERI_TIMEOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0002_ffff;
}
