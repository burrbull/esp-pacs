///Register `TXON_DELAY` reader
pub type R = crate::R<TXON_DELAY_SPEC>;
///Register `TXON_DELAY` writer
pub type W = crate::W<TXON_DELAY_SPEC>;
///Field `TXON_DELAY` reader -
pub type TXON_DELAY_R = crate::FieldReader<u16>;
///Field `TXON_DELAY` writer -
pub type TXON_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn txon_delay(&self) -> TXON_DELAY_R {
        TXON_DELAY_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXON_DELAY")
            .field("txon_delay", &self.txon_delay())
            .finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    #[must_use]
    pub fn txon_delay(&mut self) -> TXON_DELAY_W<TXON_DELAY_SPEC> {
        TXON_DELAY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`txon_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txon_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXON_DELAY_SPEC;
impl crate::RegisterSpec for TXON_DELAY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`txon_delay::R`](R) reader structure
impl crate::Readable for TXON_DELAY_SPEC {}
///`write(|w| ..)` method takes [`txon_delay::W`](W) writer structure
impl crate::Writable for TXON_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXON_DELAY to value 0
impl crate::Resettable for TXON_DELAY_SPEC {
    const RESET_VALUE: u32 = 0;
}
