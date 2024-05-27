///Register `LP_TIMER_INT_MAP` reader
pub type R = crate::R<LP_TIMER_INT_MAP_SPEC>;
///Register `LP_TIMER_INT_MAP` writer
pub type W = crate::W<LP_TIMER_INT_MAP_SPEC>;
///Field `LP_TIMER_INT_MAP` reader - Need add description
pub type LP_TIMER_INT_MAP_R = crate::FieldReader;
///Field `LP_TIMER_INT_MAP` writer - Need add description
pub type LP_TIMER_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    pub fn lp_timer_int_map(&self) -> LP_TIMER_INT_MAP_R {
        LP_TIMER_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TIMER_INT_MAP")
            .field("lp_timer_int_map", &self.lp_timer_int_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_int_map(&mut self) -> LP_TIMER_INT_MAP_W<LP_TIMER_INT_MAP_SPEC> {
        LP_TIMER_INT_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`lp_timer_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timer_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_TIMER_INT_MAP_SPEC;
impl crate::RegisterSpec for LP_TIMER_INT_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_timer_int_map::R`](R) reader structure
impl crate::Readable for LP_TIMER_INT_MAP_SPEC {}
///`write(|w| ..)` method takes [`lp_timer_int_map::W`](W) writer structure
impl crate::Writable for LP_TIMER_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_TIMER_INT_MAP to value 0
impl crate::Resettable for LP_TIMER_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
