///Register `DEBUG_SEL1` reader
pub type R = crate::R<DEBUG_SEL1_SPEC>;
///Register `DEBUG_SEL1` writer
pub type W = crate::W<DEBUG_SEL1_SPEC>;
///Field `LP_DEBUG_SEL4` reader - need des
pub type LP_DEBUG_SEL4_R = crate::FieldReader;
///Field `LP_DEBUG_SEL4` writer - need des
pub type LP_DEBUG_SEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - need des
    #[inline(always)]
    pub fn lp_debug_sel4(&self) -> LP_DEBUG_SEL4_R {
        LP_DEBUG_SEL4_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL1")
            .field("lp_debug_sel4", &self.lp_debug_sel4())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - need des
    #[inline(always)]
    #[must_use]
    pub fn lp_debug_sel4(&mut self) -> LP_DEBUG_SEL4_W<DEBUG_SEL1_SPEC> {
        LP_DEBUG_SEL4_W::new(self, 0)
    }
}
/**need des

You can [`read`](crate::generic::Reg::read) this register and get [`debug_sel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEBUG_SEL1_SPEC;
impl crate::RegisterSpec for DEBUG_SEL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`debug_sel1::R`](R) reader structure
impl crate::Readable for DEBUG_SEL1_SPEC {}
///`write(|w| ..)` method takes [`debug_sel1::W`](W) writer structure
impl crate::Writable for DEBUG_SEL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEBUG_SEL1 to value 0
impl crate::Resettable for DEBUG_SEL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
