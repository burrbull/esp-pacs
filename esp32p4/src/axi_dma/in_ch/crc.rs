#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster CRC, containing IN_CRC_INIT_DATA, RX_CRC_WIDTH, IN_CRC_CLEAR, IN_CRC_FINAL_RESULT, RX_CRC_EN_WR_DATA, RX_CRC_EN_ADDR, RX_CRC_DATA_EN_WR_DATA, RX_CRC_DATA_EN_ADDR
pub struct CRC {
    in_crc_init_data: IN_CRC_INIT_DATA,
    rx_crc_width: RX_CRC_WIDTH,
    in_crc_clear: IN_CRC_CLEAR,
    in_crc_final_result: IN_CRC_FINAL_RESULT,
    rx_crc_en_wr_data: RX_CRC_EN_WR_DATA,
    rx_crc_en_addr: RX_CRC_EN_ADDR,
    rx_crc_data_en_wr_data: RX_CRC_DATA_EN_WR_DATA,
    rx_crc_data_en_addr: RX_CRC_DATA_EN_ADDR,
}
impl CRC {
    ///0x00 - This register is used to config ch0 crc initial data(max 32 bit)
    #[inline(always)]
    pub const fn in_crc_init_data(&self) -> &IN_CRC_INIT_DATA {
        &self.in_crc_init_data
    }
    ///0x04 - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32
    #[inline(always)]
    pub const fn rx_crc_width(&self) -> &RX_CRC_WIDTH {
        &self.rx_crc_width
    }
    ///0x08 - This register is used to clear ch0 crc result
    #[inline(always)]
    pub const fn in_crc_clear(&self) -> &IN_CRC_CLEAR {
        &self.in_crc_clear
    }
    ///0x0c - This register is used to store ch0 crc result
    #[inline(always)]
    pub const fn in_crc_final_result(&self) -> &IN_CRC_FINAL_RESULT {
        &self.in_crc_final_result
    }
    ///0x10 - This resister is used to config ch0 crc en for every bit
    #[inline(always)]
    pub const fn rx_crc_en_wr_data(&self) -> &RX_CRC_EN_WR_DATA {
        &self.rx_crc_en_wr_data
    }
    ///0x14 - This register is used to config ch0 crc en addr
    #[inline(always)]
    pub const fn rx_crc_en_addr(&self) -> &RX_CRC_EN_ADDR {
        &self.rx_crc_en_addr
    }
    ///0x18 - This register is used to config crc data_8bit en
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data(&self) -> &RX_CRC_DATA_EN_WR_DATA {
        &self.rx_crc_data_en_wr_data
    }
    ///0x1c - This register is used to config addr of crc data_8bit en
    #[inline(always)]
    pub const fn rx_crc_data_en_addr(&self) -> &RX_CRC_DATA_EN_ADDR {
        &self.rx_crc_data_en_addr
    }
}
/**IN_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)

You can [`read`](crate::generic::Reg::read) this register and get [`in_crc_init_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_init_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_crc_init_data`] module*/
pub type IN_CRC_INIT_DATA = crate::Reg<in_crc_init_data::IN_CRC_INIT_DATA_SPEC>;
///This register is used to config ch0 crc initial data(max 32 bit)
pub mod in_crc_init_data;
/**RX_CRC_WIDTH (rw) register accessor: This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32

You can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_crc_width`] module*/
pub type RX_CRC_WIDTH = crate::Reg<rx_crc_width::RX_CRC_WIDTH_SPEC>;
///This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32
pub mod rx_crc_width;
/**IN_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result

You can [`read`](crate::generic::Reg::read) this register and get [`in_crc_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_crc_clear`] module*/
pub type IN_CRC_CLEAR = crate::Reg<in_crc_clear::IN_CRC_CLEAR_SPEC>;
///This register is used to clear ch0 crc result
pub mod in_crc_clear;
/**IN_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result

You can [`read`](crate::generic::Reg::read) this register and get [`in_crc_final_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_crc_final_result`] module*/
pub type IN_CRC_FINAL_RESULT = crate::Reg<in_crc_final_result::IN_CRC_FINAL_RESULT_SPEC>;
///This register is used to store ch0 crc result
pub mod in_crc_final_result;
/**RX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit

You can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_crc_en_wr_data`] module*/
pub type RX_CRC_EN_WR_DATA = crate::Reg<rx_crc_en_wr_data::RX_CRC_EN_WR_DATA_SPEC>;
///This resister is used to config ch0 crc en for every bit
pub mod rx_crc_en_wr_data;
/**RX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr

You can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_crc_en_addr`] module*/
pub type RX_CRC_EN_ADDR = crate::Reg<rx_crc_en_addr::RX_CRC_EN_ADDR_SPEC>;
///This register is used to config ch0 crc en addr
pub mod rx_crc_en_addr;
/**RX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en

You can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_crc_data_en_wr_data`] module*/
pub type RX_CRC_DATA_EN_WR_DATA = crate::Reg<
    rx_crc_data_en_wr_data::RX_CRC_DATA_EN_WR_DATA_SPEC,
>;
///This register is used to config crc data_8bit en
pub mod rx_crc_data_en_wr_data;
/**RX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en

You can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_crc_data_en_addr`] module*/
pub type RX_CRC_DATA_EN_ADDR = crate::Reg<rx_crc_data_en_addr::RX_CRC_DATA_EN_ADDR_SPEC>;
///This register is used to config addr of crc data_8bit en
pub mod rx_crc_data_en_addr;
