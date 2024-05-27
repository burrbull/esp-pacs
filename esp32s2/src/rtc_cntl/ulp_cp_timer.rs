///Register `ULP_CP_TIMER` reader
pub type R = crate::R<ULP_CP_TIMER_SPEC>;
///Register `ULP_CP_TIMER` writer
pub type W = crate::W<ULP_CP_TIMER_SPEC>;
///Field `ULP_CP_PC_INIT` reader - ULP coprocessor PC initial address
pub type ULP_CP_PC_INIT_R = crate::FieldReader<u16>;
///Field `ULP_CP_PC_INIT` writer - ULP coprocessor PC initial address
pub type ULP_CP_PC_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `ULP_CP_GPIO_WAKEUP_ENA` reader - Enable the option of ULP coprocessor woken up by RTC GPIO
pub type ULP_CP_GPIO_WAKEUP_ENA_R = crate::BitReader;
///Field `ULP_CP_GPIO_WAKEUP_ENA` writer - Enable the option of ULP coprocessor woken up by RTC GPIO
pub type ULP_CP_GPIO_WAKEUP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULP_CP_GPIO_WAKEUP_CLR` writer - Disable the option of ULP coprocessor woken up by RTC GPIO
pub type ULP_CP_GPIO_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULP_CP_SLP_TIMER_EN` reader - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer
pub type ULP_CP_SLP_TIMER_EN_R = crate::BitReader;
///Field `ULP_CP_SLP_TIMER_EN` writer - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer
pub type ULP_CP_SLP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - ULP coprocessor PC initial address
    #[inline(always)]
    pub fn ulp_cp_pc_init(&self) -> ULP_CP_PC_INIT_R {
        ULP_CP_PC_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 29 - Enable the option of ULP coprocessor woken up by RTC GPIO
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&self) -> ULP_CP_GPIO_WAKEUP_ENA_R {
        ULP_CP_GPIO_WAKEUP_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_TIMER")
            .field("ulp_cp_pc_init", &self.ulp_cp_pc_init())
            .field("ulp_cp_gpio_wakeup_ena", &self.ulp_cp_gpio_wakeup_ena())
            .field("ulp_cp_slp_timer_en", &self.ulp_cp_slp_timer_en())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - ULP coprocessor PC initial address
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_pc_init(&mut self) -> ULP_CP_PC_INIT_W<ULP_CP_TIMER_SPEC> {
        ULP_CP_PC_INIT_W::new(self, 0)
    }
    ///Bit 29 - Enable the option of ULP coprocessor woken up by RTC GPIO
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_gpio_wakeup_ena(&mut self) -> ULP_CP_GPIO_WAKEUP_ENA_W<ULP_CP_TIMER_SPEC> {
        ULP_CP_GPIO_WAKEUP_ENA_W::new(self, 29)
    }
    ///Bit 30 - Disable the option of ULP coprocessor woken up by RTC GPIO
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_gpio_wakeup_clr(&mut self) -> ULP_CP_GPIO_WAKEUP_CLR_W<ULP_CP_TIMER_SPEC> {
        ULP_CP_GPIO_WAKEUP_CLR_W::new(self, 30)
    }
    ///Bit 31 - ULP coprocessor timer enable bit. 0: Disable hardware Timer. 1: Enable hardware timer
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W<ULP_CP_TIMER_SPEC> {
        ULP_CP_SLP_TIMER_EN_W::new(self, 31)
    }
}
/**Configure coprocessor timer

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ULP_CP_TIMER_SPEC;
impl crate::RegisterSpec for ULP_CP_TIMER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ulp_cp_timer::R`](R) reader structure
impl crate::Readable for ULP_CP_TIMER_SPEC {}
///`write(|w| ..)` method takes [`ulp_cp_timer::W`](W) writer structure
impl crate::Writable for ULP_CP_TIMER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ULP_CP_TIMER to value 0
impl crate::Resettable for ULP_CP_TIMER_SPEC {
    const RESET_VALUE: u32 = 0;
}
