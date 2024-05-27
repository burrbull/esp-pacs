///Register `M_MEM[%s]` reader
pub type R = crate::R<M_MEM_SPEC>;
///Register `M_MEM[%s]` writer
pub type W = crate::W<M_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Represents M

You can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct M_MEM_SPEC;
impl crate::RegisterSpec for M_MEM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`m_mem::R`](R) reader structure
impl crate::Readable for M_MEM_SPEC {}
///`write(|w| ..)` method takes [`m_mem::W`](W) writer structure
impl crate::Writable for M_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M_MEM[%s] to value 0
impl crate::Resettable for M_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
