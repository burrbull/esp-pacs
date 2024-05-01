///Register `CH%s_TX_STATUS` reader
pub type R = crate::R<CH_TX_STATUS_SPEC>;
///Field `MEM_RADDR_EX` reader - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
pub type MEM_RADDR_EX_R = crate::FieldReader<u16>;
///Field `STATE` reader - This register records the FSM status of CHANNEL%s.
pub type STATE_R = crate::FieldReader;
///Field `APB_MEM_WADDR` reader - This register records the memory address offset when writes RAM over APB bus.
pub type APB_MEM_WADDR_R = crate::FieldReader<u16>;
///Field `APB_MEM_RD_ERR` reader - This status bit will be set if the offset address out of memory size when reading via APB bus.
pub type APB_MEM_RD_ERR_R = crate::BitReader;
///Field `MEM_EMPTY` reader - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
pub type MEM_EMPTY_R = crate::BitReader;
///Field `APB_MEM_WR_ERR` reader - This status bit will be set if the offset address out of memory size when writes via APB bus.
pub type APB_MEM_WR_ERR_R = crate::BitReader;
///Field `APB_MEM_RADDR` reader - This register records the memory address offset when reading RAM over APB bus.
pub type APB_MEM_RADDR_R = crate::FieldReader;
impl R {
    ///Bits 0:8 - This register records the memory address offset when transmitter of CHANNEL%s is using the RAM.
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:11 - This register records the FSM status of CHANNEL%s.
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:20 - This register records the memory address offset when writes RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_waddr(&self) -> APB_MEM_WADDR_R {
        APB_MEM_WADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    ///Bit 21 - This status bit will be set if the offset address out of memory size when reading via APB bus.
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - This status bit will be set when the data to be set is more than memory size and the wraparound mode is disabled.
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - This status bit will be set if the offset address out of memory size when writes via APB bus.
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - This register records the memory address offset when reading RAM over APB bus.
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_STATUS")
            .field("mem_raddr_ex", &self.mem_raddr_ex())
            .field("state", &self.state())
            .field("apb_mem_waddr", &self.apb_mem_waddr())
            .field("apb_mem_rd_err", &self.apb_mem_rd_err())
            .field("mem_empty", &self.mem_empty())
            .field("apb_mem_wr_err", &self.apb_mem_wr_err())
            .field("apb_mem_raddr", &self.apb_mem_raddr())
            .finish()
    }
}
/**Channel %s status register

You can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_TX_STATUS_SPEC;
impl crate::RegisterSpec for CH_TX_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ch_tx_status::R`](R) reader structure
impl crate::Readable for CH_TX_STATUS_SPEC {}
///`reset()` method sets CH%s_TX_STATUS to value 0
impl crate::Resettable for CH_TX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
