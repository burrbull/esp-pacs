///Register `T_STRING` reader
pub type R = crate::R<T_STRING_SPEC>;
///Register `T_STRING` writer
pub type W = crate::W<T_STRING_SPEC>;
///Field `T_STRING` reader - Defines t_string for calculating the initial Hash value for SHA-512/t.
pub type T_STRING_R = crate::FieldReader<u32>;
///Field `T_STRING` writer - Defines t_string for calculating the initial Hash value for SHA-512/t.
pub type T_STRING_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Defines t_string for calculating the initial Hash value for SHA-512/t.
    #[inline(always)]
    pub fn t_string(&self) -> T_STRING_R {
        T_STRING_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T_STRING").field("t_string", &self.t_string()).finish()
    }
}
impl W {
    ///Bits 0:31 - Defines t_string for calculating the initial Hash value for SHA-512/t.
    #[inline(always)]
    #[must_use]
    pub fn t_string(&mut self) -> T_STRING_W<T_STRING_SPEC> {
        T_STRING_W::new(self, 0)
    }
}
/**String content register for calculating initial Hash Value (only effective for SHA-512/t)

You can [`read`](crate::generic::Reg::read) this register and get [`t_string::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_string::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct T_STRING_SPEC;
impl crate::RegisterSpec for T_STRING_SPEC {
    type Ux = u32;
}
///`read()` method returns [`t_string::R`](R) reader structure
impl crate::Readable for T_STRING_SPEC {}
///`write(|w| ..)` method takes [`t_string::W`](W) writer structure
impl crate::Writable for T_STRING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets T_STRING to value 0
impl crate::Resettable for T_STRING_SPEC {
    const RESET_VALUE: u32 = 0;
}
