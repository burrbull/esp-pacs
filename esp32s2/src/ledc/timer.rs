#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE
pub struct TIMER {
    conf: CONF,
    value: VALUE,
}
impl TIMER {
    ///0x00 - Timer 0 configuration
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x04 - Timer 0 current counter value
    #[inline(always)]
    pub const fn value(&self) -> &VALUE {
        &self.value
    }
}
/**CONF (rw) register accessor: Timer 0 configuration

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///Timer 0 configuration
pub mod conf;
/**VALUE (r) register accessor: Timer 0 current counter value

You can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@value`] module*/
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
///Timer 0 current counter value
pub mod value;
