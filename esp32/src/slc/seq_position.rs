#[doc = "Register `SEQ_POSITION` reader"]
pub type R = crate::R<SEQ_POSITION_SPEC>;
#[doc = "Register `SEQ_POSITION` writer"]
pub type W = crate::W<SEQ_POSITION_SPEC>;
#[doc = "Field `SLC_SEQ_POSITION(0-1)` reader - "]
pub type SLC_SEQ_POSITION_R = crate::FieldReader;
#[doc = "Field `SLC_SEQ_POSITION(0-1)` writer - "]
pub type SLC_SEQ_POSITION_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_SEQ_POSITION` field"]
    #[inline(always)]
    pub fn slc_seq_position(&self, n: u8) -> SLC_SEQ_POSITION_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_SEQ_POSITION_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn slc_seq_position_iter(&self) -> impl Iterator<Item = SLC_SEQ_POSITION_R> + '_ {
        (0..2).map(move |n| SLC_SEQ_POSITION_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - SLC0_SEQ_POSITION"]
    #[inline(always)]
    pub fn slc0_seq_position(&self) -> SLC_SEQ_POSITION_R {
        SLC_SEQ_POSITION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SLC1_SEQ_POSITION"]
    #[inline(always)]
    pub fn slc1_seq_position(&self) -> SLC_SEQ_POSITION_R {
        SLC_SEQ_POSITION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_POSITION")
            .field(
                "slc0_seq_position",
                &format_args!("{}", self.slc0_seq_position().bits()),
            )
            .field(
                "slc1_seq_position",
                &format_args!("{}", self.slc1_seq_position().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEQ_POSITION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `SLC0_SEQ_POSITION` field"]
    #[inline(always)]
    #[must_use]
    pub fn slc_seq_position(&mut self, n: u8) -> SLC_SEQ_POSITION_W<SEQ_POSITION_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SLC_SEQ_POSITION_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - SLC0_SEQ_POSITION"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_seq_position(&mut self) -> SLC_SEQ_POSITION_W<SEQ_POSITION_SPEC> {
        SLC_SEQ_POSITION_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SLC1_SEQ_POSITION"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_seq_position(&mut self) -> SLC_SEQ_POSITION_W<SEQ_POSITION_SPEC> {
        SLC_SEQ_POSITION_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_position::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_position::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_POSITION_SPEC;
impl crate::RegisterSpec for SEQ_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_position::R`](R) reader structure"]
impl crate::Readable for SEQ_POSITION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_position::W`](W) writer structure"]
impl crate::Writable for SEQ_POSITION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_POSITION to value 0x0509"]
impl crate::Resettable for SEQ_POSITION_SPEC {
    const RESET_VALUE: u32 = 0x0509;
}
