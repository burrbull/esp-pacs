///Register `LOG_MEM_START` reader
pub type R = crate::R<LOG_MEM_START_SPEC>;
///Register `LOG_MEM_START` writer
pub type W = crate::W<LOG_MEM_START_SPEC>;
///Field `LOG_MEM_START` reader - the start address of writing logging message
pub type LOG_MEM_START_R = crate::FieldReader<u32>;
///Field `LOG_MEM_START` writer - the start address of writing logging message
pub type LOG_MEM_START_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - the start address of writing logging message
    #[inline(always)]
    pub fn log_mem_start(&self) -> LOG_MEM_START_R {
        LOG_MEM_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MEM_START")
            .field("log_mem_start", &self.log_mem_start())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - the start address of writing logging message
    #[inline(always)]
    #[must_use]
    pub fn log_mem_start(&mut self) -> LOG_MEM_START_W<LOG_MEM_START_SPEC> {
        LOG_MEM_START_W::new(self, 0)
    }
}
/**log message store range register

You can [`read`](crate::generic::Reg::read) this register and get [`log_mem_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOG_MEM_START_SPEC;
impl crate::RegisterSpec for LOG_MEM_START_SPEC {
    type Ux = u32;
}
///`read()` method returns [`log_mem_start::R`](R) reader structure
impl crate::Readable for LOG_MEM_START_SPEC {}
///`write(|w| ..)` method takes [`log_mem_start::W`](W) writer structure
impl crate::Writable for LOG_MEM_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOG_MEM_START to value 0
impl crate::Resettable for LOG_MEM_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
