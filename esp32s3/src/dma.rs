#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ch: [CH; 5],
    ahb_test: AHB_TEST,
    pd_conf: PD_CONF,
    misc_conf: MISC_CONF,
    in_sram_size_ch: (),
    _reserved5: [u8; 0x04],
    out_sram_size_ch: (),
    _reserved6: [u8; 0x24],
    extmem_reject_addr: EXTMEM_REJECT_ADDR,
    extmem_reject_st: EXTMEM_REJECT_ST,
    extmem_reject_int_raw: EXTMEM_REJECT_INT_RAW,
    extmem_reject_int_st: EXTMEM_REJECT_INT_ST,
    extmem_reject_int_ena: EXTMEM_REJECT_INT_ENA,
    extmem_reject_int_clr: EXTMEM_REJECT_INT_CLR,
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x3c0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_WIGHT_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_WIGHT_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x00..0x3c0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_WIGHT_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_WIGHT_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    ///0x3c0 - reserved
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    ///0x3c4 - reserved
    #[inline(always)]
    pub const fn pd_conf(&self) -> &PD_CONF {
        &self.pd_conf
    }
    ///0x3c8 - MISC register
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    ///0x3cc..0x3e0 - Receive L2 FIFO depth of Rx channel 0
    #[inline(always)]
    pub const fn in_sram_size_ch(&self, n: usize) -> &IN_SRAM_SIZE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(972)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x3cc..0x3e0 - Receive L2 FIFO depth of Rx channel 0
    #[inline(always)]
    pub fn in_sram_size_ch_iter(&self) -> impl Iterator<Item = &IN_SRAM_SIZE_CH> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(972)
                .add(8 * n)
                .cast()
        })
    }
    ///0x3d0..0x3e4 - Transmit L2 FIFO depth of Tx channel 0
    #[inline(always)]
    pub const fn out_sram_size_ch(&self, n: usize) -> &OUT_SRAM_SIZE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(976)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x3d0..0x3e4 - Transmit L2 FIFO depth of Tx channel 0
    #[inline(always)]
    pub fn out_sram_size_ch_iter(&self) -> impl Iterator<Item = &OUT_SRAM_SIZE_CH> {
        (0..5).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(976)
                .add(8 * n)
                .cast()
        })
    }
    ///0x3f4 - Reject address accessing external RAM
    #[inline(always)]
    pub const fn extmem_reject_addr(&self) -> &EXTMEM_REJECT_ADDR {
        &self.extmem_reject_addr
    }
    ///0x3f8 - Reject status accessing external RAM
    #[inline(always)]
    pub const fn extmem_reject_st(&self) -> &EXTMEM_REJECT_ST {
        &self.extmem_reject_st
    }
    ///0x3fc - Raw interrupt status of external RAM permission
    #[inline(always)]
    pub const fn extmem_reject_int_raw(&self) -> &EXTMEM_REJECT_INT_RAW {
        &self.extmem_reject_int_raw
    }
    ///0x400 - Masked interrupt status of external RAM permission
    #[inline(always)]
    pub const fn extmem_reject_int_st(&self) -> &EXTMEM_REJECT_INT_ST {
        &self.extmem_reject_int_st
    }
    ///0x404 - Interrupt enable bits of external RAM permission
    #[inline(always)]
    pub const fn extmem_reject_int_ena(&self) -> &EXTMEM_REJECT_INT_ENA {
        &self.extmem_reject_int_ena
    }
    ///0x408 - Interrupt clear bits of external RAM permission
    #[inline(always)]
    pub const fn extmem_reject_int_clr(&self) -> &EXTMEM_REJECT_INT_CLR {
        &self.extmem_reject_int_clr
    }
    ///0x40c - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
///Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_WIGHT_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_WIGHT_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_WIGHT_CH?, IN_PRI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_WIGHT_CH?, OUT_PRI_CH?, OUT_PERI_SEL_CH?
pub mod ch;
/**AHB_TEST (rw) register accessor: reserved

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ahb_test`] module*/
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
///reserved
pub mod ahb_test;
/**PD_CONF (rw) register accessor: reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pd_conf`] module*/
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
///reserved
pub mod pd_conf;
/**MISC_CONF (rw) register accessor: MISC register

You can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc_conf`] module*/
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
///MISC register
pub mod misc_conf;
/**IN_SRAM_SIZE_CH (rw) register accessor: Receive L2 FIFO depth of Rx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`in_sram_size_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_sram_size_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_sram_size_ch`] module*/
pub type IN_SRAM_SIZE_CH = crate::Reg<in_sram_size_ch::IN_SRAM_SIZE_CH_SPEC>;
///Receive L2 FIFO depth of Rx channel 0
pub mod in_sram_size_ch;
/**OUT_SRAM_SIZE_CH (rw) register accessor: Transmit L2 FIFO depth of Tx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`out_sram_size_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_sram_size_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_sram_size_ch`] module*/
pub type OUT_SRAM_SIZE_CH = crate::Reg<out_sram_size_ch::OUT_SRAM_SIZE_CH_SPEC>;
///Transmit L2 FIFO depth of Tx channel 0
pub mod out_sram_size_ch;
/**EXTMEM_REJECT_ADDR (r) register accessor: Reject address accessing external RAM

You can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extmem_reject_addr`] module*/
pub type EXTMEM_REJECT_ADDR = crate::Reg<extmem_reject_addr::EXTMEM_REJECT_ADDR_SPEC>;
///Reject address accessing external RAM
pub mod extmem_reject_addr;
/**EXTMEM_REJECT_ST (r) register accessor: Reject status accessing external RAM

You can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extmem_reject_st`] module*/
pub type EXTMEM_REJECT_ST = crate::Reg<extmem_reject_st::EXTMEM_REJECT_ST_SPEC>;
///Reject status accessing external RAM
pub mod extmem_reject_st;
/**EXTMEM_REJECT_INT_RAW (rw) register accessor: Raw interrupt status of external RAM permission

You can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extmem_reject_int_raw`] module*/
pub type EXTMEM_REJECT_INT_RAW = crate::Reg<extmem_reject_int_raw::EXTMEM_REJECT_INT_RAW_SPEC>;
///Raw interrupt status of external RAM permission
pub mod extmem_reject_int_raw;
/**EXTMEM_REJECT_INT_ST (r) register accessor: Masked interrupt status of external RAM permission

You can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extmem_reject_int_st`] module*/
pub type EXTMEM_REJECT_INT_ST = crate::Reg<extmem_reject_int_st::EXTMEM_REJECT_INT_ST_SPEC>;
///Masked interrupt status of external RAM permission
pub mod extmem_reject_int_st;
/**EXTMEM_REJECT_INT_ENA (rw) register accessor: Interrupt enable bits of external RAM permission

You can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extmem_reject_int_ena`] module*/
pub type EXTMEM_REJECT_INT_ENA = crate::Reg<extmem_reject_int_ena::EXTMEM_REJECT_INT_ENA_SPEC>;
///Interrupt enable bits of external RAM permission
pub mod extmem_reject_int_ena;
/**EXTMEM_REJECT_INT_CLR (w) register accessor: Interrupt clear bits of external RAM permission

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@extmem_reject_int_clr`] module*/
pub type EXTMEM_REJECT_INT_CLR = crate::Reg<extmem_reject_int_clr::EXTMEM_REJECT_INT_CLR_SPEC>;
///Interrupt clear bits of external RAM permission
pub mod extmem_reject_int_clr;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
