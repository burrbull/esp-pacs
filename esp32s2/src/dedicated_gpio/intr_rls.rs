///Register `INTR_RLS` reader
pub type R = crate::R<INTR_RLS_SPEC>;
///Register `INTR_RLS` writer
pub type W = crate::W<INTR_RLS_SPEC>;
///Field `GPIO(0-7)` reader - The enable bit for DEDIC_GPIO%s_INT_ST register.
pub type GPIO_R = crate::BitReader;
///Field `GPIO(0-7)` writer - The enable bit for DEDIC_GPIO%s_INT_ST register.
pub type GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///The enable bit for DEDIC_GPIO(0-7)_INT_ST register.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field
    #[inline(always)]
    pub fn gpio(&self, n: u8) -> GPIO_R {
        #[allow(clippy::no_effect)] [(); 8][n as usize];
        GPIO_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///The enable bit for DEDIC_GPIO(0-7)_INT_ST register.
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = GPIO_R> + '_ {
        (0..8).map(move |n| GPIO_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register.
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register.
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register.
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register.
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register.
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register.
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register.
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register.
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RLS")
            .field("gpio0", &self.gpio0())
            .field("gpio1", &self.gpio1())
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("gpio4", &self.gpio4())
            .field("gpio5", &self.gpio5())
            .field("gpio6", &self.gpio6())
            .field("gpio7", &self.gpio7())
            .finish()
    }
}
impl W {
    ///The enable bit for DEDIC_GPIO(0-7)_INT_ST register.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self, n: u8) -> GPIO_W<INTR_RLS_SPEC> {
        #[allow(clippy::no_effect)] [(); 8][n as usize];
        GPIO_W::new(self, n)
    }
    ///Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 0)
    }
    ///Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 1)
    }
    ///Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 2)
    }
    ///Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio3(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 3)
    }
    ///Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio4(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 4)
    }
    ///Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio5(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 5)
    }
    ///Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio6(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 6)
    }
    ///Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register.
    #[inline(always)]
    #[must_use]
    pub fn gpio7(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 7)
    }
}
/**Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`intr_rls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTR_RLS_SPEC;
impl crate::RegisterSpec for INTR_RLS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`intr_rls::R`](R) reader structure
impl crate::Readable for INTR_RLS_SPEC {}
///`write(|w| ..)` method takes [`intr_rls::W`](W) writer structure
impl crate::Writable for INTR_RLS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTR_RLS to value 0
impl crate::Resettable for INTR_RLS_SPEC {
    const RESET_VALUE: u32 = 0;
}
