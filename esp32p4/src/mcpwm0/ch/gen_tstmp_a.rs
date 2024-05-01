///Register `GEN_TSTMP_A` reader
pub type R = crate::R<GEN_TSTMP_A_SPEC>;
///Register `GEN_TSTMP_A` writer
pub type W = crate::W<GEN_TSTMP_A_SPEC>;
///Field `A` reader - Configures the value of PWM generator %s time stamp A's shadow register.
pub type A_R = crate::FieldReader<u16>;
///Field `A` writer - Configures the value of PWM generator %s time stamp A's shadow register.
pub type A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Configures the value of PWM generator %s time stamp A's shadow register.
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_TSTMP_A").field("a", &self.a()).finish()
    }
}
impl W {
    ///Bits 0:15 - Configures the value of PWM generator %s time stamp A's shadow register.
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<GEN_TSTMP_A_SPEC> {
        A_W::new(self, 0)
    }
}
/**Generator0 time stamp A's shadow register

You can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GEN_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN_TSTMP_A_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gen_tstmp_a::R`](R) reader structure
impl crate::Readable for GEN_TSTMP_A_SPEC {}
///`write(|w| ..)` method takes [`gen_tstmp_a::W`](W) writer structure
impl crate::Writable for GEN_TSTMP_A_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GEN_TSTMP_A to value 0
impl crate::Resettable for GEN_TSTMP_A_SPEC {
    const RESET_VALUE: u32 = 0;
}
