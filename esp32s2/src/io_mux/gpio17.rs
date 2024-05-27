///Register `GPIO17` reader
pub type R = crate::R<GPIO17_SPEC>;
///Register `GPIO17` writer
pub type W = crate::W<GPIO17_SPEC>;
///Field `MCU_OE` reader - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled.
pub type MCU_OE_R = crate::BitReader;
///Field `MCU_OE` writer - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled.
pub type MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_SEL` reader - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode.
pub type SLP_SEL_R = crate::BitReader;
///Field `SLP_SEL` writer - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode.
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_WPD` reader - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled.
pub type MCU_WPD_R = crate::BitReader;
///Field `MCU_WPD` writer - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled.
pub type MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_WPU` reader - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
pub type MCU_WPU_R = crate::BitReader;
///Field `MCU_WPU` writer - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
pub type MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_IE` reader - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled.
pub type MCU_IE_R = crate::BitReader;
///Field `MCU_IE` writer - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled.
pub type MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_WPD` reader - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled.
pub type FUN_WPD_R = crate::BitReader;
///Field `FUN_WPD` writer - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled.
pub type FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_WPU` reader - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
pub type FUN_WPU_R = crate::BitReader;
///Field `FUN_WPU` writer - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
pub type FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_IE` reader - Input enable of the pin. 1: Input enabled. 0: Input disabled.
pub type FUN_IE_R = crate::BitReader;
///Field `FUN_IE` writer - Input enable of the pin. 1: Input enabled. 0: Input disabled.
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_DRV` reader - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA.
pub type FUN_DRV_R = crate::FieldReader;
///Field `FUN_DRV` writer - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA.
pub type FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCU_SEL` reader - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc.
pub type MCU_SEL_R = crate::FieldReader;
///Field `MCU_SEL` writer - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc.
pub type MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FILTER_EN` reader - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled.
pub type FILTER_EN_R = crate::BitReader;
///Field `FILTER_EN` writer - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled.
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled.
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode.
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled.
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled.
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled.
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Input enable of the pin. 1: Input enabled. 0: Input disabled.
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA.
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc.
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled.
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO17")
            .field("mcu_oe", &self.mcu_oe())
            .field("slp_sel", &self.slp_sel())
            .field("mcu_wpd", &self.mcu_wpd())
            .field("mcu_wpu", &self.mcu_wpu())
            .field("mcu_ie", &self.mcu_ie())
            .field("fun_wpd", &self.fun_wpd())
            .field("fun_wpu", &self.fun_wpu())
            .field("fun_ie", &self.fun_ie())
            .field("fun_drv", &self.fun_drv())
            .field("mcu_sel", &self.mcu_sel())
            .field("filter_en", &self.filter_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Output enable of the pin in sleep mode. 1: Output enabled. 0: Output disabled.
    #[inline(always)]
    #[must_use]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<GPIO17_SPEC> {
        MCU_OE_W::new(self, 0)
    }
    ///Bit 1 - Sleep mode selection of this pin. Set to 1 to put the pin in sleep mode.
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<GPIO17_SPEC> {
        SLP_SEL_W::new(self, 1)
    }
    ///Bit 2 - Pull-down enable of the pin during sleep mode. 1: Internal pull-down enabled. 0: internal pull-down disabled.
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<GPIO17_SPEC> {
        MCU_WPD_W::new(self, 2)
    }
    ///Bit 3 - Pull-up enable of the pin during sleep mode. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<GPIO17_SPEC> {
        MCU_WPU_W::new(self, 3)
    }
    ///Bit 4 - Input enable of the pin during sleep mode. 1: Input enabled. 0: Input disabled.
    #[inline(always)]
    #[must_use]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<GPIO17_SPEC> {
        MCU_IE_W::new(self, 4)
    }
    ///Bit 7 - Pull-down enable of the pin. 1: Internal pull-down enabled. 0: internal pull-down disabled.
    #[inline(always)]
    #[must_use]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<GPIO17_SPEC> {
        FUN_WPD_W::new(self, 7)
    }
    ///Bit 8 - Pull-up enable of the pin. 1: Internal pull-up enabled. 0: Internal pull-up disabled.
    #[inline(always)]
    #[must_use]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<GPIO17_SPEC> {
        FUN_WPU_W::new(self, 8)
    }
    ///Bit 9 - Input enable of the pin. 1: Input enabled. 0: Input disabled.
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<GPIO17_SPEC> {
        FUN_IE_W::new(self, 9)
    }
    ///Bits 10:11 - Select the drive strength of the pin. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA.
    #[inline(always)]
    #[must_use]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<GPIO17_SPEC> {
        FUN_DRV_W::new(self, 10)
    }
    ///Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2, etc.
    #[inline(always)]
    #[must_use]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<GPIO17_SPEC> {
        MCU_SEL_W::new(self, 12)
    }
    ///Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 2: Filter disabled.
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<GPIO17_SPEC> {
        FILTER_EN_W::new(self, 15)
    }
}
/**Configuration register for pin GPIO17

You can [`read`](crate::generic::Reg::read) this register and get [`gpio17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO17_SPEC;
impl crate::RegisterSpec for GPIO17_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio17::R`](R) reader structure
impl crate::Readable for GPIO17_SPEC {}
///`write(|w| ..)` method takes [`gpio17::W`](W) writer structure
impl crate::Writable for GPIO17_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO17 to value 0x0b00
impl crate::Resettable for GPIO17_SPEC {
    const RESET_VALUE: u32 = 0x0b00;
}
