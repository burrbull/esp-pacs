#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster REG_Q%s, containing REG_Q?_WORD0, REG_Q?_WORD1
pub struct REG_Q {
    word0: WORD0,
    word1: WORD1,
}
impl REG_Q {
    ///0x00 - a
    #[inline(always)]
    pub const fn word0(&self) -> &WORD0 {
        &self.word0
    }
    ///0x04 - a
    #[inline(always)]
    pub const fn word1(&self) -> &WORD1 {
        &self.word1
    }
}
/**WORD0 (rw) register accessor: a

You can [`read`](crate::generic::Reg::read) this register and get [`word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@word0`] module*/
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
///a
pub mod word0;
/**WORD1 (rw) register accessor: a

You can [`read`](crate::generic::Reg::read) this register and get [`word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@word1`] module*/
pub type WORD1 = crate::Reg<word1::WORD1_SPEC>;
///a
pub mod word1;
