///Register `CTRL_TICK_CONF` reader
pub type R = crate::R<CTRL_TICK_CONF_SPEC>;
///Register `CTRL_TICK_CONF` writer
pub type W = crate::W<CTRL_TICK_CONF_SPEC>;
///Field `XTAL_TICK_NUM` reader - ******* Description ***********
pub type XTAL_TICK_NUM_R = crate::FieldReader;
///Field `XTAL_TICK_NUM` writer - ******* Description ***********
pub type XTAL_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FOSC_TICK_NUM` reader - ******* Description ***********
pub type FOSC_TICK_NUM_R = crate::FieldReader;
///Field `FOSC_TICK_NUM` writer - ******* Description ***********
pub type FOSC_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TICK_ENABLE` reader - ******* Description ***********
pub type TICK_ENABLE_R = crate::BitReader;
///Field `TICK_ENABLE` writer - ******* Description ***********
pub type TICK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_TICK_CNT` reader - ******* Description ***********
pub type RST_TICK_CNT_R = crate::BitReader;
///Field `RST_TICK_CNT` writer - ******* Description ***********
pub type RST_TICK_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - ******* Description ***********
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ******* Description ***********
    #[inline(always)]
    pub fn fosc_tick_num(&self) -> FOSC_TICK_NUM_R {
        FOSC_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - ******* Description ***********
    #[inline(always)]
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ******* Description ***********
    #[inline(always)]
    pub fn rst_tick_cnt(&self) -> RST_TICK_CNT_R {
        RST_TICK_CNT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_TICK_CONF")
            .field("xtal_tick_num", &self.xtal_tick_num())
            .field("fosc_tick_num", &self.fosc_tick_num())
            .field("tick_enable", &self.tick_enable())
            .field("rst_tick_cnt", &self.rst_tick_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W<CTRL_TICK_CONF_SPEC> {
        XTAL_TICK_NUM_W::new(self, 0)
    }
    ///Bits 8:15 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn fosc_tick_num(&mut self) -> FOSC_TICK_NUM_W<CTRL_TICK_CONF_SPEC> {
        FOSC_TICK_NUM_W::new(self, 8)
    }
    ///Bit 16 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W<CTRL_TICK_CONF_SPEC> {
        TICK_ENABLE_W::new(self, 16)
    }
    ///Bit 17 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<CTRL_TICK_CONF_SPEC> {
        RST_TICK_CNT_W::new(self, 17)
    }
}
/**TICK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl_tick_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_tick_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTRL_TICK_CONF_SPEC;
impl crate::RegisterSpec for CTRL_TICK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ctrl_tick_conf::R`](R) reader structure
impl crate::Readable for CTRL_TICK_CONF_SPEC {}
///`write(|w| ..)` method takes [`ctrl_tick_conf::W`](W) writer structure
impl crate::Writable for CTRL_TICK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL_TICK_CONF to value 0x0001_0727
impl crate::Resettable for CTRL_TICK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_0727;
}
