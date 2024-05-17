///Register `HI` reader
pub type R = crate::R<HI_SPEC>;
///Register `HI` writer
pub type W = crate::W<HI_SPEC>;
///Field `LOAD_HI` reader - timer unit0 load high 20 bits
pub type LOAD_HI_R = crate::FieldReader<u32>;
///Field `LOAD_HI` writer - timer unit0 load high 20 bits
pub type LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32, crate::Safe>;
impl R {
    ///Bits 0:19 - timer unit0 load high 20 bits
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HI").field("load_hi", &self.load_hi()).finish()
    }
}
impl W {
    ///Bits 0:19 - timer unit0 load high 20 bits
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<HI_SPEC> {
        LOAD_HI_W::new(self, 0)
    }
}
/**system timer unit0 value high load register

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HI_SPEC;
impl crate::RegisterSpec for HI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hi::R`](R) reader structure
impl crate::Readable for HI_SPEC {}
///`write(|w| ..)` method takes [`hi::W`](W) writer structure
impl crate::Writable for HI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HI to value 0
impl crate::Resettable for HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
