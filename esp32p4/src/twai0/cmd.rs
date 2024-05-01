///Register `CMD` writer
pub type W = crate::W<CMD_SPEC>;
///Field `TX_REQUEST` writer - 1: present, a message shall be transmitted. 0: absent
pub type TX_REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT_TX` writer - 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent
pub type ABORT_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RELEASE_BUFFER` writer - 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action
pub type RELEASE_BUFFER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEAR_DATA_OVERRUN` writer - 1: clear, the data overrun status bit is cleared. 0: no action.
pub type CLEAR_DATA_OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SELF_RX_REQUEST` writer - 1: present, a message shall be transmitted and received simultaneously. 0: absent.
pub type SELF_RX_REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - 1: present, a message shall be transmitted. 0: absent
    #[inline(always)]
    #[must_use]
    pub fn tx_request(&mut self) -> TX_REQUEST_W<CMD_SPEC> {
        TX_REQUEST_W::new(self, 0)
    }
    ///Bit 1 - 1: present, if not already in progress, a pending transmission request is cancelled. 0: absent
    #[inline(always)]
    #[must_use]
    pub fn abort_tx(&mut self) -> ABORT_TX_W<CMD_SPEC> {
        ABORT_TX_W::new(self, 1)
    }
    ///Bit 2 - 1: released, the receive buffer, representing the message memory space in the RXFIFO is released. 0: no action
    #[inline(always)]
    #[must_use]
    pub fn release_buffer(&mut self) -> RELEASE_BUFFER_W<CMD_SPEC> {
        RELEASE_BUFFER_W::new(self, 2)
    }
    ///Bit 3 - 1: clear, the data overrun status bit is cleared. 0: no action.
    #[inline(always)]
    #[must_use]
    pub fn clear_data_overrun(&mut self) -> CLEAR_DATA_OVERRUN_W<CMD_SPEC> {
        CLEAR_DATA_OVERRUN_W::new(self, 3)
    }
    ///Bit 4 - 1: present, a message shall be transmitted and received simultaneously. 0: absent.
    #[inline(always)]
    #[must_use]
    pub fn self_rx_request(&mut self) -> SELF_RX_REQUEST_W<CMD_SPEC> {
        SELF_RX_REQUEST_W::new(self, 4)
    }
}
/**TWAI command register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
