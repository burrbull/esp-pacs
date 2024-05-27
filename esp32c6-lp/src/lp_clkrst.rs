#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    lp_clk_conf: LP_CLK_CONF,
    lp_clk_po_en: LP_CLK_PO_EN,
    lp_clk_en: LP_CLK_EN,
    lp_rst_en: LP_RST_EN,
    reset_cause: RESET_CAUSE,
    cpu_reset: CPU_RESET,
    fosc_cntl: FOSC_CNTL,
    rc32k_cntl: RC32K_CNTL,
    clk_to_hp: CLK_TO_HP,
    lpmem_force: LPMEM_FORCE,
    lpperi: LPPERI,
    xtal32k: XTAL32K,
    _reserved12: [u8; 0x03cc],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - need_des
    #[inline(always)]
    pub const fn lp_clk_conf(&self) -> &LP_CLK_CONF {
        &self.lp_clk_conf
    }
    ///0x04 - need_des
    #[inline(always)]
    pub const fn lp_clk_po_en(&self) -> &LP_CLK_PO_EN {
        &self.lp_clk_po_en
    }
    ///0x08 - need_des
    #[inline(always)]
    pub const fn lp_clk_en(&self) -> &LP_CLK_EN {
        &self.lp_clk_en
    }
    ///0x0c - need_des
    #[inline(always)]
    pub const fn lp_rst_en(&self) -> &LP_RST_EN {
        &self.lp_rst_en
    }
    ///0x10 - need_des
    #[inline(always)]
    pub const fn reset_cause(&self) -> &RESET_CAUSE {
        &self.reset_cause
    }
    ///0x14 - need_des
    #[inline(always)]
    pub const fn cpu_reset(&self) -> &CPU_RESET {
        &self.cpu_reset
    }
    ///0x18 - need_des
    #[inline(always)]
    pub const fn fosc_cntl(&self) -> &FOSC_CNTL {
        &self.fosc_cntl
    }
    ///0x1c - need_des
    #[inline(always)]
    pub const fn rc32k_cntl(&self) -> &RC32K_CNTL {
        &self.rc32k_cntl
    }
    ///0x20 - need_des
    #[inline(always)]
    pub const fn clk_to_hp(&self) -> &CLK_TO_HP {
        &self.clk_to_hp
    }
    ///0x24 - need_des
    #[inline(always)]
    pub const fn lpmem_force(&self) -> &LPMEM_FORCE {
        &self.lpmem_force
    }
    ///0x28 - need_des
    #[inline(always)]
    pub const fn lpperi(&self) -> &LPPERI {
        &self.lpperi
    }
    ///0x2c - need_des
    #[inline(always)]
    pub const fn xtal32k(&self) -> &XTAL32K {
        &self.xtal32k
    }
    ///0x3fc - need_des
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**LP_CLK_CONF (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_clk_conf`] module*/
pub type LP_CLK_CONF = crate::Reg<lp_clk_conf::LP_CLK_CONF_SPEC>;
///need_des
pub mod lp_clk_conf;
/**LP_CLK_PO_EN (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_po_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_po_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_clk_po_en`] module*/
pub type LP_CLK_PO_EN = crate::Reg<lp_clk_po_en::LP_CLK_PO_EN_SPEC>;
///need_des
pub mod lp_clk_po_en;
/**LP_CLK_EN (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_clk_en`] module*/
pub type LP_CLK_EN = crate::Reg<lp_clk_en::LP_CLK_EN_SPEC>;
///need_des
pub mod lp_clk_en;
/**LP_RST_EN (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_rst_en`] module*/
pub type LP_RST_EN = crate::Reg<lp_rst_en::LP_RST_EN_SPEC>;
///need_des
pub mod lp_rst_en;
/**RESET_CAUSE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`reset_cause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_cause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_cause`] module*/
pub type RESET_CAUSE = crate::Reg<reset_cause::RESET_CAUSE_SPEC>;
///need_des
pub mod reset_cause;
/**CPU_RESET (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_reset`] module*/
pub type CPU_RESET = crate::Reg<cpu_reset::CPU_RESET_SPEC>;
///need_des
pub mod cpu_reset;
/**FOSC_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`fosc_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fosc_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fosc_cntl`] module*/
pub type FOSC_CNTL = crate::Reg<fosc_cntl::FOSC_CNTL_SPEC>;
///need_des
pub mod fosc_cntl;
/**RC32K_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`rc32k_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32k_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rc32k_cntl`] module*/
pub type RC32K_CNTL = crate::Reg<rc32k_cntl::RC32K_CNTL_SPEC>;
///need_des
pub mod rc32k_cntl;
/**CLK_TO_HP (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`clk_to_hp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_to_hp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_to_hp`] module*/
pub type CLK_TO_HP = crate::Reg<clk_to_hp::CLK_TO_HP_SPEC>;
///need_des
pub mod clk_to_hp;
/**LPMEM_FORCE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lpmem_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmem_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpmem_force`] module*/
pub type LPMEM_FORCE = crate::Reg<lpmem_force::LPMEM_FORCE_SPEC>;
///need_des
pub mod lpmem_force;
/**LPPERI (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lpperi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpperi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpperi`] module*/
pub type LPPERI = crate::Reg<lpperi::LPPERI_SPEC>;
///need_des
pub mod lpperi;
/**XTAL32K (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`xtal32k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal32k`] module*/
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
///need_des
pub mod xtal32k;
/**DATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need_des
pub mod date;
