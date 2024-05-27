#[doc = "Register `CLOCK_GATE` reader"]
pub type R = crate::R<CLOCK_GATE_SPEC>;
#[doc = "Register `CLOCK_GATE` writer"]
pub type W = crate::W<CLOCK_GATE_SPEC>;
#[doc = "Field `CORE1_REG_CLK_EN` reader - NA"]
pub type CORE1_REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `CORE1_REG_CLK_EN` writer - NA"]
pub type CORE1_REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn core1_reg_clk_en(&self) -> CORE1_REG_CLK_EN_R {
        CORE1_REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_GATE")
            .field("core1_reg_clk_en", &self.core1_reg_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn core1_reg_clk_en(&mut self) -> CORE1_REG_CLK_EN_W<CLOCK_GATE_SPEC> {
        CORE1_REG_CLK_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_gate::R`](R) reader structure"]
impl crate::Readable for CLOCK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_gate::W`](W) writer structure"]
impl crate::Writable for CLOCK_GATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0x01"]
impl crate::Resettable for CLOCK_GATE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
