///Register `L1_CACHE_PRELOAD_CTRL` reader
pub type R = crate::R<L1_CACHE_PRELOAD_CTRL_SPEC>;
///Register `L1_CACHE_PRELOAD_CTRL` writer
pub type W = crate::W<L1_CACHE_PRELOAD_CTRL_SPEC>;
///Field `L1_CACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done.
pub type L1_CACHE_PRELOAD_ENA_R = crate::BitReader;
///Field `L1_CACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done.
pub type L1_CACHE_PRELOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_CACHE_PRELOAD_DONE` reader - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished.
pub type L1_CACHE_PRELOAD_DONE_R = crate::BitReader;
///Field `L1_CACHE_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending.
pub type L1_CACHE_PRELOAD_ORDER_R = crate::BitReader;
///Field `L1_CACHE_PRELOAD_ORDER` writer - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending.
pub type L1_CACHE_PRELOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_CACHE_PRELOAD_RGID` reader - The bit is used to set the gid of l1 cache preload.
pub type L1_CACHE_PRELOAD_RGID_R = crate::FieldReader;
impl R {
    ///Bit 0 - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done.
    #[inline(always)]
    pub fn l1_cache_preload_ena(&self) -> L1_CACHE_PRELOAD_ENA_R {
        L1_CACHE_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished.
    #[inline(always)]
    pub fn l1_cache_preload_done(&self) -> L1_CACHE_PRELOAD_DONE_R {
        L1_CACHE_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending.
    #[inline(always)]
    pub fn l1_cache_preload_order(&self) -> L1_CACHE_PRELOAD_ORDER_R {
        L1_CACHE_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - The bit is used to set the gid of l1 cache preload.
    #[inline(always)]
    pub fn l1_cache_preload_rgid(&self) -> L1_CACHE_PRELOAD_RGID_R {
        L1_CACHE_PRELOAD_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_PRELOAD_CTRL")
            .field("l1_cache_preload_ena", &self.l1_cache_preload_ena())
            .field("l1_cache_preload_done", &self.l1_cache_preload_done())
            .field("l1_cache_preload_order", &self.l1_cache_preload_order())
            .field("l1_cache_preload_rgid", &self.l1_cache_preload_rgid())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done.
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_preload_ena(
        &mut self,
    ) -> L1_CACHE_PRELOAD_ENA_W<L1_CACHE_PRELOAD_CTRL_SPEC> {
        L1_CACHE_PRELOAD_ENA_W::new(self, 0)
    }
    ///Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending.
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_preload_order(
        &mut self,
    ) -> L1_CACHE_PRELOAD_ORDER_W<L1_CACHE_PRELOAD_CTRL_SPEC> {
        L1_CACHE_PRELOAD_ORDER_W::new(self, 2)
    }
}
/**L1 Cache preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_preload_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_preload_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_preload_ctrl::R`](R) reader structure
impl crate::Readable for L1_CACHE_PRELOAD_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_preload_ctrl::W`](W) writer structure
impl crate::Writable for L1_CACHE_PRELOAD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_PRELOAD_CTRL to value 0x02
impl crate::Resettable for L1_CACHE_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
