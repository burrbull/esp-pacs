///Register `PRO_AHB_3` reader
pub type R = crate::R<PRO_AHB_3_SPEC>;
///Register `PRO_AHB_3` writer
pub type W = crate::W<PRO_AHB_3_SPEC>;
///Field `PRO_AHB_ILG_CLR` reader - The clear signal for PeriBus2 access interrupt.
pub type PRO_AHB_ILG_CLR_R = crate::BitReader;
///Field `PRO_AHB_ILG_CLR` writer - The clear signal for PeriBus2 access interrupt.
pub type PRO_AHB_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_AHB_ILG_EN` reader - The enable signal for PeriBus2 access interrupt.
pub type PRO_AHB_ILG_EN_R = crate::BitReader;
///Field `PRO_AHB_ILG_EN` writer - The enable signal for PeriBus2 access interrupt.
pub type PRO_AHB_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_AHB_ILG_INTR` reader - PeriBus2 access interrupt signal.
pub type PRO_AHB_ILG_INTR_R = crate::BitReader;
impl R {
    ///Bit 0 - The clear signal for PeriBus2 access interrupt.
    #[inline(always)]
    pub fn pro_ahb_ilg_clr(&self) -> PRO_AHB_ILG_CLR_R {
        PRO_AHB_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The enable signal for PeriBus2 access interrupt.
    #[inline(always)]
    pub fn pro_ahb_ilg_en(&self) -> PRO_AHB_ILG_EN_R {
        PRO_AHB_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PeriBus2 access interrupt signal.
    #[inline(always)]
    pub fn pro_ahb_ilg_intr(&self) -> PRO_AHB_ILG_INTR_R {
        PRO_AHB_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_AHB_3")
            .field("pro_ahb_ilg_clr", &self.pro_ahb_ilg_clr())
            .field("pro_ahb_ilg_en", &self.pro_ahb_ilg_en())
            .field("pro_ahb_ilg_intr", &self.pro_ahb_ilg_intr())
            .finish()
    }
}
impl W {
    ///Bit 0 - The clear signal for PeriBus2 access interrupt.
    #[inline(always)]
    #[must_use]
    pub fn pro_ahb_ilg_clr(&mut self) -> PRO_AHB_ILG_CLR_W<PRO_AHB_3_SPEC> {
        PRO_AHB_ILG_CLR_W::new(self, 0)
    }
    ///Bit 1 - The enable signal for PeriBus2 access interrupt.
    #[inline(always)]
    #[must_use]
    pub fn pro_ahb_ilg_en(&mut self) -> PRO_AHB_ILG_EN_W<PRO_AHB_3_SPEC> {
        PRO_AHB_ILG_EN_W::new(self, 1)
    }
}
/**PeriBus2 permission control register 3.

You can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ahb_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_AHB_3_SPEC;
impl crate::RegisterSpec for PRO_AHB_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_ahb_3::R`](R) reader structure
impl crate::Readable for PRO_AHB_3_SPEC {}
///`write(|w| ..)` method takes [`pro_ahb_3::W`](W) writer structure
impl crate::Writable for PRO_AHB_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_AHB_3 to value 0
impl crate::Resettable for PRO_AHB_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
