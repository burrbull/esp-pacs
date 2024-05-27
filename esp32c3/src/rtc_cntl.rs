#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    options0: OPTIONS0,
    slp_timer0: SLP_TIMER0,
    slp_timer1: SLP_TIMER1,
    time_update: TIME_UPDATE,
    time_low0: TIME_LOW0,
    time_high0: TIME_HIGH0,
    state0: STATE0,
    timer1: TIMER1,
    timer2: TIMER2,
    timer3: TIMER3,
    timer4: TIMER4,
    timer5: TIMER5,
    timer6: TIMER6,
    ana_conf: ANA_CONF,
    reset_state: RESET_STATE,
    wakeup_state: WAKEUP_STATE,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    store0: STORE0,
    store1: STORE1,
    store2: STORE2,
    store3: STORE3,
    ext_xtl_conf: EXT_XTL_CONF,
    ext_wakeup_conf: EXT_WAKEUP_CONF,
    slp_reject_conf: SLP_REJECT_CONF,
    cpu_period_conf: CPU_PERIOD_CONF,
    clk_conf: CLK_CONF,
    slow_clk_conf: SLOW_CLK_CONF,
    sdio_conf: SDIO_CONF,
    bias_conf: BIAS_CONF,
    rtc_cntl: RTC_CNTL,
    pwc: PWC,
    dig_pwc: DIG_PWC,
    dig_iso: DIG_ISO,
    wdtconfig0: WDTCONFIG0,
    wdtconfig1: WDTCONFIG1,
    wdtconfig2: WDTCONFIG2,
    wdtconfig3: WDTCONFIG3,
    wdtconfig4: WDTCONFIG4,
    wdtfeed: WDTFEED,
    wdtwprotect: WDTWPROTECT,
    swd_conf: SWD_CONF,
    swd_wprotect: SWD_WPROTECT,
    sw_cpu_stall: SW_CPU_STALL,
    store4: STORE4,
    store5: STORE5,
    store6: STORE6,
    store7: STORE7,
    low_power_st: LOW_POWER_ST,
    diag0: DIAG0,
    pad_hold: PAD_HOLD,
    dig_pad_hold: DIG_PAD_HOLD,
    brown_out: BROWN_OUT,
    time_low1: TIME_LOW1,
    time_high1: TIME_HIGH1,
    xtal32k_clk_factor: XTAL32K_CLK_FACTOR,
    xtal32k_conf: XTAL32K_CONF,
    usb_conf: USB_CONF,
    slp_reject_cause: SLP_REJECT_CAUSE,
    option1: OPTION1,
    slp_wakeup_cause: SLP_WAKEUP_CAUSE,
    ulp_cp_timer_1: ULP_CP_TIMER_1,
    int_ena_rtc_w1ts: INT_ENA_RTC_W1TS,
    int_ena_rtc_w1tc: INT_ENA_RTC_W1TC,
    retention_ctrl: RETENTION_CTRL,
    fib_sel: FIB_SEL,
    gpio_wakeup: GPIO_WAKEUP,
    dbg_sel: DBG_SEL,
    dbg_map: DBG_MAP,
    sensor_ctrl: SENSOR_CTRL,
    dbg_sar_sel: DBG_SAR_SEL,
    pg_ctrl: PG_CTRL,
    _reserved74: [u8; 0xd4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - rtc configure register
    #[inline(always)]
    pub const fn options0(&self) -> &OPTIONS0 {
        &self.options0
    }
    ///0x04 - rtc configure register
    #[inline(always)]
    pub const fn slp_timer0(&self) -> &SLP_TIMER0 {
        &self.slp_timer0
    }
    ///0x08 - rtc configure register
    #[inline(always)]
    pub const fn slp_timer1(&self) -> &SLP_TIMER1 {
        &self.slp_timer1
    }
    ///0x0c - rtc configure register
    #[inline(always)]
    pub const fn time_update(&self) -> &TIME_UPDATE {
        &self.time_update
    }
    ///0x10 - rtc configure register
    #[inline(always)]
    pub const fn time_low0(&self) -> &TIME_LOW0 {
        &self.time_low0
    }
    ///0x14 - rtc configure register
    #[inline(always)]
    pub const fn time_high0(&self) -> &TIME_HIGH0 {
        &self.time_high0
    }
    ///0x18 - rtc configure register
    #[inline(always)]
    pub const fn state0(&self) -> &STATE0 {
        &self.state0
    }
    ///0x1c - rtc configure register
    #[inline(always)]
    pub const fn timer1(&self) -> &TIMER1 {
        &self.timer1
    }
    ///0x20 - rtc configure register
    #[inline(always)]
    pub const fn timer2(&self) -> &TIMER2 {
        &self.timer2
    }
    ///0x24 - rtc configure register
    #[inline(always)]
    pub const fn timer3(&self) -> &TIMER3 {
        &self.timer3
    }
    ///0x28 - rtc configure register
    #[inline(always)]
    pub const fn timer4(&self) -> &TIMER4 {
        &self.timer4
    }
    ///0x2c - rtc configure register
    #[inline(always)]
    pub const fn timer5(&self) -> &TIMER5 {
        &self.timer5
    }
    ///0x30 - rtc configure register
    #[inline(always)]
    pub const fn timer6(&self) -> &TIMER6 {
        &self.timer6
    }
    ///0x34 - rtc configure register
    #[inline(always)]
    pub const fn ana_conf(&self) -> &ANA_CONF {
        &self.ana_conf
    }
    ///0x38 - rtc configure register
    #[inline(always)]
    pub const fn reset_state(&self) -> &RESET_STATE {
        &self.reset_state
    }
    ///0x3c - rtc configure register
    #[inline(always)]
    pub const fn wakeup_state(&self) -> &WAKEUP_STATE {
        &self.wakeup_state
    }
    ///0x40 - rtc configure register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x44 - rtc configure register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x48 - rtc configure register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x4c - rtc configure register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x50 - rtc configure register
    #[inline(always)]
    pub const fn store0(&self) -> &STORE0 {
        &self.store0
    }
    ///0x54 - rtc configure register
    #[inline(always)]
    pub const fn store1(&self) -> &STORE1 {
        &self.store1
    }
    ///0x58 - rtc configure register
    #[inline(always)]
    pub const fn store2(&self) -> &STORE2 {
        &self.store2
    }
    ///0x5c - rtc configure register
    #[inline(always)]
    pub const fn store3(&self) -> &STORE3 {
        &self.store3
    }
    ///0x60 - rtc configure register
    #[inline(always)]
    pub const fn ext_xtl_conf(&self) -> &EXT_XTL_CONF {
        &self.ext_xtl_conf
    }
    ///0x64 - rtc configure register
    #[inline(always)]
    pub const fn ext_wakeup_conf(&self) -> &EXT_WAKEUP_CONF {
        &self.ext_wakeup_conf
    }
    ///0x68 - rtc configure register
    #[inline(always)]
    pub const fn slp_reject_conf(&self) -> &SLP_REJECT_CONF {
        &self.slp_reject_conf
    }
    ///0x6c - rtc configure register
    #[inline(always)]
    pub const fn cpu_period_conf(&self) -> &CPU_PERIOD_CONF {
        &self.cpu_period_conf
    }
    ///0x70 - rtc configure register
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    ///0x74 - rtc configure register
    #[inline(always)]
    pub const fn slow_clk_conf(&self) -> &SLOW_CLK_CONF {
        &self.slow_clk_conf
    }
    ///0x78 - rtc configure register
    #[inline(always)]
    pub const fn sdio_conf(&self) -> &SDIO_CONF {
        &self.sdio_conf
    }
    ///0x7c - rtc configure register
    #[inline(always)]
    pub const fn bias_conf(&self) -> &BIAS_CONF {
        &self.bias_conf
    }
    ///0x80 - rtc configure register
    #[inline(always)]
    pub const fn rtc_cntl(&self) -> &RTC_CNTL {
        &self.rtc_cntl
    }
    ///0x84 - rtc configure register
    #[inline(always)]
    pub const fn pwc(&self) -> &PWC {
        &self.pwc
    }
    ///0x88 - rtc configure register
    #[inline(always)]
    pub const fn dig_pwc(&self) -> &DIG_PWC {
        &self.dig_pwc
    }
    ///0x8c - rtc configure register
    #[inline(always)]
    pub const fn dig_iso(&self) -> &DIG_ISO {
        &self.dig_iso
    }
    ///0x90 - rtc configure register
    #[inline(always)]
    pub const fn wdtconfig0(&self) -> &WDTCONFIG0 {
        &self.wdtconfig0
    }
    ///0x94 - rtc configure register
    #[inline(always)]
    pub const fn wdtconfig1(&self) -> &WDTCONFIG1 {
        &self.wdtconfig1
    }
    ///0x98 - rtc configure register
    #[inline(always)]
    pub const fn wdtconfig2(&self) -> &WDTCONFIG2 {
        &self.wdtconfig2
    }
    ///0x9c - rtc configure register
    #[inline(always)]
    pub const fn wdtconfig3(&self) -> &WDTCONFIG3 {
        &self.wdtconfig3
    }
    ///0xa0 - rtc configure register
    #[inline(always)]
    pub const fn wdtconfig4(&self) -> &WDTCONFIG4 {
        &self.wdtconfig4
    }
    ///0xa4 - rtc configure register
    #[inline(always)]
    pub const fn wdtfeed(&self) -> &WDTFEED {
        &self.wdtfeed
    }
    ///0xa8 - rtc configure register
    #[inline(always)]
    pub const fn wdtwprotect(&self) -> &WDTWPROTECT {
        &self.wdtwprotect
    }
    ///0xac - rtc configure register
    #[inline(always)]
    pub const fn swd_conf(&self) -> &SWD_CONF {
        &self.swd_conf
    }
    ///0xb0 - rtc configure register
    #[inline(always)]
    pub const fn swd_wprotect(&self) -> &SWD_WPROTECT {
        &self.swd_wprotect
    }
    ///0xb4 - rtc configure register
    #[inline(always)]
    pub const fn sw_cpu_stall(&self) -> &SW_CPU_STALL {
        &self.sw_cpu_stall
    }
    ///0xb8 - rtc configure register
    #[inline(always)]
    pub const fn store4(&self) -> &STORE4 {
        &self.store4
    }
    ///0xbc - rtc configure register
    #[inline(always)]
    pub const fn store5(&self) -> &STORE5 {
        &self.store5
    }
    ///0xc0 - rtc configure register
    #[inline(always)]
    pub const fn store6(&self) -> &STORE6 {
        &self.store6
    }
    ///0xc4 - rtc configure register
    #[inline(always)]
    pub const fn store7(&self) -> &STORE7 {
        &self.store7
    }
    ///0xc8 - rtc configure register
    #[inline(always)]
    pub const fn low_power_st(&self) -> &LOW_POWER_ST {
        &self.low_power_st
    }
    ///0xcc - rtc configure register
    #[inline(always)]
    pub const fn diag0(&self) -> &DIAG0 {
        &self.diag0
    }
    ///0xd0 - rtc configure register
    #[inline(always)]
    pub const fn pad_hold(&self) -> &PAD_HOLD {
        &self.pad_hold
    }
    ///0xd4 - rtc configure register
    #[inline(always)]
    pub const fn dig_pad_hold(&self) -> &DIG_PAD_HOLD {
        &self.dig_pad_hold
    }
    ///0xd8 - rtc configure register
    #[inline(always)]
    pub const fn brown_out(&self) -> &BROWN_OUT {
        &self.brown_out
    }
    ///0xdc - rtc configure register
    #[inline(always)]
    pub const fn time_low1(&self) -> &TIME_LOW1 {
        &self.time_low1
    }
    ///0xe0 - rtc configure register
    #[inline(always)]
    pub const fn time_high1(&self) -> &TIME_HIGH1 {
        &self.time_high1
    }
    ///0xe4 - rtc configure register
    #[inline(always)]
    pub const fn xtal32k_clk_factor(&self) -> &XTAL32K_CLK_FACTOR {
        &self.xtal32k_clk_factor
    }
    ///0xe8 - rtc configure register
    #[inline(always)]
    pub const fn xtal32k_conf(&self) -> &XTAL32K_CONF {
        &self.xtal32k_conf
    }
    ///0xec - rtc configure register
    #[inline(always)]
    pub const fn usb_conf(&self) -> &USB_CONF {
        &self.usb_conf
    }
    ///0xf0 - RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG
    #[inline(always)]
    pub const fn slp_reject_cause(&self) -> &SLP_REJECT_CAUSE {
        &self.slp_reject_cause
    }
    ///0xf4 - rtc configure register
    #[inline(always)]
    pub const fn option1(&self) -> &OPTION1 {
        &self.option1
    }
    ///0xf8 - RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG
    #[inline(always)]
    pub const fn slp_wakeup_cause(&self) -> &SLP_WAKEUP_CAUSE {
        &self.slp_wakeup_cause
    }
    ///0xfc - rtc configure register
    #[inline(always)]
    pub const fn ulp_cp_timer_1(&self) -> &ULP_CP_TIMER_1 {
        &self.ulp_cp_timer_1
    }
    ///0x100 - rtc configure register
    #[inline(always)]
    pub const fn int_ena_rtc_w1ts(&self) -> &INT_ENA_RTC_W1TS {
        &self.int_ena_rtc_w1ts
    }
    ///0x104 - rtc configure register
    #[inline(always)]
    pub const fn int_ena_rtc_w1tc(&self) -> &INT_ENA_RTC_W1TC {
        &self.int_ena_rtc_w1tc
    }
    ///0x108 - rtc configure register
    #[inline(always)]
    pub const fn retention_ctrl(&self) -> &RETENTION_CTRL {
        &self.retention_ctrl
    }
    ///0x10c - rtc configure register
    #[inline(always)]
    pub const fn fib_sel(&self) -> &FIB_SEL {
        &self.fib_sel
    }
    ///0x110 - rtc configure register
    #[inline(always)]
    pub const fn gpio_wakeup(&self) -> &GPIO_WAKEUP {
        &self.gpio_wakeup
    }
    ///0x114 - rtc configure register
    #[inline(always)]
    pub const fn dbg_sel(&self) -> &DBG_SEL {
        &self.dbg_sel
    }
    ///0x118 - rtc configure register
    #[inline(always)]
    pub const fn dbg_map(&self) -> &DBG_MAP {
        &self.dbg_map
    }
    ///0x11c - rtc configure register
    #[inline(always)]
    pub const fn sensor_ctrl(&self) -> &SENSOR_CTRL {
        &self.sensor_ctrl
    }
    ///0x120 - rtc configure register
    #[inline(always)]
    pub const fn dbg_sar_sel(&self) -> &DBG_SAR_SEL {
        &self.dbg_sar_sel
    }
    ///0x124 - rtc configure register
    #[inline(always)]
    pub const fn pg_ctrl(&self) -> &PG_CTRL {
        &self.pg_ctrl
    }
    ///0x1fc - rtc configure register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**OPTIONS0 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`options0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`options0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@options0`] module*/
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
///rtc configure register
pub mod options0;
/**SLP_TIMER0 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_timer0`] module*/
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
///rtc configure register
pub mod slp_timer0;
/**SLP_TIMER1 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`slp_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_timer1`] module*/
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
///rtc configure register
pub mod slp_timer1;
/**TIME_UPDATE (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_update`] module*/
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
///rtc configure register
pub mod time_update;
/**TIME_LOW0 (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`time_low0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_low0`] module*/
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
///rtc configure register
pub mod time_low0;
/**TIME_HIGH0 (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`time_high0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_high0`] module*/
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
///rtc configure register
pub mod time_high0;
/**STATE0 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`state0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state0`] module*/
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
///rtc configure register
pub mod state0;
/**TIMER1 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer1`] module*/
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
///rtc configure register
pub mod timer1;
/**TIMER2 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer2`] module*/
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
///rtc configure register
pub mod timer2;
/**TIMER3 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer3`] module*/
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
///rtc configure register
pub mod timer3;
/**TIMER4 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer4`] module*/
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
///rtc configure register
pub mod timer4;
/**TIMER5 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer5`] module*/
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
///rtc configure register
pub mod timer5;
/**TIMER6 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`timer6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timer6`] module*/
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
///rtc configure register
pub mod timer6;
/**ANA_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`ana_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ana_conf`] module*/
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
///rtc configure register
pub mod ana_conf;
/**RESET_STATE (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`reset_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_state`] module*/
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
///rtc configure register
pub mod reset_state;
/**WAKEUP_STATE (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wakeup_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wakeup_state`] module*/
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
///rtc configure register
pub mod wakeup_state;
/**INT_ENA (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///rtc configure register
pub mod int_ena;
/**INT_RAW (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///rtc configure register
pub mod int_raw;
/**INT_ST (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///rtc configure register
pub mod int_st;
/**INT_CLR (w) register accessor: rtc configure register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///rtc configure register
pub mod int_clr;
/**STORE0 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store0`] module*/
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
///rtc configure register
pub mod store0;
/**STORE1 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store1`] module*/
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
///rtc configure register
pub mod store1;
/**STORE2 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store2`] module*/
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
///rtc configure register
pub mod store2;
/**STORE3 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store3`] module*/
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
///rtc configure register
pub mod store3;
/**EXT_XTL_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_xtl_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_xtl_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_xtl_conf`] module*/
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
///rtc configure register
pub mod ext_xtl_conf;
/**EXT_WAKEUP_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_conf`] module*/
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
///rtc configure register
pub mod ext_wakeup_conf;
/**SLP_REJECT_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_reject_conf`] module*/
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
///rtc configure register
pub mod slp_reject_conf;
/**CPU_PERIOD_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_period_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_period_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_period_conf`] module*/
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
///rtc configure register
pub mod cpu_period_conf;
/**CLK_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf`] module*/
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
///rtc configure register
pub mod clk_conf;
/**SLOW_CLK_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`slow_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slow_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slow_clk_conf`] module*/
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
///rtc configure register
pub mod slow_clk_conf;
/**SDIO_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_conf`] module*/
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
///rtc configure register
pub mod sdio_conf;
/**BIAS_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bias_conf`] module*/
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
///rtc configure register
pub mod bias_conf;
/**RTC_CNTL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_cntl`] module*/
pub type RTC_CNTL = crate::Reg<rtc_cntl::RTC_CNTL_SPEC>;
///rtc configure register
pub mod rtc_cntl;
/**PWC (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwc`] module*/
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
///rtc configure register
pub mod pwc;
/**DIG_PWC (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dig_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_pwc`] module*/
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
///rtc configure register
pub mod dig_pwc;
/**DIG_ISO (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_iso`] module*/
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
///rtc configure register
pub mod dig_iso;
/**WDTCONFIG0 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig0`] module*/
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
///rtc configure register
pub mod wdtconfig0;
/**WDTCONFIG1 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig1`] module*/
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
///rtc configure register
pub mod wdtconfig1;
/**WDTCONFIG2 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig2`] module*/
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
///rtc configure register
pub mod wdtconfig2;
/**WDTCONFIG3 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig3`] module*/
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
///rtc configure register
pub mod wdtconfig3;
/**WDTCONFIG4 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtconfig4`] module*/
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
///rtc configure register
pub mod wdtconfig4;
/**WDTFEED (w) register accessor: rtc configure register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtfeed`] module*/
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
///rtc configure register
pub mod wdtfeed;
/**WDTWPROTECT (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`wdtwprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtwprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtwprotect`] module*/
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
///rtc configure register
pub mod wdtwprotect;
/**SWD_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`swd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swd_conf`] module*/
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
///rtc configure register
pub mod swd_conf;
/**SWD_WPROTECT (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@swd_wprotect`] module*/
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
///rtc configure register
pub mod swd_wprotect;
/**SW_CPU_STALL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sw_cpu_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_cpu_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sw_cpu_stall`] module*/
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
///rtc configure register
pub mod sw_cpu_stall;
/**STORE4 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store4`] module*/
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
///rtc configure register
pub mod store4;
/**STORE5 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store5`] module*/
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
///rtc configure register
pub mod store5;
/**STORE6 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store6`] module*/
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
///rtc configure register
pub mod store6;
/**STORE7 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`store7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@store7`] module*/
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
///rtc configure register
pub mod store7;
/**LOW_POWER_ST (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@low_power_st`] module*/
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
///rtc configure register
pub mod low_power_st;
/**DIAG0 (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`diag0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@diag0`] module*/
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
///rtc configure register
pub mod diag0;
/**PAD_HOLD (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pad_hold`] module*/
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
///rtc configure register
pub mod pad_hold;
/**DIG_PAD_HOLD (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dig_pad_hold`] module*/
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
///rtc configure register
pub mod dig_pad_hold;
/**BROWN_OUT (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brown_out`] module*/
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
///rtc configure register
pub mod brown_out;
/**TIME_LOW1 (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`time_low1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_low1`] module*/
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
///rtc configure register
pub mod time_low1;
/**TIME_HIGH1 (r) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`time_high1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time_high1`] module*/
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
///rtc configure register
pub mod time_high1;
/**XTAL32K_CLK_FACTOR (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_clk_factor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_clk_factor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal32k_clk_factor`] module*/
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
///rtc configure register
pub mod xtal32k_clk_factor;
/**XTAL32K_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal32k_conf`] module*/
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
///rtc configure register
pub mod xtal32k_conf;
/**USB_CONF (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb_conf`] module*/
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
///rtc configure register
pub mod usb_conf;
/**SLP_REJECT_CAUSE (r) register accessor: RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG

You can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_reject_cause`] module*/
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
///RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG
pub mod slp_reject_cause;
/**OPTION1 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`option1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`option1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@option1`] module*/
pub type OPTION1 = crate::Reg<option1::OPTION1_SPEC>;
///rtc configure register
pub mod option1;
/**SLP_WAKEUP_CAUSE (r) register accessor: RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cause`] module*/
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
///RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG
pub mod slp_wakeup_cause;
/**ULP_CP_TIMER_1 (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_timer_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_timer_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ulp_cp_timer_1`] module*/
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
///rtc configure register
pub mod ulp_cp_timer_1;
/**INT_ENA_RTC_W1TS (w) register accessor: rtc configure register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_rtc_w1ts`] module*/
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
///rtc configure register
pub mod int_ena_rtc_w1ts;
/**INT_ENA_RTC_W1TC (w) register accessor: rtc configure register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_rtc_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_rtc_w1tc`] module*/
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
///rtc configure register
pub mod int_ena_rtc_w1tc;
/**RETENTION_CTRL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@retention_ctrl`] module*/
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
///rtc configure register
pub mod retention_ctrl;
/**FIB_SEL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`fib_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fib_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fib_sel`] module*/
pub type FIB_SEL = crate::Reg<fib_sel::FIB_SEL_SPEC>;
///rtc configure register
pub mod fib_sel;
/**GPIO_WAKEUP (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_wakeup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_wakeup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio_wakeup`] module*/
pub type GPIO_WAKEUP = crate::Reg<gpio_wakeup::GPIO_WAKEUP_SPEC>;
///rtc configure register
pub mod gpio_wakeup;
/**DBG_SEL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbg_sel`] module*/
pub type DBG_SEL = crate::Reg<dbg_sel::DBG_SEL_SPEC>;
///rtc configure register
pub mod dbg_sel;
/**DBG_MAP (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbg_map`] module*/
pub type DBG_MAP = crate::Reg<dbg_map::DBG_MAP_SPEC>;
///rtc configure register
pub mod dbg_map;
/**SENSOR_CTRL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sensor_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sensor_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sensor_ctrl`] module*/
pub type SENSOR_CTRL = crate::Reg<sensor_ctrl::SENSOR_CTRL_SPEC>;
///rtc configure register
pub mod sensor_ctrl;
/**DBG_SAR_SEL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_sar_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_sar_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbg_sar_sel`] module*/
pub type DBG_SAR_SEL = crate::Reg<dbg_sar_sel::DBG_SAR_SEL_SPEC>;
///rtc configure register
pub mod dbg_sar_sel;
/**PG_CTRL (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`pg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pg_ctrl`] module*/
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
///rtc configure register
pub mod pg_ctrl;
/**DATE (rw) register accessor: rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///rtc configure register
pub mod date;
