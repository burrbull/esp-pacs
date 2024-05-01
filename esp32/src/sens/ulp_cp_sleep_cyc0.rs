///Register `ULP_CP_SLEEP_CYC0` reader
pub type R = crate::R<ULP_CP_SLEEP_CYC0_SPEC>;
///Register `ULP_CP_SLEEP_CYC0` writer
pub type W = crate::W<ULP_CP_SLEEP_CYC0_SPEC>;
///Field `SLEEP_CYCLES_S0` reader - sleep cycles for ULP-coprocessor timer
pub type SLEEP_CYCLES_S0_R = crate::FieldReader<u32>;
///Field `SLEEP_CYCLES_S0` writer - sleep cycles for ULP-coprocessor timer
pub type SLEEP_CYCLES_S0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - sleep cycles for ULP-coprocessor timer
    #[inline(always)]
    pub fn sleep_cycles_s0(&self) -> SLEEP_CYCLES_S0_R {
        SLEEP_CYCLES_S0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_SLEEP_CYC0")
            .field("sleep_cycles_s0", &self.sleep_cycles_s0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - sleep cycles for ULP-coprocessor timer
    #[inline(always)]
    #[must_use]
    pub fn sleep_cycles_s0(&mut self) -> SLEEP_CYCLES_S0_W<ULP_CP_SLEEP_CYC0_SPEC> {
        SLEEP_CYCLES_S0_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_sleep_cyc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ULP_CP_SLEEP_CYC0_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ulp_cp_sleep_cyc0::R`](R) reader structure
impl crate::Readable for ULP_CP_SLEEP_CYC0_SPEC {}
///`write(|w| ..)` method takes [`ulp_cp_sleep_cyc0::W`](W) writer structure
impl crate::Writable for ULP_CP_SLEEP_CYC0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ULP_CP_SLEEP_CYC0 to value 0xc8
impl crate::Resettable for ULP_CP_SLEEP_CYC0_SPEC {
    const RESET_VALUE: u32 = 0xc8;
}
