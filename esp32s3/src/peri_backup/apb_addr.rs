///Register `APB_ADDR` reader
pub type R = crate::R<APB_ADDR_SPEC>;
///Register `APB_ADDR` writer
pub type W = crate::W<APB_ADDR_SPEC>;
///Field `APB_START_ADDR` reader - x
pub type APB_START_ADDR_R = crate::FieldReader<u32>;
///Field `APB_START_ADDR` writer - x
pub type APB_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - x
    #[inline(always)]
    pub fn apb_start_addr(&self) -> APB_START_ADDR_R {
        APB_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_ADDR")
            .field("apb_start_addr", &self.apb_start_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - x
    #[inline(always)]
    #[must_use]
    pub fn apb_start_addr(&mut self) -> APB_START_ADDR_W<APB_ADDR_SPEC> {
        APB_START_ADDR_W::new(self, 0)
    }
}
/**x

You can [`read`](crate::generic::Reg::read) this register and get [`apb_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB_ADDR_SPEC;
impl crate::RegisterSpec for APB_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb_addr::R`](R) reader structure
impl crate::Readable for APB_ADDR_SPEC {}
///`write(|w| ..)` method takes [`apb_addr::W`](W) writer structure
impl crate::Writable for APB_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_ADDR to value 0
impl crate::Resettable for APB_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
