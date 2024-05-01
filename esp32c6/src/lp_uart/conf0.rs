///Register `CONF0` reader
pub type R = crate::R<CONF0_SPEC>;
///Register `CONF0` writer
pub type W = crate::W<CONF0_SPEC>;
///Field `PARITY` reader - This register is used to configure the parity check mode.
pub type PARITY_R = crate::BitReader;
///Field `PARITY` writer - This register is used to configure the parity check mode.
pub type PARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PARITY_EN` reader - Set this bit to enable uart parity check.
pub type PARITY_EN_R = crate::BitReader;
///Field `PARITY_EN` writer - Set this bit to enable uart parity check.
pub type PARITY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIT_NUM` reader - This register is used to set the length of data.
pub type BIT_NUM_R = crate::FieldReader;
///Field `BIT_NUM` writer - This register is used to set the length of data.
pub type BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STOP_BIT_NUM` reader - This register is used to set the length of stop bit.
pub type STOP_BIT_NUM_R = crate::FieldReader;
///Field `STOP_BIT_NUM` writer - This register is used to set the length of stop bit.
pub type STOP_BIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TXD_BRK` reader - Set this bit to enbale transmitter to send NULL when the process of sending data is done.
pub type TXD_BRK_R = crate::BitReader;
///Field `TXD_BRK` writer - Set this bit to enbale transmitter to send NULL when the process of sending data is done.
pub type TXD_BRK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOOPBACK` reader - Set this bit to enable uart loopback test mode.
pub type LOOPBACK_R = crate::BitReader;
///Field `LOOPBACK` writer - Set this bit to enable uart loopback test mode.
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_FLOW_EN` reader - Set this bit to enable flow control function for transmitter.
pub type TX_FLOW_EN_R = crate::BitReader;
///Field `TX_FLOW_EN` writer - Set this bit to enable flow control function for transmitter.
pub type TX_FLOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXD_INV` reader - Set this bit to inverse the level value of uart rxd signal.
pub type RXD_INV_R = crate::BitReader;
///Field `RXD_INV` writer - Set this bit to inverse the level value of uart rxd signal.
pub type RXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXD_INV` reader - Set this bit to inverse the level value of uart txd signal.
pub type TXD_INV_R = crate::BitReader;
///Field `TXD_INV` writer - Set this bit to inverse the level value of uart txd signal.
pub type TXD_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_RX_DAT_OVF` reader - Disable UART Rx data overflow detect.
pub type DIS_RX_DAT_OVF_R = crate::BitReader;
///Field `DIS_RX_DAT_OVF` writer - Disable UART Rx data overflow detect.
pub type DIS_RX_DAT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_WR_MASK` reader - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
pub type ERR_WR_MASK_R = crate::BitReader;
///Field `ERR_WR_MASK` writer - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
pub type ERR_WR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_CLK_EN` reader - UART memory clock gate enable signal.
pub type MEM_CLK_EN_R = crate::BitReader;
///Field `MEM_CLK_EN` writer - UART memory clock gate enable signal.
pub type MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW_RTS` reader - This register is used to configure the software rts signal which is used in software flow control.
pub type SW_RTS_R = crate::BitReader;
///Field `SW_RTS` writer - This register is used to configure the software rts signal which is used in software flow control.
pub type SW_RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFO_RST` reader - Set this bit to reset the uart receive-FIFO.
pub type RXFIFO_RST_R = crate::BitReader;
///Field `RXFIFO_RST` writer - Set this bit to reset the uart receive-FIFO.
pub type RXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFO_RST` reader - Set this bit to reset the uart transmit-FIFO.
pub type TXFIFO_RST_R = crate::BitReader;
///Field `TXFIFO_RST` writer - Set this bit to reset the uart transmit-FIFO.
pub type TXFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This register is used to configure the parity check mode.
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to enable uart parity check.
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - This register is used to set the length of data.
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - This register is used to set the length of stop bit.
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Set this bit to enbale transmitter to send NULL when the process of sending data is done.
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - Set this bit to enable uart loopback test mode.
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Set this bit to enable flow control function for transmitter.
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Set this bit to inverse the level value of uart rxd signal.
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Set this bit to inverse the level value of uart txd signal.
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Disable UART Rx data overflow detect.
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&self) -> DIS_RX_DAT_OVF_R {
        DIS_RX_DAT_OVF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - UART memory clock gate enable signal.
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - This register is used to configure the software rts signal which is used in software flow control.
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Set this bit to reset the uart receive-FIFO.
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Set this bit to reset the uart transmit-FIFO.
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("parity", &self.parity())
            .field("parity_en", &self.parity_en())
            .field("bit_num", &self.bit_num())
            .field("stop_bit_num", &self.stop_bit_num())
            .field("txd_brk", &self.txd_brk())
            .field("loopback", &self.loopback())
            .field("tx_flow_en", &self.tx_flow_en())
            .field("rxd_inv", &self.rxd_inv())
            .field("txd_inv", &self.txd_inv())
            .field("dis_rx_dat_ovf", &self.dis_rx_dat_ovf())
            .field("err_wr_mask", &self.err_wr_mask())
            .field("mem_clk_en", &self.mem_clk_en())
            .field("sw_rts", &self.sw_rts())
            .field("rxfifo_rst", &self.rxfifo_rst())
            .field("txfifo_rst", &self.txfifo_rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - This register is used to configure the parity check mode.
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<CONF0_SPEC> {
        PARITY_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to enable uart parity check.
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> PARITY_EN_W<CONF0_SPEC> {
        PARITY_EN_W::new(self, 1)
    }
    ///Bits 2:3 - This register is used to set the length of data.
    #[inline(always)]
    #[must_use]
    pub fn bit_num(&mut self) -> BIT_NUM_W<CONF0_SPEC> {
        BIT_NUM_W::new(self, 2)
    }
    ///Bits 4:5 - This register is used to set the length of stop bit.
    #[inline(always)]
    #[must_use]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W<CONF0_SPEC> {
        STOP_BIT_NUM_W::new(self, 4)
    }
    ///Bit 6 - Set this bit to enbale transmitter to send NULL when the process of sending data is done.
    #[inline(always)]
    #[must_use]
    pub fn txd_brk(&mut self) -> TXD_BRK_W<CONF0_SPEC> {
        TXD_BRK_W::new(self, 6)
    }
    ///Bit 12 - Set this bit to enable uart loopback test mode.
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<CONF0_SPEC> {
        LOOPBACK_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to enable flow control function for transmitter.
    #[inline(always)]
    #[must_use]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W<CONF0_SPEC> {
        TX_FLOW_EN_W::new(self, 13)
    }
    ///Bit 15 - Set this bit to inverse the level value of uart rxd signal.
    #[inline(always)]
    #[must_use]
    pub fn rxd_inv(&mut self) -> RXD_INV_W<CONF0_SPEC> {
        RXD_INV_W::new(self, 15)
    }
    ///Bit 16 - Set this bit to inverse the level value of uart txd signal.
    #[inline(always)]
    #[must_use]
    pub fn txd_inv(&mut self) -> TXD_INV_W<CONF0_SPEC> {
        TXD_INV_W::new(self, 16)
    }
    ///Bit 17 - Disable UART Rx data overflow detect.
    #[inline(always)]
    #[must_use]
    pub fn dis_rx_dat_ovf(&mut self) -> DIS_RX_DAT_OVF_W<CONF0_SPEC> {
        DIS_RX_DAT_OVF_W::new(self, 17)
    }
    ///Bit 18 - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong.
    #[inline(always)]
    #[must_use]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W<CONF0_SPEC> {
        ERR_WR_MASK_W::new(self, 18)
    }
    ///Bit 20 - UART memory clock gate enable signal.
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W<CONF0_SPEC> {
        MEM_CLK_EN_W::new(self, 20)
    }
    ///Bit 21 - This register is used to configure the software rts signal which is used in software flow control.
    #[inline(always)]
    #[must_use]
    pub fn sw_rts(&mut self) -> SW_RTS_W<CONF0_SPEC> {
        SW_RTS_W::new(self, 21)
    }
    ///Bit 22 - Set this bit to reset the uart receive-FIFO.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<CONF0_SPEC> {
        RXFIFO_RST_W::new(self, 22)
    }
    ///Bit 23 - Set this bit to reset the uart transmit-FIFO.
    #[inline(always)]
    #[must_use]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W<CONF0_SPEC> {
        TXFIFO_RST_W::new(self, 23)
    }
}
/**Configuration register 0

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf0::R`](R) reader structure
impl crate::Readable for CONF0_SPEC {}
///`write(|w| ..)` method takes [`conf0::W`](W) writer structure
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF0 to value 0x0010_001c
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0x0010_001c;
}
