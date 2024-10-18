#[doc = "Register `T_LENGTH` reader"]
pub type R = crate::R<T_LENGTH_SPEC>;
#[doc = "Register `T_LENGTH` writer"]
pub type W = crate::W<T_LENGTH_SPEC>;
#[doc = "Field `T_LENGTH` reader - Defines t_length for calculating the initial Hash value for SHA-512/t."]
pub type T_LENGTH_R = crate::FieldReader;
#[doc = "Field `T_LENGTH` writer - Defines t_length for calculating the initial Hash value for SHA-512/t."]
pub type T_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Defines t_length for calculating the initial Hash value for SHA-512/t."]
    #[inline(always)]
    pub fn t_length(&self) -> T_LENGTH_R {
        T_LENGTH_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T_LENGTH")
            .field("t_length", &self.t_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Defines t_length for calculating the initial Hash value for SHA-512/t."]
    #[inline(always)]
    pub fn t_length(&mut self) -> T_LENGTH_W<T_LENGTH_SPEC> {
        T_LENGTH_W::new(self, 0)
    }
}
#[doc = "String length register for calculating initial Hash Value (only effective for SHA-512/t)\n\nYou can [`read`](crate::Reg::read) this register and get [`t_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_LENGTH_SPEC;
impl crate::RegisterSpec for T_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t_length::R`](R) reader structure"]
impl crate::Readable for T_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t_length::W`](W) writer structure"]
impl crate::Writable for T_LENGTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T_LENGTH to value 0"]
impl crate::Resettable for T_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
