///Register `DSTATAR1` reader
pub type R = crate::R<DSTATAR1_SPEC>;
///Register `DSTATAR1` writer
pub type W = crate::W<DSTATAR1_SPEC>;
///Field `CH1_DSTATAR1` reader - NA
pub type CH1_DSTATAR1_R = crate::FieldReader<u32>;
///Field `CH1_DSTATAR1` writer - NA
pub type CH1_DSTATAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn ch1_dstatar1(&self) -> CH1_DSTATAR1_R {
        CH1_DSTATAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTATAR1").field("ch1_dstatar1", &self.ch1_dstatar1()).finish()
    }
}
impl W {
    ///Bits 0:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_dstatar1(&mut self) -> CH1_DSTATAR1_W<DSTATAR1_SPEC> {
        CH1_DSTATAR1_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`dstatar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstatar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DSTATAR1_SPEC;
impl crate::RegisterSpec for DSTATAR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dstatar1::R`](R) reader structure
impl crate::Readable for DSTATAR1_SPEC {}
///`write(|w| ..)` method takes [`dstatar1::W`](W) writer structure
impl crate::Writable for DSTATAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSTATAR1 to value 0
impl crate::Resettable for DSTATAR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
