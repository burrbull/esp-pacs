#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    core_0_montr_ena: CORE_0_MONTR_ENA,
    core_0_intr_raw: CORE_0_INTR_RAW,
    core_0_intr_ena: CORE_0_INTR_ENA,
    core_0_intr_clr: CORE_0_INTR_CLR,
    core_0_sp_min: CORE_0_SP_MIN,
    core_0_sp_max: CORE_0_SP_MAX,
    core_0_sp_pc: CORE_0_SP_PC,
    core_0_rcd_en: CORE_0_RCD_EN,
    core_0_rcd_pdebugpc: CORE_0_RCD_PDEBUGPC,
    core_0_rcd_pdebugsp: CORE_0_RCD_PDEBUGSP,
    core_0_lastpc_before_exception: CORE_0_LASTPC_BEFORE_EXCEPTION,
    core_0_debug_mode: CORE_0_DEBUG_MODE,
    clock_gate: CLOCK_GATE,
    _reserved13: [u8; 0x01c8],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - core0 monitor enable configuration register
    #[inline(always)]
    pub const fn core_0_montr_ena(&self) -> &CORE_0_MONTR_ENA {
        &self.core_0_montr_ena
    }
    ///0x04 - core0 monitor interrupt status register
    #[inline(always)]
    pub const fn core_0_intr_raw(&self) -> &CORE_0_INTR_RAW {
        &self.core_0_intr_raw
    }
    ///0x08 - core0 monitor interrupt enable register
    #[inline(always)]
    pub const fn core_0_intr_ena(&self) -> &CORE_0_INTR_ENA {
        &self.core_0_intr_ena
    }
    ///0x0c - core0 monitor interrupt clr register
    #[inline(always)]
    pub const fn core_0_intr_clr(&self) -> &CORE_0_INTR_CLR {
        &self.core_0_intr_clr
    }
    ///0x10 - stack min value
    #[inline(always)]
    pub const fn core_0_sp_min(&self) -> &CORE_0_SP_MIN {
        &self.core_0_sp_min
    }
    ///0x14 - stack max value
    #[inline(always)]
    pub const fn core_0_sp_max(&self) -> &CORE_0_SP_MAX {
        &self.core_0_sp_max
    }
    ///0x18 - stack monitor pc status register
    #[inline(always)]
    pub const fn core_0_sp_pc(&self) -> &CORE_0_SP_PC {
        &self.core_0_sp_pc
    }
    ///0x1c - record enable configuration register
    #[inline(always)]
    pub const fn core_0_rcd_en(&self) -> &CORE_0_RCD_EN {
        &self.core_0_rcd_en
    }
    ///0x20 - record status regsiter
    #[inline(always)]
    pub const fn core_0_rcd_pdebugpc(&self) -> &CORE_0_RCD_PDEBUGPC {
        &self.core_0_rcd_pdebugpc
    }
    ///0x24 - record status regsiter
    #[inline(always)]
    pub const fn core_0_rcd_pdebugsp(&self) -> &CORE_0_RCD_PDEBUGSP {
        &self.core_0_rcd_pdebugsp
    }
    ///0x28 - cpu status register
    #[inline(always)]
    pub const fn core_0_lastpc_before_exception(&self) -> &CORE_0_LASTPC_BEFORE_EXCEPTION {
        &self.core_0_lastpc_before_exception
    }
    ///0x2c - cpu status register
    #[inline(always)]
    pub const fn core_0_debug_mode(&self) -> &CORE_0_DEBUG_MODE {
        &self.core_0_debug_mode
    }
    ///0x30 - clock gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x1fc - version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CORE_0_MONTR_ENA (rw) register accessor: core0 monitor enable configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_montr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_montr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_montr_ena`] module*/
pub type CORE_0_MONTR_ENA = crate::Reg<core_0_montr_ena::CORE_0_MONTR_ENA_SPEC>;
///core0 monitor enable configuration register
pub mod core_0_montr_ena;
/**CORE_0_INTR_RAW (r) register accessor: core0 monitor interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_intr_raw`] module*/
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
///core0 monitor interrupt status register
pub mod core_0_intr_raw;
/**CORE_0_INTR_ENA (rw) register accessor: core0 monitor interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_intr_ena`] module*/
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
///core0 monitor interrupt enable register
pub mod core_0_intr_ena;
/**CORE_0_INTR_CLR (w) register accessor: core0 monitor interrupt clr register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_intr_clr`] module*/
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
///core0 monitor interrupt clr register
pub mod core_0_intr_clr;
/**CORE_0_SP_MIN (rw) register accessor: stack min value

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_sp_min`] module*/
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
///stack min value
pub mod core_0_sp_min;
/**CORE_0_SP_MAX (rw) register accessor: stack max value

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_sp_max`] module*/
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
///stack max value
pub mod core_0_sp_max;
/**CORE_0_SP_PC (r) register accessor: stack monitor pc status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_sp_pc`] module*/
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
///stack monitor pc status register
pub mod core_0_sp_pc;
/**CORE_0_RCD_EN (rw) register accessor: record enable configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_rcd_en`] module*/
pub type CORE_0_RCD_EN = crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>;
///record enable configuration register
pub mod core_0_rcd_en;
/**CORE_0_RCD_PDEBUGPC (r) register accessor: record status regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_rcd_pdebugpc`] module*/
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
///record status regsiter
pub mod core_0_rcd_pdebugpc;
/**CORE_0_RCD_PDEBUGSP (r) register accessor: record status regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_rcd_pdebugsp`] module*/
pub type CORE_0_RCD_PDEBUGSP = crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>;
///record status regsiter
pub mod core_0_rcd_pdebugsp;
/**CORE_0_LASTPC_BEFORE_EXCEPTION (r) register accessor: cpu status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_lastpc_before_exception::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_lastpc_before_exception`] module*/
pub type CORE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<core_0_lastpc_before_exception::CORE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
///cpu status register
pub mod core_0_lastpc_before_exception;
/**CORE_0_DEBUG_MODE (r) register accessor: cpu status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_debug_mode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_0_debug_mode`] module*/
pub type CORE_0_DEBUG_MODE = crate::Reg<core_0_debug_mode::CORE_0_DEBUG_MODE_SPEC>;
///cpu status register
pub mod core_0_debug_mode;
/**CLOCK_GATE (rw) register accessor: clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///clock gate register
pub mod clock_gate;
/**DATE (rw) register accessor: version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///version register
pub mod date;
