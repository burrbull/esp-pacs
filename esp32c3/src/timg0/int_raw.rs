#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `T(0-0)` reader - t%s_int_raw"]
pub type T_R = crate::BitReader;
#[doc = "Field `WDT` reader - wdt_int_raw"]
pub type WDT_R = crate::BitReader;
impl R {
    #[doc = "t(0-0)_int_raw"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `T0` field.</div>"]
    #[inline(always)]
    pub fn t(&self, n: u8) -> T_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        T_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "t(0-0)_int_raw"]
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = T_R> + '_ {
        (0..1).map(move |n| T_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    #[doc = "Bit 0 - t0_int_raw"]
    #[inline(always)]
    pub fn t0(&self) -> T_R {
        T_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt_int_raw"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("t0", &self.t0())
            .field("wdt", &self.wdt())
            .finish()
    }
}
#[doc = "INT_RAW_TIMG_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}