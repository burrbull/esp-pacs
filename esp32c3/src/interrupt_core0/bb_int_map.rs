///Register `BB_INT_MAP` reader
pub type R = crate::R<BB_INT_MAP_SPEC>;
///Register `BB_INT_MAP` writer
pub type W = crate::W<BB_INT_MAP_SPEC>;
///Field `BB_INT_MAP` reader - reg_core0_bb_int_map
pub type BB_INT_MAP_R = crate::FieldReader;
///Field `BB_INT_MAP` writer - reg_core0_bb_int_map
pub type BB_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - reg_core0_bb_int_map
    #[inline(always)]
    pub fn bb_int_map(&self) -> BB_INT_MAP_R {
        BB_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BB_INT_MAP").field("bb_int_map", &self.bb_int_map()).finish()
    }
}
impl W {
    ///Bits 0:4 - reg_core0_bb_int_map
    #[inline(always)]
    #[must_use]
    pub fn bb_int_map(&mut self) -> BB_INT_MAP_W<BB_INT_MAP_SPEC> {
        BB_INT_MAP_W::new(self, 0)
    }
}
/**bb intr map register

You can [`read`](crate::generic::Reg::read) this register and get [`bb_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bb_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BB_INT_MAP_SPEC;
impl crate::RegisterSpec for BB_INT_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bb_int_map::R`](R) reader structure
impl crate::Readable for BB_INT_MAP_SPEC {}
///`write(|w| ..)` method takes [`bb_int_map::W`](W) writer structure
impl crate::Writable for BB_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BB_INT_MAP to value 0
impl crate::Resettable for BB_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
