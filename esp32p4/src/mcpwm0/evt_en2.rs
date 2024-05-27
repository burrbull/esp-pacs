///Register `EVT_EN2` reader
pub type R = crate::R<EVT_EN2_SPEC>;
///Register `EVT_EN2` writer
pub type W = crate::W<EVT_EN2_SPEC>;
///Field `OP0_TEE1` reader - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
pub type OP0_TEE1_R = crate::BitReader;
///Field `OP0_TEE1` writer - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
pub type OP0_TEE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP1_TEE1` reader - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
pub type OP1_TEE1_R = crate::BitReader;
///Field `OP1_TEE1` writer - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
pub type OP1_TEE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP2_TEE1` reader - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
pub type OP2_TEE1_R = crate::BitReader;
///Field `OP2_TEE1` writer - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
pub type OP2_TEE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP0_TEE2` reader - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
pub type OP0_TEE2_R = crate::BitReader;
///Field `OP0_TEE2` writer - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
pub type OP0_TEE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP1_TEE2` reader - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
pub type OP1_TEE2_R = crate::BitReader;
///Field `OP1_TEE2` writer - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
pub type OP1_TEE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OP2_TEE2` reader - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
pub type OP2_TEE2_R = crate::BitReader;
///Field `OP2_TEE2` writer - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
pub type OP2_TEE2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn op0_tee1(&self) -> OP0_TEE1_R {
        OP0_TEE1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn op1_tee1(&self) -> OP1_TEE1_R {
        OP1_TEE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn op2_tee1(&self) -> OP2_TEE1_R {
        OP2_TEE1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn op0_tee2(&self) -> OP0_TEE2_R {
        OP0_TEE2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn op1_tee2(&self) -> OP1_TEE2_R {
        OP1_TEE2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn op2_tee2(&self) -> OP2_TEE2_R {
        OP2_TEE2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_EN2")
            .field("op0_tee1", &self.op0_tee1())
            .field("op1_tee1", &self.op1_tee1())
            .field("op2_tee1", &self.op2_tee1())
            .field("op0_tee2", &self.op0_tee2())
            .field("op1_tee2", &self.op1_tee2())
            .field("op2_tee2", &self.op2_tee2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn op0_tee1(&mut self) -> OP0_TEE1_W<EVT_EN2_SPEC> {
        OP0_TEE1_W::new(self, 0)
    }
    ///Bit 1 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn op1_tee1(&mut self) -> OP1_TEE1_W<EVT_EN2_SPEC> {
        OP1_TEE1_W::new(self, 1)
    }
    ///Bit 2 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn op2_tee1(&mut self) -> OP2_TEE1_W<EVT_EN2_SPEC> {
        OP2_TEE1_W::new(self, 2)
    }
    ///Bit 3 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn op0_tee2(&mut self) -> OP0_TEE2_W<EVT_EN2_SPEC> {
        OP0_TEE2_W::new(self, 3)
    }
    ///Bit 4 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn op1_tee2(&mut self) -> OP1_TEE2_W<EVT_EN2_SPEC> {
        OP1_TEE2_W::new(self, 4)
    }
    ///Bit 5 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn op2_tee2(&mut self) -> OP2_TEE2_W<EVT_EN2_SPEC> {
        OP2_TEE2_W::new(self, 5)
    }
}
/**Event enable register2

You can [`read`](crate::generic::Reg::read) this register and get [`evt_en2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_en2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_EN2_SPEC;
impl crate::RegisterSpec for EVT_EN2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`evt_en2::R`](R) reader structure
impl crate::Readable for EVT_EN2_SPEC {}
///`write(|w| ..)` method takes [`evt_en2::W`](W) writer structure
impl crate::Writable for EVT_EN2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_EN2 to value 0
impl crate::Resettable for EVT_EN2_SPEC {
    const RESET_VALUE: u32 = 0;
}
