///Register `L2_CACHE_VADDR` reader
pub type R = crate::R<L2_CACHE_VADDR_SPEC>;
///Field `L2_CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed.
pub type L2_CACHE_VADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed.
    #[inline(always)]
    pub fn l2_cache_vaddr(&self) -> L2_CACHE_VADDR_R {
        L2_CACHE_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_VADDR")
            .field("l2_cache_vaddr", &self.l2_cache_vaddr())
            .finish()
    }
}
/**Cache Vaddr register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_vaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_VADDR_SPEC;
impl crate::RegisterSpec for L2_CACHE_VADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_vaddr::R`](R) reader structure
impl crate::Readable for L2_CACHE_VADDR_SPEC {}
///`reset()` method sets L2_CACHE_VADDR to value 0x4000_0000
impl crate::Resettable for L2_CACHE_VADDR_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
