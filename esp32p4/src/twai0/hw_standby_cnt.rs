///Register `HW_STANDBY_CNT` reader
pub type R = crate::R<HW_STANDBY_CNT_SPEC>;
///Register `HW_STANDBY_CNT` writer
pub type W = crate::W<HW_STANDBY_CNT_SPEC>;
///Field `STANDBY_WAIT_CNT` reader - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
pub type STANDBY_WAIT_CNT_R = crate::FieldReader<u32>;
///Field `STANDBY_WAIT_CNT` writer - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
pub type STANDBY_WAIT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    #[inline(always)]
    pub fn standby_wait_cnt(&self) -> STANDBY_WAIT_CNT_R {
        STANDBY_WAIT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_STANDBY_CNT")
            .field("standby_wait_cnt", &self.standby_wait_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Configure the number of cycles before standby becomes high when TWAI_HW_STANDBY_EN is enabled.
    #[inline(always)]
    #[must_use]
    pub fn standby_wait_cnt(&mut self) -> STANDBY_WAIT_CNT_W<HW_STANDBY_CNT_SPEC> {
        STANDBY_WAIT_CNT_W::new(self, 0)
    }
}
/**Configure standby counter.

You can [`read`](crate::generic::Reg::read) this register and get [`hw_standby_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_standby_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HW_STANDBY_CNT_SPEC;
impl crate::RegisterSpec for HW_STANDBY_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hw_standby_cnt::R`](R) reader structure
impl crate::Readable for HW_STANDBY_CNT_SPEC {}
///`write(|w| ..)` method takes [`hw_standby_cnt::W`](W) writer structure
impl crate::Writable for HW_STANDBY_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HW_STANDBY_CNT to value 0x01
impl crate::Resettable for HW_STANDBY_CNT_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
