///Register `SAR1_PATT_TAB2` reader
pub type R = crate::R<SAR1_PATT_TAB2_SPEC>;
///Register `SAR1_PATT_TAB2` writer
pub type W = crate::W<SAR1_PATT_TAB2_SPEC>;
///Field `SAR1_PATT_TAB2` reader - Item 4 ~ 7 for pattern table 1 (each item one byte)
pub type SAR1_PATT_TAB2_R = crate::FieldReader<u32>;
///Field `SAR1_PATT_TAB2` writer - Item 4 ~ 7 for pattern table 1 (each item one byte)
pub type SAR1_PATT_TAB2_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Item 4 ~ 7 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar1_patt_tab2(&self) -> SAR1_PATT_TAB2_R {
        SAR1_PATT_TAB2_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_PATT_TAB2")
            .field("sar1_patt_tab2", &self.sar1_patt_tab2())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Item 4 ~ 7 for pattern table 1 (each item one byte)
    #[inline(always)]
    #[must_use]
    pub fn sar1_patt_tab2(&mut self) -> SAR1_PATT_TAB2_W<SAR1_PATT_TAB2_SPEC> {
        SAR1_PATT_TAB2_W::new(self, 0)
    }
}
/**Register

You can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR1_PATT_TAB2_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar1_patt_tab2::R`](R) reader structure
impl crate::Readable for SAR1_PATT_TAB2_SPEC {}
///`write(|w| ..)` method takes [`sar1_patt_tab2::W`](W) writer structure
impl crate::Writable for SAR1_PATT_TAB2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR1_PATT_TAB2 to value 0
impl crate::Resettable for SAR1_PATT_TAB2_SPEC {
    const RESET_VALUE: u32 = 0;
}
