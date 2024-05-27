#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    clk_en: CLK_EN,
    reset_en: RESET_EN,
    rng_data: RNG_DATA,
    cpu: CPU,
    bus_timeout: BUS_TIMEOUT,
    bus_timeout_addr: BUS_TIMEOUT_ADDR,
    bus_timeout_uid: BUS_TIMEOUT_UID,
    mem_ctrl: MEM_CTRL,
    interrupt_source: INTERRUPT_SOURCE,
    _reserved9: [u8; 0x03d8],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - need_des
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    ///0x04 - need_des
    #[inline(always)]
    pub const fn reset_en(&self) -> &RESET_EN {
        &self.reset_en
    }
    ///0x08 - need_des
    #[inline(always)]
    pub const fn rng_data(&self) -> &RNG_DATA {
        &self.rng_data
    }
    ///0x0c - need_des
    #[inline(always)]
    pub const fn cpu(&self) -> &CPU {
        &self.cpu
    }
    ///0x10 - need_des
    #[inline(always)]
    pub const fn bus_timeout(&self) -> &BUS_TIMEOUT {
        &self.bus_timeout
    }
    ///0x14 - need_des
    #[inline(always)]
    pub const fn bus_timeout_addr(&self) -> &BUS_TIMEOUT_ADDR {
        &self.bus_timeout_addr
    }
    ///0x18 - need_des
    #[inline(always)]
    pub const fn bus_timeout_uid(&self) -> &BUS_TIMEOUT_UID {
        &self.bus_timeout_uid
    }
    ///0x1c - need_des
    #[inline(always)]
    pub const fn mem_ctrl(&self) -> &MEM_CTRL {
        &self.mem_ctrl
    }
    ///0x20 - need_des
    #[inline(always)]
    pub const fn interrupt_source(&self) -> &INTERRUPT_SOURCE {
        &self.interrupt_source
    }
    ///0x3fc - need_des
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CLK_EN (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_en`] module*/
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
///need_des
pub mod clk_en;
/**RESET_EN (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`reset_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_en`] module*/
pub type RESET_EN = crate::Reg<reset_en::RESET_EN_SPEC>;
///need_des
pub mod reset_en;
/**RNG_DATA (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`rng_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rng_data`] module*/
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
///need_des
pub mod rng_data;
/**CPU (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu`] module*/
pub type CPU = crate::Reg<cpu::CPU_SPEC>;
///need_des
pub mod cpu;
/**BUS_TIMEOUT (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bus_timeout`] module*/
pub type BUS_TIMEOUT = crate::Reg<bus_timeout::BUS_TIMEOUT_SPEC>;
///need_des
pub mod bus_timeout;
/**BUS_TIMEOUT_ADDR (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bus_timeout_addr`] module*/
pub type BUS_TIMEOUT_ADDR = crate::Reg<bus_timeout_addr::BUS_TIMEOUT_ADDR_SPEC>;
///need_des
pub mod bus_timeout_addr;
/**BUS_TIMEOUT_UID (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout_uid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bus_timeout_uid`] module*/
pub type BUS_TIMEOUT_UID = crate::Reg<bus_timeout_uid::BUS_TIMEOUT_UID_SPEC>;
///need_des
pub mod bus_timeout_uid;
/**MEM_CTRL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_ctrl`] module*/
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
///need_des
pub mod mem_ctrl;
/**INTERRUPT_SOURCE (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`interrupt_source::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@interrupt_source`] module*/
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
///need_des
pub mod interrupt_source;
/**DATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need_des
pub mod date;
