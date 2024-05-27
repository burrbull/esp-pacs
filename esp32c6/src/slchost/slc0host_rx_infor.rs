///Register `SLC0HOST_RX_INFOR` reader
pub type R = crate::R<SLC0HOST_RX_INFOR_SPEC>;
///Register `SLC0HOST_RX_INFOR` writer
pub type W = crate::W<SLC0HOST_RX_INFOR_SPEC>;
///Field `SLC0HOST_RX_INFOR` reader - *******Description***********
pub type SLC0HOST_RX_INFOR_R = crate::FieldReader<u32>;
///Field `SLC0HOST_RX_INFOR` writer - *******Description***********
pub type SLC0HOST_RX_INFOR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - *******Description***********
    #[inline(always)]
    pub fn slc0host_rx_infor(&self) -> SLC0HOST_RX_INFOR_R {
        SLC0HOST_RX_INFOR_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0HOST_RX_INFOR")
            .field("slc0host_rx_infor", &self.slc0host_rx_infor())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slc0host_rx_infor(&mut self) -> SLC0HOST_RX_INFOR_W<SLC0HOST_RX_INFOR_SPEC> {
        SLC0HOST_RX_INFOR_W::new(self, 0)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0host_rx_infor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0host_rx_infor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0HOST_RX_INFOR_SPEC;
impl crate::RegisterSpec for SLC0HOST_RX_INFOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slc0host_rx_infor::R`](R) reader structure
impl crate::Readable for SLC0HOST_RX_INFOR_SPEC {}
///`write(|w| ..)` method takes [`slc0host_rx_infor::W`](W) writer structure
impl crate::Writable for SLC0HOST_RX_INFOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLC0HOST_RX_INFOR to value 0
impl crate::Resettable for SLC0HOST_RX_INFOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
