///Register `SAR1_PATT_TAB4` reader
pub type R = crate::R<SAR1_PATT_TAB4_SPEC>;
///Register `SAR1_PATT_TAB4` writer
pub type W = crate::W<SAR1_PATT_TAB4_SPEC>;
///Field `SAR1_PATT_TAB4` reader - Item 12 ~ 15 for pattern table 1 (each item one byte)
pub type SAR1_PATT_TAB4_R = crate::FieldReader<u32>;
///Field `SAR1_PATT_TAB4` writer - Item 12 ~ 15 for pattern table 1 (each item one byte)
pub type SAR1_PATT_TAB4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Item 12 ~ 15 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar1_patt_tab4(&self) -> SAR1_PATT_TAB4_R {
        SAR1_PATT_TAB4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_PATT_TAB4")
            .field("sar1_patt_tab4", &self.sar1_patt_tab4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Item 12 ~ 15 for pattern table 1 (each item one byte)
    #[inline(always)]
    #[must_use]
    pub fn sar1_patt_tab4(&mut self) -> SAR1_PATT_TAB4_W<SAR1_PATT_TAB4_SPEC> {
        SAR1_PATT_TAB4_W::new(self, 0)
    }
}
/**Item 12 ~ 15 for pattern table 1 (each item one byte)

You can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR1_PATT_TAB4_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar1_patt_tab4::R`](R) reader structure
impl crate::Readable for SAR1_PATT_TAB4_SPEC {}
///`write(|w| ..)` method takes [`sar1_patt_tab4::W`](W) writer structure
impl crate::Writable for SAR1_PATT_TAB4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR1_PATT_TAB4 to value 0x0f0f_0f0f
impl crate::Resettable for SAR1_PATT_TAB4_SPEC {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
