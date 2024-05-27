///Register `HOST_INF_SEL` reader
pub type R = crate::R<HOST_INF_SEL_SPEC>;
///Register `HOST_INF_SEL` writer
pub type W = crate::W<HOST_INF_SEL_SPEC>;
///Field `PERI_IO_SWAP` reader -
pub type PERI_IO_SWAP_R = crate::FieldReader;
///Field `PERI_IO_SWAP` writer -
pub type PERI_IO_SWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LINK_DEVICE_SEL` reader -
pub type LINK_DEVICE_SEL_R = crate::FieldReader;
///Field `LINK_DEVICE_SEL` writer -
pub type LINK_DEVICE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn peri_io_swap(&self) -> PERI_IO_SWAP_R {
        PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn link_device_sel(&self) -> LINK_DEVICE_SEL_R {
        LINK_DEVICE_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_INF_SEL")
            .field("peri_io_swap", &self.peri_io_swap())
            .field("link_device_sel", &self.link_device_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn peri_io_swap(&mut self) -> PERI_IO_SWAP_W<HOST_INF_SEL_SPEC> {
        PERI_IO_SWAP_W::new(self, 0)
    }
    ///Bits 8:15
    #[inline(always)]
    #[must_use]
    pub fn link_device_sel(&mut self) -> LINK_DEVICE_SEL_W<HOST_INF_SEL_SPEC> {
        LINK_DEVICE_SEL_W::new(self, 8)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_inf_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_inf_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_INF_SEL_SPEC;
impl crate::RegisterSpec for HOST_INF_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_inf_sel::R`](R) reader structure
impl crate::Readable for HOST_INF_SEL_SPEC {}
///`write(|w| ..)` method takes [`host_inf_sel::W`](W) writer structure
impl crate::Writable for HOST_INF_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_INF_SEL to value 0
impl crate::Resettable for HOST_INF_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
