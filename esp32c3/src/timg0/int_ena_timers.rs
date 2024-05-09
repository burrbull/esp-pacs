#[doc = "Register `INT_ENA_TIMERS` reader"]
pub type R = crate::R<INT_ENA_TIMERS_SPEC>;
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub type W = crate::W<INT_ENA_TIMERS_SPEC>;
#[doc = "Field `T(0-0)` reader - t%s_int_ena"]
pub type T_R = crate::BitReader;
#[doc = "Field `T(0-0)` writer - t%s_int_ena"]
pub type T_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT` reader - wdt_int_ena"]
pub type WDT_R = crate::BitReader;
#[doc = "Field `WDT` writer - wdt_int_ena"]
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "t(0-0)_int_ena"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `T0` field"]
    #[inline(always)]
    pub fn t(&self, n: u8) -> T_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        T_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "t(0-0)_int_ena"]
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = T_R> + '_ {
        (0..1).map(move |n| T_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    #[doc = "Bit 0 - t0_int_ena"]
    #[inline(always)]
    pub fn t0(&self) -> T_R {
        T_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt_int_ena"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_TIMERS")
            .field("t0", &self.t0().bit())
            .field("wdt", &self.wdt().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "t(0-0)_int_ena"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `T0` field"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self, n: u8) -> T_W<INT_ENA_TIMERS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        T_W::new(self, n * 0)
    }
    #[doc = "Bit 0 - t0_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T_W<INT_ENA_TIMERS_SPEC> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 1 - wdt_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<INT_ENA_TIMERS_SPEC> {
        WDT_W::new(self, 1)
    }
}
#[doc = "INT_ENA_TIMG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena_timers::R`](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena_timers::W`](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {}
