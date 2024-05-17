///Register `PAON_DELAY` reader
pub type R = crate::R<PAON_DELAY_SPEC>;
///Register `PAON_DELAY` writer
pub type W = crate::W<PAON_DELAY_SPEC>;
///Field `PAON_DELAY` reader -
pub type PAON_DELAY_R = crate::FieldReader<u16>;
///Field `PAON_DELAY` writer -
pub type PAON_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn paon_delay(&self) -> PAON_DELAY_R {
        PAON_DELAY_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAON_DELAY").field("paon_delay", &self.paon_delay()).finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    #[must_use]
    pub fn paon_delay(&mut self) -> PAON_DELAY_W<PAON_DELAY_SPEC> {
        PAON_DELAY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`paon_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paon_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PAON_DELAY_SPEC;
impl crate::RegisterSpec for PAON_DELAY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`paon_delay::R`](R) reader structure
impl crate::Readable for PAON_DELAY_SPEC {}
///`write(|w| ..)` method takes [`paon_delay::W`](W) writer structure
impl crate::Writable for PAON_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAON_DELAY to value 0
impl crate::Resettable for PAON_DELAY_SPEC {
    const RESET_VALUE: u32 = 0;
}
