#[doc = "Register `DIN_NUM` reader"]
pub type R = crate::R<DIN_NUM_SPEC>;
#[doc = "Field `DIN_NUM(0-3)` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type DIN_NUM_R = crate::BitReader;
impl R {
    #[doc = "the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DIN0_NUM` field"]
    #[inline(always)]
    pub fn din_num(&self, n: u8) -> DIN_NUM_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DIN_NUM_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din_num_iter(&self) -> impl Iterator<Item = DIN_NUM_R> + '_ {
        (0..4).map(move |n| DIN_NUM_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_NUM")
            .field("din0_num", &format_args!("{}", self.din0_num().bit()))
            .field("din1_num", &format_args!("{}", self.din1_num().bit()))
            .field("din2_num", &format_args!("{}", self.din2_num().bit()))
            .field("din3_num", &format_args!("{}", self.din3_num().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI0 input delay number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
