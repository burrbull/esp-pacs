#[doc = "Register `INTERRUPT_REG_DATE` reader"]
pub type R = crate::R<INTERRUPT_REG_DATE_SPEC>;
#[doc = "Register `INTERRUPT_REG_DATE` writer"]
pub type W = crate::W<INTERRUPT_REG_DATE_SPEC>;
#[doc = "Field `CORE0_INTERRUPT_REG_DATE` reader - NA"]
pub type CORE0_INTERRUPT_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `CORE0_INTERRUPT_REG_DATE` writer - NA"]
pub type CORE0_INTERRUPT_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - NA"]
    #[inline(always)]
    pub fn core0_interrupt_reg_date(&self) -> CORE0_INTERRUPT_REG_DATE_R {
        CORE0_INTERRUPT_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_REG_DATE")
            .field(
                "core0_interrupt_reg_date",
                &format_args!("{}", self.core0_interrupt_reg_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_REG_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn core0_interrupt_reg_date(
        &mut self,
    ) -> CORE0_INTERRUPT_REG_DATE_W<INTERRUPT_REG_DATE_SPEC> {
        CORE0_INTERRUPT_REG_DATE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_reg_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_reg_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_REG_DATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_reg_date::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_reg_date::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_REG_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPT_REG_DATE to value 0x0200_3020"]
impl crate::Resettable for INTERRUPT_REG_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_3020;
}