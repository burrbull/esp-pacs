#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster SLC%s_DES_ADDR, containing _?_TO_EOF_DES_ADDR, _?_TX_EOF_DES_ADDR, _?_TO_EOF_BFR_DES_ADDR"]
pub struct SLC_DES_ADDR {
    to_eof: TO_EOF,
    tx_suc_eof: TX_SUC_EOF,
    to_eof_bfr: TO_EOF_BFR,
}
impl SLC_DES_ADDR {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn to_eof(&self) -> &TO_EOF {
        &self.to_eof
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn tx_suc_eof(&self) -> &TX_SUC_EOF {
        &self.tx_suc_eof
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn to_eof_bfr(&self) -> &TO_EOF_BFR {
        &self.to_eof_bfr
    }
}
#[doc = "TO_EOF (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to_eof::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to_eof`] module"]
pub type TO_EOF = crate::Reg<to_eof::TO_EOF_SPEC>;
#[doc = ""]
pub mod to_eof;
#[doc = "TX_SUC_EOF (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_suc_eof::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_suc_eof`] module"]
pub type TX_SUC_EOF = crate::Reg<tx_suc_eof::TX_SUC_EOF_SPEC>;
#[doc = ""]
pub mod tx_suc_eof;
#[doc = "TO_EOF_BFR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`to_eof_bfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to_eof_bfr`] module"]
pub type TO_EOF_BFR = crate::Reg<to_eof_bfr::TO_EOF_BFR_SPEC>;
#[doc = ""]
pub mod to_eof_bfr;
