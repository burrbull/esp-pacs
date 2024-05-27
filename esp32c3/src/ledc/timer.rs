#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster TIMER%s, containing TIMER?_CONF, TIMER?_VALUE
pub struct TIMER {
    conf: CONF,
    value: VALUE,
}
impl TIMER {
    ///0x00 - LEDC_LSTIMER0_CONF.
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x04 - LEDC_LSTIMER0_VALUE.
    #[inline(always)]
    pub const fn value(&self) -> &VALUE {
        &self.value
    }
}
/**CONF (rw) register accessor: LEDC_LSTIMER0_CONF.

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///LEDC_LSTIMER0_CONF.
pub mod conf;
/**VALUE (r) register accessor: LEDC_LSTIMER0_VALUE.

You can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@value`] module*/
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
///LEDC_LSTIMER0_VALUE.
pub mod value;
