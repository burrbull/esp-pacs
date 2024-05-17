#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Device OUT endpoints 1-6
pub struct OUT_EP {
    doepctl: DOEPCTL,
    _reserved1: [u8; 0x04],
    doepint: DOEPINT,
    _reserved2: [u8; 0x04],
    doeptsiz: DOEPTSIZ,
    doepdma: DOEPDMA,
    _reserved4: [u8; 0x04],
    doepdmab: DOEPDMAB,
}
impl OUT_EP {
    ///0x00 -
    #[inline(always)]
    pub const fn doepctl(&self) -> &DOEPCTL {
        &self.doepctl
    }
    ///0x08 -
    #[inline(always)]
    pub const fn doepint(&self) -> &DOEPINT {
        &self.doepint
    }
    ///0x10 -
    #[inline(always)]
    pub const fn doeptsiz(&self) -> &DOEPTSIZ {
        &self.doeptsiz
    }
    ///0x14 -
    #[inline(always)]
    pub const fn doepdma(&self) -> &DOEPDMA {
        &self.doepdma
    }
    ///0x1c -
    #[inline(always)]
    pub const fn doepdmab(&self) -> &DOEPDMAB {
        &self.doepdmab
    }
}
/**DOEPCTL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`doepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@doepctl`] module*/
pub type DOEPCTL = crate::Reg<doepctl::DOEPCTL_SPEC>;
///
pub mod doepctl;
pub use crate::usb0::out_ep0::DOEPINT as DOEPINT;
pub use crate::usb0::out_ep0::doepint as doepint;
/**DOEPTSIZ (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@doeptsiz`] module*/
pub type DOEPTSIZ = crate::Reg<doeptsiz::DOEPTSIZ_SPEC>;
///
pub mod doeptsiz;
pub use crate::usb0::out_ep0::DOEPDMA as DOEPDMA;
pub use crate::usb0::out_ep0::doepdma as doepdma;
pub use crate::usb0::out_ep0::DOEPDMAB as DOEPDMAB;
pub use crate::usb0::out_ep0::doepdmab as doepdmab;
