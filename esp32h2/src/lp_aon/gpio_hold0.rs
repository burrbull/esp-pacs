///Register `GPIO_HOLD0` reader
pub type R = crate::R<GPIO_HOLD0_SPEC>;
///Register `GPIO_HOLD0` writer
pub type W = crate::W<GPIO_HOLD0_SPEC>;
///Field `GPIO_HOLD0` reader - need_des
pub type GPIO_HOLD0_R = crate::FieldReader<u32>;
///Field `GPIO_HOLD0` writer - need_des
pub type GPIO_HOLD0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn gpio_hold0(&self) -> GPIO_HOLD0_R {
        GPIO_HOLD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HOLD0")
            .field("gpio_hold0", &self.gpio_hold0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn gpio_hold0(&mut self) -> GPIO_HOLD0_W<GPIO_HOLD0_SPEC> {
        GPIO_HOLD0_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO_HOLD0_SPEC;
impl crate::RegisterSpec for GPIO_HOLD0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio_hold0::R`](R) reader structure
impl crate::Readable for GPIO_HOLD0_SPEC {}
///`write(|w| ..)` method takes [`gpio_hold0::W`](W) writer structure
impl crate::Writable for GPIO_HOLD0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO_HOLD0 to value 0
impl crate::Resettable for GPIO_HOLD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
