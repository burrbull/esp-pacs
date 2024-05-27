///Register `SSTATAR1` reader
pub type R = crate::R<SSTATAR1_SPEC>;
///Register `SSTATAR1` writer
pub type W = crate::W<SSTATAR1_SPEC>;
///Field `CH1_SSTATAR1` reader - NA
pub type CH1_SSTATAR1_R = crate::FieldReader<u32>;
///Field `CH1_SSTATAR1` writer - NA
pub type CH1_SSTATAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn ch1_sstatar1(&self) -> CH1_SSTATAR1_R {
        CH1_SSTATAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSTATAR1")
            .field("ch1_sstatar1", &self.ch1_sstatar1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_sstatar1(&mut self) -> CH1_SSTATAR1_W<SSTATAR1_SPEC> {
        CH1_SSTATAR1_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`sstatar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSTATAR1_SPEC;
impl crate::RegisterSpec for SSTATAR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sstatar1::R`](R) reader structure
impl crate::Readable for SSTATAR1_SPEC {}
///`write(|w| ..)` method takes [`sstatar1::W`](W) writer structure
impl crate::Writable for SSTATAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SSTATAR1 to value 0
impl crate::Resettable for SSTATAR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
