#[doc = "Register `BLK2_WDATA4` reader"]
pub type R = crate::R<BLK2_WDATA4_SPEC>;
#[doc = "Register `BLK2_WDATA4` writer"]
pub type W = crate::W<BLK2_WDATA4_SPEC>;
#[doc = "Field `BLK2_DIN4` reader - "]
pub type BLK2_DIN4_R = crate::FieldReader<u32>;
#[doc = "Field `BLK2_DIN4` writer - "]
pub type BLK2_DIN4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn blk2_din4(&self) -> BLK2_DIN4_R {
        BLK2_DIN4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_WDATA4")
            .field("blk2_din4", &format_args!("{}", self.blk2_din4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK2_WDATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn blk2_din4(&mut self) -> BLK2_DIN4_W<BLK2_WDATA4_SPEC> {
        BLK2_DIN4_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_WDATA4_SPEC;
impl crate::RegisterSpec for BLK2_WDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_wdata4::R`](R) reader structure"]
impl crate::Readable for BLK2_WDATA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk2_wdata4::W`](W) writer structure"]
impl crate::Writable for BLK2_WDATA4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK2_WDATA4 to value 0"]
impl crate::Resettable for BLK2_WDATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
