///Register `APB_PERIPHERAL_ACCESS_0` reader
pub type R = crate::R<APB_PERIPHERAL_ACCESS_0_SPEC>;
///Register `APB_PERIPHERAL_ACCESS_0` writer
pub type W = crate::W<APB_PERIPHERAL_ACCESS_0_SPEC>;
///Field `APB_PERIPHERAL_ACCESS_LOCK` reader - Set 1 to lock APB peripheral Configuration Register.
pub type APB_PERIPHERAL_ACCESS_LOCK_R = crate::BitReader;
///Field `APB_PERIPHERAL_ACCESS_LOCK` writer - Set 1 to lock APB peripheral Configuration Register.
pub type APB_PERIPHERAL_ACCESS_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to lock APB peripheral Configuration Register.
    #[inline(always)]
    pub fn apb_peripheral_access_lock(&self) -> APB_PERIPHERAL_ACCESS_LOCK_R {
        APB_PERIPHERAL_ACCESS_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_ACCESS_0")
            .field("apb_peripheral_access_lock", &self.apb_peripheral_access_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to lock APB peripheral Configuration Register.
    #[inline(always)]
    #[must_use]
    pub fn apb_peripheral_access_lock(
        &mut self,
    ) -> APB_PERIPHERAL_ACCESS_LOCK_W<APB_PERIPHERAL_ACCESS_0_SPEC> {
        APB_PERIPHERAL_ACCESS_LOCK_W::new(self, 0)
    }
}
/**APB peripheral configuration register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_access_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_access_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB_PERIPHERAL_ACCESS_0_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_ACCESS_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb_peripheral_access_0::R`](R) reader structure
impl crate::Readable for APB_PERIPHERAL_ACCESS_0_SPEC {}
///`write(|w| ..)` method takes [`apb_peripheral_access_0::W`](W) writer structure
impl crate::Writable for APB_PERIPHERAL_ACCESS_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_PERIPHERAL_ACCESS_0 to value 0
impl crate::Resettable for APB_PERIPHERAL_ACCESS_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
