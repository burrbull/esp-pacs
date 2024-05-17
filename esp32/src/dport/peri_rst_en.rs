///Register `PERI_RST_EN` reader
pub type R = crate::R<PERI_RST_EN_SPEC>;
///Register `PERI_RST_EN` writer
pub type W = crate::W<PERI_RST_EN_SPEC>;
///Field `PERI_RST_EN` reader -
pub type PERI_RST_EN_R = crate::FieldReader<u32>;
///Field `PERI_RST_EN` writer -
pub type PERI_RST_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn peri_rst_en(&self) -> PERI_RST_EN_R {
        PERI_RST_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_RST_EN").field("peri_rst_en", &self.peri_rst_en()).finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn peri_rst_en(&mut self) -> PERI_RST_EN_W<PERI_RST_EN_SPEC> {
        PERI_RST_EN_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`peri_rst_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_rst_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_RST_EN_SPEC;
impl crate::RegisterSpec for PERI_RST_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_rst_en::R`](R) reader structure
impl crate::Readable for PERI_RST_EN_SPEC {}
///`write(|w| ..)` method takes [`peri_rst_en::W`](W) writer structure
impl crate::Writable for PERI_RST_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_RST_EN to value 0
impl crate::Resettable for PERI_RST_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
