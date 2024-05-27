///Register `STATUS` reader
pub type R = crate::R<STATUS_SPEC>;
///Register `STATUS` writer
pub type W = crate::W<STATUS_SPEC>;
///Field `INT` reader - GPIO0~17 interrupt status
pub type INT_R = crate::FieldReader<u32>;
///Field `INT` writer - GPIO0~17 interrupt status
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 14:31 - GPIO0~17 interrupt status
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS").field("int", &self.int()).finish()
    }
}
impl W {
    ///Bits 14:31 - GPIO0~17 interrupt status
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<STATUS_SPEC> {
        INT_W::new(self, 14)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUS_SPEC {}
///`write(|w| ..)` method takes [`status::W`](W) writer structure
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
