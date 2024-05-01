///Register `L1_ICACHE0_PRELOCK_SCT1_ADDR` reader
pub type R = crate::R<L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC>;
///Register `L1_ICACHE0_PRELOCK_SCT1_ADDR` writer
pub type W = crate::W<L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC>;
///Field `L1_ICACHE0_PRELOCK_SCT1_ADDR` reader - Those bits are used to configure the start virtual address of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_SIZE_REG
pub type L1_ICACHE0_PRELOCK_SCT1_ADDR_R = crate::FieldReader<u32>;
///Field `L1_ICACHE0_PRELOCK_SCT1_ADDR` writer - Those bits are used to configure the start virtual address of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_SIZE_REG
pub type L1_ICACHE0_PRELOCK_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Those bits are used to configure the start virtual address of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_SIZE_REG
    #[inline(always)]
    pub fn l1_icache0_prelock_sct1_addr(&self) -> L1_ICACHE0_PRELOCK_SCT1_ADDR_R {
        L1_ICACHE0_PRELOCK_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE0_PRELOCK_SCT1_ADDR")
            .field(
                "l1_icache0_prelock_sct1_addr",
                &self.l1_icache0_prelock_sct1_addr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Those bits are used to configure the start virtual address of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_SIZE_REG
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_prelock_sct1_addr(
        &mut self,
    ) -> L1_ICACHE0_PRELOCK_SCT1_ADDR_W<L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC> {
        L1_ICACHE0_PRELOCK_SCT1_ADDR_W::new(self, 0)
    }
}
/**L1 instruction Cache 0 prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_prelock_sct1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_prelock_sct1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_icache0_prelock_sct1_addr::R`](R) reader structure
impl crate::Readable for L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC {}
///`write(|w| ..)` method takes [`l1_icache0_prelock_sct1_addr::W`](W) writer structure
impl crate::Writable for L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_ICACHE0_PRELOCK_SCT1_ADDR to value 0
impl crate::Resettable for L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
