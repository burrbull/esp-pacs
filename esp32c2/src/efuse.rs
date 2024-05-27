#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    pgm_data0: PGM_DATA0,
    pgm_data1: PGM_DATA1,
    pgm_data2: PGM_DATA2,
    pgm_data3: PGM_DATA3,
    pgm_data4: PGM_DATA4,
    pgm_data5: PGM_DATA5,
    pgm_data6: PGM_DATA6,
    pgm_data7: PGM_DATA7,
    pgm_check_value0: PGM_CHECK_VALUE0,
    pgm_check_value1: PGM_CHECK_VALUE1,
    pgm_check_value2: PGM_CHECK_VALUE2,
    rd_wr_dis: RD_WR_DIS,
    rd_repeat_data0: RD_REPEAT_DATA0,
    rd_blk1_data0: RD_BLK1_DATA0,
    rd_blk1_data1: RD_BLK1_DATA1,
    rd_blk1_data2: RD_BLK1_DATA2,
    rd_blk2_data0: RD_BLK2_DATA0,
    rd_blk2_data1: RD_BLK2_DATA1,
    rd_blk2_data2: RD_BLK2_DATA2,
    rd_blk2_data3: RD_BLK2_DATA3,
    rd_blk2_data4: RD_BLK2_DATA4,
    rd_blk2_data5: RD_BLK2_DATA5,
    rd_blk2_data6: RD_BLK2_DATA6,
    rd_blk2_data7: RD_BLK2_DATA7,
    rd_blk3_data0: RD_BLK3_DATA0,
    rd_blk3_data1: RD_BLK3_DATA1,
    rd_blk3_data2: RD_BLK3_DATA2,
    rd_blk3_data3: RD_BLK3_DATA3,
    rd_blk3_data4: RD_BLK3_DATA4,
    rd_blk3_data5: RD_BLK3_DATA5,
    rd_blk3_data6: RD_BLK3_DATA6,
    rd_blk3_data7: RD_BLK3_DATA7,
    rd_repeat_err: RD_REPEAT_ERR,
    rd_rs_err: RD_RS_ERR,
    clk: CLK,
    conf: CONF,
    status: STATUS,
    cmd: CMD,
    int_raw: INT_RAW,
    int_st: INT_ST,
    _reserved40: [u8; 0x60],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dac_conf: DAC_CONF,
    rd_tim_conf: RD_TIM_CONF,
    wr_tim_conf0: WR_TIM_CONF0,
    wr_tim_conf1: WR_TIM_CONF1,
    wr_tim_conf2: WR_TIM_CONF2,
    _reserved47: [u8; 0xe0],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Register 0 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data0(&self) -> &PGM_DATA0 {
        &self.pgm_data0
    }
    ///0x04 - Register 1 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data1(&self) -> &PGM_DATA1 {
        &self.pgm_data1
    }
    ///0x08 - Register 2 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data2(&self) -> &PGM_DATA2 {
        &self.pgm_data2
    }
    ///0x0c - Register 3 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data3(&self) -> &PGM_DATA3 {
        &self.pgm_data3
    }
    ///0x10 - Register 4 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data4(&self) -> &PGM_DATA4 {
        &self.pgm_data4
    }
    ///0x14 - Register 5 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data5(&self) -> &PGM_DATA5 {
        &self.pgm_data5
    }
    ///0x18 - Register 6 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data6(&self) -> &PGM_DATA6 {
        &self.pgm_data6
    }
    ///0x1c - Register 7 that stores data to be programmed.
    #[inline(always)]
    pub const fn pgm_data7(&self) -> &PGM_DATA7 {
        &self.pgm_data7
    }
    ///0x20 - Register 0 that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value0(&self) -> &PGM_CHECK_VALUE0 {
        &self.pgm_check_value0
    }
    ///0x24 - Register 1 that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value1(&self) -> &PGM_CHECK_VALUE1 {
        &self.pgm_check_value1
    }
    ///0x28 - Register 2 that stores the RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_check_value2(&self) -> &PGM_CHECK_VALUE2 {
        &self.pgm_check_value2
    }
    ///0x2c - BLOCK0 data register 0.
    #[inline(always)]
    pub const fn rd_wr_dis(&self) -> &RD_WR_DIS {
        &self.rd_wr_dis
    }
    ///0x30 - BLOCK0 data register 1.
    #[inline(always)]
    pub const fn rd_repeat_data0(&self) -> &RD_REPEAT_DATA0 {
        &self.rd_repeat_data0
    }
    ///0x34 - BLOCK1 data register 0.
    #[inline(always)]
    pub const fn rd_blk1_data0(&self) -> &RD_BLK1_DATA0 {
        &self.rd_blk1_data0
    }
    ///0x38 - BLOCK1 data register 1.
    #[inline(always)]
    pub const fn rd_blk1_data1(&self) -> &RD_BLK1_DATA1 {
        &self.rd_blk1_data1
    }
    ///0x3c - BLOCK1 data register 2.
    #[inline(always)]
    pub const fn rd_blk1_data2(&self) -> &RD_BLK1_DATA2 {
        &self.rd_blk1_data2
    }
    ///0x40 - Register 0 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data0(&self) -> &RD_BLK2_DATA0 {
        &self.rd_blk2_data0
    }
    ///0x44 - Register 1 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data1(&self) -> &RD_BLK2_DATA1 {
        &self.rd_blk2_data1
    }
    ///0x48 - Register 2 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data2(&self) -> &RD_BLK2_DATA2 {
        &self.rd_blk2_data2
    }
    ///0x4c - Register 3 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data3(&self) -> &RD_BLK2_DATA3 {
        &self.rd_blk2_data3
    }
    ///0x50 - Register 4 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data4(&self) -> &RD_BLK2_DATA4 {
        &self.rd_blk2_data4
    }
    ///0x54 - Register 5 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data5(&self) -> &RD_BLK2_DATA5 {
        &self.rd_blk2_data5
    }
    ///0x58 - Register 6 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data6(&self) -> &RD_BLK2_DATA6 {
        &self.rd_blk2_data6
    }
    ///0x5c - Register 7 of BLOCK2.
    #[inline(always)]
    pub const fn rd_blk2_data7(&self) -> &RD_BLK2_DATA7 {
        &self.rd_blk2_data7
    }
    ///0x60 - Register 0 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data0(&self) -> &RD_BLK3_DATA0 {
        &self.rd_blk3_data0
    }
    ///0x64 - Register 1 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data1(&self) -> &RD_BLK3_DATA1 {
        &self.rd_blk3_data1
    }
    ///0x68 - Register 2 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data2(&self) -> &RD_BLK3_DATA2 {
        &self.rd_blk3_data2
    }
    ///0x6c - Register 3 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data3(&self) -> &RD_BLK3_DATA3 {
        &self.rd_blk3_data3
    }
    ///0x70 - Register 4 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data4(&self) -> &RD_BLK3_DATA4 {
        &self.rd_blk3_data4
    }
    ///0x74 - Register 5 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data5(&self) -> &RD_BLK3_DATA5 {
        &self.rd_blk3_data5
    }
    ///0x78 - Register 6 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data6(&self) -> &RD_BLK3_DATA6 {
        &self.rd_blk3_data6
    }
    ///0x7c - Register 7 of BLOCK3.
    #[inline(always)]
    pub const fn rd_blk3_data7(&self) -> &RD_BLK3_DATA7 {
        &self.rd_blk3_data7
    }
    ///0x80 - Programming error record register 0 of BLOCK0.
    #[inline(always)]
    pub const fn rd_repeat_err(&self) -> &RD_REPEAT_ERR {
        &self.rd_repeat_err
    }
    ///0x84 - Programming error record register 0 of BLOCK1-10.
    #[inline(always)]
    pub const fn rd_rs_err(&self) -> &RD_RS_ERR {
        &self.rd_rs_err
    }
    ///0x88 - eFuse clcok configuration register.
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x8c - eFuse operation mode configuraiton register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x90 - eFuse status register.
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x94 - eFuse command register.
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x98 - eFuse raw interrupt register.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x9c - eFuse interrupt status register.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x100 - eFuse interrupt enable register.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x104 - eFuse interrupt clear register.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x108 - Controls the eFuse programming voltage.
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DAC_CONF {
        &self.dac_conf
    }
    ///0x10c - Configures read timing parameters.
    #[inline(always)]
    pub const fn rd_tim_conf(&self) -> &RD_TIM_CONF {
        &self.rd_tim_conf
    }
    ///0x110 - Configurarion register 0 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf0(&self) -> &WR_TIM_CONF0 {
        &self.wr_tim_conf0
    }
    ///0x114 - Configurarion register 1 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf1(&self) -> &WR_TIM_CONF1 {
        &self.wr_tim_conf1
    }
    ///0x118 - Configurarion register 2 of eFuse programming timing parameters.
    #[inline(always)]
    pub const fn wr_tim_conf2(&self) -> &WR_TIM_CONF2 {
        &self.wr_tim_conf2
    }
    ///0x1fc - eFuse version register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**PGM_DATA0 (rw) register accessor: Register 0 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data0`] module*/
pub type PGM_DATA0 = crate::Reg<pgm_data0::PGM_DATA0_SPEC>;
///Register 0 that stores data to be programmed.
pub mod pgm_data0;
/**PGM_DATA1 (rw) register accessor: Register 1 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data1`] module*/
pub type PGM_DATA1 = crate::Reg<pgm_data1::PGM_DATA1_SPEC>;
///Register 1 that stores data to be programmed.
pub mod pgm_data1;
/**PGM_DATA2 (rw) register accessor: Register 2 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data2`] module*/
pub type PGM_DATA2 = crate::Reg<pgm_data2::PGM_DATA2_SPEC>;
///Register 2 that stores data to be programmed.
pub mod pgm_data2;
/**PGM_DATA3 (rw) register accessor: Register 3 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data3`] module*/
pub type PGM_DATA3 = crate::Reg<pgm_data3::PGM_DATA3_SPEC>;
///Register 3 that stores data to be programmed.
pub mod pgm_data3;
/**PGM_DATA4 (rw) register accessor: Register 4 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data4`] module*/
pub type PGM_DATA4 = crate::Reg<pgm_data4::PGM_DATA4_SPEC>;
///Register 4 that stores data to be programmed.
pub mod pgm_data4;
/**PGM_DATA5 (rw) register accessor: Register 5 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data5`] module*/
pub type PGM_DATA5 = crate::Reg<pgm_data5::PGM_DATA5_SPEC>;
///Register 5 that stores data to be programmed.
pub mod pgm_data5;
/**PGM_DATA6 (rw) register accessor: Register 6 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data6`] module*/
pub type PGM_DATA6 = crate::Reg<pgm_data6::PGM_DATA6_SPEC>;
///Register 6 that stores data to be programmed.
pub mod pgm_data6;
/**PGM_DATA7 (rw) register accessor: Register 7 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_data7`] module*/
pub type PGM_DATA7 = crate::Reg<pgm_data7::PGM_DATA7_SPEC>;
///Register 7 that stores data to be programmed.
pub mod pgm_data7;
/**PGM_CHECK_VALUE0 (rw) register accessor: Register 0 that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value0`] module*/
pub type PGM_CHECK_VALUE0 = crate::Reg<pgm_check_value0::PGM_CHECK_VALUE0_SPEC>;
///Register 0 that stores the RS code to be programmed.
pub mod pgm_check_value0;
/**PGM_CHECK_VALUE1 (rw) register accessor: Register 1 that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value1`] module*/
pub type PGM_CHECK_VALUE1 = crate::Reg<pgm_check_value1::PGM_CHECK_VALUE1_SPEC>;
///Register 1 that stores the RS code to be programmed.
pub mod pgm_check_value1;
/**PGM_CHECK_VALUE2 (rw) register accessor: Register 2 that stores the RS code to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgm_check_value2`] module*/
pub type PGM_CHECK_VALUE2 = crate::Reg<pgm_check_value2::PGM_CHECK_VALUE2_SPEC>;
///Register 2 that stores the RS code to be programmed.
pub mod pgm_check_value2;
/**RD_WR_DIS (r) register accessor: BLOCK0 data register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_wr_dis`] module*/
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
///BLOCK0 data register 0.
pub mod rd_wr_dis;
/**RD_REPEAT_DATA0 (r) register accessor: BLOCK0 data register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_data0`] module*/
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
///BLOCK0 data register 1.
pub mod rd_repeat_data0;
/**RD_BLK1_DATA0 (r) register accessor: BLOCK1 data register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk1_data0`] module*/
pub type RD_BLK1_DATA0 = crate::Reg<rd_blk1_data0::RD_BLK1_DATA0_SPEC>;
///BLOCK1 data register 0.
pub mod rd_blk1_data0;
/**RD_BLK1_DATA1 (r) register accessor: BLOCK1 data register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk1_data1`] module*/
pub type RD_BLK1_DATA1 = crate::Reg<rd_blk1_data1::RD_BLK1_DATA1_SPEC>;
///BLOCK1 data register 1.
pub mod rd_blk1_data1;
/**RD_BLK1_DATA2 (r) register accessor: BLOCK1 data register 2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk1_data2`] module*/
pub type RD_BLK1_DATA2 = crate::Reg<rd_blk1_data2::RD_BLK1_DATA2_SPEC>;
///BLOCK1 data register 2.
pub mod rd_blk1_data2;
/**RD_BLK2_DATA0 (r) register accessor: Register 0 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data0`] module*/
pub type RD_BLK2_DATA0 = crate::Reg<rd_blk2_data0::RD_BLK2_DATA0_SPEC>;
///Register 0 of BLOCK2.
pub mod rd_blk2_data0;
/**RD_BLK2_DATA1 (r) register accessor: Register 1 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data1`] module*/
pub type RD_BLK2_DATA1 = crate::Reg<rd_blk2_data1::RD_BLK2_DATA1_SPEC>;
///Register 1 of BLOCK2.
pub mod rd_blk2_data1;
/**RD_BLK2_DATA2 (r) register accessor: Register 2 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data2`] module*/
pub type RD_BLK2_DATA2 = crate::Reg<rd_blk2_data2::RD_BLK2_DATA2_SPEC>;
///Register 2 of BLOCK2.
pub mod rd_blk2_data2;
/**RD_BLK2_DATA3 (r) register accessor: Register 3 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data3`] module*/
pub type RD_BLK2_DATA3 = crate::Reg<rd_blk2_data3::RD_BLK2_DATA3_SPEC>;
///Register 3 of BLOCK2.
pub mod rd_blk2_data3;
/**RD_BLK2_DATA4 (r) register accessor: Register 4 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data4`] module*/
pub type RD_BLK2_DATA4 = crate::Reg<rd_blk2_data4::RD_BLK2_DATA4_SPEC>;
///Register 4 of BLOCK2.
pub mod rd_blk2_data4;
/**RD_BLK2_DATA5 (r) register accessor: Register 5 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data5`] module*/
pub type RD_BLK2_DATA5 = crate::Reg<rd_blk2_data5::RD_BLK2_DATA5_SPEC>;
///Register 5 of BLOCK2.
pub mod rd_blk2_data5;
/**RD_BLK2_DATA6 (r) register accessor: Register 6 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data6`] module*/
pub type RD_BLK2_DATA6 = crate::Reg<rd_blk2_data6::RD_BLK2_DATA6_SPEC>;
///Register 6 of BLOCK2.
pub mod rd_blk2_data6;
/**RD_BLK2_DATA7 (r) register accessor: Register 7 of BLOCK2.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk2_data7`] module*/
pub type RD_BLK2_DATA7 = crate::Reg<rd_blk2_data7::RD_BLK2_DATA7_SPEC>;
///Register 7 of BLOCK2.
pub mod rd_blk2_data7;
/**RD_BLK3_DATA0 (r) register accessor: Register 0 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data0`] module*/
pub type RD_BLK3_DATA0 = crate::Reg<rd_blk3_data0::RD_BLK3_DATA0_SPEC>;
///Register 0 of BLOCK3.
pub mod rd_blk3_data0;
/**RD_BLK3_DATA1 (r) register accessor: Register 1 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data1`] module*/
pub type RD_BLK3_DATA1 = crate::Reg<rd_blk3_data1::RD_BLK3_DATA1_SPEC>;
///Register 1 of BLOCK3.
pub mod rd_blk3_data1;
/**RD_BLK3_DATA2 (r) register accessor: Register 2 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data2`] module*/
pub type RD_BLK3_DATA2 = crate::Reg<rd_blk3_data2::RD_BLK3_DATA2_SPEC>;
///Register 2 of BLOCK3.
pub mod rd_blk3_data2;
/**RD_BLK3_DATA3 (r) register accessor: Register 3 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data3`] module*/
pub type RD_BLK3_DATA3 = crate::Reg<rd_blk3_data3::RD_BLK3_DATA3_SPEC>;
///Register 3 of BLOCK3.
pub mod rd_blk3_data3;
/**RD_BLK3_DATA4 (r) register accessor: Register 4 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data4`] module*/
pub type RD_BLK3_DATA4 = crate::Reg<rd_blk3_data4::RD_BLK3_DATA4_SPEC>;
///Register 4 of BLOCK3.
pub mod rd_blk3_data4;
/**RD_BLK3_DATA5 (r) register accessor: Register 5 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data5`] module*/
pub type RD_BLK3_DATA5 = crate::Reg<rd_blk3_data5::RD_BLK3_DATA5_SPEC>;
///Register 5 of BLOCK3.
pub mod rd_blk3_data5;
/**RD_BLK3_DATA6 (r) register accessor: Register 6 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data6`] module*/
pub type RD_BLK3_DATA6 = crate::Reg<rd_blk3_data6::RD_BLK3_DATA6_SPEC>;
///Register 6 of BLOCK3.
pub mod rd_blk3_data6;
/**RD_BLK3_DATA7 (r) register accessor: Register 7 of BLOCK3.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_blk3_data7`] module*/
pub type RD_BLK3_DATA7 = crate::Reg<rd_blk3_data7::RD_BLK3_DATA7_SPEC>;
///Register 7 of BLOCK3.
pub mod rd_blk3_data7;
/**RD_REPEAT_ERR (r) register accessor: Programming error record register 0 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_repeat_err`] module*/
pub type RD_REPEAT_ERR = crate::Reg<rd_repeat_err::RD_REPEAT_ERR_SPEC>;
///Programming error record register 0 of BLOCK0.
pub mod rd_repeat_err;
/**RD_RS_ERR (r) register accessor: Programming error record register 0 of BLOCK1-10.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rd_rs_err`] module*/
pub type RD_RS_ERR = crate::Reg<rd_rs_err::RD_RS_ERR_SPEC>;
///Programming error record register 0 of BLOCK1-10.
pub mod rd_rs_err;
/**CLK (rw) register accessor: eFuse clcok configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///eFuse clcok configuration register.
pub mod clk;
/**CONF (rw) register accessor: eFuse operation mode configuraiton register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///eFuse operation mode configuraiton register
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
/**INT_RAW (rw) register accessor: eFuse raw interrupt register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

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
/**WR_TIM_CONF0 (rw) register accessor: Configurarion register 0 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf0`] module*/
pub type WR_TIM_CONF0 = crate::Reg<wr_tim_conf0::WR_TIM_CONF0_SPEC>;
///Configurarion register 0 of eFuse programming timing parameters.
pub mod wr_tim_conf0;
/**WR_TIM_CONF1 (rw) register accessor: Configurarion register 1 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf1`] module*/
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
///Configurarion register 1 of eFuse programming timing parameters.
pub mod wr_tim_conf1;
/**WR_TIM_CONF2 (rw) register accessor: Configurarion register 2 of eFuse programming timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wr_tim_conf2`] module*/
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
///Configurarion register 2 of eFuse programming timing parameters.
pub mod wr_tim_conf2;
/**DATE (rw) register accessor: eFuse version register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///eFuse version register.
pub mod date;
