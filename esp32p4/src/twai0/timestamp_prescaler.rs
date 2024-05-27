///Register `TIMESTAMP_PRESCALER` reader
pub type R = crate::R<TIMESTAMP_PRESCALER_SPEC>;
///Register `TIMESTAMP_PRESCALER` writer
pub type W = crate::W<TIMESTAMP_PRESCALER_SPEC>;
///Field `TS_DIV_NUM` reader - Configures the clock division number of timestamp counter.
pub type TS_DIV_NUM_R = crate::FieldReader<u16>;
///Field `TS_DIV_NUM` writer - Configures the clock division number of timestamp counter.
pub type TS_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Configures the clock division number of timestamp counter.
    #[inline(always)]
    pub fn ts_div_num(&self) -> TS_DIV_NUM_R {
        TS_DIV_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMESTAMP_PRESCALER")
            .field("ts_div_num", &self.ts_div_num())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Configures the clock division number of timestamp counter.
    #[inline(always)]
    #[must_use]
    pub fn ts_div_num(&mut self) -> TS_DIV_NUM_W<TIMESTAMP_PRESCALER_SPEC> {
        TS_DIV_NUM_W::new(self, 0)
    }
}
/**Timestamp configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timestamp_prescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_prescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMESTAMP_PRESCALER_SPEC;
impl crate::RegisterSpec for TIMESTAMP_PRESCALER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timestamp_prescaler::R`](R) reader structure
impl crate::Readable for TIMESTAMP_PRESCALER_SPEC {}
///`write(|w| ..)` method takes [`timestamp_prescaler::W`](W) writer structure
impl crate::Writable for TIMESTAMP_PRESCALER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMESTAMP_PRESCALER to value 0x1f
impl crate::Resettable for TIMESTAMP_PRESCALER_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
