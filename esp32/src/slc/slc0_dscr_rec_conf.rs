#[doc = "Register `SLC0_DSCR_REC_CONF` reader"]
pub type R = crate::R<SLC0_DSCR_REC_CONF_SPEC>;
#[doc = "Register `SLC0_DSCR_REC_CONF` writer"]
pub type W = crate::W<SLC0_DSCR_REC_CONF_SPEC>;
#[doc = "Field `RX_DSCR_REC_LIM` reader - "]
pub type RX_DSCR_REC_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `RX_DSCR_REC_LIM` writer - "]
pub type RX_DSCR_REC_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_dscr_rec_lim(&self) -> RX_DSCR_REC_LIM_R {
        RX_DSCR_REC_LIM_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_DSCR_REC_CONF")
            .field(
                "rx_dscr_rec_lim",
                &format_args!("{}", self.rx_dscr_rec_lim().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_DSCR_REC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dscr_rec_lim(&mut self) -> RX_DSCR_REC_LIM_W<SLC0_DSCR_REC_CONF_SPEC> {
        RX_DSCR_REC_LIM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_dscr_rec_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_dscr_rec_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_DSCR_REC_CONF_SPEC;
impl crate::RegisterSpec for SLC0_DSCR_REC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_dscr_rec_conf::R`](R) reader structure"]
impl crate::Readable for SLC0_DSCR_REC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_dscr_rec_conf::W`](W) writer structure"]
impl crate::Writable for SLC0_DSCR_REC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_DSCR_REC_CONF to value 0x03ff"]
impl crate::Resettable for SLC0_DSCR_REC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
