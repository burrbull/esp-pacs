#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    pgm_data: [PGM_DATA; 8],
    pgm_check_value: [PGM_CHECK_VALUE; 3],
    rd_wr_dis: RD_WR_DIS,
    rd_repeat_data0: RD_REPEAT_DATA0,
    rd_repeat_data1: RD_REPEAT_DATA1,
    rd_repeat_data2: RD_REPEAT_DATA2,
    rd_repeat_data3: RD_REPEAT_DATA3,
    rd_repeat_data4: RD_REPEAT_DATA4,
    rd_mac_spi_sys_0: RD_MAC_SPI_SYS_0,
    rd_mac_spi_sys_1: RD_MAC_SPI_SYS_1,
    rd_mac_spi_sys_2: RD_MAC_SPI_SYS_2,
    rd_mac_spi_sys_3: RD_MAC_SPI_SYS_3,
    rd_mac_spi_sys_4: RD_MAC_SPI_SYS_4,
    rd_mac_spi_sys_5: RD_MAC_SPI_SYS_5,
    rd_sys_data_part1_: [RD_SYS_DATA_PART1_; 8],
    rd_usr_data: [RD_USR_DATA; 8],
    rd_key0_data: [RD_KEY0_DATA; 8],
    rd_key1_data: [RD_KEY1_DATA; 8],
    rd_key2_data: [RD_KEY2_DATA; 8],
    rd_key3_data: [RD_KEY3_DATA; 8],
    rd_key4_data: [RD_KEY4_DATA; 8],
    rd_key5_data: [RD_KEY5_DATA; 8],
    rd_sys_data_part2_: [RD_SYS_DATA_PART2_; 8],
    rd_repeat_err0: RD_REPEAT_ERR0,
    rd_repeat_err1: RD_REPEAT_ERR1,
    rd_repeat_err2: RD_REPEAT_ERR2,
    rd_repeat_err3: RD_REPEAT_ERR3,
    _reserved27: [u8; 0x04],
    rd_repeat_err4: RD_REPEAT_ERR4,
    _reserved28: [u8; 0x2c],
    rd_rs_err0: RD_RS_ERR0,
    rd_rs_err1: RD_RS_ERR1,
    clk: CLK,
    conf: CONF,
    status: STATUS,
    cmd: CMD,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dac_conf: DAC_CONF,
    rd_tim_conf: RD_TIM_CONF,
    wr_tim_conf0: WR_TIM_CONF0,
    wr_tim_conf1: WR_TIM_CONF1,
    wr_tim_conf2: WR_TIM_CONF2,
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x20 - Register %s that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data(&self, n: usize) -> &PGM_DATA {
        &self.pgm_data[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - Register %s that stores data to be programmed.
    #[inline(always)]
    pub fn pgm_data_iter(&self) -> impl Iterator<Item = &PGM_DATA> {
        self.pgm_data.iter()
    }
    ///0x20..0x2c - Register %s that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value(&self, n: usize) -> &PGM_CHECK_VALUE {
        &self.pgm_check_value[n]
    }
    ///Iterator for array of:
    ///0x20..0x2c - Register %s that stores the RS code to be programmed.
    #[inline(always)]
    pub fn pgm_check_value_iter(&self) -> impl Iterator<Item = &PGM_CHECK_VALUE> {
        self.pgm_check_value.iter()
    }
    ///0x2c - Register 0 of BLOCK0.
    #[inline(always)]
    pub const fn rd_wr_dis(&self) -> &RD_WR_DIS {
        &self.rd_wr_dis
    }
    ///0x30 - Register 1 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_data0(&self) -> &RD_REPEAT_DATA0 {
        &self.rd_repeat_data0
    }
    ///0x34 - Register 2 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_data1(&self) -> &RD_REPEAT_DATA1 {
        &self.rd_repeat_data1
    }
    ///0x38 - Register 3 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_data2(&self) -> &RD_REPEAT_DATA2 {
        &self.rd_repeat_data2
    }
    ///0x3c - Register 4 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_data3(&self) -> &RD_REPEAT_DATA3 {
        &self.rd_repeat_data3
    }
    ///0x40 - Register 5 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_data4(&self) -> &RD_REPEAT_DATA4 {
        &self.rd_repeat_data4
    }
    ///0x44 - Register 0 of BLOCK1.
    #[inline(always)]
    pub const fn rd_mac_spi_sys_0(&self) -> &RD_MAC_SPI_SYS_0 {
        &self.rd_mac_spi_sys_0
    }
    ///0x48 - Register 1 of BLOCK1.
    #[inline(always)]
    pub const fn rd_mac_spi_sys_1(&self) -> &RD_MAC_SPI_SYS_1 {
        &self.rd_mac_spi_sys_1
    }
    ///0x4c - Register 2 of BLOCK1.
    #[inline(always)]
    pub const fn rd_mac_spi_sys_2(&self) -> &RD_MAC_SPI_SYS_2 {
        &self.rd_mac_spi_sys_2
    }
    ///0x50 - Register 3 of BLOCK1.
    #[inline(always)]
    pub const fn rd_mac_spi_sys_3(&self) -> &RD_MAC_SPI_SYS_3 {
        &self.rd_mac_spi_sys_3
    }
    ///0x54 - Register 4 of BLOCK1.
    #[inline(always)]
    pub const fn rd_mac_spi_sys_4(&self) -> &RD_MAC_SPI_SYS_4 {
        &self.rd_mac_spi_sys_4
    }
    ///0x58 - Register 5 of BLOCK1.
    #[inline(always)]
    pub const fn rd_mac_spi_sys_5(&self) -> &RD_MAC_SPI_SYS_5 {
        &self.rd_mac_spi_sys_5
    }
    ///0x5c..0x7c - Register %s of BLOCK2 (system).
    #[inline(always)]
    pub const fn rd_sys_data_part1_(&self, n: usize) -> &RD_SYS_DATA_PART1_ {
        &self.rd_sys_data_part1_[n]
    }
    ///Iterator for array of:
    ///0x5c..0x7c - Register %s of BLOCK2 (system).
    #[inline(always)]
    pub fn rd_sys_data_part1__iter(&self) -> impl Iterator<Item = &RD_SYS_DATA_PART1_> {
        self.rd_sys_data_part1_.iter()
    }
    ///0x7c..0x9c - Register %s of BLOCK3 (user).
    #[inline(always)]
    pub const fn rd_usr_data(&self, n: usize) -> &RD_USR_DATA {
        &self.rd_usr_data[n]
    }
    ///Iterator for array of:
    ///0x7c..0x9c - Register %s of BLOCK3 (user).
    #[inline(always)]
    pub fn rd_usr_data_iter(&self) -> impl Iterator<Item = &RD_USR_DATA> {
        self.rd_usr_data.iter()
    }
    ///0x9c..0xbc - Register %s of BLOCK4 (KEY0).
    #[inline(always)]
    pub const fn rd_key0_data(&self, n: usize) -> &RD_KEY0_DATA {
        &self.rd_key0_data[n]
    }
    ///Iterator for array of:
    ///0x9c..0xbc - Register %s of BLOCK4 (KEY0).
    #[inline(always)]
    pub fn rd_key0_data_iter(&self) -> impl Iterator<Item = &RD_KEY0_DATA> {
        self.rd_key0_data.iter()
    }
    ///0xbc..0xdc - Register %s of BLOCK5 (KEY1).
    #[inline(always)]
    pub const fn rd_key1_data(&self, n: usize) -> &RD_KEY1_DATA {
        &self.rd_key1_data[n]
    }
    ///Iterator for array of:
    ///0xbc..0xdc - Register %s of BLOCK5 (KEY1).
    #[inline(always)]
    pub fn rd_key1_data_iter(&self) -> impl Iterator<Item = &RD_KEY1_DATA> {
        self.rd_key1_data.iter()
    }
    ///0xdc..0xfc - Register %s of BLOCK6 (KEY2).
    #[inline(always)]
    pub const fn rd_key2_data(&self, n: usize) -> &RD_KEY2_DATA {
        &self.rd_key2_data[n]
    }
    ///Iterator for array of:
    ///0xdc..0xfc - Register %s of BLOCK6 (KEY2).
    #[inline(always)]
    pub fn rd_key2_data_iter(&self) -> impl Iterator<Item = &RD_KEY2_DATA> {
        self.rd_key2_data.iter()
    }
    ///0xfc..0x11c - Register %s of BLOCK7 (KEY3).
    #[inline(always)]
    pub const fn rd_key3_data(&self, n: usize) -> &RD_KEY3_DATA {
        &self.rd_key3_data[n]
    }
    ///Iterator for array of:
    ///0xfc..0x11c - Register %s of BLOCK7 (KEY3).
    #[inline(always)]
    pub fn rd_key3_data_iter(&self) -> impl Iterator<Item = &RD_KEY3_DATA> {
        self.rd_key3_data.iter()
    }
    ///0x11c..0x13c - Register %s of BLOCK8 (KEY4).
    #[inline(always)]
    pub const fn rd_key4_data(&self, n: usize) -> &RD_KEY4_DATA {
        &self.rd_key4_data[n]
    }
    ///Iterator for array of:
    ///0x11c..0x13c - Register %s of BLOCK8 (KEY4).
    #[inline(always)]
    pub fn rd_key4_data_iter(&self) -> impl Iterator<Item = &RD_KEY4_DATA> {
        self.rd_key4_data.iter()
    }
    ///0x13c..0x15c - Register %s of BLOCK9 (KEY5).
    #[inline(always)]
    pub const fn rd_key5_data(&self, n: usize) -> &RD_KEY5_DATA {
        &self.rd_key5_data[n]
    }
    ///Iterator for array of:
    ///0x13c..0x15c - Register %s of BLOCK9 (KEY5).
    #[inline(always)]
    pub fn rd_key5_data_iter(&self) -> impl Iterator<Item = &RD_KEY5_DATA> {
        self.rd_key5_data.iter()
    }
    ///0x15c..0x17c - Register %s of BLOCK10 (system).
    #[inline(always)]
    pub const fn rd_sys_data_part2_(&self, n: usize) -> &RD_SYS_DATA_PART2_ {
        &self.rd_sys_data_part2_[n]
    }
    ///Iterator for array of:
    ///0x15c..0x17c - Register %s of BLOCK10 (system).
    #[inline(always)]
    pub fn rd_sys_data_part2__iter(&self) -> impl Iterator<Item = &RD_SYS_DATA_PART2_> {
        self.rd_sys_data_part2_.iter()
    }
    ///0x17c - Programming error record register 0 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err0(&self) -> &RD_REPEAT_ERR0 {
        &self.rd_repeat_err0
    }
    ///0x180 - Programming error record register 1 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err1(&self) -> &RD_REPEAT_ERR1 {
        &self.rd_repeat_err1
    }
    ///0x184 - Programming error record register 2 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err2(&self) -> &RD_REPEAT_ERR2 {
        &self.rd_repeat_err2
    }
    ///0x188 - Programming error record register 3 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err3(&self) -> &RD_REPEAT_ERR3 {
        &self.rd_repeat_err3
    }
    ///0x190 - Programming error record register 4 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err4(&self) -> &RD_REPEAT_ERR4 {
        &self.rd_repeat_err4
    }
    ///0x1c0 - Programming error record register 0 of BLOCK1-10.
    #[inline(always)]
    pub const fn rd_rs_err0(&self) -> &RD_RS_ERR0 {
        &self.rd_rs_err0
    }
    ///0x1c4 - Programming error record register 1 of BLOCK1-10.
    #[inline(always)]
    pub const fn rd_rs_err1(&self) -> &RD_RS_ERR1 {
        &self.rd_rs_err1
    }
    ///0x1c8 - eFuse clock configuration register.
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x1cc - eFuse operation mode configuration register.
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x1d0 - eFuse status register.
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x1d4 - eFuse command register.
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x1d8 - eFuse raw interrupt register.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x1dc - eFuse interrupt status register.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x1e0 - eFuse interrupt enable register.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x1e4 - eFuse interrupt clear register.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x1e8 - Controls the eFuse programming voltage.
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DAC_CONF {
        &self.dac_conf
    }
    ///0x1ec - Configures read timing parameters.
    #[inline(always)]
    pub const fn rd_tim_conf(&self) -> &RD_TIM_CONF {
        &self.rd_tim_conf
    }
    ///0x1f0 - Configuration register 0 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf0(&self) -> &WR_TIM_CONF0 {
        &self.wr_tim_conf0
    }
    ///0x1f4 - Configuration register 1 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf1(&self) -> &WR_TIM_CONF1 {
        &self.wr_tim_conf1
    }
    ///0x1f8 - Configuration register 2 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf2(&self) -> &WR_TIM_CONF2 {
        &self.wr_tim_conf2
    }
    ///0x1fc - Version control register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**PGM_DATA (rw) register accessor: Register %s that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data`] module*/
pub type PGM_DATA = crate::Reg<pgm_data::PGM_DATA_SPEC>;
///Register %s that stores data to be programmed.
pub mod pgm_data;
/**PGM_CHECK_VALUE (rw) register accessor: Register %s that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value`] module*/
pub type PGM_CHECK_VALUE = crate::Reg<pgm_check_value::PGM_CHECK_VALUE_SPEC>;
///Register %s that stores the RS code to be programmed.
pub mod pgm_check_value;
/**RD_WR_DIS (r) register accessor: Register 0 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_wr_dis`] module*/
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
///Register 0 of BLOCK0.
pub mod rd_wr_dis;
/**RD_REPEAT_DATA0 (r) register accessor: Register 1 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data0`] module*/
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
///Register 1 of BLOCK0.
pub mod rd_repeat_data0;
/**RD_REPEAT_DATA1 (r) register accessor: Register 2 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data1`] module*/
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
///Register 2 of BLOCK0.
pub mod rd_repeat_data1;
/**RD_REPEAT_DATA2 (r) register accessor: Register 3 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data2`] module*/
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
///Register 3 of BLOCK0.
pub mod rd_repeat_data2;
/**RD_REPEAT_DATA3 (r) register accessor: Register 4 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data3`] module*/
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
///Register 4 of BLOCK0.
pub mod rd_repeat_data3;
/**RD_REPEAT_DATA4 (r) register accessor: Register 5 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data4`] module*/
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
///Register 5 of BLOCK0.
pub mod rd_repeat_data4;
/**RD_MAC_SPI_SYS_0 (r) register accessor: Register 0 of BLOCK1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_spi_sys_0`] module*/
pub type RD_MAC_SPI_SYS_0 = crate::Reg<rd_mac_spi_sys_0::RD_MAC_SPI_SYS_0_SPEC>;
///Register 0 of BLOCK1.
pub mod rd_mac_spi_sys_0;
/**RD_MAC_SPI_SYS_1 (r) register accessor: Register 1 of BLOCK1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_spi_sys_1`] module*/
pub type RD_MAC_SPI_SYS_1 = crate::Reg<rd_mac_spi_sys_1::RD_MAC_SPI_SYS_1_SPEC>;
///Register 1 of BLOCK1.
pub mod rd_mac_spi_sys_1;
/**RD_MAC_SPI_SYS_2 (r) register accessor: Register 2 of BLOCK1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_spi_sys_2`] module*/
pub type RD_MAC_SPI_SYS_2 = crate::Reg<rd_mac_spi_sys_2::RD_MAC_SPI_SYS_2_SPEC>;
///Register 2 of BLOCK1.
pub mod rd_mac_spi_sys_2;
/**RD_MAC_SPI_SYS_3 (r) register accessor: Register 3 of BLOCK1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_spi_sys_3`] module*/
pub type RD_MAC_SPI_SYS_3 = crate::Reg<rd_mac_spi_sys_3::RD_MAC_SPI_SYS_3_SPEC>;
///Register 3 of BLOCK1.
pub mod rd_mac_spi_sys_3;
/**RD_MAC_SPI_SYS_4 (r) register accessor: Register 4 of BLOCK1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_spi_sys_4`] module*/
pub type RD_MAC_SPI_SYS_4 = crate::Reg<rd_mac_spi_sys_4::RD_MAC_SPI_SYS_4_SPEC>;
///Register 4 of BLOCK1.
pub mod rd_mac_spi_sys_4;
/**RD_MAC_SPI_SYS_5 (r) register accessor: Register 5 of BLOCK1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_mac_spi_sys_5`] module*/
pub type RD_MAC_SPI_SYS_5 = crate::Reg<rd_mac_spi_sys_5::RD_MAC_SPI_SYS_5_SPEC>;
///Register 5 of BLOCK1.
pub mod rd_mac_spi_sys_5;
/**RD_SYS_DATA_PART1_ (r) register accessor: Register %s of BLOCK2 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_data_part1_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_data_part1_`] module*/
pub type RD_SYS_DATA_PART1_ = crate::Reg<rd_sys_data_part1_::RD_SYS_DATA_PART1__SPEC>;
///Register %s of BLOCK2 (system).
pub mod rd_sys_data_part1_;
/**RD_USR_DATA (r) register accessor: Register %s of BLOCK3 (user).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_usr_data`] module*/
pub type RD_USR_DATA = crate::Reg<rd_usr_data::RD_USR_DATA_SPEC>;
///Register %s of BLOCK3 (user).
pub mod rd_usr_data;
/**RD_KEY0_DATA (r) register accessor: Register %s of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key0_data`] module*/
pub type RD_KEY0_DATA = crate::Reg<rd_key0_data::RD_KEY0_DATA_SPEC>;
///Register %s of BLOCK4 (KEY0).
pub mod rd_key0_data;
/**RD_KEY1_DATA (r) register accessor: Register %s of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key1_data`] module*/
pub type RD_KEY1_DATA = crate::Reg<rd_key1_data::RD_KEY1_DATA_SPEC>;
///Register %s of BLOCK5 (KEY1).
pub mod rd_key1_data;
/**RD_KEY2_DATA (r) register accessor: Register %s of BLOCK6 (KEY2).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key2_data`] module*/
pub type RD_KEY2_DATA = crate::Reg<rd_key2_data::RD_KEY2_DATA_SPEC>;
///Register %s of BLOCK6 (KEY2).
pub mod rd_key2_data;
/**RD_KEY3_DATA (r) register accessor: Register %s of BLOCK7 (KEY3).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key3_data`] module*/
pub type RD_KEY3_DATA = crate::Reg<rd_key3_data::RD_KEY3_DATA_SPEC>;
///Register %s of BLOCK7 (KEY3).
pub mod rd_key3_data;
/**RD_KEY4_DATA (r) register accessor: Register %s of BLOCK8 (KEY4).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key4_data`] module*/
pub type RD_KEY4_DATA = crate::Reg<rd_key4_data::RD_KEY4_DATA_SPEC>;
///Register %s of BLOCK8 (KEY4).
pub mod rd_key4_data;
/**RD_KEY5_DATA (r) register accessor: Register %s of BLOCK9 (KEY5).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_key5_data`] module*/
pub type RD_KEY5_DATA = crate::Reg<rd_key5_data::RD_KEY5_DATA_SPEC>;
///Register %s of BLOCK9 (KEY5).
pub mod rd_key5_data;
/**RD_SYS_DATA_PART2_ (r) register accessor: Register %s of BLOCK10 (system).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_data_part2_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_sys_data_part2_`] module*/
pub type RD_SYS_DATA_PART2_ = crate::Reg<rd_sys_data_part2_::RD_SYS_DATA_PART2__SPEC>;
///Register %s of BLOCK10 (system).
pub mod rd_sys_data_part2_;
/**RD_REPEAT_ERR0 (r) register accessor: Programming error record register 0 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err0`] module*/
pub type RD_REPEAT_ERR0 = crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>;
///Programming error record register 0 of BLOCK0.
pub mod rd_repeat_err0;
/**RD_REPEAT_ERR1 (r) register accessor: Programming error record register 1 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err1`] module*/
pub type RD_REPEAT_ERR1 = crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>;
///Programming error record register 1 of BLOCK0.
pub mod rd_repeat_err1;
/**RD_REPEAT_ERR2 (r) register accessor: Programming error record register 2 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err2`] module*/
pub type RD_REPEAT_ERR2 = crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>;
///Programming error record register 2 of BLOCK0.
pub mod rd_repeat_err2;
/**RD_REPEAT_ERR3 (r) register accessor: Programming error record register 3 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err3`] module*/
pub type RD_REPEAT_ERR3 = crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>;
///Programming error record register 3 of BLOCK0.
pub mod rd_repeat_err3;
/**RD_REPEAT_ERR4 (r) register accessor: Programming error record register 4 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err4`] module*/
pub type RD_REPEAT_ERR4 = crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>;
///Programming error record register 4 of BLOCK0.
pub mod rd_repeat_err4;
/**RD_RS_ERR0 (r) register accessor: Programming error record register 0 of BLOCK1-10.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_rs_err0`] module*/
pub type RD_RS_ERR0 = crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>;
///Programming error record register 0 of BLOCK1-10.
pub mod rd_rs_err0;
/**RD_RS_ERR1 (r) register accessor: Programming error record register 1 of BLOCK1-10.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_rs_err1`] module*/
pub type RD_RS_ERR1 = crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>;
///Programming error record register 1 of BLOCK1-10.
pub mod rd_rs_err1;
/**CLK (rw) register accessor: eFuse clock configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///eFuse clock configuration register.
pub mod clk;
/**CONF (rw) register accessor: eFuse operation mode configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///eFuse operation mode configuration register.
pub mod conf;
/**STATUS (r) register accessor: eFuse status register.

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///eFuse status register.
pub mod status;
/**CMD (rw) register accessor: eFuse command register.

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///eFuse command register.
pub mod cmd;
/**INT_RAW (r) register accessor: eFuse raw interrupt register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///eFuse raw interrupt register.
pub mod int_raw;
/**INT_ST (r) register accessor: eFuse interrupt status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///eFuse interrupt status register.
pub mod int_st;
/**INT_ENA (rw) register accessor: eFuse interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///eFuse interrupt enable register.
pub mod int_ena;
/**INT_CLR (w) register accessor: eFuse interrupt clear register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///eFuse interrupt clear register.
pub mod int_clr;
/**DAC_CONF (rw) register accessor: Controls the eFuse programming voltage.

You can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dac_conf`] module*/
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
///Controls the eFuse programming voltage.
pub mod dac_conf;
/**RD_TIM_CONF (rw) register accessor: Configures read timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_tim_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_tim_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_tim_conf`] module*/
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
///Configures read timing parameters.
pub mod rd_tim_conf;
/**WR_TIM_CONF0 (rw) register accessor: Configuration register 0 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf0`] module*/
pub type WR_TIM_CONF0 = crate::Reg<wr_tim_conf0::WR_TIM_CONF0_SPEC>;
///Configuration register 0 of eFuse programming timing parameters.
pub mod wr_tim_conf0;
/**WR_TIM_CONF1 (rw) register accessor: Configuration register 1 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf1`] module*/
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
///Configuration register 1 of eFuse programming timing parameters.
pub mod wr_tim_conf1;
/**WR_TIM_CONF2 (rw) register accessor: Configuration register 2 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf2`] module*/
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
///Configuration register 2 of eFuse programming timing parameters.
pub mod wr_tim_conf2;
/**DATE (rw) register accessor: Version control register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register.
pub mod date;
