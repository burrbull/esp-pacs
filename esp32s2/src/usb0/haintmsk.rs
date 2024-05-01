///Register `HAINTMSK` reader
pub type R = crate::R<HAINTMSK_SPEC>;
///Register `HAINTMSK` writer
pub type W = crate::W<HAINTMSK_SPEC>;
///Field `HAINTMSK` reader -
pub type HAINTMSK_R = crate::FieldReader;
///Field `HAINTMSK` writer -
pub type HAINTMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn haintmsk(&self) -> HAINTMSK_R {
        HAINTMSK_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINTMSK")
            .field("haintmsk", &self.haintmsk())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn haintmsk(&mut self) -> HAINTMSK_W<HAINTMSK_SPEC> {
        HAINTMSK_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HAINTMSK_SPEC;
impl crate::RegisterSpec for HAINTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`haintmsk::R`](R) reader structure
impl crate::Readable for HAINTMSK_SPEC {}
///`write(|w| ..)` method takes [`haintmsk::W`](W) writer structure
impl crate::Writable for HAINTMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HAINTMSK to value 0
impl crate::Resettable for HAINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
