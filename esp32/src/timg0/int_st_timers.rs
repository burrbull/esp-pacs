#[doc = "Register `INT_ST_TIMERS` reader"]
pub struct R(crate::R<INT_ST_TIMERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_TIMERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_TIMERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_TIMERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T0_INT_ST` reader - interrupt when timer0 alarm"]
pub type T0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `T1_INT_ST` reader - interrupt when timer1 alarm"]
pub type T1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT_ST` reader - Interrupt when an interrupt stage timeout"]
pub type WDT_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `LACT_INT_ST` reader - "]
pub type LACT_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_st(&self) -> T0_INT_ST_R {
        T0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_st(&self) -> T1_INT_ST_R {
        T1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact_int_st(&self) -> LACT_INT_ST_R {
        LACT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_timers](index.html) module"]
pub struct INT_ST_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ST_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st_timers::R](R) reader structure"]
impl crate::Readable for INT_ST_TIMERS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST_TIMERS to value 0"]
impl crate::Resettable for INT_ST_TIMERS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}