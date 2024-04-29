#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf0: CONF0,
    slc0_int_raw: SLC0_INT_RAW,
    slc0_int_st: SLC0_INT_ST,
    slc0_int_ena: SLC0_INT_ENA,
    slc0_int_clr: SLC0_INT_CLR,
    slc1_int_raw: SLC1_INT_RAW,
    slc1_int_st: SLC1_INT_ST,
    slc1_int_ena: SLC1_INT_ENA,
    slc1_int_clr: SLC1_INT_CLR,
    rx_status: RX_STATUS,
    slc_rxfifo_push: [SLC_RXFIFO_PUSH; 2],
    tx_status: TX_STATUS,
    slc_txfifo_pop: [SLC_TXFIFO_POP; 2],
    slc0_rx_link: SLC0_RX_LINK,
    slc_tx_link: (),
    _reserved15: [u8; 0x04],
    slc1_rx_link: SLC1_RX_LINK,
    _reserved16: [u8; 0x04],
    intvec_tohost: INTVEC_TOHOST,
    slc_token: [SLC_TOKEN; 2],
    conf1: CONF1,
    slc_state: [SLC_STATE; 2],
    bridge_conf: BRIDGE_CONF,
    slc_des_addr: [SLC_DES_ADDR; 2],
    ahb_test: AHB_TEST,
    sdio_st: SDIO_ST,
    rx_dscr_conf: RX_DSCR_CONF,
    slc_link_dscr: [SLC_LINK_DSCR; 2],
    slc_tx_erreof_des_addr: [SLC_TX_ERREOF_DES_ADDR; 2],
    token_lat: TOKEN_LAT,
    tx_dscr_conf: TX_DSCR_CONF,
    cmd_infor0: CMD_INFOR0,
    cmd_infor1: CMD_INFOR1,
    slc0_len_conf: SLC0_LEN_CONF,
    slc0_length: SLC0_LENGTH,
    slc0_txpkt_h_dscr: SLC0_TXPKT_H_DSCR,
    slc0_txpkt_e_dscr: SLC0_TXPKT_E_DSCR,
    slc0_rxpkt_h_dscr: SLC0_RXPKT_H_DSCR,
    slc0_rxpkt_e_dscr: SLC0_RXPKT_E_DSCR,
    slc0_txpktu_h_dscr: SLC0_TXPKTU_H_DSCR,
    slc0_txpktu_e_dscr: SLC0_TXPKTU_E_DSCR,
    slc0_rxpktu_h_dscr: SLC0_RXPKTU_H_DSCR,
    slc0_rxpktu_e_dscr: SLC0_RXPKTU_E_DSCR,
    _reserved41: [u8; 0x08],
    seq_position: SEQ_POSITION,
    slc0_dscr_rec_conf: SLC0_DSCR_REC_CONF,
    sdio_crc_st0: SDIO_CRC_ST0,
    sdio_crc_st1: SDIO_CRC_ST1,
    slc0_eof_start_des: SLC0_EOF_START_DES,
    slc0_push_dscr_addr: SLC0_PUSH_DSCR_ADDR,
    slc0_done_dscr_addr: SLC0_DONE_DSCR_ADDR,
    slc0_sub_start_des: SLC0_SUB_START_DES,
    slc0_dscr_cnt: SLC0_DSCR_CNT,
    slc0_len_lim_conf: SLC0_LEN_LIM_CONF,
    slc0_int_st1: SLC0_INT_ST1,
    slc0_int_ena1: SLC0_INT_ENA1,
    slc1_int_st1: SLC1_INT_ST1,
    slc1_int_ena1: SLC1_INT_ENA1,
    _reserved55: [u8; 0xac],
    date: DATE,
    id: ID,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn slc0_int_raw(&self) -> &SLC0_INT_RAW {
        &self.slc0_int_raw
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn slc0_int_st(&self) -> &SLC0_INT_ST {
        &self.slc0_int_st
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn slc0_int_ena(&self) -> &SLC0_INT_ENA {
        &self.slc0_int_ena
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn slc0_int_clr(&self) -> &SLC0_INT_CLR {
        &self.slc0_int_clr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn slc1_int_raw(&self) -> &SLC1_INT_RAW {
        &self.slc1_int_raw
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn slc1_int_st(&self) -> &SLC1_INT_ST {
        &self.slc1_int_st
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn slc1_int_ena(&self) -> &SLC1_INT_ENA {
        &self.slc1_int_ena
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn slc1_int_clr(&self) -> &SLC1_INT_CLR {
        &self.slc1_int_clr
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn rx_status(&self) -> &RX_STATUS {
        &self.rx_status
    }
    #[doc = "0x28..0x30 - "]
    #[inline(always)]
    pub const fn slc_rxfifo_push(&self, n: usize) -> &SLC_RXFIFO_PUSH {
        &self.slc_rxfifo_push[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x30 - "]
    #[inline(always)]
    pub fn slc_rxfifo_push_iter(&self) -> impl Iterator<Item = &SLC_RXFIFO_PUSH> {
        self.slc_rxfifo_push.iter()
    }
    #[doc = "0x28 - SLC0_RXFIFO_PUSH"]
    #[inline(always)]
    pub const fn slc0_rxfifo_push(&self) -> &SLC_RXFIFO_PUSH {
        self.slc_rxfifo_push(0)
    }
    #[doc = "0x2c - SLC1_RXFIFO_PUSH"]
    #[inline(always)]
    pub const fn slc1_rxfifo_push(&self) -> &SLC_RXFIFO_PUSH {
        self.slc_rxfifo_push(1)
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn tx_status(&self) -> &TX_STATUS {
        &self.tx_status
    }
    #[doc = "0x34..0x3c - "]
    #[inline(always)]
    pub const fn slc_txfifo_pop(&self, n: usize) -> &SLC_TXFIFO_POP {
        &self.slc_txfifo_pop[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x3c - "]
    #[inline(always)]
    pub fn slc_txfifo_pop_iter(&self) -> impl Iterator<Item = &SLC_TXFIFO_POP> {
        self.slc_txfifo_pop.iter()
    }
    #[doc = "0x34 - SLC0_TXFIFO_POP"]
    #[inline(always)]
    pub const fn slc0_txfifo_pop(&self) -> &SLC_TXFIFO_POP {
        self.slc_txfifo_pop(0)
    }
    #[doc = "0x38 - SLC1_TXFIFO_POP"]
    #[inline(always)]
    pub const fn slc1_txfifo_pop(&self) -> &SLC_TXFIFO_POP {
        self.slc_txfifo_pop(1)
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn slc0_rx_link(&self) -> &SLC0_RX_LINK {
        &self.slc0_rx_link
    }
    #[doc = "0x40..0x48 - "]
    #[inline(always)]
    pub const fn slc_tx_link(&self, n: usize) -> &SLC_TX_LINK {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(64).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - "]
    #[inline(always)]
    pub fn slc_tx_link_iter(&self) -> impl Iterator<Item = &SLC_TX_LINK> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(64).add(8 * n).cast() })
    }
    #[doc = "0x40 - SLC0_TX_LINK"]
    #[inline(always)]
    pub const fn slc0_tx_link(&self) -> &SLC_TX_LINK {
        self.slc_tx_link(0)
    }
    #[doc = "0x48 - SLC1_TX_LINK"]
    #[inline(always)]
    pub const fn slc1_tx_link(&self) -> &SLC_TX_LINK {
        self.slc_tx_link(1)
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn slc1_rx_link(&self) -> &SLC1_RX_LINK {
        &self.slc1_rx_link
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn intvec_tohost(&self) -> &INTVEC_TOHOST {
        &self.intvec_tohost
    }
    #[doc = "0x50..0x60 - Cluster SLC%s_TOKEN, containing _?TOKEN0, _?TOKEN1"]
    #[inline(always)]
    pub const fn slc_token(&self, n: usize) -> &SLC_TOKEN {
        &self.slc_token[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - Cluster SLC%s_TOKEN, containing _?TOKEN0, _?TOKEN1"]
    #[inline(always)]
    pub fn slc_token_iter(&self) -> impl Iterator<Item = &SLC_TOKEN> {
        self.slc_token.iter()
    }
    #[doc = "0x50..0x58 - Cluster SLC0_TOKEN, containing _?TOKEN0, _?TOKEN1"]
    #[inline(always)]
    pub const fn slc0_token(&self) -> &SLC_TOKEN {
        self.slc_token(0)
    }
    #[doc = "0x58..0x60 - Cluster SLC1_TOKEN, containing _?TOKEN0, _?TOKEN1"]
    #[inline(always)]
    pub const fn slc1_token(&self) -> &SLC_TOKEN {
        self.slc_token(1)
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x64..0x74 - Cluster SLC%s_STATE, containing _?_STATE0, _?_STATE1"]
    #[inline(always)]
    pub const fn slc_state(&self, n: usize) -> &SLC_STATE {
        &self.slc_state[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x74 - Cluster SLC%s_STATE, containing _?_STATE0, _?_STATE1"]
    #[inline(always)]
    pub fn slc_state_iter(&self) -> impl Iterator<Item = &SLC_STATE> {
        self.slc_state.iter()
    }
    #[doc = "0x64..0x6c - Cluster SLC0_STATE, containing _?_STATE0, _?_STATE1"]
    #[inline(always)]
    pub const fn slc0_state(&self) -> &SLC_STATE {
        self.slc_state(0)
    }
    #[doc = "0x6c..0x74 - Cluster SLC1_STATE, containing _?_STATE0, _?_STATE1"]
    #[inline(always)]
    pub const fn slc1_state(&self) -> &SLC_STATE {
        self.slc_state(1)
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn bridge_conf(&self) -> &BRIDGE_CONF {
        &self.bridge_conf
    }
    #[doc = "0x78..0x90 - Cluster SLC%s_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
    #[inline(always)]
    pub const fn slc_des_addr(&self, n: usize) -> &SLC_DES_ADDR {
        &self.slc_des_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x78..0x90 - Cluster SLC%s_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
    #[inline(always)]
    pub fn slc_des_addr_iter(&self) -> impl Iterator<Item = &SLC_DES_ADDR> {
        self.slc_des_addr.iter()
    }
    #[doc = "0x78..0x84 - Cluster SLC0_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
    #[inline(always)]
    pub const fn slc0_des_addr(&self) -> &SLC_DES_ADDR {
        self.slc_des_addr(0)
    }
    #[doc = "0x84..0x90 - Cluster SLC1_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
    #[inline(always)]
    pub const fn slc1_des_addr(&self) -> &SLC_DES_ADDR {
        self.slc_des_addr(1)
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn sdio_st(&self) -> &SDIO_ST {
        &self.sdio_st
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn rx_dscr_conf(&self) -> &RX_DSCR_CONF {
        &self.rx_dscr_conf
    }
    #[doc = "0x9c..0xcc - Cluster SLC%s_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
    #[inline(always)]
    pub const fn slc_link_dscr(&self, n: usize) -> &SLC_LINK_DSCR {
        &self.slc_link_dscr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0xcc - Cluster SLC%s_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
    #[inline(always)]
    pub fn slc_link_dscr_iter(&self) -> impl Iterator<Item = &SLC_LINK_DSCR> {
        self.slc_link_dscr.iter()
    }
    #[doc = "0x9c..0xb4 - Cluster SLC0_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
    #[inline(always)]
    pub const fn slc0_link_dscr(&self) -> &SLC_LINK_DSCR {
        self.slc_link_dscr(0)
    }
    #[doc = "0xb4..0xcc - Cluster SLC1_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
    #[inline(always)]
    pub const fn slc1_link_dscr(&self) -> &SLC_LINK_DSCR {
        self.slc_link_dscr(1)
    }
    #[doc = "0xcc..0xd4 - "]
    #[inline(always)]
    pub const fn slc_tx_erreof_des_addr(&self, n: usize) -> &SLC_TX_ERREOF_DES_ADDR {
        &self.slc_tx_erreof_des_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xcc..0xd4 - "]
    #[inline(always)]
    pub fn slc_tx_erreof_des_addr_iter(&self) -> impl Iterator<Item = &SLC_TX_ERREOF_DES_ADDR> {
        self.slc_tx_erreof_des_addr.iter()
    }
    #[doc = "0xcc - SLC0_TX_ERREOF_DES_ADDR"]
    #[inline(always)]
    pub const fn slc0_tx_erreof_des_addr(&self) -> &SLC_TX_ERREOF_DES_ADDR {
        self.slc_tx_erreof_des_addr(0)
    }
    #[doc = "0xd0 - SLC1_TX_ERREOF_DES_ADDR"]
    #[inline(always)]
    pub const fn slc1_tx_erreof_des_addr(&self) -> &SLC_TX_ERREOF_DES_ADDR {
        self.slc_tx_erreof_des_addr(1)
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn token_lat(&self) -> &TOKEN_LAT {
        &self.token_lat
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn tx_dscr_conf(&self) -> &TX_DSCR_CONF {
        &self.tx_dscr_conf
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn cmd_infor0(&self) -> &CMD_INFOR0 {
        &self.cmd_infor0
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn cmd_infor1(&self) -> &CMD_INFOR1 {
        &self.cmd_infor1
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn slc0_len_conf(&self) -> &SLC0_LEN_CONF {
        &self.slc0_len_conf
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn slc0_length(&self) -> &SLC0_LENGTH {
        &self.slc0_length
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn slc0_txpkt_h_dscr(&self) -> &SLC0_TXPKT_H_DSCR {
        &self.slc0_txpkt_h_dscr
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn slc0_txpkt_e_dscr(&self) -> &SLC0_TXPKT_E_DSCR {
        &self.slc0_txpkt_e_dscr
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn slc0_rxpkt_h_dscr(&self) -> &SLC0_RXPKT_H_DSCR {
        &self.slc0_rxpkt_h_dscr
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn slc0_rxpkt_e_dscr(&self) -> &SLC0_RXPKT_E_DSCR {
        &self.slc0_rxpkt_e_dscr
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn slc0_txpktu_h_dscr(&self) -> &SLC0_TXPKTU_H_DSCR {
        &self.slc0_txpktu_h_dscr
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn slc0_txpktu_e_dscr(&self) -> &SLC0_TXPKTU_E_DSCR {
        &self.slc0_txpktu_e_dscr
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn slc0_rxpktu_h_dscr(&self) -> &SLC0_RXPKTU_H_DSCR {
        &self.slc0_rxpktu_h_dscr
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn slc0_rxpktu_e_dscr(&self) -> &SLC0_RXPKTU_E_DSCR {
        &self.slc0_rxpktu_e_dscr
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn seq_position(&self) -> &SEQ_POSITION {
        &self.seq_position
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn slc0_dscr_rec_conf(&self) -> &SLC0_DSCR_REC_CONF {
        &self.slc0_dscr_rec_conf
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn sdio_crc_st0(&self) -> &SDIO_CRC_ST0 {
        &self.sdio_crc_st0
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn sdio_crc_st1(&self) -> &SDIO_CRC_ST1 {
        &self.sdio_crc_st1
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn slc0_eof_start_des(&self) -> &SLC0_EOF_START_DES {
        &self.slc0_eof_start_des
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn slc0_push_dscr_addr(&self) -> &SLC0_PUSH_DSCR_ADDR {
        &self.slc0_push_dscr_addr
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn slc0_done_dscr_addr(&self) -> &SLC0_DONE_DSCR_ADDR {
        &self.slc0_done_dscr_addr
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn slc0_sub_start_des(&self) -> &SLC0_SUB_START_DES {
        &self.slc0_sub_start_des
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn slc0_dscr_cnt(&self) -> &SLC0_DSCR_CNT {
        &self.slc0_dscr_cnt
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn slc0_len_lim_conf(&self) -> &SLC0_LEN_LIM_CONF {
        &self.slc0_len_lim_conf
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn slc0_int_st1(&self) -> &SLC0_INT_ST1 {
        &self.slc0_int_st1
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn slc0_int_ena1(&self) -> &SLC0_INT_ENA1 {
        &self.slc0_int_ena1
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn slc1_int_st1(&self) -> &SLC1_INT_ST1 {
        &self.slc1_int_st1
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn slc1_int_ena1(&self) -> &SLC1_INT_ENA1 {
        &self.slc1_int_ena1
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
}
#[doc = "CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "SLC0_INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_int_raw`] module"]
pub type SLC0_INT_RAW = crate::Reg<slc0_int_raw::SLC0_INT_RAW_SPEC>;
#[doc = ""]
pub mod slc0_int_raw;
#[doc = "SLC0_INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_int_st`] module"]
pub type SLC0_INT_ST = crate::Reg<slc0_int_st::SLC0_INT_ST_SPEC>;
#[doc = ""]
pub mod slc0_int_st;
#[doc = "SLC0_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_int_ena`] module"]
pub type SLC0_INT_ENA = crate::Reg<slc0_int_ena::SLC0_INT_ENA_SPEC>;
#[doc = ""]
pub mod slc0_int_ena;
#[doc = "SLC0_INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_int_clr`] module"]
pub type SLC0_INT_CLR = crate::Reg<slc0_int_clr::SLC0_INT_CLR_SPEC>;
#[doc = ""]
pub mod slc0_int_clr;
#[doc = "SLC1_INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_int_raw`] module"]
pub type SLC1_INT_RAW = crate::Reg<slc1_int_raw::SLC1_INT_RAW_SPEC>;
#[doc = ""]
pub mod slc1_int_raw;
#[doc = "SLC1_INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_int_st`] module"]
pub type SLC1_INT_ST = crate::Reg<slc1_int_st::SLC1_INT_ST_SPEC>;
#[doc = ""]
pub mod slc1_int_st;
#[doc = "SLC1_INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_int_ena`] module"]
pub type SLC1_INT_ENA = crate::Reg<slc1_int_ena::SLC1_INT_ENA_SPEC>;
#[doc = ""]
pub mod slc1_int_ena;
#[doc = "SLC1_INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_int_clr`] module"]
pub type SLC1_INT_CLR = crate::Reg<slc1_int_clr::SLC1_INT_CLR_SPEC>;
#[doc = ""]
pub mod slc1_int_clr;
#[doc = "RX_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_status`] module"]
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
#[doc = ""]
pub mod rx_status;
#[doc = "SLC_RXFIFO_PUSH (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rxfifo_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rxfifo_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_rxfifo_push`] module"]
pub type SLC_RXFIFO_PUSH = crate::Reg<slc_rxfifo_push::SLC_RXFIFO_PUSH_SPEC>;
#[doc = ""]
pub mod slc_rxfifo_push;
#[doc = "TX_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_status`] module"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = ""]
pub mod tx_status;
#[doc = "SLC_TXFIFO_POP (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_txfifo_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_txfifo_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_txfifo_pop`] module"]
pub type SLC_TXFIFO_POP = crate::Reg<slc_txfifo_pop::SLC_TXFIFO_POP_SPEC>;
#[doc = ""]
pub mod slc_txfifo_pop;
#[doc = "SLC0_RX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_rx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_rx_link`] module"]
pub type SLC0_RX_LINK = crate::Reg<slc0_rx_link::SLC0_RX_LINK_SPEC>;
#[doc = ""]
pub mod slc0_rx_link;
#[doc = "SLC_TX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_tx_link`] module"]
pub type SLC_TX_LINK = crate::Reg<slc_tx_link::SLC_TX_LINK_SPEC>;
#[doc = ""]
pub mod slc_tx_link;
#[doc = "SLC1_RX_LINK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_rx_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_rx_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_rx_link`] module"]
pub type SLC1_RX_LINK = crate::Reg<slc1_rx_link::SLC1_RX_LINK_SPEC>;
#[doc = ""]
pub mod slc1_rx_link;
#[doc = "INTVEC_TOHOST (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intvec_tohost::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intvec_tohost`] module"]
pub type INTVEC_TOHOST = crate::Reg<intvec_tohost::INTVEC_TOHOST_SPEC>;
#[doc = ""]
pub mod intvec_tohost;
#[doc = "Cluster SLC%s_TOKEN, containing _?TOKEN0, _?TOKEN1"]
pub use self::slc_token::SLC_TOKEN;
#[doc = r"Cluster"]
#[doc = "Cluster SLC%s_TOKEN, containing _?TOKEN0, _?TOKEN1"]
pub mod slc_token;
#[doc = "CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "Cluster SLC%s_STATE, containing _?_STATE0, _?_STATE1"]
pub use self::slc_state::SLC_STATE;
#[doc = r"Cluster"]
#[doc = "Cluster SLC%s_STATE, containing _?_STATE0, _?_STATE1"]
pub mod slc_state;
#[doc = "BRIDGE_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bridge_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bridge_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bridge_conf`] module"]
pub type BRIDGE_CONF = crate::Reg<bridge_conf::BRIDGE_CONF_SPEC>;
#[doc = ""]
pub mod bridge_conf;
#[doc = "Cluster SLC%s_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
pub use self::slc_des_addr::SLC_DES_ADDR;
#[doc = r"Cluster"]
#[doc = "Cluster SLC%s_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
pub mod slc_des_addr;
#[doc = "AHB_TEST (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = ""]
pub mod ahb_test;
#[doc = "SDIO_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_st`] module"]
pub type SDIO_ST = crate::Reg<sdio_st::SDIO_ST_SPEC>;
#[doc = ""]
pub mod sdio_st;
#[doc = "RX_DSCR_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_dscr_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_dscr_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_dscr_conf`] module"]
pub type RX_DSCR_CONF = crate::Reg<rx_dscr_conf::RX_DSCR_CONF_SPEC>;
#[doc = ""]
pub mod rx_dscr_conf;
#[doc = "Cluster SLC%s_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
pub use self::slc_link_dscr::SLC_LINK_DSCR;
#[doc = r"Cluster"]
#[doc = "Cluster SLC%s_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
pub mod slc_link_dscr;
#[doc = "SLC_TX_ERREOF_DES_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_erreof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc_tx_erreof_des_addr`] module"]
pub type SLC_TX_ERREOF_DES_ADDR = crate::Reg<slc_tx_erreof_des_addr::SLC_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod slc_tx_erreof_des_addr;
#[doc = "TOKEN_LAT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token_lat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@token_lat`] module"]
pub type TOKEN_LAT = crate::Reg<token_lat::TOKEN_LAT_SPEC>;
#[doc = ""]
pub mod token_lat;
#[doc = "TX_DSCR_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_dscr_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_dscr_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_dscr_conf`] module"]
pub type TX_DSCR_CONF = crate::Reg<tx_dscr_conf::TX_DSCR_CONF_SPEC>;
#[doc = ""]
pub mod tx_dscr_conf;
#[doc = "CMD_INFOR0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_infor0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_infor0`] module"]
pub type CMD_INFOR0 = crate::Reg<cmd_infor0::CMD_INFOR0_SPEC>;
#[doc = ""]
pub mod cmd_infor0;
#[doc = "CMD_INFOR1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_infor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_infor1`] module"]
pub type CMD_INFOR1 = crate::Reg<cmd_infor1::CMD_INFOR1_SPEC>;
#[doc = ""]
pub mod cmd_infor1;
#[doc = "SLC0_LEN_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_len_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_len_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_len_conf`] module"]
pub type SLC0_LEN_CONF = crate::Reg<slc0_len_conf::SLC0_LEN_CONF_SPEC>;
#[doc = ""]
pub mod slc0_len_conf;
#[doc = "SLC0_LENGTH (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_length::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_length`] module"]
pub type SLC0_LENGTH = crate::Reg<slc0_length::SLC0_LENGTH_SPEC>;
#[doc = ""]
pub mod slc0_length;
#[doc = "SLC0_TXPKT_H_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_txpkt_h_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_txpkt_h_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_txpkt_h_dscr`] module"]
pub type SLC0_TXPKT_H_DSCR = crate::Reg<slc0_txpkt_h_dscr::SLC0_TXPKT_H_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_txpkt_h_dscr;
#[doc = "SLC0_TXPKT_E_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_txpkt_e_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_txpkt_e_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_txpkt_e_dscr`] module"]
pub type SLC0_TXPKT_E_DSCR = crate::Reg<slc0_txpkt_e_dscr::SLC0_TXPKT_E_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_txpkt_e_dscr;
#[doc = "SLC0_RXPKT_H_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rxpkt_h_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_rxpkt_h_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_rxpkt_h_dscr`] module"]
pub type SLC0_RXPKT_H_DSCR = crate::Reg<slc0_rxpkt_h_dscr::SLC0_RXPKT_H_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_rxpkt_h_dscr;
#[doc = "SLC0_RXPKT_E_DSCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rxpkt_e_dscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_rxpkt_e_dscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_rxpkt_e_dscr`] module"]
pub type SLC0_RXPKT_E_DSCR = crate::Reg<slc0_rxpkt_e_dscr::SLC0_RXPKT_E_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_rxpkt_e_dscr;
#[doc = "SLC0_TXPKTU_H_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_txpktu_h_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_txpktu_h_dscr`] module"]
pub type SLC0_TXPKTU_H_DSCR = crate::Reg<slc0_txpktu_h_dscr::SLC0_TXPKTU_H_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_txpktu_h_dscr;
#[doc = "SLC0_TXPKTU_E_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_txpktu_e_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_txpktu_e_dscr`] module"]
pub type SLC0_TXPKTU_E_DSCR = crate::Reg<slc0_txpktu_e_dscr::SLC0_TXPKTU_E_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_txpktu_e_dscr;
#[doc = "SLC0_RXPKTU_H_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rxpktu_h_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_rxpktu_h_dscr`] module"]
pub type SLC0_RXPKTU_H_DSCR = crate::Reg<slc0_rxpktu_h_dscr::SLC0_RXPKTU_H_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_rxpktu_h_dscr;
#[doc = "SLC0_RXPKTU_E_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_rxpktu_e_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_rxpktu_e_dscr`] module"]
pub type SLC0_RXPKTU_E_DSCR = crate::Reg<slc0_rxpktu_e_dscr::SLC0_RXPKTU_E_DSCR_SPEC>;
#[doc = ""]
pub mod slc0_rxpktu_e_dscr;
#[doc = "SEQ_POSITION (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_position`] module"]
pub type SEQ_POSITION = crate::Reg<seq_position::SEQ_POSITION_SPEC>;
#[doc = ""]
pub mod seq_position;
#[doc = "SLC0_DSCR_REC_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_dscr_rec_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_dscr_rec_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_dscr_rec_conf`] module"]
pub type SLC0_DSCR_REC_CONF = crate::Reg<slc0_dscr_rec_conf::SLC0_DSCR_REC_CONF_SPEC>;
#[doc = ""]
pub mod slc0_dscr_rec_conf;
#[doc = "SDIO_CRC_ST0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_crc_st0`] module"]
pub type SDIO_CRC_ST0 = crate::Reg<sdio_crc_st0::SDIO_CRC_ST0_SPEC>;
#[doc = ""]
pub mod sdio_crc_st0;
#[doc = "SDIO_CRC_ST1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_crc_st1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_crc_st1`] module"]
pub type SDIO_CRC_ST1 = crate::Reg<sdio_crc_st1::SDIO_CRC_ST1_SPEC>;
#[doc = ""]
pub mod sdio_crc_st1;
#[doc = "SLC0_EOF_START_DES (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_eof_start_des::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_eof_start_des`] module"]
pub type SLC0_EOF_START_DES = crate::Reg<slc0_eof_start_des::SLC0_EOF_START_DES_SPEC>;
#[doc = ""]
pub mod slc0_eof_start_des;
#[doc = "SLC0_PUSH_DSCR_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_push_dscr_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_push_dscr_addr`] module"]
pub type SLC0_PUSH_DSCR_ADDR = crate::Reg<slc0_push_dscr_addr::SLC0_PUSH_DSCR_ADDR_SPEC>;
#[doc = ""]
pub mod slc0_push_dscr_addr;
#[doc = "SLC0_DONE_DSCR_ADDR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_done_dscr_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_done_dscr_addr`] module"]
pub type SLC0_DONE_DSCR_ADDR = crate::Reg<slc0_done_dscr_addr::SLC0_DONE_DSCR_ADDR_SPEC>;
#[doc = ""]
pub mod slc0_done_dscr_addr;
#[doc = "SLC0_SUB_START_DES (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_sub_start_des::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_sub_start_des`] module"]
pub type SLC0_SUB_START_DES = crate::Reg<slc0_sub_start_des::SLC0_SUB_START_DES_SPEC>;
#[doc = ""]
pub mod slc0_sub_start_des;
#[doc = "SLC0_DSCR_CNT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_dscr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_dscr_cnt`] module"]
pub type SLC0_DSCR_CNT = crate::Reg<slc0_dscr_cnt::SLC0_DSCR_CNT_SPEC>;
#[doc = ""]
pub mod slc0_dscr_cnt;
#[doc = "SLC0_LEN_LIM_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_len_lim_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_len_lim_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_len_lim_conf`] module"]
pub type SLC0_LEN_LIM_CONF = crate::Reg<slc0_len_lim_conf::SLC0_LEN_LIM_CONF_SPEC>;
#[doc = ""]
pub mod slc0_len_lim_conf;
#[doc = "SLC0_INT_ST1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_int_st1`] module"]
pub type SLC0_INT_ST1 = crate::Reg<slc0_int_st1::SLC0_INT_ST1_SPEC>;
#[doc = ""]
pub mod slc0_int_st1;
#[doc = "SLC0_INT_ENA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_int_ena1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_int_ena1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc0_int_ena1`] module"]
pub type SLC0_INT_ENA1 = crate::Reg<slc0_int_ena1::SLC0_INT_ENA1_SPEC>;
#[doc = ""]
pub mod slc0_int_ena1;
#[doc = "SLC1_INT_ST1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_int_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_int_st1`] module"]
pub type SLC1_INT_ST1 = crate::Reg<slc1_int_st1::SLC1_INT_ST1_SPEC>;
#[doc = ""]
pub mod slc1_int_st1;
#[doc = "SLC1_INT_ENA1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1_int_ena1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc1_int_ena1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slc1_int_ena1`] module"]
pub type SLC1_INT_ENA1 = crate::Reg<slc1_int_ena1::SLC1_INT_ENA1_SPEC>;
#[doc = ""]
pub mod slc1_int_ena1;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ID (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`] module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = ""]
pub mod id;
