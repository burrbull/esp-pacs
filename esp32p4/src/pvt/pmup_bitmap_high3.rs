#[doc = "Register `PMUP_BITMAP_HIGH3` reader"]
pub type R = crate::R<PMUP_BITMAP_HIGH3_SPEC>;
#[doc = "Register `PMUP_BITMAP_HIGH3` writer"]
pub type W = crate::W<PMUP_BITMAP_HIGH3_SPEC>;
#[doc = "Field `PUMP_BITMAP_HIGH3` reader - select valid high channel3"]
pub type PUMP_BITMAP_HIGH3_R = crate::FieldReader<u32>;
#[doc = "Field `PUMP_BITMAP_HIGH3` writer - select valid high channel3"]
pub type PUMP_BITMAP_HIGH3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - select valid high channel3"]
    #[inline(always)]
    pub fn pump_bitmap_high3(&self) -> PUMP_BITMAP_HIGH3_R {
        PUMP_BITMAP_HIGH3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_BITMAP_HIGH3")
            .field("pump_bitmap_high3", &self.pump_bitmap_high3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - select valid high channel3"]
    #[inline(always)]
    pub fn pump_bitmap_high3(&mut self) -> PUMP_BITMAP_HIGH3_W<PMUP_BITMAP_HIGH3_SPEC> {
        PUMP_BITMAP_HIGH3_W::new(self, 0)
    }
}
#[doc = "select valid pvt channel\n\nYou can [`read`](crate::Reg::read) this register and get [`pmup_bitmap_high3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmup_bitmap_high3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUP_BITMAP_HIGH3_SPEC;
impl crate::RegisterSpec for PMUP_BITMAP_HIGH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_bitmap_high3::R`](R) reader structure"]
impl crate::Readable for PMUP_BITMAP_HIGH3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmup_bitmap_high3::W`](W) writer structure"]
impl crate::Writable for PMUP_BITMAP_HIGH3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUP_BITMAP_HIGH3 to value 0"]
impl crate::Resettable for PMUP_BITMAP_HIGH3_SPEC {
    const RESET_VALUE: u32 = 0;
}
