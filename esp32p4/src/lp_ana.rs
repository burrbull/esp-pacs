#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    bod_mode0_cntl: BOD_MODE0_CNTL,
    bod_mode1_cntl: BOD_MODE1_CNTL,
    vdd_source_cntl: VDD_SOURCE_CNTL,
    vddbat_bod_cntl: VDDBAT_BOD_CNTL,
    vddbat_charge_cntl: VDDBAT_CHARGE_CNTL,
    ck_glitch_cntl: CK_GLITCH_CNTL,
    pg_glitch_cntl: PG_GLITCH_CNTL,
    fib_enable: FIB_ENABLE,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    lp_int_raw: LP_INT_RAW,
    lp_int_st: LP_INT_ST,
    lp_int_ena: LP_INT_ENA,
    lp_int_clr: LP_INT_CLR,
    _reserved16: [u8; 0xbc],
    touch_approach_work_meas_num: TOUCH_APPROACH_WORK_MEAS_NUM,
    touch_scan_ctrl1: TOUCH_SCAN_CTRL1,
    touch_scan_ctrl2: TOUCH_SCAN_CTRL2,
    touch_work: TOUCH_WORK,
    touch_work_meas_num: TOUCH_WORK_MEAS_NUM,
    touch_filter1: TOUCH_FILTER1,
    touch_filter2: TOUCH_FILTER2,
    touch_filter3: TOUCH_FILTER3,
    touch_slp0: TOUCH_SLP0,
    touch_slp1: TOUCH_SLP1,
    touch_clr: TOUCH_CLR,
    touch_approach: TOUCH_APPROACH,
    touch_freq0_scan_para: TOUCH_FREQ0_SCAN_PARA,
    touch_freq1_scan_para: TOUCH_FREQ1_SCAN_PARA,
    touch_freq2_scan_para: TOUCH_FREQ2_SCAN_PARA,
    touch_ana_para: TOUCH_ANA_PARA,
    touch_mux0: TOUCH_MUX0,
    touch_mux1: TOUCH_MUX1,
    touch_pad: [TOUCH_PAD; 15],
    _reserved35: [u8; 0x0204],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn bod_mode0_cntl(&self) -> &BOD_MODE0_CNTL {
        &self.bod_mode0_cntl
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn bod_mode1_cntl(&self) -> &BOD_MODE1_CNTL {
        &self.bod_mode1_cntl
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn vdd_source_cntl(&self) -> &VDD_SOURCE_CNTL {
        &self.vdd_source_cntl
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn vddbat_bod_cntl(&self) -> &VDDBAT_BOD_CNTL {
        &self.vddbat_bod_cntl
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn vddbat_charge_cntl(&self) -> &VDDBAT_CHARGE_CNTL {
        &self.vddbat_charge_cntl
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn ck_glitch_cntl(&self) -> &CK_GLITCH_CNTL {
        &self.ck_glitch_cntl
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn pg_glitch_cntl(&self) -> &PG_GLITCH_CNTL {
        &self.pg_glitch_cntl
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn fib_enable(&self) -> &FIB_ENABLE {
        &self.fib_enable
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LP_INT_RAW {
        &self.lp_int_raw
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LP_INT_ST {
        &self.lp_int_st
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LP_INT_ENA {
        &self.lp_int_ena
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LP_INT_CLR {
        &self.lp_int_clr
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn touch_approach_work_meas_num(&self) -> &TOUCH_APPROACH_WORK_MEAS_NUM {
        &self.touch_approach_work_meas_num
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn touch_scan_ctrl1(&self) -> &TOUCH_SCAN_CTRL1 {
        &self.touch_scan_ctrl1
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn touch_scan_ctrl2(&self) -> &TOUCH_SCAN_CTRL2 {
        &self.touch_scan_ctrl2
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn touch_work(&self) -> &TOUCH_WORK {
        &self.touch_work
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn touch_work_meas_num(&self) -> &TOUCH_WORK_MEAS_NUM {
        &self.touch_work_meas_num
    }
    #[doc = "0x110 - need_des"]
    #[inline(always)]
    pub const fn touch_filter1(&self) -> &TOUCH_FILTER1 {
        &self.touch_filter1
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn touch_filter2(&self) -> &TOUCH_FILTER2 {
        &self.touch_filter2
    }
    #[doc = "0x118 - need_des"]
    #[inline(always)]
    pub const fn touch_filter3(&self) -> &TOUCH_FILTER3 {
        &self.touch_filter3
    }
    #[doc = "0x11c - need_des"]
    #[inline(always)]
    pub const fn touch_slp0(&self) -> &TOUCH_SLP0 {
        &self.touch_slp0
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn touch_slp1(&self) -> &TOUCH_SLP1 {
        &self.touch_slp1
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn touch_clr(&self) -> &TOUCH_CLR {
        &self.touch_clr
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn touch_approach(&self) -> &TOUCH_APPROACH {
        &self.touch_approach
    }
    #[doc = "0x12c - need_des"]
    #[inline(always)]
    pub const fn touch_freq0_scan_para(&self) -> &TOUCH_FREQ0_SCAN_PARA {
        &self.touch_freq0_scan_para
    }
    #[doc = "0x130 - need_des"]
    #[inline(always)]
    pub const fn touch_freq1_scan_para(&self) -> &TOUCH_FREQ1_SCAN_PARA {
        &self.touch_freq1_scan_para
    }
    #[doc = "0x134 - need_des"]
    #[inline(always)]
    pub const fn touch_freq2_scan_para(&self) -> &TOUCH_FREQ2_SCAN_PARA {
        &self.touch_freq2_scan_para
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn touch_ana_para(&self) -> &TOUCH_ANA_PARA {
        &self.touch_ana_para
    }
    #[doc = "0x13c - need_des"]
    #[inline(always)]
    pub const fn touch_mux0(&self) -> &TOUCH_MUX0 {
        &self.touch_mux0
    }
    #[doc = "0x140 - need_des"]
    #[inline(always)]
    pub const fn touch_mux1(&self) -> &TOUCH_MUX1 {
        &self.touch_mux1
    }
    #[doc = "0x144..0x1f8 - Cluster TOUCH_PAD%s, containing TOUCH_PAD*_TH0, TOUCH_PAD*_TH1, TOUCH_PAD*_TH2"]
    #[inline(always)]
    pub const fn touch_pad(&self, n: usize) -> &TOUCH_PAD {
        &self.touch_pad[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x144..0x1f8 - Cluster TOUCH_PAD%s, containing TOUCH_PAD*_TH0, TOUCH_PAD*_TH1, TOUCH_PAD*_TH2"]
    #[inline(always)]
    pub fn touch_pad_iter(&self) -> impl Iterator<Item = &TOUCH_PAD> {
        self.touch_pad.iter()
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "BOD_MODE0_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bod_mode0_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bod_mode0_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod_mode0_cntl`] module"]
pub type BOD_MODE0_CNTL = crate::Reg<bod_mode0_cntl::BOD_MODE0_CNTL_SPEC>;
#[doc = "need_des"]
pub mod bod_mode0_cntl;
#[doc = "BOD_MODE1_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bod_mode1_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bod_mode1_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod_mode1_cntl`] module"]
pub type BOD_MODE1_CNTL = crate::Reg<bod_mode1_cntl::BOD_MODE1_CNTL_SPEC>;
#[doc = "need_des"]
pub mod bod_mode1_cntl;
#[doc = "VDD_SOURCE_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_source_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_source_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_source_cntl`] module"]
pub type VDD_SOURCE_CNTL = crate::Reg<vdd_source_cntl::VDD_SOURCE_CNTL_SPEC>;
#[doc = "need_des"]
pub mod vdd_source_cntl;
#[doc = "VDDBAT_BOD_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vddbat_bod_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddbat_bod_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vddbat_bod_cntl`] module"]
pub type VDDBAT_BOD_CNTL = crate::Reg<vddbat_bod_cntl::VDDBAT_BOD_CNTL_SPEC>;
#[doc = "need_des"]
pub mod vddbat_bod_cntl;
#[doc = "VDDBAT_CHARGE_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vddbat_charge_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddbat_charge_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vddbat_charge_cntl`] module"]
pub type VDDBAT_CHARGE_CNTL = crate::Reg<vddbat_charge_cntl::VDDBAT_CHARGE_CNTL_SPEC>;
#[doc = "need_des"]
pub mod vddbat_charge_cntl;
#[doc = "CK_GLITCH_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ck_glitch_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_glitch_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ck_glitch_cntl`] module"]
pub type CK_GLITCH_CNTL = crate::Reg<ck_glitch_cntl::CK_GLITCH_CNTL_SPEC>;
#[doc = "need_des"]
pub mod ck_glitch_cntl;
#[doc = "PG_GLITCH_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_glitch_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_glitch_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pg_glitch_cntl`] module"]
pub type PG_GLITCH_CNTL = crate::Reg<pg_glitch_cntl::PG_GLITCH_CNTL_SPEC>;
#[doc = "need_des"]
pub mod pg_glitch_cntl;
#[doc = "FIB_ENABLE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fib_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fib_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fib_enable`] module"]
pub type FIB_ENABLE = crate::Reg<fib_enable::FIB_ENABLE_SPEC>;
#[doc = "need_des"]
pub mod fib_enable;
#[doc = "INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "need_des"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod int_clr;
#[doc = "LP_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_raw`] module"]
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_st`] module"]
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_ena`] module"]
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_clr`] module"]
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_int_clr;
#[doc = "TOUCH_APPROACH_WORK_MEAS_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_approach_work_meas_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach_work_meas_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_approach_work_meas_num`] module"]
pub type TOUCH_APPROACH_WORK_MEAS_NUM =
    crate::Reg<touch_approach_work_meas_num::TOUCH_APPROACH_WORK_MEAS_NUM_SPEC>;
#[doc = "need_des"]
pub mod touch_approach_work_meas_num;
#[doc = "TOUCH_SCAN_CTRL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_scan_ctrl1`] module"]
pub type TOUCH_SCAN_CTRL1 = crate::Reg<touch_scan_ctrl1::TOUCH_SCAN_CTRL1_SPEC>;
#[doc = "need_des"]
pub mod touch_scan_ctrl1;
#[doc = "TOUCH_SCAN_CTRL2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_scan_ctrl2`] module"]
pub type TOUCH_SCAN_CTRL2 = crate::Reg<touch_scan_ctrl2::TOUCH_SCAN_CTRL2_SPEC>;
#[doc = "need_des"]
pub mod touch_scan_ctrl2;
#[doc = "TOUCH_WORK (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_work::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_work::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_work`] module"]
pub type TOUCH_WORK = crate::Reg<touch_work::TOUCH_WORK_SPEC>;
#[doc = "need_des"]
pub mod touch_work;
#[doc = "TOUCH_WORK_MEAS_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_work_meas_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_work_meas_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_work_meas_num`] module"]
pub type TOUCH_WORK_MEAS_NUM = crate::Reg<touch_work_meas_num::TOUCH_WORK_MEAS_NUM_SPEC>;
#[doc = "need_des"]
pub mod touch_work_meas_num;
#[doc = "TOUCH_FILTER1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_filter1`] module"]
pub type TOUCH_FILTER1 = crate::Reg<touch_filter1::TOUCH_FILTER1_SPEC>;
#[doc = "need_des"]
pub mod touch_filter1;
#[doc = "TOUCH_FILTER2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_filter2`] module"]
pub type TOUCH_FILTER2 = crate::Reg<touch_filter2::TOUCH_FILTER2_SPEC>;
#[doc = "need_des"]
pub mod touch_filter2;
#[doc = "TOUCH_FILTER3 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_filter3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_filter3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_filter3`] module"]
pub type TOUCH_FILTER3 = crate::Reg<touch_filter3::TOUCH_FILTER3_SPEC>;
#[doc = "need_des"]
pub mod touch_filter3;
#[doc = "TOUCH_SLP0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_slp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_slp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_slp0`] module"]
pub type TOUCH_SLP0 = crate::Reg<touch_slp0::TOUCH_SLP0_SPEC>;
#[doc = "need_des"]
pub mod touch_slp0;
#[doc = "TOUCH_SLP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_slp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_slp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_slp1`] module"]
pub type TOUCH_SLP1 = crate::Reg<touch_slp1::TOUCH_SLP1_SPEC>;
#[doc = "need_des"]
pub mod touch_slp1;
#[doc = "TOUCH_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_clr`] module"]
pub type TOUCH_CLR = crate::Reg<touch_clr::TOUCH_CLR_SPEC>;
#[doc = "need_des"]
pub mod touch_clr;
#[doc = "TOUCH_APPROACH (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_approach`] module"]
pub type TOUCH_APPROACH = crate::Reg<touch_approach::TOUCH_APPROACH_SPEC>;
#[doc = "need_des"]
pub mod touch_approach;
#[doc = "TOUCH_FREQ0_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_freq0_scan_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_freq0_scan_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_freq0_scan_para`] module"]
pub type TOUCH_FREQ0_SCAN_PARA = crate::Reg<touch_freq0_scan_para::TOUCH_FREQ0_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod touch_freq0_scan_para;
#[doc = "TOUCH_FREQ1_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_freq1_scan_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_freq1_scan_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_freq1_scan_para`] module"]
pub type TOUCH_FREQ1_SCAN_PARA = crate::Reg<touch_freq1_scan_para::TOUCH_FREQ1_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod touch_freq1_scan_para;
#[doc = "TOUCH_FREQ2_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_freq2_scan_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_freq2_scan_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_freq2_scan_para`] module"]
pub type TOUCH_FREQ2_SCAN_PARA = crate::Reg<touch_freq2_scan_para::TOUCH_FREQ2_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod touch_freq2_scan_para;
#[doc = "TOUCH_ANA_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ana_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ana_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_ana_para`] module"]
pub type TOUCH_ANA_PARA = crate::Reg<touch_ana_para::TOUCH_ANA_PARA_SPEC>;
#[doc = "need_des"]
pub mod touch_ana_para;
#[doc = "TOUCH_MUX0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_mux0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_mux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_mux0`] module"]
pub type TOUCH_MUX0 = crate::Reg<touch_mux0::TOUCH_MUX0_SPEC>;
#[doc = "need_des"]
pub mod touch_mux0;
#[doc = "TOUCH_MUX1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_mux1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_mux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_mux1`] module"]
pub type TOUCH_MUX1 = crate::Reg<touch_mux1::TOUCH_MUX1_SPEC>;
#[doc = "need_des"]
pub mod touch_mux1;
#[doc = "Cluster TOUCH_PAD%s, containing TOUCH_PAD*_TH0, TOUCH_PAD*_TH1, TOUCH_PAD*_TH2"]
pub use self::touch_pad::TOUCH_PAD;
#[doc = r"Cluster"]
#[doc = "Cluster TOUCH_PAD%s, containing TOUCH_PAD*_TH0, TOUCH_PAD*_TH1, TOUCH_PAD*_TH2"]
pub mod touch_pad;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
