#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    gpio: [GPIO; 54],
    _reserved1: [u8; 0x28],
    date: DATE,
}
impl RegisterBlock {
    ///0x04..0xdc - IO_MUX Control Register
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> &GPIO {
        &self.gpio[n]
    }
    ///Iterator for array of:
    ///0x04..0xdc - IO_MUX Control Register
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = &GPIO> {
        self.gpio.iter()
    }
    ///0x104 - iomux version
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**DATE (rw) register accessor: iomux version

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///iomux version
pub mod date;
/**GPIO (rw) register accessor: IO_MUX Control Register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gpio`] module*/
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
///IO_MUX Control Register
pub mod gpio;
