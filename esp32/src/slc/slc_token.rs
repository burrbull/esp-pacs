#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster SLC%s_TOKEN, containing _?TOKEN0, _?TOKEN1"]
pub struct SLC_TOKEN {
    token: [TOKEN; 2],
}
impl SLC_TOKEN {
    #[doc = "0x00..0x08 - "]
    #[inline(always)]
    pub const fn token(&self, n: usize) -> &TOKEN {
        &self.token[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - "]
    #[inline(always)]
    pub fn token_iter(&self) -> impl Iterator<Item = &TOKEN> {
        self.token.iter()
    }
}
#[doc = "TOKEN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`token::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@token`] module"]
pub type TOKEN = crate::Reg<token::TOKEN_SPEC>;
#[doc = ""]
pub mod token;
