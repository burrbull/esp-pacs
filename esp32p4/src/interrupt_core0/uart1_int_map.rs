#[doc = "Register `UART1_INT_MAP` reader"]
pub type R = crate::R<UART1_INT_MAP_SPEC>;
#[doc = "Register `UART1_INT_MAP` writer"]
pub type W = crate::W<UART1_INT_MAP_SPEC>;
#[doc = "Field `CORE0_UART1_INT_MAP` reader - NA"]
pub type CORE0_UART1_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE0_UART1_INT_MAP` writer - NA"]
pub type CORE0_UART1_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core0_uart1_int_map(&self) -> CORE0_UART1_INT_MAP_R {
        CORE0_UART1_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1_INT_MAP")
            .field("core0_uart1_int_map", &self.core0_uart1_int_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART1_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn core0_uart1_int_map(&mut self) -> CORE0_UART1_INT_MAP_W<UART1_INT_MAP_SPEC> {
        CORE0_UART1_INT_MAP_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart1_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART1_INT_MAP_SPEC;
impl crate::RegisterSpec for UART1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart1_int_map::R`](R) reader structure"]
impl crate::Readable for UART1_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart1_int_map::W`](W) writer structure"]
impl crate::Writable for UART1_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART1_INT_MAP to value 0"]
impl crate::Resettable for UART1_INT_MAP_SPEC {}
