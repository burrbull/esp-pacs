///Register `TXOFF_DELAY` reader
pub type R = crate::R<TXOFF_DELAY_SPEC>;
///Register `TXOFF_DELAY` writer
pub type W = crate::W<TXOFF_DELAY_SPEC>;
///Field `TXOFF_DELAY` reader -
pub type TXOFF_DELAY_R = crate::FieldReader;
///Field `TXOFF_DELAY` writer -
pub type TXOFF_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5
    #[inline(always)]
    pub fn txoff_delay(&self) -> TXOFF_DELAY_R {
        TXOFF_DELAY_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXOFF_DELAY")
            .field("txoff_delay", &self.txoff_delay())
            .finish()
    }
}
impl W {
    ///Bits 0:5
    #[inline(always)]
    #[must_use]
    pub fn txoff_delay(&mut self) -> TXOFF_DELAY_W<TXOFF_DELAY_SPEC> {
        TXOFF_DELAY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`txoff_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txoff_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXOFF_DELAY_SPEC;
impl crate::RegisterSpec for TXOFF_DELAY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`txoff_delay::R`](R) reader structure
impl crate::Readable for TXOFF_DELAY_SPEC {}
///`write(|w| ..)` method takes [`txoff_delay::W`](W) writer structure
impl crate::Writable for TXOFF_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXOFF_DELAY to value 0
impl crate::Resettable for TXOFF_DELAY_SPEC {
    const RESET_VALUE: u32 = 0;
}
