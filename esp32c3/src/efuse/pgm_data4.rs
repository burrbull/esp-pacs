///Register `PGM_DATA4` reader
pub type R = crate::R<PGM_DATA4_SPEC>;
///Register `PGM_DATA4` writer
pub type W = crate::W<PGM_DATA4_SPEC>;
///Field `PGM_DATA_4` reader - The content of the 4th 32-bit data to be programmed.
pub type PGM_DATA_4_R = crate::FieldReader<u32>;
///Field `PGM_DATA_4` writer - The content of the 4th 32-bit data to be programmed.
pub type PGM_DATA_4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The content of the 4th 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_4(&self) -> PGM_DATA_4_R {
        PGM_DATA_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGM_DATA4").field("pgm_data_4", &self.pgm_data_4()).finish()
    }
}
impl W {
    ///Bits 0:31 - The content of the 4th 32-bit data to be programmed.
    #[inline(always)]
    #[must_use]
    pub fn pgm_data_4(&mut self) -> PGM_DATA_4_W<PGM_DATA4_SPEC> {
        PGM_DATA_4_W::new(self, 0)
    }
}
/**Register 4 that stores data to be programmed.

You can [`read`](crate::generic::Reg::read) this register and get [`pgm_data4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PGM_DATA4_SPEC;
impl crate::RegisterSpec for PGM_DATA4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pgm_data4::R`](R) reader structure
impl crate::Readable for PGM_DATA4_SPEC {}
///`write(|w| ..)` method takes [`pgm_data4::W`](W) writer structure
impl crate::Writable for PGM_DATA4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PGM_DATA4 to value 0
impl crate::Resettable for PGM_DATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
