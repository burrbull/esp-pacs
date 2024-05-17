///Register `FIB_SEL` reader
pub type R = crate::R<FIB_SEL_SPEC>;
///Register `FIB_SEL` writer
pub type W = crate::W<FIB_SEL_SPEC>;
///Field `FIB_SEL` reader - select use analog fib signal
pub type FIB_SEL_R = crate::FieldReader;
///Field `FIB_SEL` writer - select use analog fib signal
pub type FIB_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - select use analog fib signal
    #[inline(always)]
    pub fn fib_sel(&self) -> FIB_SEL_R {
        FIB_SEL_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIB_SEL").field("fib_sel", &self.fib_sel()).finish()
    }
}
impl W {
    ///Bits 0:2 - select use analog fib signal
    #[inline(always)]
    #[must_use]
    pub fn fib_sel(&mut self) -> FIB_SEL_W<FIB_SEL_SPEC> {
        FIB_SEL_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`fib_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fib_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIB_SEL_SPEC;
impl crate::RegisterSpec for FIB_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fib_sel::R`](R) reader structure
impl crate::Readable for FIB_SEL_SPEC {}
///`write(|w| ..)` method takes [`fib_sel::W`](W) writer structure
impl crate::Writable for FIB_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIB_SEL to value 0x07
impl crate::Resettable for FIB_SEL_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
