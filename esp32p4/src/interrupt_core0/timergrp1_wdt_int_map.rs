#[doc = "Register `TIMERGRP1_WDT_INT_MAP` reader"]
pub type R = crate::R<TIMERGRP1_WDT_INT_MAP_SPEC>;
#[doc = "Register `TIMERGRP1_WDT_INT_MAP` writer"]
pub type W = crate::W<TIMERGRP1_WDT_INT_MAP_SPEC>;
#[doc = "Field `CORE0_TIMERGRP1_WDT_INT_MAP` reader - NA"]
pub type CORE0_TIMERGRP1_WDT_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE0_TIMERGRP1_WDT_INT_MAP` writer - NA"]
pub type CORE0_TIMERGRP1_WDT_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core0_timergrp1_wdt_int_map(&self) -> CORE0_TIMERGRP1_WDT_INT_MAP_R {
        CORE0_TIMERGRP1_WDT_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGRP1_WDT_INT_MAP")
            .field(
                "core0_timergrp1_wdt_int_map",
                &self.core0_timergrp1_wdt_int_map(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core0_timergrp1_wdt_int_map(
        &mut self,
    ) -> CORE0_TIMERGRP1_WDT_INT_MAP_W<TIMERGRP1_WDT_INT_MAP_SPEC> {
        CORE0_TIMERGRP1_WDT_INT_MAP_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp1_wdt_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp1_wdt_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGRP1_WDT_INT_MAP_SPEC;
impl crate::RegisterSpec for TIMERGRP1_WDT_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergrp1_wdt_int_map::R`](R) reader structure"]
impl crate::Readable for TIMERGRP1_WDT_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergrp1_wdt_int_map::W`](W) writer structure"]
impl crate::Writable for TIMERGRP1_WDT_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMERGRP1_WDT_INT_MAP to value 0"]
impl crate::Resettable for TIMERGRP1_WDT_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
