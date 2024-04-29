#[doc = "Register `INTVEC_TOHOST` writer"]
pub type W = crate::W<INTVEC_TOHOST_SPEC>;
#[doc = "Field `SLC_TOHOST_INTVEC(0-1)` writer - "]
pub type SLC_TOHOST_INTVEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTVEC_TOHOST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_TOHOST_INTVEC` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tohost_intvec(&mut self, n: u8) -> SLC_TOHOST_INTVEC_W<INTVEC_TOHOST_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_TOHOST_INTVEC_W::new(self, n * 16)
    }
    #[doc = "Bits 0:7 - SLC0_TOHOST_INTVEC"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_intvec(&mut self) -> SLC_TOHOST_INTVEC_W<INTVEC_TOHOST_SPEC> {
        SLC_TOHOST_INTVEC_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - SLC1_TOHOST_INTVEC"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_intvec(&mut self) -> SLC_TOHOST_INTVEC_W<INTVEC_TOHOST_SPEC> {
        SLC_TOHOST_INTVEC_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intvec_tohost::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTVEC_TOHOST_SPEC;
impl crate::RegisterSpec for INTVEC_TOHOST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intvec_tohost::W`](W) writer structure"]
impl crate::Writable for INTVEC_TOHOST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTVEC_TOHOST to value 0"]
impl crate::Resettable for INTVEC_TOHOST_SPEC {
    const RESET_VALUE: u32 = 0;
}
