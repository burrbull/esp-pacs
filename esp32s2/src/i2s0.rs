#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    conf: CONF,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    timing: TIMING,
    fifo_conf: FIFO_CONF,
    rxeof_num: RXEOF_NUM,
    conf_sigle_data: CONF_SIGLE_DATA,
    conf_chan: CONF_CHAN,
    out_link: OUT_LINK,
    in_link: IN_LINK,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    in_eof_des_addr: IN_EOF_DES_ADDR,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    _reserved15: [u8; 0x04],
    inlink_dscr: INLINK_DSCR,
    inlink_dscr_bf0: INLINK_DSCR_BF0,
    inlink_dscr_bf1: INLINK_DSCR_BF1,
    outlink_dscr: OUTLINK_DSCR,
    outlink_dscr_bf0: OUTLINK_DSCR_BF0,
    outlink_dscr_bf1: OUTLINK_DSCR_BF1,
    lc_conf: LC_CONF,
    outfifo_push: OUTFIFO_PUSH,
    infifo_pop: INFIFO_POP,
    lc_state0: LC_STATE0,
    lc_state1: LC_STATE1,
    lc_hung_conf: LC_HUNG_CONF,
    _reserved27: [u8; 0x28],
    conf1: CONF1,
    pd_conf: PD_CONF,
    conf2: CONF2,
    clkm_conf: CLKM_CONF,
    sample_rate_conf: SAMPLE_RATE_CONF,
    _reserved32: [u8; 0x08],
    state: STATE,
    _reserved33: [u8; 0x3c],
    date: DATE,
}
impl RegisterBlock {
    ///0x08 - I2S configuration register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x0c - Raw interrupt status
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x10 - Masked interrupt status
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x14 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x18 - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x1c - I2S timing register
    #[inline(always)]
    pub const fn timing(&self) -> &TIMING {
        &self.timing
    }
    ///0x20 - I2S FIFO configuration register
    #[inline(always)]
    pub const fn fifo_conf(&self) -> &FIFO_CONF {
        &self.fifo_conf
    }
    ///0x24 - I2S DMA RX EOF data length
    #[inline(always)]
    pub const fn rxeof_num(&self) -> &RXEOF_NUM {
        &self.rxeof_num
    }
    ///0x28 - Constant single channel data
    #[inline(always)]
    pub const fn conf_sigle_data(&self) -> &CONF_SIGLE_DATA {
        &self.conf_sigle_data
    }
    ///0x2c - I2S channel configuration register
    #[inline(always)]
    pub const fn conf_chan(&self) -> &CONF_CHAN {
        &self.conf_chan
    }
    ///0x30 - I2S DMA TX configuration register
    #[inline(always)]
    pub const fn out_link(&self) -> &OUT_LINK {
        &self.out_link
    }
    ///0x34 - I2S DMA RX configuration register
    #[inline(always)]
    pub const fn in_link(&self) -> &IN_LINK {
        &self.in_link
    }
    ///0x38 - Address of outlink descriptor that produces EOF
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    ///0x3c - Address of inlink descriptor that produces EOF
    #[inline(always)]
    pub const fn in_eof_des_addr(&self) -> &IN_EOF_DES_ADDR {
        &self.in_eof_des_addr
    }
    ///0x40 - Address of buffer relative to the outlink descriptor that produces EOF
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    ///0x48 - Address of current inlink descriptor
    #[inline(always)]
    pub const fn inlink_dscr(&self) -> &INLINK_DSCR {
        &self.inlink_dscr
    }
    ///0x4c - Address of next inlink descriptor
    #[inline(always)]
    pub const fn inlink_dscr_bf0(&self) -> &INLINK_DSCR_BF0 {
        &self.inlink_dscr_bf0
    }
    ///0x50 - Address of next inlink data buffer
    #[inline(always)]
    pub const fn inlink_dscr_bf1(&self) -> &INLINK_DSCR_BF1 {
        &self.inlink_dscr_bf1
    }
    ///0x54 - Address of current outlink descriptor
    #[inline(always)]
    pub const fn outlink_dscr(&self) -> &OUTLINK_DSCR {
        &self.outlink_dscr
    }
    ///0x58 - Address of next outlink descriptor
    #[inline(always)]
    pub const fn outlink_dscr_bf0(&self) -> &OUTLINK_DSCR_BF0 {
        &self.outlink_dscr_bf0
    }
    ///0x5c - Address of next outlink data buffer
    #[inline(always)]
    pub const fn outlink_dscr_bf1(&self) -> &OUTLINK_DSCR_BF1 {
        &self.outlink_dscr_bf1
    }
    ///0x60 - I2S DMA configuration register
    #[inline(always)]
    pub const fn lc_conf(&self) -> &LC_CONF {
        &self.lc_conf
    }
    ///0x64 - APB out FIFO mode register
    #[inline(always)]
    pub const fn outfifo_push(&self) -> &OUTFIFO_PUSH {
        &self.outfifo_push
    }
    ///0x68 - APB in FIFO mode register
    #[inline(always)]
    pub const fn infifo_pop(&self) -> &INFIFO_POP {
        &self.infifo_pop
    }
    ///0x6c - I2S DMA TX status
    #[inline(always)]
    pub const fn lc_state0(&self) -> &LC_STATE0 {
        &self.lc_state0
    }
    ///0x70 - I2S DMA RX status
    #[inline(always)]
    pub const fn lc_state1(&self) -> &LC_STATE1 {
        &self.lc_state1
    }
    ///0x74 - I2S Hung configuration register
    #[inline(always)]
    pub const fn lc_hung_conf(&self) -> &LC_HUNG_CONF {
        &self.lc_hung_conf
    }
    ///0xa0 - I2S configuration register 1
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    ///0xa4 - I2S power-down configuration register
    #[inline(always)]
    pub const fn pd_conf(&self) -> &PD_CONF {
        &self.pd_conf
    }
    ///0xa8 - I2S configuration register 2
    #[inline(always)]
    pub const fn conf2(&self) -> &CONF2 {
        &self.conf2
    }
    ///0xac - I2S module clock configuration register
    #[inline(always)]
    pub const fn clkm_conf(&self) -> &CLKM_CONF {
        &self.clkm_conf
    }
    ///0xb0 - I2S sample rate register
    #[inline(always)]
    pub const fn sample_rate_conf(&self) -> &SAMPLE_RATE_CONF {
        &self.sample_rate_conf
    }
    ///0xbc - I2S TX status register
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    ///0xfc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CONF (rw) register accessor: I2S configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///I2S configuration register
pub mod conf;
/**INT_RAW (r) register accessor: Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Raw interrupt status
pub mod int_raw;
/**INT_ST (r) register accessor: Masked interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Masked interrupt status
pub mod int_st;
/**INT_ENA (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Interrupt enable bits
pub mod int_ena;
/**INT_CLR (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Interrupt clear bits
pub mod int_clr;
/**TIMING (rw) register accessor: I2S timing register

You can [`read`](crate::generic::Reg::read) this register and get [`timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timing`] module*/
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
///I2S timing register
pub mod timing;
/**FIFO_CONF (rw) register accessor: I2S FIFO configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo_conf`] module*/
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
///I2S FIFO configuration register
pub mod fifo_conf;
/**RXEOF_NUM (rw) register accessor: I2S DMA RX EOF data length

You can [`read`](crate::generic::Reg::read) this register and get [`rxeof_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxeof_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxeof_num`] module*/
pub type RXEOF_NUM = crate::Reg<rxeof_num::RXEOF_NUM_SPEC>;
///I2S DMA RX EOF data length
pub mod rxeof_num;
/**CONF_SIGLE_DATA (rw) register accessor: Constant single channel data

You can [`read`](crate::generic::Reg::read) this register and get [`conf_sigle_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_sigle_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf_sigle_data`] module*/
pub type CONF_SIGLE_DATA = crate::Reg<conf_sigle_data::CONF_SIGLE_DATA_SPEC>;
///Constant single channel data
pub mod conf_sigle_data;
/**CONF_CHAN (rw) register accessor: I2S channel configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`conf_chan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_chan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf_chan`] module*/
pub type CONF_CHAN = crate::Reg<conf_chan::CONF_CHAN_SPEC>;
///I2S channel configuration register
pub mod conf_chan;
/**OUT_LINK (rw) register accessor: I2S DMA TX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`out_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_link`] module*/
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
///I2S DMA TX configuration register
pub mod out_link;
/**IN_LINK (rw) register accessor: I2S DMA RX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`in_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_link`] module*/
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
///I2S DMA RX configuration register
pub mod in_link;
/**OUT_EOF_DES_ADDR (r) register accessor: Address of outlink descriptor that produces EOF

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_eof_des_addr`] module*/
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
///Address of outlink descriptor that produces EOF
pub mod out_eof_des_addr;
/**IN_EOF_DES_ADDR (r) register accessor: Address of inlink descriptor that produces EOF

You can [`read`](crate::generic::Reg::read) this register and get [`in_eof_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_eof_des_addr`] module*/
pub type IN_EOF_DES_ADDR = crate::Reg<in_eof_des_addr::IN_EOF_DES_ADDR_SPEC>;
///Address of inlink descriptor that produces EOF
pub mod in_eof_des_addr;
/**OUT_EOF_BFR_DES_ADDR (r) register accessor: Address of buffer relative to the outlink descriptor that produces EOF

You can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_eof_bfr_des_addr`] module*/
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
///Address of buffer relative to the outlink descriptor that produces EOF
pub mod out_eof_bfr_des_addr;
/**INLINK_DSCR (r) register accessor: Address of current inlink descriptor

You can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inlink_dscr`] module*/
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
///Address of current inlink descriptor
pub mod inlink_dscr;
/**INLINK_DSCR_BF0 (r) register accessor: Address of next inlink descriptor

You can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inlink_dscr_bf0`] module*/
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
///Address of next inlink descriptor
pub mod inlink_dscr_bf0;
/**INLINK_DSCR_BF1 (r) register accessor: Address of next inlink data buffer

You can [`read`](crate::generic::Reg::read) this register and get [`inlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inlink_dscr_bf1`] module*/
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
///Address of next inlink data buffer
pub mod inlink_dscr_bf1;
/**OUTLINK_DSCR (r) register accessor: Address of current outlink descriptor

You can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outlink_dscr`] module*/
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
///Address of current outlink descriptor
pub mod outlink_dscr;
/**OUTLINK_DSCR_BF0 (r) register accessor: Address of next outlink descriptor

You can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outlink_dscr_bf0`] module*/
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
///Address of next outlink descriptor
pub mod outlink_dscr_bf0;
/**OUTLINK_DSCR_BF1 (r) register accessor: Address of next outlink data buffer

You can [`read`](crate::generic::Reg::read) this register and get [`outlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outlink_dscr_bf1`] module*/
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
///Address of next outlink data buffer
pub mod outlink_dscr_bf1;
/**LC_CONF (rw) register accessor: I2S DMA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_conf`] module*/
pub type LC_CONF = crate::Reg<lc_conf::LC_CONF_SPEC>;
///I2S DMA configuration register
pub mod lc_conf;
/**OUTFIFO_PUSH (rw) register accessor: APB out FIFO mode register

You can [`read`](crate::generic::Reg::read) this register and get [`outfifo_push::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outfifo_push::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@outfifo_push`] module*/
pub type OUTFIFO_PUSH = crate::Reg<outfifo_push::OUTFIFO_PUSH_SPEC>;
///APB out FIFO mode register
pub mod outfifo_push;
/**INFIFO_POP (rw) register accessor: APB in FIFO mode register

You can [`read`](crate::generic::Reg::read) this register and get [`infifo_pop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infifo_pop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@infifo_pop`] module*/
pub type INFIFO_POP = crate::Reg<infifo_pop::INFIFO_POP_SPEC>;
///APB in FIFO mode register
pub mod infifo_pop;
/**LC_STATE0 (r) register accessor: I2S DMA TX status

You can [`read`](crate::generic::Reg::read) this register and get [`lc_state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_state0`] module*/
pub type LC_STATE0 = crate::Reg<lc_state0::LC_STATE0_SPEC>;
///I2S DMA TX status
pub mod lc_state0;
/**LC_STATE1 (r) register accessor: I2S DMA RX status

You can [`read`](crate::generic::Reg::read) this register and get [`lc_state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_state1`] module*/
pub type LC_STATE1 = crate::Reg<lc_state1::LC_STATE1_SPEC>;
///I2S DMA RX status
pub mod lc_state1;
/**LC_HUNG_CONF (rw) register accessor: I2S Hung configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_hung_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_hung_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_hung_conf`] module*/
pub type LC_HUNG_CONF = crate::Reg<lc_hung_conf::LC_HUNG_CONF_SPEC>;
///I2S Hung configuration register
pub mod lc_hung_conf;
/**CONF1 (rw) register accessor: I2S configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf1`] module*/
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
///I2S configuration register 1
pub mod conf1;
/**PD_CONF (rw) register accessor: I2S power-down configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pd_conf`] module*/
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
///I2S power-down configuration register
pub mod pd_conf;
/**CONF2 (rw) register accessor: I2S configuration register 2

You can [`read`](crate::generic::Reg::read) this register and get [`conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf2`] module*/
pub type CONF2 = crate::Reg<conf2::CONF2_SPEC>;
///I2S configuration register 2
pub mod conf2;
/**CLKM_CONF (rw) register accessor: I2S module clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clkm_conf`] module*/
pub type CLKM_CONF = crate::Reg<clkm_conf::CLKM_CONF_SPEC>;
///I2S module clock configuration register
pub mod clkm_conf;
/**SAMPLE_RATE_CONF (rw) register accessor: I2S sample rate register

You can [`read`](crate::generic::Reg::read) this register and get [`sample_rate_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_rate_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sample_rate_conf`] module*/
pub type SAMPLE_RATE_CONF = crate::Reg<sample_rate_conf::SAMPLE_RATE_CONF_SPEC>;
///I2S sample rate register
pub mod sample_rate_conf;
/**STATE (r) register accessor: I2S TX status register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state`] module*/
pub type STATE = crate::Reg<state::STATE_SPEC>;
///I2S TX status register
pub mod state;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
