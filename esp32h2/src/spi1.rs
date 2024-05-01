#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    cmd: CMD,
    addr: ADDR,
    ctrl: CTRL,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    clock: CLOCK,
    user: USER,
    user1: USER1,
    user2: USER2,
    mosi_dlen: MOSI_DLEN,
    miso_dlen: MISO_DLEN,
    rd_status: RD_STATUS,
    _reserved12: [u8; 0x04],
    misc: MISC,
    tx_crc: TX_CRC,
    cache_fctrl: CACHE_FCTRL,
    _reserved15: [u8; 0x18],
    w: [W; 16],
    flash_waiti_ctrl: FLASH_WAITI_CTRL,
    flash_sus_ctrl: FLASH_SUS_CTRL,
    flash_sus_cmd: FLASH_SUS_CMD,
    sus_status: SUS_STATUS,
    _reserved20: [u8; 0x18],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved24: [u8; 0x04],
    ddr: DDR,
    _reserved25: [u8; 0xa8],
    timing_cali: TIMING_CALI,
    _reserved26: [u8; 0x7c],
    clock_gate: CLOCK_GATE,
    _reserved27: [u8; 0x01f8],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - SPI1 memory command register
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x04 - SPI1 address register
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    ///0x08 - SPI1 control register.
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x0c - SPI1 control1 register.
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    ///0x10 - SPI1 control2 register.
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    ///0x14 - SPI1 clock division control register.
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    ///0x18 - SPI1 user register.
    #[inline(always)]
    pub const fn user(&self) -> &USER {
        &self.user
    }
    ///0x1c - SPI1 user1 register.
    #[inline(always)]
    pub const fn user1(&self) -> &USER1 {
        &self.user1
    }
    ///0x20 - SPI1 user2 register.
    #[inline(always)]
    pub const fn user2(&self) -> &USER2 {
        &self.user2
    }
    ///0x24 - SPI1 send data bit length control register.
    #[inline(always)]
    pub const fn mosi_dlen(&self) -> &MOSI_DLEN {
        &self.mosi_dlen
    }
    ///0x28 - SPI1 receive data bit length control register.
    #[inline(always)]
    pub const fn miso_dlen(&self) -> &MISO_DLEN {
        &self.miso_dlen
    }
    ///0x2c - SPI1 status register.
    #[inline(always)]
    pub const fn rd_status(&self) -> &RD_STATUS {
        &self.rd_status
    }
    ///0x34 - SPI1 misc register
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x38 - SPI1 TX CRC data register.
    #[inline(always)]
    pub const fn tx_crc(&self) -> &TX_CRC {
        &self.tx_crc
    }
    ///0x3c - SPI1 bit mode control register.
    #[inline(always)]
    pub const fn cache_fctrl(&self) -> &CACHE_FCTRL {
        &self.cache_fctrl
    }
    ///0x58..0x98 - SPI1 memory data buffer%s
    #[inline(always)]
    pub const fn w(&self, n: usize) -> &W {
        &self.w[n]
    }
    ///Iterator for array of:
    ///0x58..0x98 - SPI1 memory data buffer%s
    #[inline(always)]
    pub fn w_iter(&self) -> impl Iterator<Item = &W> {
        self.w.iter()
    }
    ///0x98 - SPI1 wait idle control register
    #[inline(always)]
    pub const fn flash_waiti_ctrl(&self) -> &FLASH_WAITI_CTRL {
        &self.flash_waiti_ctrl
    }
    ///0x9c - SPI1 flash suspend control register
    #[inline(always)]
    pub const fn flash_sus_ctrl(&self) -> &FLASH_SUS_CTRL {
        &self.flash_sus_ctrl
    }
    ///0xa0 - SPI1 flash suspend command register
    #[inline(always)]
    pub const fn flash_sus_cmd(&self) -> &FLASH_SUS_CMD {
        &self.flash_sus_cmd
    }
    ///0xa4 - SPI1 flash suspend status register
    #[inline(always)]
    pub const fn sus_status(&self) -> &SUS_STATUS {
        &self.sus_status
    }
    ///0xc0 - SPI1 interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0xc4 - SPI1 interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0xc8 - SPI1 interrupt raw register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0xcc - SPI1 interrupt status register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0xd4 - SPI1 DDR control register
    #[inline(always)]
    pub const fn ddr(&self) -> &DDR {
        &self.ddr
    }
    ///0x180 - SPI1 timing control register
    #[inline(always)]
    pub const fn timing_cali(&self) -> &TIMING_CALI {
        &self.timing_cali
    }
    ///0x200 - SPI1 clk_gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x3fc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CMD (rw) register accessor: SPI1 memory command register

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///SPI1 memory command register
pub mod cmd;
/**ADDR (rw) register accessor: SPI1 address register

You can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
///SPI1 address register
pub mod addr;
/**CTRL (rw) register accessor: SPI1 control register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SPI1 control register.
pub mod ctrl;
/**CTRL1 (rw) register accessor: SPI1 control1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl1`] module*/
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
///SPI1 control1 register.
pub mod ctrl1;
/**CTRL2 (w) register accessor: SPI1 control2 register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl2`] module*/
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
///SPI1 control2 register.
pub mod ctrl2;
/**CLOCK (rw) register accessor: SPI1 clock division control register.

You can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock`] module*/
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
///SPI1 clock division control register.
pub mod clock;
/**USER (rw) register accessor: SPI1 user register.

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user`] module*/
pub type USER = crate::Reg<user::USER_SPEC>;
///SPI1 user register.
pub mod user;
/**USER1 (rw) register accessor: SPI1 user1 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user1`] module*/
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
///SPI1 user1 register.
pub mod user1;
/**USER2 (rw) register accessor: SPI1 user2 register.

You can [`read`](crate::generic::Reg::read) this register and get [`user2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@user2`] module*/
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
///SPI1 user2 register.
pub mod user2;
/**MOSI_DLEN (rw) register accessor: SPI1 send data bit length control register.

You can [`read`](crate::generic::Reg::read) this register and get [`mosi_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mosi_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mosi_dlen`] module*/
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
///SPI1 send data bit length control register.
pub mod mosi_dlen;
/**MISO_DLEN (rw) register accessor: SPI1 receive data bit length control register.

You can [`read`](crate::generic::Reg::read) this register and get [`miso_dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`miso_dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@miso_dlen`] module*/
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
///SPI1 receive data bit length control register.
pub mod miso_dlen;
/**RD_STATUS (rw) register accessor: SPI1 status register.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_status`] module*/
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
///SPI1 status register.
pub mod rd_status;
/**MISC (rw) register accessor: SPI1 misc register

You can [`read`](crate::generic::Reg::read) this register and get [`misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISC_SPEC>;
///SPI1 misc register
pub mod misc;
/**TX_CRC (r) register accessor: SPI1 TX CRC data register.

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_crc`] module*/
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
///SPI1 TX CRC data register.
pub mod tx_crc;
/**CACHE_FCTRL (rw) register accessor: SPI1 bit mode control register.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_fctrl`] module*/
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
///SPI1 bit mode control register.
pub mod cache_fctrl;
/**W (rw) register accessor: SPI1 memory data buffer%s

You can [`read`](crate::generic::Reg::read) this register and get [`w::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@w`] module*/
pub type W = crate::Reg<w::W_SPEC>;
///SPI1 memory data buffer%s
pub mod w;
/**FLASH_WAITI_CTRL (rw) register accessor: SPI1 wait idle control register

You can [`read`](crate::generic::Reg::read) this register and get [`flash_waiti_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_waiti_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flash_waiti_ctrl`] module*/
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
///SPI1 wait idle control register
pub mod flash_waiti_ctrl;
/**FLASH_SUS_CTRL (rw) register accessor: SPI1 flash suspend control register

You can [`read`](crate::generic::Reg::read) this register and get [`flash_sus_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flash_sus_ctrl`] module*/
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
///SPI1 flash suspend control register
pub mod flash_sus_ctrl;
/**FLASH_SUS_CMD (rw) register accessor: SPI1 flash suspend command register

You can [`read`](crate::generic::Reg::read) this register and get [`flash_sus_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_sus_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flash_sus_cmd`] module*/
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
///SPI1 flash suspend command register
pub mod flash_sus_cmd;
/**SUS_STATUS (rw) register accessor: SPI1 flash suspend status register

You can [`read`](crate::generic::Reg::read) this register and get [`sus_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sus_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sus_status`] module*/
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
///SPI1 flash suspend status register
pub mod sus_status;
/**INT_ENA (rw) register accessor: SPI1 interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///SPI1 interrupt enable register
pub mod int_ena;
/**INT_CLR (w) register accessor: SPI1 interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///SPI1 interrupt clear register
pub mod int_clr;
/**INT_RAW (rw) register accessor: SPI1 interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///SPI1 interrupt raw register
pub mod int_raw;
/**INT_ST (r) register accessor: SPI1 interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///SPI1 interrupt status register
pub mod int_st;
/**DDR (r) register accessor: SPI1 DDR control register

You can [`read`](crate::generic::Reg::read) this register and get [`ddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ddr`] module*/
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
///SPI1 DDR control register
pub mod ddr;
/**TIMING_CALI (rw) register accessor: SPI1 timing control register

You can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing_cali::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timing_cali`] module*/
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
///SPI1 timing control register
pub mod timing_cali;
/**CLOCK_GATE (rw) register accessor: SPI1 clk_gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///SPI1 clk_gate register
pub mod clock_gate;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
