///Register `TICK_CONF` reader
pub type R = crate::R<TICK_CONF_SPEC>;
///Register `TICK_CONF` writer
pub type W = crate::W<TICK_CONF_SPEC>;
///Field `XTAL_TICK_NUM` reader - reg_xtal_tick_num
pub type XTAL_TICK_NUM_R = crate::FieldReader;
///Field `XTAL_TICK_NUM` writer - reg_xtal_tick_num
pub type XTAL_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CK8M_TICK_NUM` reader - reg_ck8m_tick_num
pub type CK8M_TICK_NUM_R = crate::FieldReader;
///Field `CK8M_TICK_NUM` writer - reg_ck8m_tick_num
pub type CK8M_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TICK_ENABLE` reader - reg_tick_enable
pub type TICK_ENABLE_R = crate::BitReader;
///Field `TICK_ENABLE` writer - reg_tick_enable
pub type TICK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - reg_xtal_tick_num
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - reg_ck8m_tick_num
    #[inline(always)]
    pub fn ck8m_tick_num(&self) -> CK8M_TICK_NUM_R {
        CK8M_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - reg_tick_enable
    #[inline(always)]
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TICK_CONF")
            .field("xtal_tick_num", &self.xtal_tick_num())
            .field("ck8m_tick_num", &self.ck8m_tick_num())
            .field("tick_enable", &self.tick_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reg_xtal_tick_num
    #[inline(always)]
    #[must_use]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W<TICK_CONF_SPEC> {
        XTAL_TICK_NUM_W::new(self, 0)
    }
    ///Bits 8:15 - reg_ck8m_tick_num
    #[inline(always)]
    #[must_use]
    pub fn ck8m_tick_num(&mut self) -> CK8M_TICK_NUM_W<TICK_CONF_SPEC> {
        CK8M_TICK_NUM_W::new(self, 8)
    }
    ///Bit 16 - reg_tick_enable
    #[inline(always)]
    #[must_use]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W<TICK_CONF_SPEC> {
        TICK_ENABLE_W::new(self, 16)
    }
}
/**APB_CTRL_TICK_CONF_REG

You can [`read`](crate::generic::Reg::read) this register and get [`tick_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tick_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tick_conf::R`](R) reader structure
impl crate::Readable for TICK_CONF_SPEC {}
///`write(|w| ..)` method takes [`tick_conf::W`](W) writer structure
impl crate::Writable for TICK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TICK_CONF to value 0x0001_0727
impl crate::Resettable for TICK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_0727;
}
