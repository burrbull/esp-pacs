///Register `SAR_PATT_TAB1` reader
pub type R = crate::R<SAR_PATT_TAB1_SPEC>;
///Register `SAR_PATT_TAB1` writer
pub type W = crate::W<SAR_PATT_TAB1_SPEC>;
///Field `SAR_PATT_TAB1` reader - item 0 ~ 3 for pattern table 1 (each item one byte)
pub type SAR_PATT_TAB1_R = crate::FieldReader<u32>;
///Field `SAR_PATT_TAB1` writer - item 0 ~ 3 for pattern table 1 (each item one byte)
pub type SAR_PATT_TAB1_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - item 0 ~ 3 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar_patt_tab1(&self) -> SAR_PATT_TAB1_R {
        SAR_PATT_TAB1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PATT_TAB1")
            .field("sar_patt_tab1", &self.sar_patt_tab1())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - item 0 ~ 3 for pattern table 1 (each item one byte)
    #[inline(always)]
    #[must_use]
    pub fn sar_patt_tab1(&mut self) -> SAR_PATT_TAB1_W<SAR_PATT_TAB1_SPEC> {
        SAR_PATT_TAB1_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`sar_patt_tab1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_patt_tab1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_PATT_TAB1_SPEC;
impl crate::RegisterSpec for SAR_PATT_TAB1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_patt_tab1::R`](R) reader structure
impl crate::Readable for SAR_PATT_TAB1_SPEC {}
///`write(|w| ..)` method takes [`sar_patt_tab1::W`](W) writer structure
impl crate::Writable for SAR_PATT_TAB1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_PATT_TAB1 to value 0x00ff_ffff
impl crate::Resettable for SAR_PATT_TAB1_SPEC {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
