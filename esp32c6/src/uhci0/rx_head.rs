///Register `RX_HEAD` reader
pub type R = crate::R<RX_HEAD_SPEC>;
///Field `RX_HEAD` reader - Stores the head of received packet.
pub type RX_HEAD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the head of received packet.
    #[inline(always)]
    pub fn rx_head(&self) -> RX_HEAD_R {
        RX_HEAD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_HEAD")
            .field("rx_head", &self.rx_head())
            .finish()
    }
}
/**UHCI Head Register

You can [`read`](crate::generic::Reg::read) this register and get [`rx_head::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_HEAD_SPEC;
impl crate::RegisterSpec for RX_HEAD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_head::R`](R) reader structure
impl crate::Readable for RX_HEAD_SPEC {}
///`reset()` method sets RX_HEAD to value 0
impl crate::Resettable for RX_HEAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
