///Register `INTERRUPT_REG_DATE` reader
pub type R = crate::R<INTERRUPT_REG_DATE_SPEC>;
///Register `INTERRUPT_REG_DATE` writer
pub type W = crate::W<INTERRUPT_REG_DATE_SPEC>;
///Field `CORE1_INTERRUPT_REG_DATE` reader - NA
pub type CORE1_INTERRUPT_REG_DATE_R = crate::FieldReader<u32>;
///Field `CORE1_INTERRUPT_REG_DATE` writer - NA
pub type CORE1_INTERRUPT_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:27 - NA
    #[inline(always)]
    pub fn core1_interrupt_reg_date(&self) -> CORE1_INTERRUPT_REG_DATE_R {
        CORE1_INTERRUPT_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_REG_DATE")
            .field("core1_interrupt_reg_date", &self.core1_interrupt_reg_date())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - NA
    #[inline(always)]
    #[must_use]
    pub fn core1_interrupt_reg_date(
        &mut self,
    ) -> CORE1_INTERRUPT_REG_DATE_W<INTERRUPT_REG_DATE_SPEC> {
        CORE1_INTERRUPT_REG_DATE_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTERRUPT_REG_DATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_REG_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`interrupt_reg_date::R`](R) reader structure
impl crate::Readable for INTERRUPT_REG_DATE_SPEC {}
///`write(|w| ..)` method takes [`interrupt_reg_date::W`](W) writer structure
impl crate::Writable for INTERRUPT_REG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTERRUPT_REG_DATE to value 0x0200_3020
impl crate::Resettable for INTERRUPT_REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0200_3020;
}
