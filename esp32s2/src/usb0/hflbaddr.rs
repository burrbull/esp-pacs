///Register `HFLBADDR` reader
pub type R = crate::R<HFLBADDR_SPEC>;
///Register `HFLBADDR` writer
pub type W = crate::W<HFLBADDR_SPEC>;
///Field `HFLBADDR` reader -
pub type HFLBADDR_R = crate::FieldReader<u32>;
///Field `HFLBADDR` writer -
pub type HFLBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFLBADDR")
            .field("hflbaddr", &self.hflbaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<HFLBADDR_SPEC> {
        HFLBADDR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`hflbaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hflbaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HFLBADDR_SPEC;
impl crate::RegisterSpec for HFLBADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hflbaddr::R`](R) reader structure
impl crate::Readable for HFLBADDR_SPEC {}
///`write(|w| ..)` method takes [`hflbaddr::W`](W) writer structure
impl crate::Writable for HFLBADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFLBADDR to value 0
impl crate::Resettable for HFLBADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
