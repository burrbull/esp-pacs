///Register `POWER_PD_CNNT_MASK` reader
pub type R = crate::R<POWER_PD_CNNT_MASK_SPEC>;
///Register `POWER_PD_CNNT_MASK` writer
pub type W = crate::W<POWER_PD_CNNT_MASK_SPEC>;
///Field `XPD_CNNT_MASK` reader - need_des
pub type XPD_CNNT_MASK_R = crate::FieldReader;
///Field `XPD_CNNT_MASK` writer - need_des
pub type XPD_CNNT_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PD_CNNT_MASK` reader - need_des
pub type PD_CNNT_MASK_R = crate::FieldReader;
///Field `PD_CNNT_MASK` writer - need_des
pub type PD_CNNT_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - need_des
    #[inline(always)]
    pub fn xpd_cnnt_mask(&self) -> XPD_CNNT_MASK_R {
        XPD_CNNT_MASK_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 27:31 - need_des
    #[inline(always)]
    pub fn pd_cnnt_mask(&self) -> PD_CNNT_MASK_R {
        PD_CNNT_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_CNNT_MASK")
            .field("xpd_cnnt_mask", &self.xpd_cnnt_mask())
            .field("pd_cnnt_mask", &self.pd_cnnt_mask())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - need_des
    #[inline(always)]
    #[must_use]
    pub fn xpd_cnnt_mask(&mut self) -> XPD_CNNT_MASK_W<POWER_PD_CNNT_MASK_SPEC> {
        XPD_CNNT_MASK_W::new(self, 0)
    }
    ///Bits 27:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pd_cnnt_mask(&mut self) -> PD_CNNT_MASK_W<POWER_PD_CNNT_MASK_SPEC> {
        PD_CNNT_MASK_W::new(self, 27)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_cnnt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_cnnt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct POWER_PD_CNNT_MASK_SPEC;
impl crate::RegisterSpec for POWER_PD_CNNT_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`power_pd_cnnt_mask::R`](R) reader structure
impl crate::Readable for POWER_PD_CNNT_MASK_SPEC {}
///`write(|w| ..)` method takes [`power_pd_cnnt_mask::W`](W) writer structure
impl crate::Writable for POWER_PD_CNNT_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER_PD_CNNT_MASK to value 0
impl crate::Resettable for POWER_PD_CNNT_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
