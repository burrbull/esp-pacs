///Register `SLP_WAKEUP_CNTL1` reader
pub type R = crate::R<SLP_WAKEUP_CNTL1_SPEC>;
///Register `SLP_WAKEUP_CNTL1` writer
pub type W = crate::W<SLP_WAKEUP_CNTL1_SPEC>;
///Field `SLEEP_REJECT_ENA` reader - need_des
pub type SLEEP_REJECT_ENA_R = crate::FieldReader<u32>;
///Field `SLEEP_REJECT_ENA` writer - need_des
pub type SLEEP_REJECT_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `SLP_REJECT_EN` reader - need_des
pub type SLP_REJECT_EN_R = crate::BitReader;
///Field `SLP_REJECT_EN` writer - need_des
pub type SLP_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - need_des
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SLEEP_REJECT_ENA_R {
        SLEEP_REJECT_ENA_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn slp_reject_en(&self) -> SLP_REJECT_EN_R {
        SLP_REJECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL1")
            .field("sleep_reject_ena", &self.sleep_reject_ena())
            .field("slp_reject_en", &self.slp_reject_en())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sleep_reject_ena(&mut self) -> SLEEP_REJECT_ENA_W<SLP_WAKEUP_CNTL1_SPEC> {
        SLEEP_REJECT_ENA_W::new(self, 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_en(&mut self) -> SLP_REJECT_EN_W<SLP_WAKEUP_CNTL1_SPEC> {
        SLP_REJECT_EN_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLP_WAKEUP_CNTL1_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slp_wakeup_cntl1::R`](R) reader structure
impl crate::Readable for SLP_WAKEUP_CNTL1_SPEC {}
///`write(|w| ..)` method takes [`slp_wakeup_cntl1::W`](W) writer structure
impl crate::Writable for SLP_WAKEUP_CNTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_WAKEUP_CNTL1 to value 0
impl crate::Resettable for SLP_WAKEUP_CNTL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
