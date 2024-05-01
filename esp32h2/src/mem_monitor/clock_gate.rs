///Register `CLOCK_GATE` reader
pub type R = crate::R<CLOCK_GATE_SPEC>;
///Register `CLOCK_GATE` writer
pub type W = crate::W<CLOCK_GATE_SPEC>;
///Field `CLK_EN` reader - Set 1 to force on the clk of mem_monitor register
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - Set 1 to force on the clk of mem_monitor register
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to force on the clk of mem_monitor register
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_GATE")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to force on the clk of mem_monitor register
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLOCK_GATE_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
/**clock gate force on register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clock_gate::R`](R) reader structure
impl crate::Readable for CLOCK_GATE_SPEC {}
///`write(|w| ..)` method takes [`clock_gate::W`](W) writer structure
impl crate::Writable for CLOCK_GATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLOCK_GATE to value 0
impl crate::Resettable for CLOCK_GATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
