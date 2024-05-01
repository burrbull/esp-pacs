///Register `DIN_NUM` reader
pub type R = crate::R<DIN_NUM_SPEC>;
///Register `DIN_NUM` writer
pub type W = crate::W<DIN_NUM_SPEC>;
///Field `DIN0_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN0_NUM_R = crate::FieldReader;
///Field `DIN0_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN1_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN1_NUM_R = crate::FieldReader;
///Field `DIN1_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN2_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN2_NUM_R = crate::FieldReader;
///Field `DIN2_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN3_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN3_NUM_R = crate::FieldReader;
///Field `DIN3_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN4_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN4_NUM_R = crate::FieldReader;
///Field `DIN4_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN4_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN5_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN5_NUM_R = crate::FieldReader;
///Field `DIN5_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN5_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN6_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN6_NUM_R = crate::FieldReader;
///Field `DIN6_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN6_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIN7_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN7_NUM_R = crate::FieldReader;
///Field `DIN7_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
pub type DIN7_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din4_num(&self) -> DIN4_NUM_R {
        DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din5_num(&self) -> DIN5_NUM_R {
        DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din6_num(&self) -> DIN6_NUM_R {
        DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    pub fn din7_num(&self) -> DIN7_NUM_R {
        DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_NUM")
            .field("din0_num", &self.din0_num())
            .field("din1_num", &self.din1_num())
            .field("din2_num", &self.din2_num())
            .field("din3_num", &self.din3_num())
            .field("din4_num", &self.din4_num())
            .field("din5_num", &self.din5_num())
            .field("din6_num", &self.din6_num())
            .field("din7_num", &self.din7_num())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din0_num(&mut self) -> DIN0_NUM_W<DIN_NUM_SPEC> {
        DIN0_NUM_W::new(self, 0)
    }
    ///Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din1_num(&mut self) -> DIN1_NUM_W<DIN_NUM_SPEC> {
        DIN1_NUM_W::new(self, 2)
    }
    ///Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din2_num(&mut self) -> DIN2_NUM_W<DIN_NUM_SPEC> {
        DIN2_NUM_W::new(self, 4)
    }
    ///Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din3_num(&mut self) -> DIN3_NUM_W<DIN_NUM_SPEC> {
        DIN3_NUM_W::new(self, 6)
    }
    ///Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din4_num(&mut self) -> DIN4_NUM_W<DIN_NUM_SPEC> {
        DIN4_NUM_W::new(self, 8)
    }
    ///Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din5_num(&mut self) -> DIN5_NUM_W<DIN_NUM_SPEC> {
        DIN5_NUM_W::new(self, 10)
    }
    ///Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din6_num(&mut self) -> DIN6_NUM_W<DIN_NUM_SPEC> {
        DIN6_NUM_W::new(self, 12)
    }
    ///Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn din7_num(&mut self) -> DIN7_NUM_W<DIN_NUM_SPEC> {
        DIN7_NUM_W::new(self, 14)
    }
}
/**SPI input delay number configuration

You can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`din_num::R`](R) reader structure
impl crate::Readable for DIN_NUM_SPEC {}
///`write(|w| ..)` method takes [`din_num::W`](W) writer structure
impl crate::Writable for DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIN_NUM to value 0
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
