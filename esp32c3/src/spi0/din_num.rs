#[doc = "Register `DIN_NUM` reader"]
pub type R = crate::R<DIN_NUM_SPEC>;
#[doc = "Register `DIN_NUM` writer"]
pub type W = crate::W<DIN_NUM_SPEC>;
#[doc = "Configure the delays to input signal FSPID based on the setting of DIN%s_MODE. Can be configured in CONF state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELAY {
    #[doc = "0: Delayed by 1 clock cycle"]
    Cycle1 = 0,
    #[doc = "1: Delayed by 2 clock cycles"]
    Cycles2 = 1,
    #[doc = "2: Delayed by 3 clock cycles"]
    Cycles3 = 2,
    #[doc = "3: Delayed by 4 clock cycles"]
    Cycles4 = 3,
}
impl From<DELAY> for u8 {
    #[inline(always)]
    fn from(variant: DELAY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELAY {
    type Ux = u8;
}
impl crate::IsEnum for DELAY {}
#[doc = "Field `DIN0_NUM` reader - Configure the delays to input signal FSPID based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub type DIN0_NUM_R = crate::FieldReader<DELAY>;
impl DIN0_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DELAY {
        match self.bits {
            0 => DELAY::Cycle1,
            1 => DELAY::Cycles2,
            2 => DELAY::Cycles3,
            3 => DELAY::Cycles4,
            _ => unreachable!(),
        }
    }
    #[doc = "Delayed by 1 clock cycle"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == DELAY::Cycle1
    }
    #[doc = "Delayed by 2 clock cycles"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == DELAY::Cycles2
    }
    #[doc = "Delayed by 3 clock cycles"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == DELAY::Cycles3
    }
    #[doc = "Delayed by 4 clock cycles"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == DELAY::Cycles4
    }
}
#[doc = "Field `DIN0_NUM` writer - Configure the delays to input signal FSPID based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub type DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELAY, crate::Safe>;
impl<'a, REG> DIN0_NUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delayed by 1 clock cycle"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY::Cycle1)
    }
    #[doc = "Delayed by 2 clock cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY::Cycles2)
    }
    #[doc = "Delayed by 3 clock cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY::Cycles3)
    }
    #[doc = "Delayed by 4 clock cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY::Cycles4)
    }
}
#[doc = "Field `DIN1_NUM` reader - Configure the delays to input signal FSPIQ based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub use DIN0_NUM_R as DIN1_NUM_R;
#[doc = "Field `DIN2_NUM` reader - Configure the delays to input signal FSPIWP based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub use DIN0_NUM_R as DIN2_NUM_R;
#[doc = "Field `DIN3_NUM` reader - Configure the delays to input signal FSPIHF based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub use DIN0_NUM_R as DIN3_NUM_R;
#[doc = "Field `DIN1_NUM` writer - Configure the delays to input signal FSPIQ based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub use DIN0_NUM_W as DIN1_NUM_W;
#[doc = "Field `DIN2_NUM` writer - Configure the delays to input signal FSPIWP based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub use DIN0_NUM_W as DIN2_NUM_W;
#[doc = "Field `DIN3_NUM` writer - Configure the delays to input signal FSPIHF based on the setting of DIN%s_MODE. Can be configured in CONF state"]
pub use DIN0_NUM_W as DIN3_NUM_W;
impl R {
    #[doc = "Bits 0:1 - Configure the delays to input signal FSPID based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configure the delays to input signal FSPIQ based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configure the delays to input signal FSPIWP based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configure the delays to input signal FSPIHF based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_NUM")
            .field("din0_num", &format_args!("{}", self.din0_num().bits()))
            .field("din1_num", &format_args!("{}", self.din1_num().bits()))
            .field("din2_num", &format_args!("{}", self.din2_num().bits()))
            .field("din3_num", &format_args!("{}", self.din3_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the delays to input signal FSPID based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din0_num(&mut self) -> DIN0_NUM_W<DIN_NUM_SPEC> {
        DIN0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configure the delays to input signal FSPIQ based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din1_num(&mut self) -> DIN1_NUM_W<DIN_NUM_SPEC> {
        DIN1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configure the delays to input signal FSPIWP based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din2_num(&mut self) -> DIN2_NUM_W<DIN_NUM_SPEC> {
        DIN2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configure the delays to input signal FSPIHF based on the setting of DIN%s_MODE. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn din3_num(&mut self) -> DIN3_NUM_W<DIN_NUM_SPEC> {
        DIN3_NUM_W::new(self, 6)
    }
}
#[doc = "SPI0 input delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_num::R`](R) reader structure"]
impl crate::Readable for DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`din_num::W`](W) writer structure"]
impl crate::Writable for DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIN_NUM to value 0"]
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
