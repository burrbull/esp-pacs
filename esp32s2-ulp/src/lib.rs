/*!Peripheral access API for ESP32-S2-ULP microcontrollers (generated using svd2rust v0.33.3 ( ))

You can find an overview of the generated API [here].

API features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.

[here]: https://docs.rs/svd2rust/0.33.3/svd2rust/#peripheral-api
[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased
[repository]: https://github.com/rust-embedded/svd2rust*/
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/46717278")]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 4;
#[allow(unused_imports)]
use generic::*;
///Common register and bit access and modify traits
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn TOUCH_DONE_INT();
    fn TOUCH_INACTIVE_INT();
    fn TOUCH_ACTIVE_INT();
    fn SARADC1_DONE_INT();
    fn SARADC2_DONE_INT();
    fn TSENS_DONE_INT();
    fn RISCV_START_INT();
    fn SW_INT();
    fn SWD_INT();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".rwtext"]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 9] = [
    Vector {
        _handler: TOUCH_DONE_INT,
    },
    Vector {
        _handler: TOUCH_INACTIVE_INT,
    },
    Vector {
        _handler: TOUCH_ACTIVE_INT,
    },
    Vector {
        _handler: SARADC1_DONE_INT,
    },
    Vector {
        _handler: SARADC2_DONE_INT,
    },
    Vector {
        _handler: TSENS_DONE_INT,
    },
    Vector {
        _handler: RISCV_START_INT,
    },
    Vector { _handler: SW_INT },
    Vector { _handler: SWD_INT },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
///Low-power Input/Output
pub struct RTC_IO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_IO {}
impl RTC_IO {
    ///Pointer to the register block
    pub const PTR: *const rtc_io::RegisterBlock = 0xa400 as *const _;
    ///Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rtc_io::RegisterBlock {
        Self::PTR
    }
    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances, for example by only
    /// accessing read-only or write-only registers, or by consuming the
    /// original peripheral and using critical sections to coordinate
    /// access between multiple new instances.
    ///
    /// Additionally, other software such as HALs may rely on only one
    /// peripheral instance existing to ensure memory safety; ensure
    /// no stolen instances are passed to such software.
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC_IO {
    type Target = rtc_io::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_IO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO").finish()
    }
}
///Low-power Input/Output
pub mod rtc_io;
///Real-Time Clock Control
pub struct RTC_CNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_CNTL {}
impl RTC_CNTL {
    ///Pointer to the register block
    pub const PTR: *const rtc_cntl::RegisterBlock = 0x8000 as *const _;
    ///Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rtc_cntl::RegisterBlock {
        Self::PTR
    }
    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances, for example by only
    /// accessing read-only or write-only registers, or by consuming the
    /// original peripheral and using critical sections to coordinate
    /// access between multiple new instances.
    ///
    /// Additionally, other software such as HALs may rely on only one
    /// peripheral instance existing to ensure memory safety; ensure
    /// no stolen instances are passed to such software.
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC_CNTL {
    type Target = rtc_cntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_CNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL").finish()
    }
}
///Real-Time Clock Control
pub mod rtc_cntl;
///Low-power I2C (Inter-Integrated Circuit) Controller
pub struct RTC_I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_I2C {}
impl RTC_I2C {
    ///Pointer to the register block
    pub const PTR: *const rtc_i2c::RegisterBlock = 0xec00 as *const _;
    ///Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rtc_i2c::RegisterBlock {
        Self::PTR
    }
    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances, for example by only
    /// accessing read-only or write-only registers, or by consuming the
    /// original peripheral and using critical sections to coordinate
    /// access between multiple new instances.
    ///
    /// Additionally, other software such as HALs may rely on only one
    /// peripheral instance existing to ensure memory safety; ensure
    /// no stolen instances are passed to such software.
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC_I2C {
    type Target = rtc_i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_I2C").finish()
    }
}
///Low-power I2C (Inter-Integrated Circuit) Controller
pub mod rtc_i2c;
///SENS Peripheral
pub struct SENS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENS {}
impl SENS {
    ///Pointer to the register block
    pub const PTR: *const sens::RegisterBlock = 0xc800 as *const _;
    ///Return the pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sens::RegisterBlock {
        Self::PTR
    }
    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances, for example by only
    /// accessing read-only or write-only registers, or by consuming the
    /// original peripheral and using critical sections to coordinate
    /// access between multiple new instances.
    ///
    /// Additionally, other software such as HALs may rely on only one
    /// peripheral instance existing to ensure memory safety; ensure
    /// no stolen instances are passed to such software.
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SENS {
    type Target = sens::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SENS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENS").finish()
    }
}
///SENS Peripheral
pub mod sens;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    ///RTC_IO
    pub RTC_IO: RTC_IO,
    ///RTC_CNTL
    pub RTC_CNTL: RTC_CNTL,
    ///RTC_I2C
    pub RTC_I2C: RTC_I2C,
    ///SENS
    pub SENS: SENS,
}
impl Peripherals {
    /// Returns all the peripherals *once*.
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            RTC_IO: RTC_IO {
                _marker: PhantomData,
            },
            RTC_CNTL: RTC_CNTL {
                _marker: PhantomData,
            },
            RTC_I2C: RTC_I2C {
                _marker: PhantomData,
            },
            SENS: SENS {
                _marker: PhantomData,
            },
        }
    }
}
