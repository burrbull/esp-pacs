#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    scl_low_period: SCL_LOW_PERIOD,
    ctr: CTR,
    sr: SR,
    to: TO,
    slave_addr: SLAVE_ADDR,
    fifo_st: FIFO_ST,
    fifo_conf: FIFO_CONF,
    data: DATA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_ena: INT_ENA,
    int_st: INT_ST,
    sda_hold: SDA_HOLD,
    sda_sample: SDA_SAMPLE,
    scl_high_period: SCL_HIGH_PERIOD,
    _reserved15: [u8; 0x04],
    scl_start_hold: SCL_START_HOLD,
    scl_rstart_setup: SCL_RSTART_SETUP,
    scl_stop_hold: SCL_STOP_HOLD,
    scl_stop_setup: SCL_STOP_SETUP,
    scl_filter_cfg: SCL_FILTER_CFG,
    sda_filter_cfg: SDA_FILTER_CFG,
    comd: [COMD; 16],
    scl_st_time_out: SCL_ST_TIME_OUT,
    scl_main_st_time_out: SCL_MAIN_ST_TIME_OUT,
    scl_sp_conf: SCL_SP_CONF,
    scl_stretch_conf: SCL_STRETCH_CONF,
    _reserved26: [u8; 0x50],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Configures the low level width of the SCL clock
    #[inline(always)]
    pub const fn scl_low_period(&self) -> &SCL_LOW_PERIOD {
        &self.scl_low_period
    }
    ///0x04 - Transmission setting
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    ///0x08 - Describe I2C work status
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x0c - Setting time out control for receiving data
    #[inline(always)]
    pub const fn to(&self) -> &TO {
        &self.to
    }
    ///0x10 - Local slave address setting
    #[inline(always)]
    pub const fn slave_addr(&self) -> &SLAVE_ADDR {
        &self.slave_addr
    }
    ///0x14 - FIFO status register
    #[inline(always)]
    pub const fn fifo_st(&self) -> &FIFO_ST {
        &self.fifo_st
    }
    ///0x18 - FIFO configuration register
    #[inline(always)]
    pub const fn fifo_conf(&self) -> &FIFO_CONF {
        &self.fifo_conf
    }
    ///0x1c - RX FIFO read data
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    ///0x20 - Raw interrupt status
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x24 - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x28 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x2c - Status of captured I2C communication events
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x30 - Configures the hold time after a negative SCL edge
    #[inline(always)]
    pub const fn sda_hold(&self) -> &SDA_HOLD {
        &self.sda_hold
    }
    ///0x34 - Configures the sample time after a positive SCL edge
    #[inline(always)]
    pub const fn sda_sample(&self) -> &SDA_SAMPLE {
        &self.sda_sample
    }
    ///0x38 - Configures the high level width of the SCL clock
    #[inline(always)]
    pub const fn scl_high_period(&self) -> &SCL_HIGH_PERIOD {
        &self.scl_high_period
    }
    ///0x40 - Configures the interval between pulling SDA low and pulling SCL low when the master generates a START condition
    #[inline(always)]
    pub const fn scl_start_hold(&self) -> &SCL_START_HOLD {
        &self.scl_start_hold
    }
    ///0x44 - Configures the interval between the positive edge of SCL and the negative edge of SDA
    #[inline(always)]
    pub const fn scl_rstart_setup(&self) -> &SCL_RSTART_SETUP {
        &self.scl_rstart_setup
    }
    ///0x48 - Configures the delay after the SCL clock edge for a stop condition
    #[inline(always)]
    pub const fn scl_stop_hold(&self) -> &SCL_STOP_HOLD {
        &self.scl_stop_hold
    }
    ///0x4c - Configures the delay between the SDA and SCL positive edge for a stop condition
    #[inline(always)]
    pub const fn scl_stop_setup(&self) -> &SCL_STOP_SETUP {
        &self.scl_stop_setup
    }
    ///0x50 - SCL filter configuration register
    #[inline(always)]
    pub const fn scl_filter_cfg(&self) -> &SCL_FILTER_CFG {
        &self.scl_filter_cfg
    }
    ///0x54 - SDA filter configuration register
    #[inline(always)]
    pub const fn sda_filter_cfg(&self) -> &SDA_FILTER_CFG {
        &self.sda_filter_cfg
    }
    ///0x58..0x98 - I2C command register %s
    #[inline(always)]
    pub const fn comd(&self, n: usize) -> &COMD {
        &self.comd[n]
    }
    ///Iterator for array of:
    ///0x58..0x98 - I2C command register %s
    #[inline(always)]
    pub fn comd_iter(&self) -> impl Iterator<Item = &COMD> {
        self.comd.iter()
    }
    ///0x98 - SCL status time out register
    #[inline(always)]
    pub const fn scl_st_time_out(&self) -> &SCL_ST_TIME_OUT {
        &self.scl_st_time_out
    }
    ///0x9c - SCL main status time out register
    #[inline(always)]
    pub const fn scl_main_st_time_out(&self) -> &SCL_MAIN_ST_TIME_OUT {
        &self.scl_main_st_time_out
    }
    ///0xa0 - Power configuration register
    #[inline(always)]
    pub const fn scl_sp_conf(&self) -> &SCL_SP_CONF {
        &self.scl_sp_conf
    }
    ///0xa4 - Set SCL stretch of I2C slave
    #[inline(always)]
    pub const fn scl_stretch_conf(&self) -> &SCL_STRETCH_CONF {
        &self.scl_stretch_conf
    }
    ///0xf8 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**SCL_LOW_PERIOD (rw) register accessor: Configures the low level width of the SCL clock

You can [`read`](crate::generic::Reg::read) this register and get [`scl_low_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_low_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_low_period`] module*/
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
///Configures the low level width of the SCL clock
pub mod scl_low_period;
/**CTR (rw) register accessor: Transmission setting

You can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctr`] module*/
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
///Transmission setting
pub mod ctr;
/**SR (r) register accessor: Describe I2C work status

You can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SR_SPEC>;
///Describe I2C work status
pub mod sr;
/**TO (rw) register accessor: Setting time out control for receiving data

You can [`read`](crate::generic::Reg::read) this register and get [`to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@to`] module*/
pub type TO = crate::Reg<to::TO_SPEC>;
///Setting time out control for receiving data
pub mod to;
/**SLAVE_ADDR (rw) register accessor: Local slave address setting

You can [`read`](crate::generic::Reg::read) this register and get [`slave_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slave_addr`] module*/
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
///Local slave address setting
pub mod slave_addr;
/**FIFO_ST (rw) register accessor: FIFO status register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo_st`] module*/
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
///FIFO status register
pub mod fifo_st;
/**FIFO_CONF (rw) register accessor: FIFO configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo_conf`] module*/
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
///FIFO configuration register
pub mod fifo_conf;
/**DATA (rw) register accessor: RX FIFO read data

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
pub type DATA = crate::Reg<data::DATA_SPEC>;
///RX FIFO read data
pub mod data;
/**INT_RAW (r) register accessor: Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Raw interrupt status
pub mod int_raw;
/**INT_CLR (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Interrupt clear bits
pub mod int_clr;
/**INT_ENA (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Interrupt enable bits
pub mod int_ena;
/**INT_ST (r) register accessor: Status of captured I2C communication events

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Status of captured I2C communication events
pub mod int_st;
/**SDA_HOLD (rw) register accessor: Configures the hold time after a negative SCL edge

You can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_hold`] module*/
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
///Configures the hold time after a negative SCL edge
pub mod sda_hold;
/**SDA_SAMPLE (rw) register accessor: Configures the sample time after a positive SCL edge

You can [`read`](crate::generic::Reg::read) this register and get [`sda_sample::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_sample::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_sample`] module*/
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
///Configures the sample time after a positive SCL edge
pub mod sda_sample;
/**SCL_HIGH_PERIOD (rw) register accessor: Configures the high level width of the SCL clock

You can [`read`](crate::generic::Reg::read) this register and get [`scl_high_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_high_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_high_period`] module*/
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
///Configures the high level width of the SCL clock
pub mod scl_high_period;
/**SCL_START_HOLD (rw) register accessor: Configures the interval between pulling SDA low and pulling SCL low when the master generates a START condition

You can [`read`](crate::generic::Reg::read) this register and get [`scl_start_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_start_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_start_hold`] module*/
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
///Configures the interval between pulling SDA low and pulling SCL low when the master generates a START condition
pub mod scl_start_hold;
/**SCL_RSTART_SETUP (rw) register accessor: Configures the interval between the positive edge of SCL and the negative edge of SDA

You can [`read`](crate::generic::Reg::read) this register and get [`scl_rstart_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_rstart_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_rstart_setup`] module*/
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
///Configures the interval between the positive edge of SCL and the negative edge of SDA
pub mod scl_rstart_setup;
/**SCL_STOP_HOLD (rw) register accessor: Configures the delay after the SCL clock edge for a stop condition

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_stop_hold`] module*/
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
///Configures the delay after the SCL clock edge for a stop condition
pub mod scl_stop_hold;
/**SCL_STOP_SETUP (rw) register accessor: Configures the delay between the SDA and SCL positive edge for a stop condition

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_stop_setup`] module*/
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
///Configures the delay between the SDA and SCL positive edge for a stop condition
pub mod scl_stop_setup;
/**SCL_FILTER_CFG (rw) register accessor: SCL filter configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`scl_filter_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_filter_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_filter_cfg`] module*/
pub type SCL_FILTER_CFG = crate::Reg<scl_filter_cfg::SCL_FILTER_CFG_SPEC>;
///SCL filter configuration register
pub mod scl_filter_cfg;
/**SDA_FILTER_CFG (rw) register accessor: SDA filter configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sda_filter_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_filter_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sda_filter_cfg`] module*/
pub type SDA_FILTER_CFG = crate::Reg<sda_filter_cfg::SDA_FILTER_CFG_SPEC>;
///SDA filter configuration register
pub mod sda_filter_cfg;
/**COMD (rw) register accessor: I2C command register %s

You can [`read`](crate::generic::Reg::read) this register and get [`comd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comd`] module*/
pub type COMD = crate::Reg<comd::COMD_SPEC>;
///I2C command register %s
pub mod comd;
/**SCL_ST_TIME_OUT (rw) register accessor: SCL status time out register

You can [`read`](crate::generic::Reg::read) this register and get [`scl_st_time_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_st_time_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_st_time_out`] module*/
pub type SCL_ST_TIME_OUT = crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>;
///SCL status time out register
pub mod scl_st_time_out;
/**SCL_MAIN_ST_TIME_OUT (rw) register accessor: SCL main status time out register

You can [`read`](crate::generic::Reg::read) this register and get [`scl_main_st_time_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_main_st_time_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_main_st_time_out`] module*/
pub type SCL_MAIN_ST_TIME_OUT = crate::Reg<
    scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC,
>;
///SCL main status time out register
pub mod scl_main_st_time_out;
/**SCL_SP_CONF (rw) register accessor: Power configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`scl_sp_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_sp_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_sp_conf`] module*/
pub type SCL_SP_CONF = crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>;
///Power configuration register
pub mod scl_sp_conf;
/**SCL_STRETCH_CONF (rw) register accessor: Set SCL stretch of I2C slave

You can [`read`](crate::generic::Reg::read) this register and get [`scl_stretch_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stretch_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scl_stretch_conf`] module*/
pub type SCL_STRETCH_CONF = crate::Reg<scl_stretch_conf::SCL_STRETCH_CONF_SPEC>;
///Set SCL stretch of I2C slave
pub mod scl_stretch_conf;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
