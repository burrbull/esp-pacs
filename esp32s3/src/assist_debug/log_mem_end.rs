///Register `LOG_MEM_END` reader
pub type R = crate::R<LOG_MEM_END_SPEC>;
///Register `LOG_MEM_END` writer
pub type W = crate::W<LOG_MEM_END_SPEC>;
///Field `LOG_MEM_END` reader - mem end addr
pub type LOG_MEM_END_R = crate::FieldReader<u32>;
///Field `LOG_MEM_END` writer - mem end addr
pub type LOG_MEM_END_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - mem end addr
    #[inline(always)]
    pub fn log_mem_end(&self) -> LOG_MEM_END_R {
        LOG_MEM_END_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MEM_END").field("log_mem_end", &self.log_mem_end()).finish()
    }
}
impl W {
    ///Bits 0:31 - mem end addr
    #[inline(always)]
    #[must_use]
    pub fn log_mem_end(&mut self) -> LOG_MEM_END_W<LOG_MEM_END_SPEC> {
        LOG_MEM_END_W::new(self, 0)
    }
}
/**log mem region configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`log_mem_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOG_MEM_END_SPEC;
impl crate::RegisterSpec for LOG_MEM_END_SPEC {
    type Ux = u32;
}
///`read()` method returns [`log_mem_end::R`](R) reader structure
impl crate::Readable for LOG_MEM_END_SPEC {}
///`write(|w| ..)` method takes [`log_mem_end::W`](W) writer structure
impl crate::Writable for LOG_MEM_END_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOG_MEM_END to value 0
impl crate::Resettable for LOG_MEM_END_SPEC {
    const RESET_VALUE: u32 = 0;
}
