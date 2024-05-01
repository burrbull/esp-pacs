#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    otg_conf: OTG_CONF,
    test_conf: TEST_CONF,
    _reserved2: [u8; 0x03f4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - USB wrapper configuration registers.
    #[inline(always)]
    pub const fn otg_conf(&self) -> &OTG_CONF {
        &self.otg_conf
    }
    ///0x04 - USB wrapper test configuration registers.
    #[inline(always)]
    pub const fn test_conf(&self) -> &TEST_CONF {
        &self.test_conf
    }
    ///0x3fc - Date register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**OTG_CONF (rw) register accessor: USB wrapper configuration registers.

You can [`read`](crate::generic::Reg::read) this register and get [`otg_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@otg_conf`] module*/
pub type OTG_CONF = crate::Reg<otg_conf::OTG_CONF_SPEC>;
///USB wrapper configuration registers.
pub mod otg_conf;
/**TEST_CONF (rw) register accessor: USB wrapper test configuration registers.

You can [`read`](crate::generic::Reg::read) this register and get [`test_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@test_conf`] module*/
pub type TEST_CONF = crate::Reg<test_conf::TEST_CONF_SPEC>;
///USB wrapper test configuration registers.
pub mod test_conf;
/**DATE (r) register accessor: Date register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Date register.
pub mod date;
