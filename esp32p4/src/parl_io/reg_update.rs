///Register `REG_UPDATE` writer
pub type W = crate::W<REG_UPDATE_SPEC>;
///Field `RX_REG_UPDATE` writer - Set this bit to update rx register configuration.
pub type RX_REG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 31 - Set this bit to update rx register configuration.
    #[inline(always)]
    #[must_use]
    pub fn rx_reg_update(&mut self) -> RX_REG_UPDATE_W<REG_UPDATE_SPEC> {
        RX_REG_UPDATE_W::new(self, 31)
    }
}
/**Parallel IO FIFO configuration register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REG_UPDATE_SPEC;
impl crate::RegisterSpec for REG_UPDATE_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`reg_update::W`](W) writer structure
impl crate::Writable for REG_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REG_UPDATE to value 0
impl crate::Resettable for REG_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
