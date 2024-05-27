///Register `ENABLE1` reader
pub type R = crate::R<ENABLE1_SPEC>;
///Register `ENABLE1` writer
pub type W = crate::W<ENABLE1_SPEC>;
///Field `DATA` reader - GPIO output enable register for GPIO32-53
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - GPIO output enable register for GPIO32-53
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - GPIO output enable register for GPIO32-53
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE1")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - GPIO output enable register for GPIO32-53
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<ENABLE1_SPEC> {
        DATA_W::new(self, 0)
    }
}
/**GPIO output enable register for GPIO32-53

You can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ENABLE1_SPEC;
impl crate::RegisterSpec for ENABLE1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`enable1::R`](R) reader structure
impl crate::Readable for ENABLE1_SPEC {}
///`write(|w| ..)` method takes [`enable1::W`](W) writer structure
impl crate::Writable for ENABLE1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENABLE1 to value 0
impl crate::Resettable for ENABLE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
