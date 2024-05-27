#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    t: [T; 2],
    wdtconfig0: WDTCONFIG0,
    wdtconfig1: WDTCONFIG1,
    wdtconfig2: WDTCONFIG2,
    wdtconfig3: WDTCONFIG3,
    wdtconfig4: WDTCONFIG4,
    wdtconfig5: WDTCONFIG5,
    wdtfeed: WDTFEED,
    wdtwprotect: WDTWPROTECT,
    rtccalicfg: RTCCALICFG,
    rtccalicfg1: RTCCALICFG1,
    lactconfig: LACTCONFIG,
    lactrtc: LACTRTC,
    lactlo: LACTLO,
    lacthi: LACTHI,
    lactupdate: LACTUPDATE,
    lactalarmlo: LACTALARMLO,
    lactalarmhi: LACTALARMHI,
    lactloadlo: LACTLOADLO,
    lactloadhi: LACTLOADHI,
    lactload: LACTLOAD,
    int_ena_timers: INT_ENA_TIMERS,
    int_raw_timers: INT_RAW_TIMERS,
    int_st_timers: INT_ST_TIMERS,
    int_clr_timers: INT_CLR_TIMERS,
    _reserved25: [u8; 0x50],
    ntimers_date: NTIMERS_DATE,
    timgclk: TIMGCLK,
}
impl RegisterBlock {
    ///0x00..0x48 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub const fn t(&self, n: usize) -> &T {
        &self.t[n]
    }
    ///Iterator for array of:
    ///0x00..0x48 - Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = &T> {
        self.t.iter()
    }
    ///0x48 -
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    ///0x4c -
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    ///0x50 -
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    ///0x54 -
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    ///0x58 -
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    ///0x5c -
    #[inline(always)]
    pub const fn wdtconfig5(&self) -> &WDTCONFIG5 {
        &self.wdtconfig5
    }
    ///0x60 -
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    ///0x64 -
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    ///0x68 -
    #[inline(always)]
    pub const fn rtccalicfg(&self) -> &RTCCALICFG {
        &self.rtccalicfg
    }
    ///0x6c -
    #[inline(always)]
    pub const fn rtccalicfg1(&self) -> &RTCCALICFG1 {
        &self.rtccalicfg1
    }
    ///0x70 -
    #[inline(always)]
    pub const fn lactconfig(&self) -> &LACTCONFIG {
        &self.lactconfig
    }
    ///0x74 -
    #[inline(always)]
    pub const fn lactrtc(&self) -> &LACTRTC {
        &self.lactrtc
    }
    ///0x78 -
    #[inline(always)]
    pub const fn lactlo(&self) -> &LACTLO {
        &self.lactlo
    }
    ///0x7c -
    #[inline(always)]
    pub const fn lacthi(&self) -> &LACTHI {
        &self.lacthi
    }
    ///0x80 -
    #[inline(always)]
    pub const fn lactupdate(&self) -> &LACTUPDATE {
        &self.lactupdate
    }
    ///0x84 -
    #[inline(always)]
    pub const fn lactalarmlo(&self) -> &LACTALARMLO {
        &self.lactalarmlo
    }
    ///0x88 -
    #[inline(always)]
    pub const fn lactalarmhi(&self) -> &LACTALARMHI {
        &self.lactalarmhi
    }
    ///0x8c -
    #[inline(always)]
    pub const fn lactloadlo(&self) -> &LACTLOADLO {
        &self.lactloadlo
    }
    ///0x90 -
    #[inline(always)]
    pub const fn lactloadhi(&self) -> &LACTLOADHI {
        &self.lactloadhi
    }
    ///0x94 -
    #[inline(always)]
    pub const fn lactload(&self) -> &LACTLOAD {
        &self.lactload
    }
    ///0x98 -
    #[inline(always)]
    pub const fn int_ena_timers(&self) -> &INT_ENA_TIMERS {
        &self.int_ena_timers
    }
    ///0x9c -
    #[inline(always)]
    pub const fn int_raw_timers(&self) -> &INT_RAW_TIMERS {
        &self.int_raw_timers
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn int_st_timers(&self) -> &INT_ST_TIMERS {
        &self.int_st_timers
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn int_clr_timers(&self) -> &INT_CLR_TIMERS {
        &self.int_clr_timers
    }
    ///0xf8 -
    #[inline(always)]
    pub const fn ntimers_date(&self) -> &NTIMERS_DATE {
        &self.ntimers_date
    }
    ///0xfc -
    #[inline(always)]
    pub const fn timgclk(&self) -> &TIMGCLK {
        &self.timgclk
    }
}
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub use self::t::T;
///Cluster
///Cluster T%s, containing T?CONFIG, T?LO, T?HI, T?UPDATE, T?ALARMLO, T?ALARMHI, T?LOADLO, T?LOADHI, T?LOAD
pub mod t;
/**WDTCONFIG0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig0`] module*/
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
///
pub mod wdtconfig0;
/**WDTCONFIG1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig1`] module*/
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
///
pub mod wdtconfig1;
/**WDTCONFIG2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig2`] module*/
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
///
pub mod wdtconfig2;
/**WDTCONFIG3 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig3`] module*/
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
///
pub mod wdtconfig3;
/**WDTCONFIG4 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig4`] module*/
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
///
pub mod wdtconfig4;
/**WDTCONFIG5 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig5`] module*/
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
///
pub mod wdtconfig5;
/**WDTFEED (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtfeed`] module*/
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
///
pub mod wdtfeed;
/**WDTWPROTECT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtwprotect`] module*/
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
///
pub mod wdtwprotect;
/**RTCCALICFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccalicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg`] module*/
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
///
pub mod rtccalicfg;
/**RTCCALICFG1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rtccalicfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccalicfg1`] module*/
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
///
pub mod rtccalicfg1;
/**LACTCONFIG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactconfig`] module*/
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
///
pub mod lactconfig;
/**LACTRTC (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactrtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactrtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactrtc`] module*/
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
///
pub mod lactrtc;
/**LACTLO (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactlo`] module*/
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
///
pub mod lactlo;
/**LACTHI (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lacthi`] module*/
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
///
pub mod lacthi;
/**LACTUPDATE (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactupdate`] module*/
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
///
pub mod lactupdate;
/**LACTALARMLO (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactalarmlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactalarmlo`] module*/
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
///
pub mod lactalarmlo;
/**LACTALARMHI (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactalarmhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactalarmhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactalarmhi`] module*/
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
///
pub mod lactalarmhi;
/**LACTLOADLO (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactloadlo`] module*/
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
///
pub mod lactloadlo;
/**LACTLOADHI (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`lactloadhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactloadhi`] module*/
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
///
pub mod lactloadhi;
/**LACTLOAD (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactload::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lactload`] module*/
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
///
pub mod lactload;
/**INT_ENA_TIMERS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena_timers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_timers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_timers`] module*/
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
///
pub mod int_ena_timers;
/**INT_RAW_TIMERS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw_timers`] module*/
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
///
pub mod int_raw_timers;
/**INT_ST_TIMERS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st_timers::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st_timers`] module*/
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
///
pub mod int_st_timers;
/**INT_CLR_TIMERS (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr_timers::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr_timers`] module*/
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
///
pub mod int_clr_timers;
/**NTIMERS_DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ntimers_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ntimers_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ntimers_date`] module*/
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
///
pub mod ntimers_date;
/**TIMGCLK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`timgclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timgclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timgclk`] module*/
pub type TIMGCLK = crate::Reg<timgclk::TIMGCLK_SPEC>;
///
pub mod timgclk;
