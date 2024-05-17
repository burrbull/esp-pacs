///Register `CH%sDATA` reader
pub type R = crate::R<CHDATA_SPEC>;
///Register `CH%sDATA` writer
pub type W = crate::W<CHDATA_SPEC>;
///Field `DATA` reader - Read and write data for channel %s via APB FIFO.
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - Read and write data for channel %s via APB FIFO.
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHDATA").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - Read and write data for channel %s via APB FIFO.
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<CHDATA_SPEC> {
        DATA_W::new(self, 0)
    }
}
/**The read and write data register for CHANNEL%s by apb fifo access.

You can [`read`](crate::generic::Reg::read) this register and get [`chdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CHDATA_SPEC;
impl crate::RegisterSpec for CHDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`chdata::R`](R) reader structure
impl crate::Readable for CHDATA_SPEC {}
///`write(|w| ..)` method takes [`chdata::W`](W) writer structure
impl crate::Writable for CHDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH%sDATA to value 0
impl crate::Resettable for CHDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
