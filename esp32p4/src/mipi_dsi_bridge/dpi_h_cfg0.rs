///Register `DPI_H_CFG0` reader
pub type R = crate::R<DPI_H_CFG0_SPEC>;
///Register `DPI_H_CFG0` writer
pub type W = crate::W<DPI_H_CFG0_SPEC>;
///Field `HTOTAL` reader - this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
pub type HTOTAL_R = crate::FieldReader<u16>;
///Field `HTOTAL` writer - this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
pub type HTOTAL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HDISP` reader - this field configures the length of valid pixel data (by pixel num) for dpi output
pub type HDISP_R = crate::FieldReader<u16>;
///Field `HDISP` writer - this field configures the length of valid pixel data (by pixel num) for dpi output
pub type HDISP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    #[inline(always)]
    pub fn htotal(&self) -> HTOTAL_R {
        HTOTAL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - this field configures the length of valid pixel data (by pixel num) for dpi output
    #[inline(always)]
    pub fn hdisp(&self) -> HDISP_R {
        HDISP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_H_CFG0")
            .field("htotal", &self.htotal())
            .field("hdisp", &self.hdisp())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    #[inline(always)]
    #[must_use]
    pub fn htotal(&mut self) -> HTOTAL_W<DPI_H_CFG0_SPEC> {
        HTOTAL_W::new(self, 0)
    }
    ///Bits 16:27 - this field configures the length of valid pixel data (by pixel num) for dpi output
    #[inline(always)]
    #[must_use]
    pub fn hdisp(&mut self) -> HDISP_W<DPI_H_CFG0_SPEC> {
        HDISP_W::new(self, 16)
    }
}
/**dsi bridge dpi h config register 0

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_h_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_h_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPI_H_CFG0_SPEC;
impl crate::RegisterSpec for DPI_H_CFG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpi_h_cfg0::R`](R) reader structure
impl crate::Readable for DPI_H_CFG0_SPEC {}
///`write(|w| ..)` method takes [`dpi_h_cfg0::W`](W) writer structure
impl crate::Writable for DPI_H_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_H_CFG0 to value 0x0280_0320
impl crate::Resettable for DPI_H_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x0280_0320;
}
