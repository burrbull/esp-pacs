#[doc = "Register `TZ_CFG1` reader"]
pub type R = crate::R<TZ_CFG1_SPEC>;
#[doc = "Register `TZ_CFG1` writer"]
pub type W = crate::W<TZ_CFG1_SPEC>;
#[doc = "Field `CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type CLR_OST_R = crate::BitReader;
#[doc = "Field `CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type CBCPULSE_R = crate::FieldReader;
#[doc = "Field `CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type FORCE_CBC_R = crate::BitReader;
#[doc = "Field `FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type FORCE_OST_R = crate::BitReader;
#[doc = "Field `FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn clr_ost(&self) -> CLR_OST_R {
        CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    pub fn cbcpulse(&self) -> CBCPULSE_R {
        CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn force_cbc(&self) -> FORCE_CBC_R {
        FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn force_ost(&self) -> FORCE_OST_R {
        FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ_CFG1")
            .field("clr_ost", &self.clr_ost().bit())
            .field("cbcpulse", &self.cbcpulse().bits())
            .field("force_cbc", &self.force_cbc().bit())
            .field("force_ost", &self.force_ost().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn clr_ost(&mut self) -> CLR_OST_W<TZ_CFG1_SPEC> {
        CLR_OST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    #[must_use]
    pub fn cbcpulse(&mut self) -> CBCPULSE_W<TZ_CFG1_SPEC> {
        CBCPULSE_W::new(self, 1)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    #[must_use]
    pub fn force_cbc(&mut self) -> FORCE_CBC_W<TZ_CFG1_SPEC> {
        FORCE_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn force_ost(&mut self) -> FORCE_OST_W<TZ_CFG1_SPEC> {
        FORCE_OST_W::new(self, 4)
    }
}
#[doc = "Software triggers for fault handler actions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tz_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tz_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZ_CFG1_SPEC;
impl crate::RegisterSpec for TZ_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz_cfg1::R`](R) reader structure"]
impl crate::Readable for TZ_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tz_cfg1::W`](W) writer structure"]
impl crate::Writable for TZ_CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZ_CFG1 to value 0"]
impl crate::Resettable for TZ_CFG1_SPEC {}
