///Register `IV_%s` reader
pub type R = crate::R<IV__SPEC>;
///Register `IV_%s` writer
pub type W = crate::W<IV__SPEC>;
///Field `IV` reader - Stores IV block data
pub type IV_R = crate::FieldReader<u32>;
///Field `IV` writer - Stores IV block data
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stores IV block data
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_").field("iv", &self.iv()).finish()
    }
}
impl W {
    ///Bits 0:31 - Stores IV block data
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IV__SPEC> {
        IV_W::new(self, 0)
    }
}
/**IV block data

You can [`read`](crate::generic::Reg::read) this register and get [`iv_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
///`read()` method returns [`iv_::R`](R) reader structure
impl crate::Readable for IV__SPEC {}
///`write(|w| ..)` method takes [`iv_::W`](W) writer structure
impl crate::Writable for IV__SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IV_%s to value 0
impl crate::Resettable for IV__SPEC {
    const RESET_VALUE: u32 = 0;
}
