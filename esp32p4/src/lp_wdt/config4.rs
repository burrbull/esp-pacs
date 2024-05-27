///Register `CONFIG4` reader
pub type R = crate::R<CONFIG4_SPEC>;
///Register `CONFIG4` writer
pub type W = crate::W<CONFIG4_SPEC>;
///Field `WDT_STG3_HOLD` reader - need_des
pub type WDT_STG3_HOLD_R = crate::FieldReader<u32>;
///Field `WDT_STG3_HOLD` writer - need_des
pub type WDT_STG3_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> WDT_STG3_HOLD_R {
        WDT_STG3_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG4")
            .field("wdt_stg3_hold", &self.wdt_stg3_hold())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3_hold(&mut self) -> WDT_STG3_HOLD_W<CONFIG4_SPEC> {
        WDT_STG3_HOLD_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`config4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONFIG4_SPEC;
impl crate::RegisterSpec for CONFIG4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`config4::R`](R) reader structure
impl crate::Readable for CONFIG4_SPEC {}
///`write(|w| ..)` method takes [`config4::W`](W) writer structure
impl crate::Writable for CONFIG4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONFIG4 to value 0x0fff
impl crate::Resettable for CONFIG4_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
