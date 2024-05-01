///Register `MISC` reader
pub type R = crate::R<MISC_SPEC>;
///Register `MISC` writer
pub type W = crate::W<MISC_SPEC>;
///Field `FSUB_PIN` reader - Flash is connected to SPI SUBPIN bus.
pub type FSUB_PIN_R = crate::BitReader;
///Field `FSUB_PIN` writer - Flash is connected to SPI SUBPIN bus.
pub type FSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSUB_PIN` reader - Ext_RAM is connected to SPI SUBPIN bus.
pub type SSUB_PIN_R = crate::BitReader;
///Field `SSUB_PIN` writer - Ext_RAM is connected to SPI SUBPIN bus.
pub type SSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle
pub type CK_IDLE_EDGE_R = crate::BitReader;
///Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set.
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
///Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set.
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Flash is connected to SPI SUBPIN bus.
    #[inline(always)]
    pub fn fsub_pin(&self) -> FSUB_PIN_R {
        FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Ext_RAM is connected to SPI SUBPIN bus.
    #[inline(always)]
    pub fn ssub_pin(&self) -> SSUB_PIN_R {
        SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPI_CS line keep low when the bit is set.
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("fsub_pin", &self.fsub_pin())
            .field("ssub_pin", &self.ssub_pin())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .finish()
    }
}
impl W {
    ///Bit 7 - Flash is connected to SPI SUBPIN bus.
    #[inline(always)]
    #[must_use]
    pub fn fsub_pin(&mut self) -> FSUB_PIN_W<MISC_SPEC> {
        FSUB_PIN_W::new(self, 7)
    }
    ///Bit 8 - Ext_RAM is connected to SPI SUBPIN bus.
    #[inline(always)]
    #[must_use]
    pub fn ssub_pin(&mut self) -> SSUB_PIN_W<MISC_SPEC> {
        SSUB_PIN_W::new(self, 8)
    }
    ///Bit 9 - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    ///Bit 10 - SPI_CS line keep low when the bit is set.
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
/**SPI0 misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`misc::R`](R) reader structure
impl crate::Readable for MISC_SPEC {}
///`write(|w| ..)` method takes [`misc::W`](W) writer structure
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MISC to value 0
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
