#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster OUT_CRC_CH%s, containing OUT_CRC_INIT_DATA_CH?, TX_CRC_WIDTH_CH?, OUT_CRC_CLEAR_CH?, OUT_CRC_FINAL_RESULT_CH?, TX_CRC_EN_WR_DATA_CH?, TX_CRC_EN_ADDR_CH?, TX_CRC_DATA_EN_WR_DATA_CH?, TX_CRC_DATA_EN_ADDR_CH?, TX_CH_ARB_WEIGH_CH?, TX_ARB_WEIGH_OPT_DIR_CH?
pub struct OUT_CRC_CH {
    out_crc_init_data: OUT_CRC_INIT_DATA,
    tx_crc_width: TX_CRC_WIDTH,
    out_crc_clear: OUT_CRC_CLEAR,
    out_crc_final_result: OUT_CRC_FINAL_RESULT,
    tx_crc_en_wr_data: TX_CRC_EN_WR_DATA,
    tx_crc_en_addr: TX_CRC_EN_ADDR,
    tx_crc_data_en_wr_data: TX_CRC_DATA_EN_WR_DATA,
    tx_crc_data_en_addr: TX_CRC_DATA_EN_ADDR,
    tx_ch_arb_weigh: TX_CH_ARB_WEIGH,
    tx_arb_weigh_opt_dir: TX_ARB_WEIGH_OPT_DIR,
}
impl OUT_CRC_CH {
    ///0x00 - This register is used to config ch0 crc initial data(max 32 bit)
    #[inline(always)]
    pub const fn out_crc_init_data(&self) -> &OUT_CRC_INIT_DATA {
        &self.out_crc_init_data
    }
    ///0x04 - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32
    #[inline(always)]
    pub const fn tx_crc_width(&self) -> &TX_CRC_WIDTH {
        &self.tx_crc_width
    }
    ///0x08 - This register is used to clear ch0 crc result
    #[inline(always)]
    pub const fn out_crc_clear(&self) -> &OUT_CRC_CLEAR {
        &self.out_crc_clear
    }
    ///0x0c - This register is used to store ch0 crc result
    #[inline(always)]
    pub const fn out_crc_final_result(&self) -> &OUT_CRC_FINAL_RESULT {
        &self.out_crc_final_result
    }
    ///0x10 - This resister is used to config ch0 crc en for every bit
    #[inline(always)]
    pub const fn tx_crc_en_wr_data(&self) -> &TX_CRC_EN_WR_DATA {
        &self.tx_crc_en_wr_data
    }
    ///0x14 - This register is used to config ch0 crc en addr
    #[inline(always)]
    pub const fn tx_crc_en_addr(&self) -> &TX_CRC_EN_ADDR {
        &self.tx_crc_en_addr
    }
    ///0x18 - This register is used to config crc data_8bit en
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data(&self) -> &TX_CRC_DATA_EN_WR_DATA {
        &self.tx_crc_data_en_wr_data
    }
    ///0x1c - This register is used to config addr of crc data_8bit en
    #[inline(always)]
    pub const fn tx_crc_data_en_addr(&self) -> &TX_CRC_DATA_EN_ADDR {
        &self.tx_crc_data_en_addr
    }
    ///0x20 - This register is used to config ch0 arbiter weigh
    #[inline(always)]
    pub const fn tx_ch_arb_weigh(&self) -> &TX_CH_ARB_WEIGH {
        &self.tx_ch_arb_weigh
    }
    ///0x24 - This register is used to config off or on weigh optimization
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir(&self) -> &TX_ARB_WEIGH_OPT_DIR {
        &self.tx_arb_weigh_opt_dir
    }
}
/**OUT_CRC_INIT_DATA (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)

You can [`read`](crate::generic::Reg::read) this register and get [`out_crc_init_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_init_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_crc_init_data`] module*/
pub type OUT_CRC_INIT_DATA = crate::Reg<out_crc_init_data::OUT_CRC_INIT_DATA_SPEC>;
///This register is used to config ch0 crc initial data(max 32 bit)
pub mod out_crc_init_data;
/**TX_CRC_WIDTH (rw) register accessor: This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_crc_width`] module*/
pub type TX_CRC_WIDTH = crate::Reg<tx_crc_width::TX_CRC_WIDTH_SPEC>;
///This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32
pub mod tx_crc_width;
/**OUT_CRC_CLEAR (rw) register accessor: This register is used to clear ch0 crc result

You can [`read`](crate::generic::Reg::read) this register and get [`out_crc_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_crc_clear`] module*/
pub type OUT_CRC_CLEAR = crate::Reg<out_crc_clear::OUT_CRC_CLEAR_SPEC>;
///This register is used to clear ch0 crc result
pub mod out_crc_clear;
/**OUT_CRC_FINAL_RESULT (r) register accessor: This register is used to store ch0 crc result

You can [`read`](crate::generic::Reg::read) this register and get [`out_crc_final_result::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_crc_final_result`] module*/
pub type OUT_CRC_FINAL_RESULT = crate::Reg<
    out_crc_final_result::OUT_CRC_FINAL_RESULT_SPEC,
>;
///This register is used to store ch0 crc result
pub mod out_crc_final_result;
/**TX_CRC_EN_WR_DATA (rw) register accessor: This resister is used to config ch0 crc en for every bit

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_crc_en_wr_data`] module*/
pub type TX_CRC_EN_WR_DATA = crate::Reg<tx_crc_en_wr_data::TX_CRC_EN_WR_DATA_SPEC>;
///This resister is used to config ch0 crc en for every bit
pub mod tx_crc_en_wr_data;
/**TX_CRC_EN_ADDR (rw) register accessor: This register is used to config ch0 crc en addr

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_crc_en_addr`] module*/
pub type TX_CRC_EN_ADDR = crate::Reg<tx_crc_en_addr::TX_CRC_EN_ADDR_SPEC>;
///This register is used to config ch0 crc en addr
pub mod tx_crc_en_addr;
/**TX_CRC_DATA_EN_WR_DATA (rw) register accessor: This register is used to config crc data_8bit en

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_wr_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_crc_data_en_wr_data`] module*/
pub type TX_CRC_DATA_EN_WR_DATA = crate::Reg<
    tx_crc_data_en_wr_data::TX_CRC_DATA_EN_WR_DATA_SPEC,
>;
///This register is used to config crc data_8bit en
pub mod tx_crc_data_en_wr_data;
/**TX_CRC_DATA_EN_ADDR (rw) register accessor: This register is used to config addr of crc data_8bit en

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_crc_data_en_addr`] module*/
pub type TX_CRC_DATA_EN_ADDR = crate::Reg<tx_crc_data_en_addr::TX_CRC_DATA_EN_ADDR_SPEC>;
///This register is used to config addr of crc data_8bit en
pub mod tx_crc_data_en_addr;
/**TX_CH_ARB_WEIGH (rw) register accessor: This register is used to config ch0 arbiter weigh

You can [`read`](crate::generic::Reg::read) this register and get [`tx_ch_arb_weigh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ch_arb_weigh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_ch_arb_weigh`] module*/
pub type TX_CH_ARB_WEIGH = crate::Reg<tx_ch_arb_weigh::TX_CH_ARB_WEIGH_SPEC>;
///This register is used to config ch0 arbiter weigh
pub mod tx_ch_arb_weigh;
/**TX_ARB_WEIGH_OPT_DIR (rw) register accessor: This register is used to config off or on weigh optimization

You can [`read`](crate::generic::Reg::read) this register and get [`tx_arb_weigh_opt_dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_arb_weigh_opt_dir`] module*/
pub type TX_ARB_WEIGH_OPT_DIR = crate::Reg<
    tx_arb_weigh_opt_dir::TX_ARB_WEIGH_OPT_DIR_SPEC,
>;
///This register is used to config off or on weigh optimization
pub mod tx_arb_weigh_opt_dir;
