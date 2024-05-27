///Register `MEM_TEST_CONF` reader
pub type R = crate::R<MEM_TEST_CONF_SPEC>;
///Register `MEM_TEST_CONF` writer
pub type W = crate::W<MEM_TEST_CONF_SPEC>;
///Field `HP_MEM_WPULSE` reader - This field controls hp system memory WPULSE parameter.
pub type HP_MEM_WPULSE_R = crate::FieldReader;
///Field `HP_MEM_WPULSE` writer - This field controls hp system memory WPULSE parameter.
pub type HP_MEM_WPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HP_MEM_WA` reader - This field controls hp system memory WA parameter.
pub type HP_MEM_WA_R = crate::FieldReader;
///Field `HP_MEM_WA` writer - This field controls hp system memory WA parameter.
pub type HP_MEM_WA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HP_MEM_RA` reader - This field controls hp system memory RA parameter.
pub type HP_MEM_RA_R = crate::FieldReader;
///Field `HP_MEM_RA` writer - This field controls hp system memory RA parameter.
pub type HP_MEM_RA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - This field controls hp system memory WPULSE parameter.
    #[inline(always)]
    pub fn hp_mem_wpulse(&self) -> HP_MEM_WPULSE_R {
        HP_MEM_WPULSE_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - This field controls hp system memory WA parameter.
    #[inline(always)]
    pub fn hp_mem_wa(&self) -> HP_MEM_WA_R {
        HP_MEM_WA_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - This field controls hp system memory RA parameter.
    #[inline(always)]
    pub fn hp_mem_ra(&self) -> HP_MEM_RA_R {
        HP_MEM_RA_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_TEST_CONF")
            .field("hp_mem_wpulse", &self.hp_mem_wpulse())
            .field("hp_mem_wa", &self.hp_mem_wa())
            .field("hp_mem_ra", &self.hp_mem_ra())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - This field controls hp system memory WPULSE parameter.
    #[inline(always)]
    #[must_use]
    pub fn hp_mem_wpulse(&mut self) -> HP_MEM_WPULSE_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_WPULSE_W::new(self, 0)
    }
    ///Bits 3:5 - This field controls hp system memory WA parameter.
    #[inline(always)]
    #[must_use]
    pub fn hp_mem_wa(&mut self) -> HP_MEM_WA_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_WA_W::new(self, 3)
    }
    ///Bits 6:7 - This field controls hp system memory RA parameter.
    #[inline(always)]
    #[must_use]
    pub fn hp_mem_ra(&mut self) -> HP_MEM_RA_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_RA_W::new(self, 6)
    }
}
/**MEM_TEST configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_test_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_test_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_TEST_CONF_SPEC;
impl crate::RegisterSpec for MEM_TEST_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_test_conf::R`](R) reader structure
impl crate::Readable for MEM_TEST_CONF_SPEC {}
///`write(|w| ..)` method takes [`mem_test_conf::W`](W) writer structure
impl crate::Writable for MEM_TEST_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_TEST_CONF to value 0x20
impl crate::Resettable for MEM_TEST_CONF_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
