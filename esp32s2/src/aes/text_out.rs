///Register `TEXT_OUT[%s]` reader
pub type R = crate::R<TEXT_OUT_SPEC>;
///Register `TEXT_OUT[%s]` writer
pub type W = crate::W<TEXT_OUT_SPEC>;
///Field `TEXT_OUT` reader - Stores the result data when the AES Accelerator operates in the Typical AES working mode.
pub type TEXT_OUT_R = crate::FieldReader<u32>;
///Field `TEXT_OUT` writer - Stores the result data when the AES Accelerator operates in the Typical AES working mode.
pub type TEXT_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stores the result data when the AES Accelerator operates in the Typical AES working mode.
    #[inline(always)]
    pub fn text_out(&self) -> TEXT_OUT_R {
        TEXT_OUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_OUT")
            .field("text_out", &self.text_out())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stores the result data when the AES Accelerator operates in the Typical AES working mode.
    #[inline(always)]
    #[must_use]
    pub fn text_out(&mut self) -> TEXT_OUT_W<TEXT_OUT_SPEC> {
        TEXT_OUT_W::new(self, 0)
    }
}
/**Result data register %s

You can [`read`](crate::generic::Reg::read) this register and get [`text_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXT_OUT_SPEC;
impl crate::RegisterSpec for TEXT_OUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`text_out::R`](R) reader structure
impl crate::Readable for TEXT_OUT_SPEC {}
///`write(|w| ..)` method takes [`text_out::W`](W) writer structure
impl crate::Writable for TEXT_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TEXT_OUT[%s] to value 0
impl crate::Resettable for TEXT_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
