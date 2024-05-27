///Register `OUT` reader
pub type R = crate::R<OUT_SPEC>;
///Register `OUT` writer
pub type W = crate::W<OUT_SPEC>;
///Field `DATA` reader - GPIO0~17 output value
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - GPIO0~17 output value
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 14:31 - GPIO0~17 output value
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 14:31 - GPIO0~17 output value
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<OUT_SPEC> {
        DATA_W::new(self, 14)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out::R`](R) reader structure
impl crate::Readable for OUT_SPEC {}
///`write(|w| ..)` method takes [`out::W`](W) writer structure
impl crate::Writable for OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT to value 0
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
