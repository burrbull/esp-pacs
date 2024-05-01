#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    m_mode_ctrl: [M_MODE_CTRL; 1],
    clock_gate: CLOCK_GATE,
    _reserved2: [u8; 0x88],
    force_acc_hp: FORCE_ACC_HP,
    _reserved3: [u8; 0x68],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Tee mode control register
    #[inline(always)]
    pub const fn m_mode_ctrl(&self, n: usize) -> &M_MODE_CTRL {
        &self.m_mode_ctrl[n]
    }
    ///Iterator for array of:
    ///0x00 - Tee mode control register
    #[inline(always)]
    pub fn m_mode_ctrl_iter(&self) -> impl Iterator<Item = &M_MODE_CTRL> {
        self.m_mode_ctrl.iter()
    }
    ///0x00 - Tee mode control register
    #[inline(always)]
    pub const fn m0_mode_ctrl(&self) -> &M_MODE_CTRL {
        self.m_mode_ctrl(0)
    }
    ///0x04 - Clock gating register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x90 - need_des
    #[inline(always)]
    pub const fn force_acc_hp(&self) -> &FORCE_ACC_HP {
        &self.force_acc_hp
    }
    ///0xfc - Version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**M_MODE_CTRL (rw) register accessor: Tee mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`m_mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m_mode_ctrl`] module*/
pub type M_MODE_CTRL = crate::Reg<m_mode_ctrl::M_MODE_CTRL_SPEC>;
///Tee mode control register
pub mod m_mode_ctrl;
/**CLOCK_GATE (rw) register accessor: Clock gating register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///Clock gating register
pub mod clock_gate;
/**FORCE_ACC_HP (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`force_acc_hp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_acc_hp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@force_acc_hp`] module*/
pub type FORCE_ACC_HP = crate::Reg<force_acc_hp::FORCE_ACC_HP_SPEC>;
///need_des
pub mod force_acc_hp;
/**DATE (rw) register accessor: Version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version register
pub mod date;
