///Register `MODE` reader
pub type R = crate::R<MODE_SPEC>;
///Register `MODE` writer
pub type W = crate::W<MODE_SPEC>;
///Field `MODE` reader - This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE").field("mode", &self.mode()).finish()
    }
}
impl W {
    ///Bits 0:2 - This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<MODE_SPEC> {
        MODE_W::new(self, 0)
    }
}
/**AES Mode register

You can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mode::R`](R) reader structure
impl crate::Readable for MODE_SPEC {}
///`write(|w| ..)` method takes [`mode::W`](W) writer structure
impl crate::Writable for MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODE to value 0
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
