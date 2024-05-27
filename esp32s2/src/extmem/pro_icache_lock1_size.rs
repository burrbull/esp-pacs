///Register `PRO_ICACHE_LOCK1_SIZE` reader
pub type R = crate::R<PRO_ICACHE_LOCK1_SIZE_SPEC>;
///Register `PRO_ICACHE_LOCK1_SIZE` writer
pub type W = crate::W<PRO_ICACHE_LOCK1_SIZE_SPEC>;
///Field `PRO_ICACHE_LOCK1_SIZE` reader - The bits are used to configure the second length of data locking, which is combined with PRO_ICACHE_LOCK1_ADDR_REG
pub type PRO_ICACHE_LOCK1_SIZE_R = crate::FieldReader<u16>;
///Field `PRO_ICACHE_LOCK1_SIZE` writer - The bits are used to configure the second length of data locking, which is combined with PRO_ICACHE_LOCK1_ADDR_REG
pub type PRO_ICACHE_LOCK1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with PRO_ICACHE_LOCK1_ADDR_REG
    #[inline(always)]
    pub fn pro_icache_lock1_size(&self) -> PRO_ICACHE_LOCK1_SIZE_R {
        PRO_ICACHE_LOCK1_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_LOCK1_SIZE")
            .field("pro_icache_lock1_size", &self.pro_icache_lock1_size())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with PRO_ICACHE_LOCK1_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_lock1_size(&mut self) -> PRO_ICACHE_LOCK1_SIZE_W<PRO_ICACHE_LOCK1_SIZE_SPEC> {
        PRO_ICACHE_LOCK1_SIZE_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_lock1_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_lock1_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_ICACHE_LOCK1_SIZE_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_LOCK1_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_icache_lock1_size::R`](R) reader structure
impl crate::Readable for PRO_ICACHE_LOCK1_SIZE_SPEC {}
///`write(|w| ..)` method takes [`pro_icache_lock1_size::W`](W) writer structure
impl crate::Writable for PRO_ICACHE_LOCK1_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_ICACHE_LOCK1_SIZE to value 0
impl crate::Resettable for PRO_ICACHE_LOCK1_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
