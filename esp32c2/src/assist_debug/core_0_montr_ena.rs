///Register `CORE_0_MONTR_ENA` reader
pub type R = crate::R<CORE_0_MONTR_ENA_SPEC>;
///Register `CORE_0_MONTR_ENA` writer
pub type W = crate::W<CORE_0_MONTR_ENA_SPEC>;
///Field `CORE_0_SP_SPILL_MIN_ENA` reader - enbale sp underlow monitor
pub type CORE_0_SP_SPILL_MIN_ENA_R = crate::BitReader;
///Field `CORE_0_SP_SPILL_MIN_ENA` writer - enbale sp underlow monitor
pub type CORE_0_SP_SPILL_MIN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_0_SP_SPILL_MAX_ENA` reader - enbale sp overflow monitor
pub type CORE_0_SP_SPILL_MAX_ENA_R = crate::BitReader;
///Field `CORE_0_SP_SPILL_MAX_ENA` writer - enbale sp overflow monitor
pub type CORE_0_SP_SPILL_MAX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enbale sp underlow monitor
    #[inline(always)]
    pub fn core_0_sp_spill_min_ena(&self) -> CORE_0_SP_SPILL_MIN_ENA_R {
        CORE_0_SP_SPILL_MIN_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enbale sp overflow monitor
    #[inline(always)]
    pub fn core_0_sp_spill_max_ena(&self) -> CORE_0_SP_SPILL_MAX_ENA_R {
        CORE_0_SP_SPILL_MAX_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_MONTR_ENA")
            .field("core_0_sp_spill_min_ena", &self.core_0_sp_spill_min_ena())
            .field("core_0_sp_spill_max_ena", &self.core_0_sp_spill_max_ena())
            .finish()
    }
}
impl W {
    ///Bit 0 - enbale sp underlow monitor
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_min_ena(&mut self) -> CORE_0_SP_SPILL_MIN_ENA_W<CORE_0_MONTR_ENA_SPEC> {
        CORE_0_SP_SPILL_MIN_ENA_W::new(self, 0)
    }
    ///Bit 1 - enbale sp overflow monitor
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_max_ena(&mut self) -> CORE_0_SP_SPILL_MAX_ENA_W<CORE_0_MONTR_ENA_SPEC> {
        CORE_0_SP_SPILL_MAX_ENA_W::new(self, 1)
    }
}
/**core0 monitor enable configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_montr_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_montr_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_MONTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_0_MONTR_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_montr_ena::R`](R) reader structure
impl crate::Readable for CORE_0_MONTR_ENA_SPEC {}
///`write(|w| ..)` method takes [`core_0_montr_ena::W`](W) writer structure
impl crate::Writable for CORE_0_MONTR_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CORE_0_MONTR_ENA to value 0
impl crate::Resettable for CORE_0_MONTR_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
