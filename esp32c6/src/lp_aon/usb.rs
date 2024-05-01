///Register `USB` reader
pub type R = crate::R<USB_SPEC>;
///Register `USB` writer
pub type W = crate::W<USB_SPEC>;
///Field `RESET_DISABLE` reader - need_des
pub type RESET_DISABLE_R = crate::BitReader;
///Field `RESET_DISABLE` writer - need_des
pub type RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn reset_disable(&self) -> RESET_DISABLE_R {
        RESET_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB")
            .field("reset_disable", &self.reset_disable())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn reset_disable(&mut self) -> RESET_DISABLE_W<USB_SPEC> {
        RESET_DISABLE_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`usb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USB_SPEC;
impl crate::RegisterSpec for USB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`usb::R`](R) reader structure
impl crate::Readable for USB_SPEC {}
///`write(|w| ..)` method takes [`usb::W`](W) writer structure
impl crate::Writable for USB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USB to value 0
impl crate::Resettable for USB_SPEC {
    const RESET_VALUE: u32 = 0;
}
