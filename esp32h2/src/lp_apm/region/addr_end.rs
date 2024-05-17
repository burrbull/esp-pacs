///Register `ADDR_END` reader
pub type R = crate::R<ADDR_END_SPEC>;
///Register `ADDR_END` writer
pub type W = crate::W<ADDR_END_SPEC>;
///Field `ADDR_END` reader - End address of region0
pub type ADDR_END_R = crate::FieldReader<u32>;
///Field `ADDR_END` writer - End address of region0
pub type ADDR_END_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - End address of region0
    #[inline(always)]
    pub fn addr_end(&self) -> ADDR_END_R {
        ADDR_END_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR_END").field("addr_end", &self.addr_end()).finish()
    }
}
impl W {
    ///Bits 0:31 - End address of region0
    #[inline(always)]
    #[must_use]
    pub fn addr_end(&mut self) -> ADDR_END_W<ADDR_END_SPEC> {
        ADDR_END_W::new(self, 0)
    }
}
/**Region address register

You can [`read`](crate::generic::Reg::read) this register and get [`addr_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADDR_END_SPEC;
impl crate::RegisterSpec for ADDR_END_SPEC {
    type Ux = u32;
}
///`read()` method returns [`addr_end::R`](R) reader structure
impl crate::Readable for ADDR_END_SPEC {}
///`write(|w| ..)` method takes [`addr_end::W`](W) writer structure
impl crate::Writable for ADDR_END_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADDR_END to value 0xffff_ffff
impl crate::Resettable for ADDR_END_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
