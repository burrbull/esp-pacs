///Register `RS485_CONF_SYNC` reader
pub type R = crate::R<RS485_CONF_SYNC_SPEC>;
///Register `RS485_CONF_SYNC` writer
pub type W = crate::W<RS485_CONF_SYNC_SPEC>;
///Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit.
pub type DL0_EN_R = crate::BitReader;
///Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit.
pub type DL0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit.
pub type DL1_EN_R = crate::BitReader;
///Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit.
pub type DL1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485_CONF_SYNC")
            .field("dl0_en", &self.dl0_en())
            .field("dl1_en", &self.dl1_en())
            .finish()
    }
}
impl W {
    ///Bit 1 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    #[must_use]
    pub fn dl0_en(&mut self) -> DL0_EN_W<RS485_CONF_SYNC_SPEC> {
        DL0_EN_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    #[must_use]
    pub fn dl1_en(&mut self) -> DL1_EN_W<RS485_CONF_SYNC_SPEC> {
        DL1_EN_W::new(self, 2)
    }
}
/**RS485 mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf_sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf_sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RS485_CONF_SYNC_SPEC;
impl crate::RegisterSpec for RS485_CONF_SYNC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rs485_conf_sync::R`](R) reader structure
impl crate::Readable for RS485_CONF_SYNC_SPEC {}
///`write(|w| ..)` method takes [`rs485_conf_sync::W`](W) writer structure
impl crate::Writable for RS485_CONF_SYNC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RS485_CONF_SYNC to value 0
impl crate::Resettable for RS485_CONF_SYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
