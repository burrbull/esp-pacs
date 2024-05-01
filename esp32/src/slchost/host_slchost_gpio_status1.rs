///Register `HOST_SLCHOST_GPIO_STATUS1` reader
pub type R = crate::R<HOST_SLCHOST_GPIO_STATUS1_SPEC>;
///Field `HOST_GPIO_SDIO_INT1` reader -
pub type HOST_GPIO_SDIO_INT1_R = crate::FieldReader;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn host_gpio_sdio_int1(&self) -> HOST_GPIO_SDIO_INT1_R {
        HOST_GPIO_SDIO_INT1_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_GPIO_STATUS1")
            .field("host_gpio_sdio_int1", &self.host_gpio_sdio_int1())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_GPIO_STATUS1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_GPIO_STATUS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_gpio_status1::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_GPIO_STATUS1_SPEC {}
///`reset()` method sets HOST_SLCHOST_GPIO_STATUS1 to value 0
impl crate::Resettable for HOST_SLCHOST_GPIO_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
