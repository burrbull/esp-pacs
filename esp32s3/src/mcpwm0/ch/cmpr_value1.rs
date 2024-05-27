///Register `CMPR_VALUE1` reader
pub type R = crate::R<CMPR_VALUE1_SPEC>;
///Register `CMPR_VALUE1` writer
pub type W = crate::W<CMPR_VALUE1_SPEC>;
///Field `B` reader - PWM generator 0 time stamp B's shadow register
pub type B_R = crate::FieldReader<u16>;
///Field `B` writer - PWM generator 0 time stamp B's shadow register
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PWM generator 0 time stamp B's shadow register
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPR_VALUE1").field("b", &self.b()).finish()
    }
}
impl W {
    ///Bits 0:15 - PWM generator 0 time stamp B's shadow register
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<CMPR_VALUE1_SPEC> {
        B_W::new(self, 0)
    }
}
/**Shadow register for register B.

You can [`read`](crate::generic::Reg::read) this register and get [`cmpr_value1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr_value1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMPR_VALUE1_SPEC;
impl crate::RegisterSpec for CMPR_VALUE1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cmpr_value1::R`](R) reader structure
impl crate::Readable for CMPR_VALUE1_SPEC {}
///`write(|w| ..)` method takes [`cmpr_value1::W`](W) writer structure
impl crate::Writable for CMPR_VALUE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMPR_VALUE1 to value 0
impl crate::Resettable for CMPR_VALUE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
