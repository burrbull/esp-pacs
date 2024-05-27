///Register `M_PRIME` reader
pub type R = crate::R<M_PRIME_SPEC>;
///Register `M_PRIME` writer
pub type W = crate::W<M_PRIME_SPEC>;
///Field `M_PRIME` reader - Those bits stores m'
pub type M_PRIME_R = crate::FieldReader<u32>;
///Field `M_PRIME` writer - Those bits stores m'
pub type M_PRIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Those bits stores m'
    #[inline(always)]
    pub fn m_prime(&self) -> M_PRIME_R {
        M_PRIME_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M_PRIME")
            .field("m_prime", &self.m_prime())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Those bits stores m'
    #[inline(always)]
    #[must_use]
    pub fn m_prime(&mut self) -> M_PRIME_W<M_PRIME_SPEC> {
        M_PRIME_W::new(self, 0)
    }
}
/**RSA M_prime register

You can [`read`](crate::generic::Reg::read) this register and get [`m_prime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_prime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct M_PRIME_SPEC;
impl crate::RegisterSpec for M_PRIME_SPEC {
    type Ux = u32;
}
///`read()` method returns [`m_prime::R`](R) reader structure
impl crate::Readable for M_PRIME_SPEC {}
///`write(|w| ..)` method takes [`m_prime::W`](W) writer structure
impl crate::Writable for M_PRIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M_PRIME to value 0
impl crate::Resettable for M_PRIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
