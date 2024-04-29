#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster SLC%s_LINK_DSCR, containing _?_TXLINK_DSCR, _?_TXLINK_DSCR_BF0, _?_TXLINK_DSCR_BF1, _?_RXLINK_DSCR, _?_RXLINK_DSCR_BF0, _?_RXLINK_DSCR_BF1"]
pub struct SLC_LINK_DSCR {
    txlink_dscr: TXLINK_DSCR,
    txlink_dscr_bf0: TXLINK_DSCR_BF0,
    txlink_dscr_bf1: TXLINK_DSCR_BF1,
    rxlink_dscr: RXLINK_DSCR,
    rxlink_dscr_bf0: RXLINK_DSCR_BF0,
    rxlink_dscr_bf1: RXLINK_DSCR_BF1,
}
impl SLC_LINK_DSCR {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn txlink_dscr(&self) -> &TXLINK_DSCR {
        &self.txlink_dscr
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn txlink_dscr_bf0(&self) -> &TXLINK_DSCR_BF0 {
        &self.txlink_dscr_bf0
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn txlink_dscr_bf1(&self) -> &TXLINK_DSCR_BF1 {
        &self.txlink_dscr_bf1
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn rxlink_dscr(&self) -> &RXLINK_DSCR {
        &self.rxlink_dscr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn rxlink_dscr_bf0(&self) -> &RXLINK_DSCR_BF0 {
        &self.rxlink_dscr_bf0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn rxlink_dscr_bf1(&self) -> &RXLINK_DSCR_BF1 {
        &self.rxlink_dscr_bf1
    }
}
#[doc = "TXLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlink_dscr`] module"]
pub type TXLINK_DSCR = crate::Reg<txlink_dscr::TXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod txlink_dscr;
#[doc = "TXLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlink_dscr_bf0`] module"]
pub type TXLINK_DSCR_BF0 = crate::Reg<txlink_dscr_bf0::TXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod txlink_dscr_bf0;
#[doc = "TXLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlink_dscr_bf1`] module"]
pub type TXLINK_DSCR_BF1 = crate::Reg<txlink_dscr_bf1::TXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod txlink_dscr_bf1;
#[doc = "RXLINK_DSCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlink_dscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlink_dscr`] module"]
pub type RXLINK_DSCR = crate::Reg<rxlink_dscr::RXLINK_DSCR_SPEC>;
#[doc = ""]
pub mod rxlink_dscr;
#[doc = "RXLINK_DSCR_BF0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlink_dscr_bf0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlink_dscr_bf0`] module"]
pub type RXLINK_DSCR_BF0 = crate::Reg<rxlink_dscr_bf0::RXLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod rxlink_dscr_bf0;
#[doc = "RXLINK_DSCR_BF1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlink_dscr_bf1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlink_dscr_bf1`] module"]
pub type RXLINK_DSCR_BF1 = crate::Reg<rxlink_dscr_bf1::RXLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod rxlink_dscr_bf1;
