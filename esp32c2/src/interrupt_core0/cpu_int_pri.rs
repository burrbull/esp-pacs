///Register `CPU_INT_PRI%s` reader
pub type R = crate::R<CPU_INT_PRI_SPEC>;
///Register `CPU_INT_PRI%s` writer
pub type W = crate::W<CPU_INT_PRI_SPEC>;
///Field `MAP` reader - Need add description
pub type MAP_R = crate::FieldReader;
///Field `MAP` writer - Need add description
pub type MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Need add description
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INT_PRI")
            .field("map", &self.map())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn map(&mut self) -> MAP_W<CPU_INT_PRI_SPEC> {
        MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_int_pri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_int_pri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_INT_PRI_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_int_pri::R`](R) reader structure
impl crate::Readable for CPU_INT_PRI_SPEC {}
///`write(|w| ..)` method takes [`cpu_int_pri::W`](W) writer structure
impl crate::Writable for CPU_INT_PRI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_INT_PRI%s to value 0
impl crate::Resettable for CPU_INT_PRI_SPEC {
    const RESET_VALUE: u32 = 0;
}
