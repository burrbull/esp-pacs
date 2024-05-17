///Register `VID_VFP_LINES` reader
pub type R = crate::R<VID_VFP_LINES_SPEC>;
///Register `VID_VFP_LINES` writer
pub type W = crate::W<VID_VFP_LINES_SPEC>;
///Field `VFP_LINES` reader - NA
pub type VFP_LINES_R = crate::FieldReader<u16>;
///Field `VFP_LINES` writer - NA
pub type VFP_LINES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - NA
    #[inline(always)]
    pub fn vfp_lines(&self) -> VFP_LINES_R {
        VFP_LINES_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_VFP_LINES").field("vfp_lines", &self.vfp_lines()).finish()
    }
}
impl W {
    ///Bits 0:9 - NA
    #[inline(always)]
    #[must_use]
    pub fn vfp_lines(&mut self) -> VFP_LINES_W<VID_VFP_LINES_SPEC> {
        VFP_LINES_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vfp_lines::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vfp_lines::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VID_VFP_LINES_SPEC;
impl crate::RegisterSpec for VID_VFP_LINES_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vid_vfp_lines::R`](R) reader structure
impl crate::Readable for VID_VFP_LINES_SPEC {}
///`write(|w| ..)` method takes [`vid_vfp_lines::W`](W) writer structure
impl crate::Writable for VID_VFP_LINES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VID_VFP_LINES to value 0
impl crate::Resettable for VID_VFP_LINES_SPEC {
    const RESET_VALUE: u32 = 0;
}
