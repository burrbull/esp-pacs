#[doc = "Register `DIN_NUM` reader"]
pub type R = crate::R<DIN_NUM_SPEC>;
#[doc = "Field `DIN_NUM(0-7)` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type DIN_NUM_R = crate::FieldReader;
impl R {
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DIN0_NUM` field"]
    #[inline(always)]
    pub fn din_num(&self, n: u8) -> DIN_NUM_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_NUM_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din_num_iter(&self) -> impl Iterator<Item = DIN_NUM_R> + '_ {
        (0..8).map(move |n| DIN_NUM_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din4_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din5_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din6_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn din7_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 14) & 3) as u8)
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
            .field("din4_num", &format_args!("{}", self.din4_num().bits()))
            .field("din5_num", &format_args!("{}", self.din5_num().bits()))
            .field("din6_num", &format_args!("{}", self.din6_num().bits()))
            .field("din7_num", &format_args!("{}", self.din7_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI input delay number configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_num::R`](R) reader structure"]
impl crate::Readable for DIN_NUM_SPEC {}
#[doc = "`reset()` method sets DIN_NUM to value 0"]
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
