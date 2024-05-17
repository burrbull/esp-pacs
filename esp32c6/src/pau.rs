#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    regdma_conf: REGDMA_CONF,
    regdma_clk_conf: REGDMA_CLK_CONF,
    regdma_etm_ctrl: REGDMA_ETM_CTRL,
    regdma_link_0_addr: REGDMA_LINK_0_ADDR,
    regdma_link_1_addr: REGDMA_LINK_1_ADDR,
    regdma_link_2_addr: REGDMA_LINK_2_ADDR,
    regdma_link_3_addr: REGDMA_LINK_3_ADDR,
    regdma_link_mac_addr: REGDMA_LINK_MAC_ADDR,
    regdma_current_link_addr: REGDMA_CURRENT_LINK_ADDR,
    regdma_backup_addr: REGDMA_BACKUP_ADDR,
    regdma_mem_addr: REGDMA_MEM_ADDR,
    regdma_bkp_conf: REGDMA_BKP_CONF,
    retention_link_base: RETENTION_LINK_BASE,
    retention_cfg: RETENTION_CFG,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    _reserved18: [u8; 0x03b4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Peri backup control register
    #[inline(always)]
    pub const fn regdma_conf(&self) -> &REGDMA_CONF {
        &self.regdma_conf
    }
    ///0x04 - Clock control register
    #[inline(always)]
    pub const fn regdma_clk_conf(&self) -> &REGDMA_CLK_CONF {
        &self.regdma_clk_conf
    }
    ///0x08 - ETM start ctrl reg
    #[inline(always)]
    pub const fn regdma_etm_ctrl(&self) -> &REGDMA_ETM_CTRL {
        &self.regdma_etm_ctrl
    }
    ///0x0c - link_0_addr
    #[inline(always)]
    pub const fn regdma_link_0_addr(&self) -> &REGDMA_LINK_0_ADDR {
        &self.regdma_link_0_addr
    }
    ///0x10 - Link_1_addr
    #[inline(always)]
    pub const fn regdma_link_1_addr(&self) -> &REGDMA_LINK_1_ADDR {
        &self.regdma_link_1_addr
    }
    ///0x14 - Link_2_addr
    #[inline(always)]
    pub const fn regdma_link_2_addr(&self) -> &REGDMA_LINK_2_ADDR {
        &self.regdma_link_2_addr
    }
    ///0x18 - Link_3_addr
    #[inline(always)]
    pub const fn regdma_link_3_addr(&self) -> &REGDMA_LINK_3_ADDR {
        &self.regdma_link_3_addr
    }
    ///0x1c - Link_mac_addr
    #[inline(always)]
    pub const fn regdma_link_mac_addr(&self) -> &REGDMA_LINK_MAC_ADDR {
        &self.regdma_link_mac_addr
    }
    ///0x20 - current link addr
    #[inline(always)]
    pub const fn regdma_current_link_addr(&self) -> &REGDMA_CURRENT_LINK_ADDR {
        &self.regdma_current_link_addr
    }
    ///0x24 - Backup addr
    #[inline(always)]
    pub const fn regdma_backup_addr(&self) -> &REGDMA_BACKUP_ADDR {
        &self.regdma_backup_addr
    }
    ///0x28 - mem addr
    #[inline(always)]
    pub const fn regdma_mem_addr(&self) -> &REGDMA_MEM_ADDR {
        &self.regdma_mem_addr
    }
    ///0x2c - backup config
    #[inline(always)]
    pub const fn regdma_bkp_conf(&self) -> &REGDMA_BKP_CONF {
        &self.regdma_bkp_conf
    }
    ///0x30 - retention dma link base
    #[inline(always)]
    pub const fn retention_link_base(&self) -> &RETENTION_LINK_BASE {
        &self.retention_link_base
    }
    ///0x34 - retention_cfg
    #[inline(always)]
    pub const fn retention_cfg(&self) -> &RETENTION_CFG {
        &self.retention_cfg
    }
    ///0x38 - Read only register for error and done
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x3c - Read only register for error and done
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x40 - Read only register for error and done
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x44 - Read only register for error and done
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x3fc - Date register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**REGDMA_CONF (rw) register accessor: Peri backup control register

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_conf`] module*/
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
///Peri backup control register
pub mod regdma_conf;
/**REGDMA_CLK_CONF (rw) register accessor: Clock control register

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_clk_conf`] module*/
pub type REGDMA_CLK_CONF = crate::Reg<regdma_clk_conf::REGDMA_CLK_CONF_SPEC>;
///Clock control register
pub mod regdma_clk_conf;
/**REGDMA_ETM_CTRL (w) register accessor: ETM start ctrl reg

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_etm_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_etm_ctrl`] module*/
pub type REGDMA_ETM_CTRL = crate::Reg<regdma_etm_ctrl::REGDMA_ETM_CTRL_SPEC>;
///ETM start ctrl reg
pub mod regdma_etm_ctrl;
/**REGDMA_LINK_0_ADDR (rw) register accessor: link_0_addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_link_0_addr`] module*/
pub type REGDMA_LINK_0_ADDR = crate::Reg<regdma_link_0_addr::REGDMA_LINK_0_ADDR_SPEC>;
///link_0_addr
pub mod regdma_link_0_addr;
/**REGDMA_LINK_1_ADDR (rw) register accessor: Link_1_addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_link_1_addr`] module*/
pub type REGDMA_LINK_1_ADDR = crate::Reg<regdma_link_1_addr::REGDMA_LINK_1_ADDR_SPEC>;
///Link_1_addr
pub mod regdma_link_1_addr;
/**REGDMA_LINK_2_ADDR (rw) register accessor: Link_2_addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_link_2_addr`] module*/
pub type REGDMA_LINK_2_ADDR = crate::Reg<regdma_link_2_addr::REGDMA_LINK_2_ADDR_SPEC>;
///Link_2_addr
pub mod regdma_link_2_addr;
/**REGDMA_LINK_3_ADDR (rw) register accessor: Link_3_addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_link_3_addr`] module*/
pub type REGDMA_LINK_3_ADDR = crate::Reg<regdma_link_3_addr::REGDMA_LINK_3_ADDR_SPEC>;
///Link_3_addr
pub mod regdma_link_3_addr;
/**REGDMA_LINK_MAC_ADDR (rw) register accessor: Link_mac_addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_mac_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_mac_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_link_mac_addr`] module*/
pub type REGDMA_LINK_MAC_ADDR = crate::Reg<
    regdma_link_mac_addr::REGDMA_LINK_MAC_ADDR_SPEC,
>;
///Link_mac_addr
pub mod regdma_link_mac_addr;
/**REGDMA_CURRENT_LINK_ADDR (r) register accessor: current link addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_current_link_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_current_link_addr`] module*/
pub type REGDMA_CURRENT_LINK_ADDR = crate::Reg<
    regdma_current_link_addr::REGDMA_CURRENT_LINK_ADDR_SPEC,
>;
///current link addr
pub mod regdma_current_link_addr;
/**REGDMA_BACKUP_ADDR (r) register accessor: Backup addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_backup_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_backup_addr`] module*/
pub type REGDMA_BACKUP_ADDR = crate::Reg<regdma_backup_addr::REGDMA_BACKUP_ADDR_SPEC>;
///Backup addr
pub mod regdma_backup_addr;
/**REGDMA_MEM_ADDR (r) register accessor: mem addr

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_mem_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_mem_addr`] module*/
pub type REGDMA_MEM_ADDR = crate::Reg<regdma_mem_addr::REGDMA_MEM_ADDR_SPEC>;
///mem addr
pub mod regdma_mem_addr;
/**REGDMA_BKP_CONF (rw) register accessor: backup config

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_bkp_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_bkp_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_bkp_conf`] module*/
pub type REGDMA_BKP_CONF = crate::Reg<regdma_bkp_conf::REGDMA_BKP_CONF_SPEC>;
///backup config
pub mod regdma_bkp_conf;
/**RETENTION_LINK_BASE (rw) register accessor: retention dma link base

You can [`read`](crate::generic::Reg::read) this register and get [`retention_link_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_link_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@retention_link_base`] module*/
pub type RETENTION_LINK_BASE = crate::Reg<retention_link_base::RETENTION_LINK_BASE_SPEC>;
///retention dma link base
pub mod retention_link_base;
/**RETENTION_CFG (rw) register accessor: retention_cfg

You can [`read`](crate::generic::Reg::read) this register and get [`retention_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@retention_cfg`] module*/
pub type RETENTION_CFG = crate::Reg<retention_cfg::RETENTION_CFG_SPEC>;
///retention_cfg
pub mod retention_cfg;
/**INT_ENA (rw) register accessor: Read only register for error and done

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Read only register for error and done
pub mod int_ena;
/**INT_RAW (rw) register accessor: Read only register for error and done

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Read only register for error and done
pub mod int_raw;
/**INT_CLR (w) register accessor: Read only register for error and done

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Read only register for error and done
pub mod int_clr;
/**INT_ST (r) register accessor: Read only register for error and done

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Read only register for error and done
pub mod int_st;
/**DATE (rw) register accessor: Date register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Date register.
pub mod date;
