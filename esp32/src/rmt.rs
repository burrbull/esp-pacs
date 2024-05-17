#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    chdata: [CHDATA; 8],
    chconf0: (),
    _reserved2: [u8; 0x04],
    chconf1: (),
    _reserved3: [u8; 0x3c],
    chstatus: [CHSTATUS; 8],
    chaddr: [CHADDR; 8],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    chcarrier_duty: [CHCARRIER_DUTY; 8],
    ch_tx_lim: [CH_TX_LIM; 8],
    apb_conf: APB_CONF,
    _reserved12: [u8; 0x08],
    date: DATE,
}
impl RegisterBlock {
    ///0x00..0x20 -
    #[inline(always)]
    pub const fn chdata(&self, n: usize) -> &CHDATA {
        &self.chdata[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 -
    #[inline(always)]
    pub fn chdata_iter(&self) -> impl Iterator<Item = &CHDATA> {
        self.chdata.iter()
    }
    ///0x00 - CH0DATA
    #[inline(always)]
    pub const fn ch0data(&self) -> &CHDATA {
        self.chdata(0)
    }
    ///0x04 - CH1DATA
    #[inline(always)]
    pub const fn ch1data(&self) -> &CHDATA {
        self.chdata(1)
    }
    ///0x08 - CH2DATA
    #[inline(always)]
    pub const fn ch2data(&self) -> &CHDATA {
        self.chdata(2)
    }
    ///0x0c - CH3DATA
    #[inline(always)]
    pub const fn ch3data(&self) -> &CHDATA {
        self.chdata(3)
    }
    ///0x10 - CH4DATA
    #[inline(always)]
    pub const fn ch4data(&self) -> &CHDATA {
        self.chdata(4)
    }
    ///0x14 - CH5DATA
    #[inline(always)]
    pub const fn ch5data(&self) -> &CHDATA {
        self.chdata(5)
    }
    ///0x18 - CH6DATA
    #[inline(always)]
    pub const fn ch6data(&self) -> &CHDATA {
        self.chdata(6)
    }
    ///0x1c - CH7DATA
    #[inline(always)]
    pub const fn ch7data(&self) -> &CHDATA {
        self.chdata(7)
    }
    ///0x20..0x40 -
    #[inline(always)]
    pub const fn chconf0(&self, n: usize) -> &CHCONF0 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(32).add(8 * n).cast() }
    }
    ///Iterator for array of:
    ///0x20..0x40 -
    #[inline(always)]
    pub fn chconf0_iter(&self) -> impl Iterator<Item = &CHCONF0> {
        (0..8)
            .map(move |n| unsafe {
                &*(self as *const Self).cast::<u8>().add(32).add(8 * n).cast()
            })
    }
    ///0x20 - CH0CONF0
    #[inline(always)]
    pub const fn ch0conf0(&self) -> &CHCONF0 {
        self.chconf0(0)
    }
    ///0x28 - CH1CONF0
    #[inline(always)]
    pub const fn ch1conf0(&self) -> &CHCONF0 {
        self.chconf0(1)
    }
    ///0x30 - CH2CONF0
    #[inline(always)]
    pub const fn ch2conf0(&self) -> &CHCONF0 {
        self.chconf0(2)
    }
    ///0x38 - CH3CONF0
    #[inline(always)]
    pub const fn ch3conf0(&self) -> &CHCONF0 {
        self.chconf0(3)
    }
    ///0x40 - CH4CONF0
    #[inline(always)]
    pub const fn ch4conf0(&self) -> &CHCONF0 {
        self.chconf0(4)
    }
    ///0x48 - CH5CONF0
    #[inline(always)]
    pub const fn ch5conf0(&self) -> &CHCONF0 {
        self.chconf0(5)
    }
    ///0x50 - CH6CONF0
    #[inline(always)]
    pub const fn ch6conf0(&self) -> &CHCONF0 {
        self.chconf0(6)
    }
    ///0x58 - CH7CONF0
    #[inline(always)]
    pub const fn ch7conf0(&self) -> &CHCONF0 {
        self.chconf0(7)
    }
    ///0x24..0x44 -
    #[inline(always)]
    pub const fn chconf1(&self, n: usize) -> &CHCONF1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(36).add(8 * n).cast() }
    }
    ///Iterator for array of:
    ///0x24..0x44 -
    #[inline(always)]
    pub fn chconf1_iter(&self) -> impl Iterator<Item = &CHCONF1> {
        (0..8)
            .map(move |n| unsafe {
                &*(self as *const Self).cast::<u8>().add(36).add(8 * n).cast()
            })
    }
    ///0x24 - CH0CONF1
    #[inline(always)]
    pub const fn ch0conf1(&self) -> &CHCONF1 {
        self.chconf1(0)
    }
    ///0x2c - CH1CONF1
    #[inline(always)]
    pub const fn ch1conf1(&self) -> &CHCONF1 {
        self.chconf1(1)
    }
    ///0x34 - CH2CONF1
    #[inline(always)]
    pub const fn ch2conf1(&self) -> &CHCONF1 {
        self.chconf1(2)
    }
    ///0x3c - CH3CONF1
    #[inline(always)]
    pub const fn ch3conf1(&self) -> &CHCONF1 {
        self.chconf1(3)
    }
    ///0x44 - CH4CONF1
    #[inline(always)]
    pub const fn ch4conf1(&self) -> &CHCONF1 {
        self.chconf1(4)
    }
    ///0x4c - CH5CONF1
    #[inline(always)]
    pub const fn ch5conf1(&self) -> &CHCONF1 {
        self.chconf1(5)
    }
    ///0x54 - CH6CONF1
    #[inline(always)]
    pub const fn ch6conf1(&self) -> &CHCONF1 {
        self.chconf1(6)
    }
    ///0x5c - CH7CONF1
    #[inline(always)]
    pub const fn ch7conf1(&self) -> &CHCONF1 {
        self.chconf1(7)
    }
    ///0x60..0x80 -
    #[inline(always)]
    pub const fn chstatus(&self, n: usize) -> &CHSTATUS {
        &self.chstatus[n]
    }
    ///Iterator for array of:
    ///0x60..0x80 -
    #[inline(always)]
    pub fn chstatus_iter(&self) -> impl Iterator<Item = &CHSTATUS> {
        self.chstatus.iter()
    }
    ///0x60 - CH0STATUS
    #[inline(always)]
    pub const fn ch0status(&self) -> &CHSTATUS {
        self.chstatus(0)
    }
    ///0x64 - CH1STATUS
    #[inline(always)]
    pub const fn ch1status(&self) -> &CHSTATUS {
        self.chstatus(1)
    }
    ///0x68 - CH2STATUS
    #[inline(always)]
    pub const fn ch2status(&self) -> &CHSTATUS {
        self.chstatus(2)
    }
    ///0x6c - CH3STATUS
    #[inline(always)]
    pub const fn ch3status(&self) -> &CHSTATUS {
        self.chstatus(3)
    }
    ///0x70 - CH4STATUS
    #[inline(always)]
    pub const fn ch4status(&self) -> &CHSTATUS {
        self.chstatus(4)
    }
    ///0x74 - CH5STATUS
    #[inline(always)]
    pub const fn ch5status(&self) -> &CHSTATUS {
        self.chstatus(5)
    }
    ///0x78 - CH6STATUS
    #[inline(always)]
    pub const fn ch6status(&self) -> &CHSTATUS {
        self.chstatus(6)
    }
    ///0x7c - CH7STATUS
    #[inline(always)]
    pub const fn ch7status(&self) -> &CHSTATUS {
        self.chstatus(7)
    }
    ///0x80..0xa0 -
    #[inline(always)]
    pub const fn chaddr(&self, n: usize) -> &CHADDR {
        &self.chaddr[n]
    }
    ///Iterator for array of:
    ///0x80..0xa0 -
    #[inline(always)]
    pub fn chaddr_iter(&self) -> impl Iterator<Item = &CHADDR> {
        self.chaddr.iter()
    }
    ///0x80 - CH0ADDR
    #[inline(always)]
    pub const fn ch0addr(&self) -> &CHADDR {
        self.chaddr(0)
    }
    ///0x84 - CH1ADDR
    #[inline(always)]
    pub const fn ch1addr(&self) -> &CHADDR {
        self.chaddr(1)
    }
    ///0x88 - CH2ADDR
    #[inline(always)]
    pub const fn ch2addr(&self) -> &CHADDR {
        self.chaddr(2)
    }
    ///0x8c - CH3ADDR
    #[inline(always)]
    pub const fn ch3addr(&self) -> &CHADDR {
        self.chaddr(3)
    }
    ///0x90 - CH4ADDR
    #[inline(always)]
    pub const fn ch4addr(&self) -> &CHADDR {
        self.chaddr(4)
    }
    ///0x94 - CH5ADDR
    #[inline(always)]
    pub const fn ch5addr(&self) -> &CHADDR {
        self.chaddr(5)
    }
    ///0x98 - CH6ADDR
    #[inline(always)]
    pub const fn ch6addr(&self) -> &CHADDR {
        self.chaddr(6)
    }
    ///0x9c - CH7ADDR
    #[inline(always)]
    pub const fn ch7addr(&self) -> &CHADDR {
        self.chaddr(7)
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0xac -
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0xb0..0xd0 -
    #[inline(always)]
    pub const fn chcarrier_duty(&self, n: usize) -> &CHCARRIER_DUTY {
        &self.chcarrier_duty[n]
    }
    ///Iterator for array of:
    ///0xb0..0xd0 -
    #[inline(always)]
    pub fn chcarrier_duty_iter(&self) -> impl Iterator<Item = &CHCARRIER_DUTY> {
        self.chcarrier_duty.iter()
    }
    ///0xb0 - CH0CARRIER_DUTY
    #[inline(always)]
    pub const fn ch0carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(0)
    }
    ///0xb4 - CH1CARRIER_DUTY
    #[inline(always)]
    pub const fn ch1carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(1)
    }
    ///0xb8 - CH2CARRIER_DUTY
    #[inline(always)]
    pub const fn ch2carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(2)
    }
    ///0xbc - CH3CARRIER_DUTY
    #[inline(always)]
    pub const fn ch3carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(3)
    }
    ///0xc0 - CH4CARRIER_DUTY
    #[inline(always)]
    pub const fn ch4carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(4)
    }
    ///0xc4 - CH5CARRIER_DUTY
    #[inline(always)]
    pub const fn ch5carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(5)
    }
    ///0xc8 - CH6CARRIER_DUTY
    #[inline(always)]
    pub const fn ch6carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(6)
    }
    ///0xcc - CH7CARRIER_DUTY
    #[inline(always)]
    pub const fn ch7carrier_duty(&self) -> &CHCARRIER_DUTY {
        self.chcarrier_duty(7)
    }
    ///0xd0..0xf0 -
    #[inline(always)]
    pub const fn ch_tx_lim(&self, n: usize) -> &CH_TX_LIM {
        &self.ch_tx_lim[n]
    }
    ///Iterator for array of:
    ///0xd0..0xf0 -
    #[inline(always)]
    pub fn ch_tx_lim_iter(&self) -> impl Iterator<Item = &CH_TX_LIM> {
        self.ch_tx_lim.iter()
    }
    ///0xd0 - CH0_TX_LIM
    #[inline(always)]
    pub const fn ch0_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(0)
    }
    ///0xd4 - CH1_TX_LIM
    #[inline(always)]
    pub const fn ch1_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(1)
    }
    ///0xd8 - CH2_TX_LIM
    #[inline(always)]
    pub const fn ch2_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(2)
    }
    ///0xdc - CH3_TX_LIM
    #[inline(always)]
    pub const fn ch3_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(3)
    }
    ///0xe0 - CH4_TX_LIM
    #[inline(always)]
    pub const fn ch4_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(4)
    }
    ///0xe4 - CH5_TX_LIM
    #[inline(always)]
    pub const fn ch5_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(5)
    }
    ///0xe8 - CH6_TX_LIM
    #[inline(always)]
    pub const fn ch6_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(6)
    }
    ///0xec - CH7_TX_LIM
    #[inline(always)]
    pub const fn ch7_tx_lim(&self) -> &CH_TX_LIM {
        self.ch_tx_lim(7)
    }
    ///0xf0 -
    #[inline(always)]
    pub const fn apb_conf(&self) -> &APB_CONF {
        &self.apb_conf
    }
    ///0xfc -
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CHDATA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`chdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chdata`] module*/
pub type CHDATA = crate::Reg<chdata::CHDATA_SPEC>;
///
pub mod chdata;
/**CHCONF0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`chconf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chconf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chconf0`] module*/
pub type CHCONF0 = crate::Reg<chconf0::CHCONF0_SPEC>;
///
pub mod chconf0;
/**CHCONF1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`chconf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chconf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chconf1`] module*/
pub type CHCONF1 = crate::Reg<chconf1::CHCONF1_SPEC>;
///
pub mod chconf1;
/**CHSTATUS (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chstatus`] module*/
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
///
pub mod chstatus;
/**CHADDR (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`chaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chaddr`] module*/
pub type CHADDR = crate::Reg<chaddr::CHADDR_SPEC>;
///
pub mod chaddr;
/**INT_RAW (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///
pub mod int_raw;
/**INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///
pub mod int_st;
/**INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///
pub mod int_ena;
/**INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///
pub mod int_clr;
/**CHCARRIER_DUTY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`chcarrier_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcarrier_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chcarrier_duty`] module*/
pub type CHCARRIER_DUTY = crate::Reg<chcarrier_duty::CHCARRIER_DUTY_SPEC>;
///
pub mod chcarrier_duty;
/**CH_TX_LIM (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_lim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_lim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ch_tx_lim`] module*/
pub type CH_TX_LIM = crate::Reg<ch_tx_lim::CH_TX_LIM_SPEC>;
///
pub mod ch_tx_lim;
/**APB_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`apb_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@apb_conf`] module*/
pub type APB_CONF = crate::Reg<apb_conf::APB_CONF_SPEC>;
///
pub mod apb_conf;
/**DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///
pub mod date;
