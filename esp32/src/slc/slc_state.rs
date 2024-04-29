#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster SLC%s_STATE, containing _?_STATE0, _?_STATE1"]
pub struct SLC_STATE {
    state: [STATE; 2],
}
impl SLC_STATE {
    #[doc = "0x00..0x08 - "]
    #[inline(always)]
    pub const fn state(&self, n: usize) -> &STATE {
        &self.state[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - "]
    #[inline(always)]
    pub fn state_iter(&self) -> impl Iterator<Item = &STATE> {
        self.state.iter()
    }
}
#[doc = "STATE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = ""]
pub mod state;
