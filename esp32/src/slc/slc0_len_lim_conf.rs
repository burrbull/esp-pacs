#[doc = "Register `SLC0_LEN_LIM_CONF` reader"]
pub type R = crate::R<SLC0_LEN_LIM_CONF_SPEC>;
#[doc = "Register `SLC0_LEN_LIM_CONF` writer"]
pub type W = crate::W<SLC0_LEN_LIM_CONF_SPEC>;
#[doc = "Field `LEN_LIM` reader - "]
pub type LEN_LIM_R = crate::FieldReader<u32>;
#[doc = "Field `LEN_LIM` writer - "]
pub type LEN_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn len_lim(&self) -> LEN_LIM_R {
        LEN_LIM_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_LEN_LIM_CONF")
            .field("len_lim", &format_args!("{}", self.len_lim().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_LEN_LIM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn len_lim(&mut self) -> LEN_LIM_W<SLC0_LEN_LIM_CONF_SPEC> {
        LEN_LIM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_len_lim_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_len_lim_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_LEN_LIM_CONF_SPEC;
impl crate::RegisterSpec for SLC0_LEN_LIM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_len_lim_conf::R`](R) reader structure"]
impl crate::Readable for SLC0_LEN_LIM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_len_lim_conf::W`](W) writer structure"]
impl crate::Writable for SLC0_LEN_LIM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_LEN_LIM_CONF to value 0x5400"]
impl crate::Resettable for SLC0_LEN_LIM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x5400;
}
